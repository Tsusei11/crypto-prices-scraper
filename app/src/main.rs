mod exchange;
mod utils;

use std::error::Error;
use dotenv::dotenv;
use futures_util::StreamExt;
use rustls::crypto::ring;
use tokio_tungstenite::tungstenite::Message;
use crate::exchange::{Binance, Exchange};
use crate::utils::load_markets;

#[tokio::main]
async fn main() {
    ring::default_provider().install_default().unwrap();
    dotenv().ok();

    let mut binance = Binance::connect_with_subscription_async(
        Binance::orderbook_subscription_url(load_markets().unwrap()),
    ).await.unwrap();

    loop {
        let stream = binance.read_stream();
        if let Some(msg) = (*stream).next().await {
            match msg {
                Ok(msg) => {
                    if let Message::Text(text) = msg {
                        println!("{}", text);
                    }
                },
                Err(e) => {
                    println!("Error receiving message: {}", e);
                }
            }
        }
    }
}
