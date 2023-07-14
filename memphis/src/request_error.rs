use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("NatsError: {0}")]
    NatsError(async_nats::Error),

    #[error("MemphisError: {0}")]
    MemphisError(String),

    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Tried to send request whilst not connected")]
    NotConnected,
}
