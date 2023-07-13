use crate::request_error::RequestError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProducerError {
    #[error("RequestError: {0}")]
    RequestError(#[from] RequestError),

    #[error("The payload is empty.")]
    PayloadEmpty,
}
