pub mod structs;
pub mod traits;
pub mod enums;
pub mod exchanges;
mod util;

pub use exchanges::binance::Binance;
pub use exchanges::kucoin::KuCoin;
pub use exchanges::bybit::ByBit;
pub use traits::Exchange;

use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use futures_util::stream::{SplitSink, SplitStream};

type ReadStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type WriteStream = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;