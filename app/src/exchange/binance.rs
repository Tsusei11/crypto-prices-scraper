use std::collections::HashMap;
use futures_util::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use crate::exchange::traits::{Exchange, Orderbook};
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use url::Url;
use anyhow::Result;

pub struct Binance {
    read_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    write_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>
}

impl Exchange for Binance {
    fn name() -> &'static str {
        "Binance"
    }

    fn url() -> &'static str {
        "wss://stream.binance.com/stream"
    }

    fn orderbook_subscription_url(markets: Vec<String>) -> Result<Url> {
        let url = Url::parse_with_params(
            Self::url(),
            &[("streams", markets.join("@bookTicker/") + "@bookTicker")]
        )?;

        Ok(url)
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
            .as_str()?;

        let ask = data
            .get("a")?
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

    fn new(
        read_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        write_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>
    ) -> Self {
        Self { read_stream, write_stream }
    }
}