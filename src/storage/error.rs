use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DataStorageError {
    #[error("Unknown error occured: {0}")]
    Unknown(String),
    #[error("Not found")]
    NotFound
}
