use std::collections::HashMap;
use anyhow::bail;
use serde_json::{json, Value};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use crate::exchange::Exchange;
use crate::exchange::structs::Orderbook;
use crate::{ReadStream, WriteStream};
use crate::exchange::traits::Connectable;

pub struct KuCoin {
    read_stream: Option<ReadStream>,
    write_stream: Option<WriteStream>
}

impl KuCoin {
    
    pub(crate) fn new() -> Self {
        Self {
            read_stream: None,
            write_stream: None
        }
    }

    // Returns the token required for Websocket to establish a Spot/Margin connection
    async fn get_public_token() -> Result<String> {
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
}

impl Connectable for KuCoin {
    async fn connect_with_subscription_async(&mut self, markets: Vec<String>) -> Result<()> {
        let token = Self::get_public_token().await?;
        let url = Url::parse_with_params(
            self.url(),
            &[("token", token)],
        )?;

        let (ws_stream, _) = connect_async(url.as_str()).await?;

        let (mut write_stream,
            read_stream) = ws_stream.split();

        let markets = markets
            .iter()
            .map(|m| {m.to_uppercase()})
            .collect::<Vec<String>>()
            .join(",");

        let msg = json!({
            "id": 1,
            "type": "subscribe",
            "topic": format!("/spotMarket/level1:{}", markets),
            "response": true
        });

        write_stream.send(Message::text(msg.to_string())).await?;

        self.set_read_stream(read_stream);
        self.set_write_stream(write_stream);

        Ok(())
    }

}

impl Exchange for KuCoin {
    fn name(&self) -> &'static str {
        "KuCoin"
    }

    fn url(&self) -> &'static str {
        "wss://ws-api-spot.kucoin.com/"
    }
    
    fn parse_orderbook_data(&self, raw_data: &HashMap<String, Value>) -> Option<Orderbook> {
        let symbol = raw_data
            .get("topic")?
            .as_str()?
            .split(":")
            .collect::<Vec<&str>>()[1]
            .replace("-", "");

        let data = raw_data
            .get("data")?
            .as_object()?;

        let bid = data
            .get("bids")?
            .as_array()?
            .get(0)?
            .as_str()?;

        let ask = data
            .get("asks")?
            .as_array()?
            .get(0)?
            .as_str()?;

        Some(
            Orderbook::new(
                self.name(),
                symbol.as_str(),
                bid,
                ask
            )
        )
    }

    fn read_stream(&mut self) -> &mut Option<ReadStream>{
        &mut self.read_stream
    }

    fn write_stream(&mut self) -> &mut Option<WriteStream> {
        &mut self.write_stream
    }

    fn set_read_stream(&mut self, stream: ReadStream) {
        self.read_stream = Some(stream);
    }

    fn set_write_stream(&mut self, stream: WriteStream) {
        self.write_stream = Some(stream);
    }
}

