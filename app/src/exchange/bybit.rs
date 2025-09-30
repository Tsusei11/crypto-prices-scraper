use std::collections::HashMap;
use anyhow::Result;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use crate::exchange::Exchange;
use crate::exchange::traits::Orderbook;

pub struct ByBit {
    read_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    write_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>
}

impl Exchange for ByBit {
    fn name() -> &'static str {
        "ByBit"
    }

    fn url() -> &'static str {
        "wss://stream.bybit.com/v5/public/spot"
    }

    async fn connect_with_subscription_async(markets: Vec<String>) -> Result<Self> {
        let (ws_stream, _) = connect_async(Self::url()).await?;
        let (mut write_stream,
            read_stream) = ws_stream.split();

        let markets = markets
            .iter()
            .map(|m| format!("orderbook.1.{}", m.to_uppercase()))
            .collect::<Vec<String>>();

        let msg = json!({
            "op": "subscribe",
            "args": markets,
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
        let data = raw_data
            .get("data")?
            .as_object()?;

        let symbol = data
            .get("s")?
            .as_str()?;

        let bid = data
            .get("b")?
            .as_array()?
            .get(0)?
            .as_array()?
            .get(0)?
            .as_str()?;

        let ask = data
            .get("a")?
            .as_array()?
            .get(0)?
            .as_array()?
            .get(0)?
            .as_str()?;

        Some(
            Orderbook::new(
                Self::name(),
                symbol,
                bid,
                ask
            )
        )
    }

    fn read_stream(&mut self) -> &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        &mut self.read_stream
    }

    fn write_stream(&mut self) -> &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message> {
        &mut self.write_stream
    }

    fn new(read_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>, write_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) -> Self {
        Self { read_stream, write_stream }
    }
}