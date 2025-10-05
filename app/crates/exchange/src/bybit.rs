use crate::Exchange;
use crate::structs::Orderbook;
use crate::{ReadStream, WriteStream};
use crate::traits::Connectable;

use std::collections::HashMap;

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;


pub struct ByBit {
    read_stream: Option<ReadStream>,
    write_stream: Option<WriteStream>
}

impl ByBit {
    pub fn new() -> Self {
        Self {
            read_stream: None,
            write_stream: None
        }
    }
}

impl Connectable for ByBit {
    async fn connect_with_subscription_async(&mut self, markets: Vec<String>) -> Result<()> {
        let (ws_stream, _) = connect_async(self.url()).await?;
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

        self.set_read_stream(read_stream);
        self.set_write_stream(write_stream);

        Ok(())
    }
}

impl Exchange for ByBit {
    fn name(&self) -> &'static str {
        "ByBit"
    }

    fn url(&self) -> &'static str {
        "wss://stream.bybit.com/v5/public/spot"
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
