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
        if let Some(data) = raw_data.get("data") {

            if let Some(data_map) = data.as_object() {

                if let (Some(raw_s), Some(bids), Some(asks)) =
                    (data_map.get("s"), data_map.get("b"), data_map.get("a")) {

                    if let (Some(symbol), Some(bids), Some(asks))
                        = (raw_s.as_str(), bids.as_array(), asks.as_array()) {

                        if let (Some(bids), Some(asks)) = (bids.get(0), asks.get(0)) {

                            if let (Some(bids), Some(asks)) = (bids.as_array(), asks.as_array()) {

                                if let (Some(bids), Some(asks)) = (bids.get(0), asks.get(0)) {

                                    if let (Some(bid), Some(ask)) = (bids.as_str(), asks.as_str()) {

                                        return Some(Orderbook::new(
                                            Self::name(),
                                            symbol,
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

    fn new(read_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>, write_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) -> Self {
        Self { read_stream, write_stream }
    }
}