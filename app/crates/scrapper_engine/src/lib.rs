use std::collections::HashMap;
use futures_util::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use crate::structs::OLHC;

mod utils;
pub mod engine;
mod structs;

type ReadStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type WriteStream = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type MapOLHC = HashMap<String, HashMap<String, OLHC>>;
