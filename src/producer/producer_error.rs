use crate::request_error::RequestError;
use thiserror::Error;
use crate::schemaverse::schema::SchemaValidationError;

#[derive(Error, Debug)]
pub enum ProducerError {
    #[error("RequestError: {0}")]
    RequestError(#[from] RequestError),

    #[error("The payload is empty.")]
    PayloadEmpty,

    #[error("SchemaValidationError: {0}")]
    SchemaValidationError(SchemaValidationError),
}
