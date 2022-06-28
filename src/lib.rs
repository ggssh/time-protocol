#[macro_use]
extern crate lazy_static;

pub mod time_client;
pub mod time_server;
pub mod tool;

pub use time_client::Client;
pub use time_server::Server;