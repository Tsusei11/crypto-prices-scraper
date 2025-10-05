use crate::utils::load_markets;

use std::collections::HashMap;
use std::sync::Arc;

use exchange::Exchange;
use exchange::traits::Connectable;
use exchange::structs::Orderbook;

use anyhow::{bail, Result};
use dotenv::dotenv;
use futures_util::lock::Mutex;
use futures_util::StreamExt;
use rustls::crypto::ring;
use serde_json::Value;
use tokio::sync::mpsc::unbounded_channel;
use tokio_tungstenite::tungstenite::Message;

pub struct Engine {
    pub exchanges: Vec<Box<dyn Exchange>>,
}

impl Engine {
    pub fn new() -> Self {
        ring::default_provider().install_default().unwrap();
        dotenv().ok();
        Self { exchanges: Vec::new() }
    }

    pub async fn connect_to(&mut self, mut exchange: impl Connectable + 'static) {
        exchange.connect_with_subscription_async(
            load_markets(exchange.name()).expect(format!("Error loading markets for {}", exchange.name()).as_str()),
        ).await.expect(format!("Error connecting to {}", exchange.name()).as_str());

        self.exchanges.push(Box::new(exchange));
    }

    pub async fn read_all_orderbooks(exchanges: Vec<Box<dyn Exchange>>) -> Result<()> {
        let (tx, mut rx) = unbounded_channel::<Orderbook>();

        for exchange in exchanges {
            let name = exchange.name();
            let exchange = Arc::new(Mutex::new(exchange));
            let tx = tx.clone();
            tokio::spawn(async move {
                println!("Started task for exchange {}", name);
                loop {
                    let data = Engine::read_orderbooks(exchange.clone())
                        .await
                        .expect(format!("Error reading orderbooks from {}", name).as_str());

                    if let Some(data) = data {
                        if let Some(orderbook) = exchange.lock().await.parse_orderbook_data(&data) {
                            tx.send(orderbook)
                                .expect(format!("Error sending orderbook from {}", name).as_str());
                        }
                    }
                }
            });
        }

        loop {
            if let Some(orderbook) = rx.recv().await {
                println!("{:?}", orderbook);
            }
        }
    }

    async fn read_orderbooks(exchange: Arc<Mutex<Box<dyn Exchange>>>) -> Result<Option<HashMap<String, Value>>> {
        if let Some(stream) = exchange.lock().await.read_stream() {
            if let Some(msg) = (*stream).next().await {
                match msg {
                    Ok(msg) => {
                        if let Message::Text(text) = msg {
                            let data = serde_json::from_str::<HashMap<String, Value>>(&text)?;

                            return Ok(Some(data));
                        }
                    }
                    Err(e) => {
                        bail!("Error receiving message: {}", e);
                    }
                }
            }
        } else {
            bail!("No read stream available for {}", exchange.lock().await.name());
        }

        Ok(None)
    }
}
