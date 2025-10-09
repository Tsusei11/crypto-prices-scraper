use ::exchange::{Binance, ByBit, KuCoin};
use scrapper_engine::engine::Engine;

#[tokio::main]
async fn main() {

    let mut engine = Engine::new();
    engine.connect_to(Binance::new()).await;
    engine.connect_to(ByBit::new()).await;
    engine.connect_to(KuCoin::new()).await;

    let rx = Engine::get_orderbooks_receiver(engine.exchanges).await;

    Engine::save_bars_1min(rx).await.expect("Error saving 1 min bars");
}
