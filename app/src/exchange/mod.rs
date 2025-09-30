pub mod binance;
pub mod kucoin;
pub mod bybit;

pub use binance::Binance;
pub use traits::Exchange;

pub mod traits {
    use tokio::net::TcpStream;
    use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
    use anyhow::Result;
    use futures_util::stream::{SplitSink, SplitStream};
    use futures_util::StreamExt;
    use tokio_tungstenite::tungstenite::Message;

    // DTO for orderbook
    pub struct Orderbook {
        pub exchange: String,
        pub symbol: String,
        pub bid: f64,
        pub ask: f64,
    }

    // Trait for
    pub trait Exchange: Sized {
        
        // Returns the name of an exchange
        fn name() -> &'static str;
        
        // Returns the url of exchange's websocket stream
        fn url() -> &'static str;

        // Return url with orderbook subscription for given markets
        fn orderbook_subscription_url(markets: Vec<String>) -> String;

        fn new(
            read_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
            write_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>
        ) -> Self;

        // Returns an instance of an exchange with opened websocket connection with certain subscription
        async fn connect_with_subscription_async(subscription: String) -> Result<Self> {
            let (ws_stream, _) = connect_async(subscription).await?;

            let (write_stream,
                read_stream) = ws_stream.split();

            Ok(
                Self::new(read_stream, write_stream)
            )
        }

        // Returns a vector of orderbooks parsed from websocket server message
        async fn parse_orderbook_data(data: String) -> Result<Vec<Orderbook>>;

        // Getters and setters for r/w streams fields
        fn read_stream(&mut self) -> &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

        fn write_stream(&mut self) -> &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

        fn set_read_stream(&mut self, stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>);

        fn set_write_stream(&mut self, stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>);
    }
}