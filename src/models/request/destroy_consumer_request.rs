use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DestroyConsumerRequest<'a> {
    #[serde(rename = "name")]
    pub(crate) consumer_name: &'a str,

    #[serde(rename = "station_name")]
    pub(crate) station_name: &'a str,

    #[serde(rename = "connection_id")]
    pub(crate) connection_id: &'a str,

    #[serde(rename = "username")]
    pub(crate) username: &'a str,
}
