use crate::core::memphis_message::MemphisMessage;
use async_nats::Error;
use std::sync::Arc;

#[derive(Clone)]
pub enum MemphisEvent {
    MessageReceived(MemphisMessage),
    StationUnavailable(Arc<Error>),
    ConsumerUnavailable(Arc<Error>),
}
