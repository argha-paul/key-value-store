use thiserror::Error;

#[derive(Debug, Error)]
pub enum KvStoreError {
    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Invalid input")]
    InvalidInput,
}
