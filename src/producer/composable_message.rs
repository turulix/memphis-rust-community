use async_nats::HeaderMap;
use async_nats::header::{IntoHeaderName, IntoHeaderValue};
use bytes::Bytes;

#[derive(Debug, Default)]
pub struct ComposableMessage {
    pub(crate) headers: HeaderMap,
    pub(crate) payload: Bytes,
    pub(crate) msg_id: Option<String>,
}

impl ComposableMessage {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_header(mut self, name: impl IntoHeaderName, value: impl IntoHeaderValue) -> Self {
        self.headers.insert(name, value);

        self
    }

    pub fn with_payload(mut self, payload: impl Into<Bytes>) -> Self {
        self.payload = payload.into();

        self
    }

    pub fn with_msg_id(mut self, msg_id: impl Into<String>) -> Self {
        self.msg_id = Some(msg_id.into());

        self
    }
}


