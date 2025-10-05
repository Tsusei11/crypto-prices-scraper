use ::exchange::{Binance, ByBit, KuCoin};
use scrapper_engine::engine::Engine;

#[tokio::main]
async fn main() {

    let mut engine = Engine::new();
    engine.connect_to(Binance::new()).await;
    engine.connect_to(ByBit::new()).await;
    engine.connect_to(KuCoin::new()).await;

    Engine::read_all_orderbooks(engine.exchanges).await.expect("Error reading all orderbooks");
}
