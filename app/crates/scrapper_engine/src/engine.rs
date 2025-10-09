use crate::utils::load_markets;
use crate::{MapOLHC, ReadStream};
use crate::structs::OLHC;

use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;
use std::time::Duration;
use exchange::Exchange;
use exchange::traits::Connectable;
use exchange::structs::Orderbook;

use anyhow::{bail, Result};
use dotenv::dotenv;
use futures_util::lock::Mutex;
use futures_util::StreamExt;
use rustls::crypto::ring;
use serde_json::Value;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
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

    pub async fn save_bars_1min(mut rx: UnboundedReceiver<Orderbook>) -> Result<()> {
        let map_olhc = Arc::new(Mutex::new(MapOLHC::new()));
        let writer_map = map_olhc.clone();
        let saver_map = map_olhc.clone();

        let writer = tokio::spawn(async move {
            loop {
                if let Some(orderbook) = rx.recv().await {
                    println!("Got orderbook {:#?}", orderbook);
                    OLHC::update_map(writer_map.clone(), orderbook).await;
                }
            }
        });

        let saver = tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(60));
            ticker.tick().await;
            loop {
                ticker.tick().await;

                let guard = {
                    let mut guard = saver_map.lock().await;
                    let data = guard.clone();

                    guard.clear();
                    data
                };

                tokio::task::spawn_blocking(move || {
                    OLHC::save_map(guard)
                });

                println!("\nSaved data to the database\n");
            }
        });

        tokio::try_join!(writer, saver)?;
        Ok(())
    }

    pub async fn get_orderbooks_receiver(exchanges: Vec<Box<dyn Exchange>>) -> UnboundedReceiver<Orderbook> {
        let (tx, rx) = unbounded_channel::<Orderbook>();

        for mut exchange in exchanges {
            let name = exchange.name();

            let tx = tx.clone();
            tokio::spawn(async move {
                loop {
                    let data = Engine::read_orderbooks(exchange.read_stream())
                        .await
                        .expect(format!("Error reading orderbooks from {}", name).as_str());

                    if let Some(data) = data {
                        if let Some(orderbook) = exchange.parse_orderbook_data(&data) {
                            tx.send(orderbook)
                                .expect(format!("Error sending orderbook from {}", name).as_str());
                        }
                    }
                }
            });
        }

        rx
    }

    async fn read_orderbooks(stream: &mut Option<ReadStream>) -> Result<Option<HashMap<String, Value>>> {
        if let Some(stream) = stream {
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
        }

        Ok(None)
    }
}
