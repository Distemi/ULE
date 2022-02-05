mod handler;
mod network_client;
pub mod proto;
mod server;

pub use server::network_server_start;
pub use server::{NET_SERVER_WORKS, SHUTDOWN_SERVER};
