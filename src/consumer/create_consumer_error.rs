use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CreateConsumerError {
    NatsError(async_nats::RequestError),
    MemphisError(String),
    NotConnected,
    InvalidSequence,
}

impl Display for CreateConsumerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateConsumerError::NatsError(e) => write!(f, "NatsError: {}", e),
            CreateConsumerError::MemphisError(e) => write!(f, "MemphisError: {}", e),
            CreateConsumerError::NotConnected => write!(
                f,
                "Tried to create consumer without being connected to Memphis"
            ),
            CreateConsumerError::InvalidSequence => {
                write!(f, "start_consume_from_sequence has to be a positive number")
            }
        }
    }
}
