use crate::rpc::call_rpc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use tauri::command;
use zeroize::Zeroizing;

#[derive(Serialize)]
pub struct SpendableAddress {
    address: String,
    balance: f64,
    addr_type: String, // "transparent", "sapling", "unified"
}

#[derive(Deserialize)]
pub struct TxTarget {
    address: String,
    amount: f64,
    memo: Option<String>,
}

// --- READ OPERATIONS ---

#[command]
pub async fn get_seed_phrase(port: u16, user: String, pass: String) -> Result<String, String> {
    let res = call_rpc("z_getseedphrase", vec![], port, &user, &pass).await?;

    // The RPC returns a file path where the seed phrase is stored
    let file_path = res.as_str().ok_or_else(|| {
        "Failed to retrieve seed phrase path. Is the wallet encrypted/locked?".to_string()
    })?;

    // Clean up the path string
    let path_str = file_path.trim();

    // Zeroizing ensures that 'seed_wrapper' is wiped from memory when this function ends.
    let seed_wrapper = Zeroizing::new(
        std::fs::read_to_string(path_str)
            .map_err(|e| format!("Failed to read seed phrase file: {}", e))?,
    );
    // We attempt to remove it; if it fails, we log it but don't stop (we already have the seed)
    if let Err(e) = std::fs::remove_file(path_str) {
        println!(
            "SECURITY WARNING: Failed to delete seed file at {}: {}",
            path_str, e
        );
    }
    // Note: We must convert back to a standard String to pass it to the frontend (Tauri/Serde).
    // This creates a copy that is sent to the UI, but the 'seed_wrapper' staying in Rust RAM
    // will be zeroed out immediately after this line executes.
    Ok(seed_wrapper.to_string())
}

#[command]
pub async fn get_balance(port: u16, user: String, pass: String) -> Result<Value, String> {
    // z_gettotalbalance provides the most complete view (transparent + shielded)
    // Returns: { "total": 0.0, "transparent": 0.0, "private": 0.0 }
    call_rpc("z_gettotalbalance", vec![], port, &user, &pass).await
}

#[command]
pub async fn list_transactions(port: u16, user: String, pass: String) -> Result<Value, String> {
    // listtransactions accounts for transparent txs.
    // z_listreceivedbyaddress is needed for shielded.
    // For a simple UI, we might just query 'listtransactions' for now.
    // Params: account ("*"), count (20), skip (0)
    let params = vec![json!("*"), json!(50), json!(0)];
    call_rpc("listtransactions", params, port, &user, &pass).await
}

#[command]
pub async fn get_block_count(port: u16, user: String, pass: String) -> Result<u64, String> {
    // Call the "getblockcount" RPC method
    let res = call_rpc("getblockcount", vec![], port, &user, &pass).await?;

    // Attempt to parse the result as a number (u64)
    res.as_u64()
        .ok_or_else(|| "Failed to parse block count".to_string())
}

#[command]
pub async fn get_all_addresses(port: u16, user: String, pass: String) -> Result<Value, String> {
    // 'listaddresses' returns the hierarchical structure of the wallet
    call_rpc("listaddresses", vec![], port, &user, &pass).await
}

#[command]
pub async fn get_operation_status(port: u16, user: String, pass: String) -> Result<Value, String> {
    // z_getoperationresult returns the result AND removes it from the node's memory list
    call_rpc("z_getoperationresult", vec![], port, &user, &pass).await
}

#[command]
pub async fn get_blockchain_info(port: u16, user: String, pass: String) -> Result<Value, String> {
    // Returns detailed info including 'verificationprogress', 'blocks', 'headers'
    call_rpc("getblockchaininfo", vec![], port, &user, &pass).await
}

#[command]
pub async fn get_spendable_addresses(
    port: u16,
    user: String,
    pass: String,
) -> Result<Vec<SpendableAddress>, String> {
    let mut balance_map: HashMap<String, f64> = HashMap::new();
    let mut type_map: HashMap<String, String> = HashMap::new();

    // 1. Get Transparent UTXOs
    // listunspent 0 9999999
    let t_res = call_rpc("listunspent", vec![json!(0)], port, &user, &pass).await?;
    if let Some(utxos) = t_res.as_array() {
        for u in utxos {
            if let (Some(addr), Some(amount)) = (u["address"].as_str(), u["amount"].as_f64()) {
                *balance_map.entry(addr.to_string()).or_insert(0.0) += amount;
                type_map.insert(addr.to_string(), "transparent".to_string());
            }
        }
    }

    // 2. Get Shielded Notes
    // z_listunspent
    let z_res = call_rpc("z_listunspent", vec![], port, &user, &pass).await?;
    if let Some(notes) = z_res.as_array() {
        for n in notes {
            if let (Some(addr), Some(amount)) = (n["address"].as_str(), n["amount"].as_f64()) {
                *balance_map.entry(addr.to_string()).or_insert(0.0) += amount;

                // Determine type based on prefix
                let t = if addr.starts_with("zs") {
                    "sapling"
                } else if addr.starts_with("j1") {
                    "unified"
                } else {
                    "shielded"
                };
                type_map.insert(addr.to_string(), t.to_string());
            }
        }
    }

    // Convert to Vec
    let mut results = Vec::new();
    for (addr, bal) in balance_map {
        if bal > 0.0 {
            results.push(SpendableAddress {
                address: addr.clone(),
                balance: bal,
                addr_type: type_map
                    .get(&addr)
                    .unwrap_or(&"unknown".to_string())
                    .clone(),
            });
        }
    }

    Ok(results)
}

// --- WRITE OPERATIONS ---

#[command]
pub async fn send_transaction(
    from_address: Option<String>,
    targets: Vec<TxTarget>,
    port: u16,
    user: String,
    pass: String,
) -> Result<String, String> {
    let mut json_targets = Vec::new();
    for t in targets {
        let mut target_obj = json!({
            "address": t.address,
            "amount": t.amount
        });
        if let Some(m) = t.memo {
            if !m.is_empty() {
                target_obj
                    .as_object_mut()
                    .unwrap()
                    .insert("memo".to_string(), json!(hex::encode(m)));
            }
        }
        json_targets.push(target_obj);
    }

    // Default to "*" (Auto-select) if None provided
    let source = from_address.unwrap_or("*".to_string());

    let params = vec![
        json!(source),       // Source
        json!(json_targets), // Targets
        json!(1),            // minconf
        json!(0.0001),       // fee
    ];

    let res = call_rpc("z_sendmany", params, port, &user, &pass).await?;
    Ok(res.as_str().unwrap_or("").to_string())
}

#[command]
pub async fn get_new_address(
    type_param: Option<String>,
    port: u16,
    user: String,
    pass: String,
) -> Result<String, String> {
    // If they want a Unified Address (recommended for Juno), we might need to ensure an account exists.
    // However, z_getnewaddress "unified" is the standard generic way.
    let addr_type = type_param.unwrap_or("unified".to_string());

    let res = call_rpc(
        "z_getnewaddress",
        vec![json!(addr_type)],
        port,
        &user,
        &pass,
    )
    .await?;
    Ok(res.as_str().unwrap_or("").to_string())
}
