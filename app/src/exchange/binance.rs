use std::collections::HashMap;
use crate::exchange::traits::{Connectable, Exchange};
use serde_json::Value;
use url::Url;
use anyhow::Result;
use crate::exchange::structs::Orderbook;
use crate::{ReadStream, WriteStream};

pub struct Binance {
    read_stream: Option<ReadStream>,
    write_stream: Option<WriteStream>
}

impl Binance {
    pub fn new() -> Self {
        Self {
            read_stream: None,
            write_stream: None
        }
    }
}

impl Connectable for Binance {}

impl Exchange for Binance {
    fn name(&self) -> &'static str {
        "Binance"
    }

    fn url(&self) -> &'static str {
        "wss://stream.binance.com/stream"
    }

    fn orderbook_subscription_url(&self, markets: Vec<String>) -> Result<Url> {
        let url = Url::parse_with_params(
            self.url(),
            &[("streams", markets.join("@bookTicker/") + "@bookTicker")]
        )?;

        Ok(url)
    }

    fn parse_orderbook_data(&self, raw_data: &HashMap<String, Value>) -> Option<Orderbook> {
        let data = raw_data
            .get("data")?
            .as_object()?;

        let symbol = data
            .get("s")?
            .as_str()?;

        let bid = data
            .get("b")?
            .as_str()?;

        let ask = data
            .get("a")?
            .as_str()?;

        Some(
            Orderbook::new(
                self.name(),
                symbol,
                bid,
                ask
            )
        )
    }

    fn read_stream(&mut self) -> &mut Option<ReadStream>{
        &mut self.read_stream
    }

    fn write_stream(&mut self) -> &mut Option<WriteStream> {
        &mut self.write_stream
    }

    fn set_read_stream(&mut self, stream: ReadStream) {
        self.read_stream = Some(stream);
    }

    fn set_write_stream(&mut self, stream: WriteStream) {
        self.write_stream = Some(stream);
    }
}
