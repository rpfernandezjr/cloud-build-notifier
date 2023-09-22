pub mod auth;
pub mod logging;
pub mod pubsub;
pub mod secret_manager;
pub mod storage;
pub mod project;

#[derive(Debug)]
pub enum Error {
    GCPAuthToken(String),
    GCPSecretManager(String),
    GCPRequest(String),
    Serialize(String),
}
