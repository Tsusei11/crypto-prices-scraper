use crate::structs::Orderbook;
use crate::{ReadStream, WriteStream};
use crate::enums::AnyExchange;

use std::collections::HashMap;
use serde_json::Value;

pub trait Exchange: Send {

    // Returns the name of an exchange
    fn name(&self) -> &'static str;

    // Returns the url of exchange's websocket stream
    fn url(&self) -> &'static str;

    // Returns a vector of orderbooks parsed from websocket server message
    fn get_type(&self) -> &AnyExchange;

    fn parse_orderbook_data(&self, raw_data: &HashMap<String, Value>) -> Option<Orderbook>;

    // Getters and setters for r/w streams fields
    fn read_stream(&mut self) -> &mut Option<ReadStream>;

    fn write_stream(&mut self) -> &mut Option<WriteStream>;

    fn set_read_stream(&mut self, stream: ReadStream);

    fn set_write_stream(&mut self, stream: WriteStream);
}