use thiserror::Error;
use crate::request_error::RequestError;

#[derive(Error, Debug)]
pub enum ConsumerError {
    #[error("RequestError: {0}")]
    RequestError(#[from] RequestError),

    #[error("InvalidSequence")]
    InvalidSequence,
}
