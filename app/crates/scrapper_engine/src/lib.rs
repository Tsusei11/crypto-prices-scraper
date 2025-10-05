use futures_util::stream::SplitStream;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

mod utils;
pub mod engine;

type ReadStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

