use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct CreateProducerRequest<'a> {
    #[serde(rename = "name")]
    pub(crate) producer_name: &'a str,

    #[serde(rename = "station_name")]
    pub(crate) station_name: &'a str,

    pub(crate) producer_type: &'a str,

    #[serde(rename = "connection_id")]
    pub(crate) connection_id: &'a str,

    #[serde(rename = "req_version")]
    pub(crate) req_version: u32,

    #[serde(rename = "username")]
    pub(crate) username: &'a str,
}

#[derive(Debug, Serialize)]
pub(crate) struct DestroyProducerRequest<'a> {
    #[serde(rename = "name")]
    pub(crate) producer_name: &'a str,

    #[serde(rename = "station_name")]
    pub(crate) station_name: &'a str,

    #[serde(rename = "connection_id")]
    pub(crate) connection_id: &'a str,

    #[serde(rename = "username")]
    pub(crate) username: &'a str,

    #[serde(rename = "req_version")]
    pub(crate) req_version: u32,
}
