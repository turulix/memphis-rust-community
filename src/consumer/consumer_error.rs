use crate::request_error::RequestError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConsumerError {
    #[error("RequestError: {0}")]
    RequestError(#[from] RequestError),

    #[error("InvalidSequence")]
    InvalidSequence,
}
