mod exchange;
mod utils;

use std::collections::HashMap;
use dotenv::dotenv;
use futures_util::StreamExt;
use rustls::crypto::ring;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use crate::exchange::{Exchange, ByBit, KuCoin, Binance};
use crate::utils::load_markets;

#[tokio::main]
async fn main() {
    ring::default_provider().install_default().unwrap();
    dotenv().ok();

    let mut exchange = ByBit::connect_with_subscription_async(
        load_markets(ByBit::name()).expect("Unable to load markets")
    ).await.expect("Error connecting to exchange");

    loop {
        let stream = exchange.read_stream();
        if let Some(msg) = (*stream).next().await {
            match msg {
                Ok(msg) => {
                    if let Message::Text(text) = msg {
                        match serde_json::from_str::<HashMap<String, Value>>(&text) {
                            Ok(data) => {
                                if let Some(orderbook) = ByBit::parse_orderbook_data(&data) {
                                    println!("{:?}", orderbook);
                                }
                            },
                            Err(e) => {
                                eprintln!("Error parsing orderbook data: {}", e);
                            }
                        }
                        // println!("{}", text)
                    }
                },
                Err(e) => {
                    println!("Error receiving message: {}", e);
                }
            }
        }
    }
}
