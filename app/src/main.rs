mod exchange;
mod utils;
mod engine;

use dotenv::dotenv;
use futures_util::stream::{SplitSink, SplitStream};
use rustls::crypto::ring;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use crate::exchange::{ByBit, KuCoin, Binance};
use crate::engine::Engine;

type ReadStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type WriteStream = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

#[tokio::main]
async fn main() {
    ring::default_provider().install_default().unwrap();
    dotenv().ok();

    let mut engine = Engine::new();

    engine.connect_to(Binance::new()).await;
    engine.connect_to(ByBit::new()).await;
    engine.connect_to(KuCoin::new()).await;

    Engine::read_all_orderbooks(engine.exchanges).await;
}
