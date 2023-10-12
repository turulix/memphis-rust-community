use async_nats::header::{IntoHeaderName, IntoHeaderValue};
use async_nats::jetstream::context::Publish;
use async_nats::{HeaderMap, Request};
use bytes::Bytes;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct ComposableMessage {
    pub(crate) headers: HeaderMap,
    #[serde(serialize_with = "hex::serde::serialize")]
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

impl From<ComposableMessage> for Request {
    fn from(value: ComposableMessage) -> Self {
        Request::new().payload(value.payload).headers(value.headers)
    }
}

impl From<ComposableMessage> for Publish {
    fn from(value: ComposableMessage) -> Self {
        Publish::build()
            .payload(value.payload)
            .headers(value.headers)
    }
}
