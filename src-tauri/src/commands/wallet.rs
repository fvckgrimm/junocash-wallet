use crate::rpc::call_rpc;
use serde_json::{json, Value};
use tauri::command;

// --- READ OPERATIONS ---

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

// --- WRITE OPERATIONS ---

#[command]
pub async fn send_transaction(
    from_address: String,
    to_address: String,
    amount: f64,
    memo: Option<String>,
    port: u16,
    user: String,
    pass: String,
) -> Result<String, String> {
    // Construct the amounts array for z_sendmany
    // z_sendmany "fromaddress" [{"address":..., "amount":...}, ...]

    let mut target = json!({
        "address": to_address,
        "amount": amount
    });

    // If sending to a shielded address, we can add a memo (hex encoded)
    if let Some(m) = memo {
        if !m.is_empty() {
            // Simple hex encoding for the memo
            let hex_memo = hex::encode(m);
            target
                .as_object_mut()
                .unwrap()
                .insert("memo".to_string(), json!(hex_memo));
        }
    }

    let params = vec![
        json!(from_address), // From (use "*" to autopick from wallet)
        json!([target]),     // Recipients list
        json!(1),            // minconf (1 confirmation required)
        json!(0.0001),       // fee (standard Juno/Zcash fee)
    ];

    let res = call_rpc("z_sendmany", params, port, &user, &pass).await?;

    // Returns the operation ID (e.g. "opid-1234...")
    // You must poll z_getoperationresult to see if it succeeded,
    // but returning the ID is enough for the frontend to show "Sending..."
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
