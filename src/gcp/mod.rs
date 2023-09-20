pub mod auth;
pub mod pubsub;
pub mod secret_manager;
pub mod storage;

#[derive(Debug)]
pub enum Error {
    GCPAuthToken(String),
    GCPSecretManager(String),
}
