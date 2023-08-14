use std::fmt::Debug;
use std::path::Path;

use bytes::Bytes;
use thiserror::Error;

use crate::RequestError;

#[async_trait::async_trait]
pub trait SchemaValidator: Send + Sync {
    fn validate(&self, message: &Bytes) -> Result<(), SchemaValidationError>;

    fn from_bytes(bytes: &Bytes) -> Result<Self, SchemaValidationError>
    where
        Self: Sized;

    async fn from_file(path: &Path) -> Result<Self, SchemaValidationError>
    where
        Self: Sized,
    {
        let bytes = tokio::fs::read(path)
            .await
            .map_err(SchemaValidationError::ReadFileError)?;
        Self::from_bytes(&bytes.into())
    }
}

pub trait ErrorData: Debug + Send + Sync {}

#[derive(Debug, Error)]
pub enum SchemaValidationError {
    #[error("Request error: {0}")]
    RequestError(#[from] RequestError),

    #[error("Could not read file: {0}")]
    ReadFileError(std::io::Error),

    #[error("Schema invalid: {0:?}")]
    SchemaInvalid(Box<dyn ErrorData>),
}

impl<E: ErrorData + 'static> From<E> for SchemaValidationError {
    fn from(value: E) -> Self {
        SchemaValidationError::SchemaInvalid(Box::new(value))
    }
}

#[cfg(feature = "validator_json")]
pub mod json;
