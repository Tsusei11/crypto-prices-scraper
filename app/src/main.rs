mod exchange;
mod utils;
mod engine;

use dotenv::dotenv;
use futures_util::stream::{SplitSink, SplitStream};
use pprof::ProfilerGuard;
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
    let guard = ProfilerGuard::new(50).unwrap();

    let mut engine = Engine::new();

    engine.connect_to(Binance::new()).await;
    engine.connect_to(ByBit::new()).await;
    engine.connect_to(KuCoin::new()).await;

    let read_task = tokio::spawn(async move {
        Engine::read_all_orderbooks(engine.exchanges)
        .await
    });

    let profiler_task = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        if let Ok(report) = guard.report().build() {
            let mut file = std::fs::File::create("report.svg").unwrap();
            report.flamegraph(&mut file).unwrap();
        }
    });

    tokio::try_join!(read_task, profiler_task).expect("Error joining tasks");
}
