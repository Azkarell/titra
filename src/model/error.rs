use chrono::ParseError;
use thiserror::Error;

use crate::{export::ExportError, storage::error::DataStorageError};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ApplicationError {
    #[error("{0}")]
    Storage(DataStorageError),
    #[error("{0}")]
    Export(ExportError),
    #[error("{0}")]
    ChronoParseError(ParseError),
    #[error("{0}")]
    ChronoeTimezoneError(String),
    #[error("Ungülitige Start- und Endzeit")]
    InvalidRange,
    #[error("Still in edit")]
    InEdit,
}


impl From<ParseError> for ApplicationError {
    fn from(value: ParseError) -> Self {
        ApplicationError::ChronoParseError(value)
    }
}
impl From<DataStorageError> for ApplicationError {
    fn from(value: DataStorageError) -> Self {
        ApplicationError::Storage(value)
    }
}