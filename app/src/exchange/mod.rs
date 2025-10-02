pub mod binance;
pub mod kucoin;
pub mod bybit;
pub mod structs;

pub use binance::Binance;
pub use kucoin::KuCoin;
pub use bybit::ByBit;
pub use traits::Exchange;

pub mod traits {
    use std::collections::HashMap;
    use tokio_tungstenite::connect_async;
    use anyhow::{bail, Result};
    use futures_util::StreamExt;
    use serde_json::Value;
    use url::Url;
    use crate::exchange::structs::Orderbook;
    use crate::{ReadStream, WriteStream};
    
    pub trait Connectable: Exchange {
        // Returns an instance of an exchange with opened websocket connection with certain subscription
        async fn connect_with_subscription_async(&mut self, markets: Vec<String>) -> Result<()> {
            let subscription = &self.orderbook_subscription_url(markets)?;
            let (ws_stream, _) = connect_async(subscription.to_string()).await?;
            let (write_stream,
                read_stream) = ws_stream.split();

            self.set_read_stream(read_stream);
            self.set_write_stream(write_stream);

            Ok(())
        }
    }
    
    pub trait Exchange: Send {
        
        // Returns the name of an exchange
        fn name(&self) -> &'static str;
        
        // Returns the url of exchange's websocket stream
        fn url(&self) -> &'static str;

        // Return url with orderbook subscription for given markets
        fn orderbook_subscription_url(&self, _: Vec<String>) -> Result<Url> {
            bail!("No subscription url available for {}", self.name());
        }

        // Returns a vector of orderbooks parsed from websocket server message
        fn parse_orderbook_data(&self, raw_data: &HashMap<String, Value>) -> Option<Orderbook>;

        // Getters and setters for r/w streams fields
        fn read_stream(&mut self) -> &mut Option<ReadStream>;

        fn write_stream(&mut self) -> &mut Option<WriteStream>;

        fn set_read_stream(&mut self, stream: ReadStream);

        fn set_write_stream(&mut self, stream: WriteStream);
    }
}