use crate::utils::{load_markets, load_ping_interval};
use crate::{MapOLHC, ReadStream};
use crate::structs::OLHC;
use crate::engine::MessageType::Closed;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use exchange::Exchange;
use exchange::structs::Orderbook;

use anyhow::Result;
use futures_util::lock::Mutex;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::time::Instant;
use tokio_tungstenite::tungstenite::{Bytes, Message};
use db::db::DbPool;
use exchange::enums::AnyExchange;

enum MessageType {
    Data(HashMap<String, Value>),
    Ping(Bytes),
    Pong,
    Closed
}

pub struct Engine {
    pub exchanges: Vec<Box<dyn Exchange>>,
}

impl Engine {
    pub fn new() -> Self {

        Self {
            exchanges: Vec::new(),
        }
    }

    pub async fn add(mut self, mut exchange: impl Exchange + 'static) -> Self {
        Engine::connect_to(&mut exchange).await;
        self.exchanges.push(Box::new(exchange));

        self
    }

    async fn connect_to(exchange: &mut dyn Exchange) {
        let name = exchange.name();
        println!("Connecting to {}", name);
        AnyExchange::connect_orderbooks_async(
            exchange,
            load_markets(name).expect(format!("Error loading markets for {}", name).as_str()),
        ).await.expect(format!("Error connecting to {}", name).as_str());
    }

    pub async fn save_bars_1min(mut rx: UnboundedReceiver<Orderbook>, pool: DbPool) -> Result<()> {
        let map_olhc = Arc::new(Mutex::new(MapOLHC::new()));
        let writer_map = map_olhc.clone();
        let saver_map = map_olhc.clone();

        let writer = tokio::spawn(async move {
            loop {
                if let Some(orderbook) = rx.recv().await {
                    OLHC::update_map(writer_map.clone(), orderbook).await;
                }
            }
        });

        tokio::spawn(async move {
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

                let pool = pool.clone();

                tokio::task::spawn_blocking(move || {
                    OLHC::save_map(guard, &mut pool.get().expect("Error getting DB connection"))
                });

                println!("\nSaved data to the database\n");
            }
        });

        writer.await?;
        Ok(())
    }

    pub async fn get_orderbooks_receiver(exchanges: Vec<Box<dyn Exchange>>) -> UnboundedReceiver<Orderbook> {
        let (tx, rx) = unbounded_channel::<Orderbook>();

        let ping_interval = load_ping_interval().expect("Error loading ping interval");

        for mut exchange in exchanges {
            let name = exchange.name();

            let tx = tx.clone();

            tokio::spawn(async move {
                let mut start = Instant::now();
                loop {
                    let data = Engine::read_orderbooks(exchange.read_stream())
                        .await
                        .expect(format!("Error reading orderbooks from {}", name).as_str());

                    if let Some(data) = data {

                        match data {
                            MessageType::Data(data) => {
                                if let Some(orderbook) = exchange.parse_orderbook_data(&data) {
                                    tx.send(orderbook)
                                        .expect(format!("Error sending orderbook from {}", name).as_str());
                                }
                            },
                            MessageType::Ping(payload) => {
                                if let Some(write_stream) = exchange.write_stream().as_mut() {
                                    write_stream.send(Message::Pong(payload))
                                        .await
                                        .expect("Error responding to ping");
                                    println!("Responding to ping from {}", name);
                                }
                            },
                            MessageType::Pong => {
                                println!("Received pong from {}", name);
                            }
                            Closed => {
                                Self::connect_to(exchange.as_mut()).await;
                            }
                        }
                    }

                    if start.elapsed().as_secs() >= ping_interval {
                        if let Some(write_stream) = exchange.write_stream().as_mut() {
                            println!("Sending ping to {}", name);
                            write_stream.send(Message::Ping(Bytes::new())).await.expect("Error sending ping");
                        }
                        start = Instant::now();
                    }
                }
            });

        }

        rx
    }

    async fn read_orderbooks(r_stream: &mut Option<ReadStream>) -> Result<Option<MessageType>> {
        if let Some(r_stream) = r_stream {
            if let Some(msg) = (*r_stream).next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        let data = serde_json::from_str::<HashMap<String, Value>>(&text)?;

                        return Ok(Some(MessageType::Data(data)));
                    },
                    Ok(Message::Close(_)) => {
                        return Ok(Some(Closed));
                    },
                    Ok(Message::Ping(p)) => {
                        return Ok(Some(MessageType::Ping(p)));
                    },
                    Ok(Message::Pong(_)) => {
                        return Ok(Some(MessageType::Pong));
                    }
                    Err(e) => {
                        println!("Error receiving message: {}", e);
                        return Ok(Some(Closed));
                    },
                    _ => {
                        println!("Received unexpected message from the server");
                    }
                }
            }
        }

        Ok(None)
    }
}
