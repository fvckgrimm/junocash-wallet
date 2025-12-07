use crate::rpc::call_rpc;
use serde_json::{json, Value};
use tauri::command;

#[command]
pub async fn get_block_subsidy(
    height: u64,
    port: u16,
    user: String,
    pass: String,
) -> Result<f64, String> {
    let params = vec![json!(height)];
    let res = call_rpc("getblocksubsidy", params, port, &user, &pass).await?;

    res["miner"]
        .as_f64()
        .ok_or_else(|| "Failed to parse miner subsidy".to_string())
}

#[command]
pub async fn get_mining_info(port: u16, user: String, pass: String) -> Result<Value, String> {
    // Returns { "localsolps": 12.5, "networksolps": 120000, "difficulty": ... }
    call_rpc("getmininginfo", vec![], port, &user, &pass).await
}

#[command]
pub async fn set_mining(
    enabled: bool,
    threads: i32,
    port: u16,
    user: String,
    pass: String,
) -> Result<String, String> {
    // Params for setgenerate: [bool generate, int genproclimit]
    let params = vec![json!(enabled), json!(threads)];
    call_rpc("setgenerate", params, port, &user, &pass).await?;
    Ok(if enabled {
        "Mining Started"
    } else {
        "Mining Stopped"
    }
    .to_string())
}

#[command]
pub async fn shield_coinbase(
    to_address: String,
    port: u16,
    user: String,
    pass: String,
) -> Result<String, String> {
    // z_shieldcoinbase params: from ("*"), to_address
    let params = vec![json!("*"), json!(to_address)];
    let res = call_rpc("z_shieldcoinbase", params, port, &user, &pass).await?;

    // Result contains an operationid
    let op_id = res["operationid"].as_str().unwrap_or("");
    Ok(op_id.to_string())
}
