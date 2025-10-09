use std::collections::HashMap;
use futures_util::stream::SplitStream;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use crate::structs::OLHC;

mod utils;
pub mod engine;
mod structs;

type ReadStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type MapOLHC = HashMap<String, HashMap<String, OLHC>>;
