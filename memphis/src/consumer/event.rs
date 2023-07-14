use std::sync::Arc;

use async_nats::Error;

use crate::consumer::MemphisMessage;

#[derive(Clone)]
pub enum MemphisEvent {
    MessageReceived(MemphisMessage),
    StationUnavailable(Arc<Error>),
    ConsumerUnavailable(Arc<Error>),
}
