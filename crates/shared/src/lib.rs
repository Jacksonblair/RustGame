pub mod arg_parser;
pub mod game_events;
use std::net::SocketAddr;

pub use game_events::GameEvents;

pub const PROTOCOL_ID: u64 = 7;
pub const SERVER_ADDR: &str = "127.0.0.1:5000";
