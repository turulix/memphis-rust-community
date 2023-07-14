use crate::request_error::RequestError;
use thiserror::Error;

#[cfg(feature = "schemaverse")]
use crate::schemaverse::schema::SchemaValidationError;

#[derive(Error, Debug)]
pub enum ProducerError {
    #[error("RequestError: {0}")]
    RequestError(#[from] RequestError),

    #[error("The payload is empty.")]
    PayloadEmpty,

    #[cfg(feature = "schemaverse")]
    #[error("SchemaValidationError: {0}")]
    SchemaValidationError(SchemaValidationError),
}
