use bigdecimal::BigDecimal;

// DTO for orderbook
#[derive(Debug)]
pub struct Orderbook {
    pub exchange: String,
    pub symbol: String,
    pub bid: BigDecimal,
    pub ask: BigDecimal,
}

impl Orderbook {
    pub fn new(exchange: &str, symbol: &str, bid: &str, ask: &str) -> Orderbook {
        Self {
            exchange: exchange.to_string(),
            symbol: symbol.to_string(),
            bid: bid.to_string().parse().unwrap_or(BigDecimal::from(-1)),
            ask: ask.to_string().parse().unwrap_or(BigDecimal::from(-1)),
        }
    }
}
