use futures_util::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use crate::exchange::traits::{Exchange, Orderbook};
use anyhow::Result;
use tokio_tungstenite::tungstenite::Message;

pub struct Binance {
    read_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    write_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>
}

impl Exchange for Binance {
    fn name() -> &'static str {
        "Binance"
    }

    fn url() -> &'static str {
        "wss://stream.binance.com/"
    }

    fn orderbook_subscription_url(markets: Vec<String>) -> String {
        format!("{}stream?streams={}@bookTicker", Self::url(), markets.join("@bookTicker/"))
    }

    fn new(
        read_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        write_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>
    ) -> Self {
        Self { read_stream, write_stream }
    }

    async fn parse_orderbook_data(message: String) -> Result<Vec<Orderbook>> {
        todo!()
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
}