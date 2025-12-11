use crate::rpc::call_rpc;
use crate::state::NodeState;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{command, State};

#[command]
pub async fn get_network_info(
    host: String,
    port: u16,
    user: String,
    pass: String,
) -> Result<Value, String> {
    call_rpc("getnetworkinfo", vec![], &host, port, &user, &pass).await
}

#[command]
pub async fn get_peer_info(
    host: String,
    port: u16,
    user: String,
    pass: String,
) -> Result<Value, String> {
    call_rpc("getpeerinfo", vec![], &host, port, &user, &pass).await
}

#[command]
pub fn get_default_juno_paths() -> serde_json::Value {
    let home = dirs::home_dir().unwrap_or(PathBuf::from("/"));
    let (default_bin, default_data) = if cfg!(target_os = "windows") {
        (
            "junocashd.exe".to_string(),
            dirs::data_dir()
                .unwrap_or(home.clone())
                .join("JunoCash")
                .to_string_lossy()
                .to_string(),
        )
    } else if cfg!(target_os = "macos") {
        (
            "/usr/local/bin/junocashd".to_string(),
            home.join("Library/Application Support/JunoCash")
                .to_string_lossy()
                .to_string(),
        )
    } else {
        (
            "/usr/bin/junocashd".to_string(),
            home.join(".junocash").to_string_lossy().to_string(),
        )
    };
    serde_json::json!({
        "binary": default_bin,
        "data_dir": default_data
    })
}

fn resolve_data_dir(input: String) -> PathBuf {
    if !input.trim().is_empty() {
        return PathBuf::from(input);
    }

    // Default Paths Logic
    let home = dirs::home_dir().unwrap_or(PathBuf::from("/"));

    if cfg!(target_os = "windows") {
        dirs::data_dir().unwrap_or(home).join("JunoCash")
    } else if cfg!(target_os = "macos") {
        home.join("Library/Application Support/JunoCash")
    } else {
        // Linux / Unix
        home.join(".junocash")
    }
}

#[command]
pub async fn launch_node(
    bin_path: String,
    data_dir: String,
    rpc_port: u16,
    rpc_user: String,
    rpc_pass: String,
    randomx_fast_mode: bool,
    donation_percent: u32,
    state: State<'_, NodeState>,
) -> Result<String, String> {
    let bin = PathBuf::from(&bin_path);
    if !bin.exists() {
        return Err(format!("Binary not found at: {}", bin_path));
    }

    let mut guard = state.process.lock().unwrap();
    if guard.is_some() {
        return Ok("Node is already running".into());
    }

    let mut cmd = Command::new(bin);

    // CHANGE: Only add the flag if data_dir is not empty
    if !data_dir.trim().is_empty() {
        cmd.arg(format!("-datadir={}", data_dir));
    }

    cmd.arg(format!("-rpcport={}", rpc_port))
        .arg(format!("-rpcuser={}", rpc_user))
        .arg(format!("-rpcpassword={}", rpc_pass))
        .arg("-daemon=0");

    if randomx_fast_mode {
        cmd.arg("-randomxfastmode");
    }

    cmd.arg(format!("-donationpercentage={}", donation_percent));

    let child = cmd.spawn().map_err(|e| format!("Failed to start: {}", e))?;
    *guard = Some(child);

    Ok("Node started".into())
}

#[command]
pub async fn stop_node(
    host: String,
    port: u16,
    user: String,
    pass: String,
    state: State<'_, NodeState>,
) -> Result<String, String> {
    let mut messages: Vec<String> = Vec::new();

    // 1. Try to stop via RPC (Handles external nodes)
    match call_rpc("stop", vec![], &host, port, &user, &pass).await {
        Ok(_) => messages.push("RPC stop command sent".to_string()),
        Err(e) => messages.push(format!("RPC stop failed (node might be down): {}", e)),
    }

    // 2. Kill the child process if we manage it (Handles frozen nodes/local child)
    let mut guard = state.process.lock().unwrap();
    if let Some(mut child) = guard.take() {
        // We give it a second to shut down from the RPC command above, then force kill
        std::thread::sleep(std::time::Duration::from_millis(1000));
        match child.kill() {
            Ok(_) => messages.push("Child process killed".to_string()),
            Err(e) => messages.push(format!("Failed to kill child: {}", e)),
        }
    }

    Ok(messages.join(". "))
}

#[command]
pub async fn backup_local_wallet(data_dir: String) -> Result<String, String> {
    // 1. Resolve the actual path (Handling empty/default case)
    let actual_dir = resolve_data_dir(data_dir);
    let wallet_path = actual_dir.join("wallet.dat");

    // 2. Case: Fresh Install / No Wallet
    if !wallet_path.exists() {
        return Ok("No existing wallet.dat found (skipping backup)".to_string());
    }

    // 3. Case: Existing Wallet -> Backup
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let backup_name = format!("wallet.dat.backup.{}", timestamp);
    let backup_path = actual_dir.join(&backup_name);

    fs::rename(&wallet_path, &backup_path)
        .map_err(|e| format!("Failed to rename wallet.dat: {}", e))?;

    Ok(format!("Backed up to {}", backup_name))
}

#[command]
pub async fn launch_recovery_node(
    bin_path: String,
    data_dir: String,
    rpc_port: u16,
    rpc_user: String,
    rpc_pass: String,
    state: State<'_, NodeState>,
) -> Result<String, String> {
    let bin = PathBuf::from(&bin_path);
    if !bin.exists() {
        return Err(format!("Binary not found at: {}", bin_path));
    }

    // Ensure any managed process is dead (double check)
    let mut guard = state.process.lock().unwrap();
    if let Some(mut child) = guard.take() {
        let _ = child.kill();
    }

    let mut cmd = Command::new(bin);

    // 1. Handle Data Dir Flag
    // If input is not empty, pass -datadir.
    // If empty, DO NOT pass it (let junocashd find its default).
    if !data_dir.trim().is_empty() {
        cmd.arg(format!("-datadir={}", data_dir));
    }

    // 2. Standard Flags
    cmd.arg(format!("-rpcport={}", rpc_port))
        .arg(format!("-rpcuser={}", rpc_user))
        .arg(format!("-rpcpassword={}", rpc_pass))
        .arg("-skipwalletinit") // <--- REQUIRED FOR RECOVERY
        .arg("-daemon=0");

    let child = cmd
        .spawn()
        .map_err(|e| format!("Failed to start recovery node: {}", e))?;
    *guard = Some(child);

    Ok("Recovery Node started".into())
}
