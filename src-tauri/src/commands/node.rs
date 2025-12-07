use crate::state::NodeState; // Import the struct
use std::path::PathBuf;
use std::process::Command;
use tauri::{command, State}; // Import State

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

#[command]
pub async fn launch_node(
    bin_path: String,
    data_dir: String,
    rpc_port: u16,
    rpc_user: String,
    rpc_pass: String,
    state: State<'_, NodeState>, // <--- Inject State
) -> Result<String, String> {
    let bin = PathBuf::from(&bin_path);
    if !bin.exists() {
        return Err(format!("Binary not found at: {}", bin_path));
    }

    // Check if we already have a handle (optional safety)
    let mut guard = state.process.lock().unwrap();
    if guard.is_some() {
        return Ok("Node is already running (managed by wallet)".into());
    }

    let child = Command::new(bin)
        .arg(format!("-datadir={}", data_dir))
        .arg(format!("-rpcport={}", rpc_port))
        .arg(format!("-rpcuser={}", rpc_user))
        .arg(format!("-rpcpassword={}", rpc_pass))
        .arg("-daemon=0")
        .spawn()
        .map_err(|e| format!("Failed to start: {}", e))?;

    // Store the process handle so we can kill it later
    *guard = Some(child);

    Ok("Node started".into())
}

#[command]
pub async fn stop_node(state: State<'_, NodeState>) -> Result<String, String> {
    let mut guard = state.process.lock().unwrap();

    if let Some(mut child) = guard.take() {
        // Kill the process
        child
            .kill()
            .map_err(|e| format!("Failed to kill node: {}", e))?;
        Ok("Node stopped".into())
    } else {
        Ok("Node was not running".into())
    }
}
