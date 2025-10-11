use crate::Exchange;
use crate::structs::Orderbook;
use crate::{ReadStream, WriteStream};

use std::collections::HashMap;

use serde_json::Value;
use crate::enums::AnyExchange;

pub struct ByBit {
    read_stream: Option<ReadStream>,
    write_stream: Option<WriteStream>,
    exchange_type: AnyExchange,
}

impl ByBit {
    pub fn new() -> Self {
        Self {
            read_stream: None,
            write_stream: None,
            exchange_type: AnyExchange::ByBit
        }
    }
}

impl Exchange for ByBit {
    fn name(&self) -> &'static str {
        "ByBit"
    }

    fn url(&self) -> &'static str {
        "wss://stream.bybit.com/v5/public/spot"
    }

    fn get_type(&self) -> &AnyExchange {
        &self.exchange_type
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
            .as_array()?
            .get(0)?
            .as_array()?
            .get(0)?
            .as_str()?;

        let ask = data
            .get("a")?
            .as_array()?
            .get(0)?
            .as_array()?
            .get(0)?
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
