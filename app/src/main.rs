use dotenv::dotenv;
use rustls::crypto::ring;
use ::exchange::exchanges::{Binance, ByBit, KuCoin};
use db::db::init_pool;
use scrapper_engine::engine::Engine;

#[tokio::main]
async fn main() {

    ring::default_provider().install_default().unwrap();
    dotenv().ok();

    let pool = init_pool();
    let engine = Engine::new()
        .add(Binance::new()).await
        .add(ByBit::new()).await
        .add(KuCoin::new()).await;

    Engine::save_bars_1min(
        Engine::get_orderbooks_receiver(engine.exchanges).await,
        pool
    ).await.expect("Error saving 1 min bars");
}
