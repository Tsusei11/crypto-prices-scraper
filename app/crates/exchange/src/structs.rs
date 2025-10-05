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
