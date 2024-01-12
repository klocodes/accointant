use thiserror::Error;

pub mod client;
pub mod network;
pub mod server;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Client Error: {0}")]
    Client(client::ClientErrors),

    #[error("Network Error: {0}")]
    Network(network::NetworkErrors),

    #[error("Server Error: {0}")]
    Server(server::ServerErrors),
}