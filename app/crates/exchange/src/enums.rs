use crate::{util, Exchange, ReadStream, WriteStream};

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

pub enum AnyExchange {
    Binance,
    ByBit,
    KuCoin
}

impl AnyExchange {
    pub async fn connect_orderbooks_async(exchange: &mut dyn Exchange, markets: Vec<String>) -> Result<()>{
        let res;
        
        match exchange.get_type() {
            AnyExchange::Binance => {
                res = Self::connect_orderbooks_binance_async(exchange, markets).await?;
            },
            AnyExchange::ByBit => {
                res = Self::connect_orderbooks_bybit_async(exchange, markets).await?;
            },
            AnyExchange::KuCoin => {
                res = Self::connect_orderbooks_kucoin_async(exchange, markets).await?;
            }
        }
        
        let (read_stream, write_stream) = res;
        exchange.set_read_stream(read_stream);
        exchange.set_write_stream(write_stream);
        
        Ok(())
    }
    
    async fn connect_orderbooks_binance_async(exchange: &mut dyn Exchange, markets: Vec<String>) -> Result<(ReadStream, WriteStream)> {
        let url = Url::parse_with_params(
            exchange.url(),
            &[("streams", markets.join("@bookTicker/") + "@bookTicker")]
        )?;
        
        let (ws_stream, _) = connect_async(url.to_string()).await?;
        let (write_stream,
            read_stream) = ws_stream.split();
        
        Ok((read_stream, write_stream))
    }

    async fn connect_orderbooks_bybit_async(exchange: &mut dyn Exchange, markets: Vec<String>) -> Result<(ReadStream, WriteStream)> {
        let (ws_stream, _) = connect_async(exchange.url()).await?;
        let (mut write_stream,
            read_stream) = ws_stream.split();

        let markets = markets
            .iter()
            .map(|m| format!("orderbook.1.{}", m.to_uppercase()))
            .collect::<Vec<String>>();

        let msg = json!({
            "op": "subscribe",
            "args": markets,
        });

        write_stream.send(Message::text(msg.to_string())).await?;
        
        Ok((read_stream, write_stream))
    }

    async fn connect_orderbooks_kucoin_async(exchange: &mut dyn Exchange, markets: Vec<String>) -> Result<(ReadStream, WriteStream)> {
        let token = util::get_public_token_kucoin().await?;
        let url = Url::parse_with_params(
            exchange.url(),
            &[("token", token)],
        )?;

        let (ws_stream, _) = connect_async(url.as_str()).await?;

        let (mut write_stream,
            read_stream) = ws_stream.split();

        let markets = markets
            .iter()
            .map(|m| {m.to_uppercase()})
            .collect::<Vec<String>>()
            .join(",");

        let msg = json!({
            "id": 1,
            "type": "subscribe",
            "topic": format!("/spotMarket/level1:{}", markets),
            "response": true
        });

        write_stream.send(Message::text(msg.to_string())).await?;
        
        Ok((read_stream, write_stream))
    }
}