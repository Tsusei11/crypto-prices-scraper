use axum::serve;
use dotenv::dotenv;
use rustls::crypto::ring;
use tokio::net::TcpListener;
use ::exchange::exchanges::{Binance, ByBit, KuCoin};
use api::get_app;
use db::db::init_pool;
use scrapper_engine::engine::Engine;

#[tokio::main]
async fn main() {

    ring::default_provider().install_default().unwrap();
    dotenv().ok();

    let pool = init_pool();
    let (scraper_pool, api_pool) = (pool.clone(), pool.clone());
    // SCRAPER ENGINE
    tokio::spawn(async move {
        let engine = Engine::new()
            .add(Binance::new()).await
            .add(ByBit::new()).await
            .add(KuCoin::new()).await;

        Engine::save_bars_1min(
            Engine::get_orderbooks_receiver(engine.exchanges).await,
            scraper_pool,
        ).await.expect("Error saving 1 min bars");
    });

    // REST API
    let app = get_app(api_pool);
    let listener = TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Error creating TCP listener");

    println!("Starting api server at http://127.0.0.1:8000");
    serve(listener, app).await.expect("Error starting api server");
}
