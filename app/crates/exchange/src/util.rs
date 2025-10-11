use std::collections::HashMap;
use anyhow::bail;
use serde_json::Value;

// Returns the token required for Websocket to establish a Spot/Margin connection
pub async fn get_public_token_kucoin() -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let response = client.post("https://api.kucoin.com/api/v1/bullet-public")
        .send()
        .await?
        .text()
        .await?;

    let map = serde_json::from_str::<HashMap<String, Value>>(&response)?;

    if let Some(data) = map.get("data") {

        if let Some(data_map) = data.as_object() {

            if let Some(token) = data_map.get("token") {
                return Ok(token.as_str().unwrap().to_string());
            }
        }
    }

    bail!("No token found in the response");
}