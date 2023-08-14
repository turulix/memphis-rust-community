use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct CreateConsumerRequest<'a> {
    #[serde(rename = "name")]
    pub(crate) consumer_name: &'a str,

    #[serde(rename = "station_name")]
    pub(crate) station_name: &'a str,

    #[serde(rename = "connection_id")]
    pub(crate) connection_id: &'a str,

    #[serde(rename = "consumer_type")]
    pub(crate) consumer_type: &'a str,

    #[serde(rename = "consumers_group")]
    pub(crate) consumer_group: &'a str,

    #[serde(rename = "max_ack_time_ms")]
    pub(crate) max_ack_time_ms: i32,

    #[serde(rename = "max_msg_deliveries")]
    pub(crate) max_msg_deliveries: i32,

    #[serde(rename = "start_consume_from_sequence")]
    pub(crate) start_consume_from_sequence: i32,

    #[serde(rename = "last_messages")]
    pub(crate) last_messages: i32,

    #[serde(rename = "req_version")]
    pub(crate) req_version: u32,

    #[serde(rename = "username")]
    pub(crate) username: &'a str,
}

#[derive(Debug, Serialize)]
pub(crate) struct DestroyConsumerRequest<'a> {
    #[serde(rename = "name")]
    pub(crate) consumer_name: &'a str,

    #[serde(rename = "station_name")]
    pub(crate) station_name: &'a str,

    #[serde(rename = "connection_id")]
    pub(crate) connection_id: &'a str,

    #[serde(rename = "username")]
    pub(crate) username: &'a str,

    #[serde(rename = "req_version")]
    pub(crate) req_version: u32,
}
