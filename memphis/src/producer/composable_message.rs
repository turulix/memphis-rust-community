use async_nats::header::{IntoHeaderName, IntoHeaderValue};
use async_nats::HeaderMap;
use bytes::Bytes;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

#[derive(Debug, Default, Serialize)]
pub struct ComposableMessage {
    #[serde(serialize_with = "serialize_headers")]
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

fn serialize_headers<S>(headers: &HeaderMap, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = s.serialize_map(None)?;
    for (k, v) in headers.iter() {
        map.serialize_entry(&format!("{:?}", k), v)?;
    }

    map.end()
}
