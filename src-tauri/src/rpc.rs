use reqwest::Client;
use serde_json::{json, Value};

pub async fn call_rpc(
    method: &str,
    params: Vec<Value>,
    port: u16,
    user: &str,
    pass: &str,
) -> Result<Value, String> {
    let client = Client::new();
    let url = format!("http://127.0.0.1:{}", port);

    let response = client
        .post(&url)
        .basic_auth(user, Some(pass))
        .json(&json!({
            "jsonrpc": "1.0",
            "id": "tauri-client",
            "method": method,
            "params": params
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body: Value = response.json().await.map_err(|e| e.to_string())?;

    if !body["error"].is_null() {
        return Err(body["error"]["message"]
            .as_str()
            .unwrap_or("Unknown RPC error")
            .to_string());
    }

    Ok(body["result"].clone())
}
