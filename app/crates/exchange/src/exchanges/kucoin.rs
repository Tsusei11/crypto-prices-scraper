use crate::Exchange;
use crate::structs::Orderbook;
use crate::{ReadStream, WriteStream};

use std::collections::HashMap;

use serde_json::Value;
use crate::enums::AnyExchange;

pub struct KuCoin {
    read_stream: Option<ReadStream>,
    write_stream: Option<WriteStream>,
    exchange_type: AnyExchange,
}

impl KuCoin {
    
    pub fn new() -> Self {
        Self {
            read_stream: None,
            write_stream: None,
            exchange_type: AnyExchange::KuCoin,
        }
    }
}

impl Exchange for KuCoin {
    fn name(&self) -> &'static str {
        "KuCoin"
    }

    fn url(&self) -> &'static str {
        "wss://ws-api-spot.kucoin.com/"
    }

    fn get_type(&self) -> &AnyExchange {
        &self.exchange_type
    }

    fn parse_orderbook_data(&self, raw_data: &HashMap<String, Value>) -> Option<Orderbook> {
        let symbol = raw_data
            .get("topic")?
            .as_str()?
            .split(":")
            .collect::<Vec<&str>>()[1]
            .replace("-", "");

        let data = raw_data
            .get("data")?
            .as_object()?;

        let bid = data
            .get("bids")?
            .as_array()?
            .get(0)?
            .as_str()?;

        let ask = data
            .get("asks")?
            .as_array()?
            .get(0)?
            .as_str()?;

        Some(
            Orderbook::new(
                self.name(),
                symbol.as_str(),
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

