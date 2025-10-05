pub mod binance;
pub mod kucoin;
pub mod bybit;
pub mod structs;
pub mod traits;

pub use binance::Binance;
pub use kucoin::KuCoin;
pub use bybit::ByBit;
pub use traits::Exchange;

use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use futures_util::stream::{SplitSink, SplitStream};

type ReadStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type WriteStream = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;