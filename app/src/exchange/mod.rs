pub mod binance;
pub mod kucoin;
pub mod bybit;

pub use binance::Binance;
pub use kucoin::KuCoin;
pub use bybit::ByBit;
pub use traits::Exchange;

pub mod traits {
    use std::collections::HashMap;
    use tokio::net::TcpStream;
    use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
    use anyhow::{bail, Result};
    use futures_util::stream::{SplitSink, SplitStream};
    use futures_util::StreamExt;
    use serde_json::Value;
    use tokio_tungstenite::tungstenite::handshake::client::Request;
    use tokio_tungstenite::tungstenite::Message;
    use url::Url;

    // DTO for orderbook
    #[derive(Debug)]
    pub struct Orderbook {
        pub exchange: String,
        pub symbol: String,
        pub bid: f64,
        pub ask: f64,
    }
    
    impl Orderbook {
        pub fn new(exchange: &str, symbol: &str, bid: &str, ask: &str) -> Orderbook {
            Self {
                exchange: exchange.to_string(),
                symbol: symbol.to_string(),
                bid: bid.parse::<f64>().unwrap_or(-1.0),
                ask: ask.parse::<f64>().unwrap_or(-1.0),
            }
        }
    }

    // Trait for
    pub trait Exchange: Sized {
        
        // Returns the name of an exchange
        fn name() -> &'static str;
        
        // Returns the url of exchange's websocket stream
        fn url() -> &'static str;

        // Return url with orderbook subscription for given markets
        fn orderbook_subscription_url(_: Vec<String>) -> Result<Request> {
            bail!("No subscription url available for {}", Self::name());
        }

        // Returns an instance of an exchange with opened websocket connection with certain subscription
        async fn connect_with_subscription_async(markets: Vec<String>) -> Result<Self> {
            let subscription = Self::orderbook_subscription_url(markets)?;
            let (ws_stream, _) = connect_async(subscription).await?;
            let (write_stream,
                read_stream) = ws_stream.split();

            Ok(
                Self::new(read_stream, write_stream)
            )
        }

        // Returns a vector of orderbooks parsed from websocket server message
        fn parse_orderbook_data(raw_data: &HashMap<String, Value>) -> Option<Orderbook>;

        // Getters and setters for r/w streams fields
        fn read_stream(&mut self) -> &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

        fn write_stream(&mut self) -> &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

        fn set_read_stream(&mut self, stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>);

        fn set_write_stream(&mut self, stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>);
        
        // Constructor
        fn new(
            read_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
            write_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>
        ) -> Self;
    }
}