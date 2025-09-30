use std::collections::HashMap;
use futures_util::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use crate::exchange::traits::{Exchange, Orderbook};
use serde_json::Value;
use tokio_tungstenite::tungstenite::handshake::client::Request;
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

    fn orderbook_subscription_url(markets: Vec<String>) -> Result<Request> {
        let url = Url::parse_with_params(
            Self::url(),
            &[("streams", markets.join("@bookTicker/") + "@bookTicker")]
        )?;
        
        let request = Request::builder().uri(url.as_str()).body(())?;
        Ok(request)
    }

    fn parse_orderbook_data(raw_data: &HashMap<String, Value>) -> Option<Orderbook> {
        if let Some(data) = raw_data.get("data") {

            if let Some(map) = data.as_object(){

                if let (Some(raw_bid), Some(raw_ask), Some(raw_s)) = (map.get("b"), map.get("a"), map.get("s")) {

                    if let (Some(bid), Some(ask), Some(symbol)) =
                        (raw_bid.as_str(), raw_ask.as_str(), raw_s.as_str()) {

                        return Some(Orderbook::new(
                            Self::name(),
                            symbol,
                            bid,
                            ask
                        ));
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