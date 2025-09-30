use std::collections::HashMap;
use anyhow::bail;
use futures_util::stream::{SplitSink, SplitStream};
use serde_json::{json, Value};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use url::Url;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use crate::exchange::Exchange;
use crate::exchange::traits::Orderbook;

pub struct KuCoin {
    read_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    write_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>
}

impl KuCoin {

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

impl Exchange for KuCoin {
    fn name() -> &'static str {
        "KuCoin"
    }

    fn url() -> &'static str {
        "wss://ws-api-spot.kucoin.com/"
    }

    async fn connect_with_subscription_async(markets: Vec<String>) -> Result<Self> {
        let token = Self::get_public_token().await?;
        let url = Url::parse_with_params(
            Self::url(),
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

        Ok(
            Self {
                read_stream,
                write_stream
            }
        )
    }

    fn parse_orderbook_data(raw_data: &HashMap<String, Value>) -> Option<Orderbook> {

        if let Some(topic) = raw_data.get("topic") {

            if let Some(symbol) = topic.as_str() {
                let symbol = symbol
                    .split(":")
                    .collect::<Vec<&str>>()[1]
                    .replace("-", "");

                if let Some(data) = raw_data.get("data") {

                    if let Some(data_map) = data.as_object() {

                        if let (Some(bids), Some(asks)) = (data_map.get("bids"), data_map.get("asks")) {

                            if let (Some(bids), Some(asks)) = (bids.as_array(), asks.as_array()) {

                                if let (Some(bid), Some(ask)) = (bids.get(0), asks.get(0)) {

                                    if let (Some(bid), Some(ask)) = (bid.as_str(), ask.as_str()) {

                                        return Some(Orderbook::new(
                                            Self::name(),
                                            symbol.as_str(),
                                            bid,
                                            ask
                                        ))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn read_stream(&mut self) -> &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        &mut self.read_stream
    }

    fn write_stream(&mut self) -> &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message> {
        &mut self.write_stream
    }

    fn set_read_stream(&mut self, stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) {
        self.read_stream = stream;
    }

    fn set_write_stream(&mut self, stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) {
        self.write_stream = stream;
    }

    fn new(
        read_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        write_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>
    ) -> Self {
        Self { read_stream, write_stream }
    }
}
