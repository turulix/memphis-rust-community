use std::sync::Arc;

use async_nats::jetstream::context::GetStreamError;
use async_nats::Error;

use crate::consumer::MemphisMessage;

#[derive(Clone, Debug)]
pub enum MemphisEvent {
    MessageReceived(MemphisMessage),
    StationUnavailable(Arc<GetStreamError>),
    ConsumerUnavailable(Arc<Error>),
}
