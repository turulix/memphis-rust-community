use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CreateConsumerRequest {
    #[serde(rename = "name")]
    pub(crate) consumer_name: String,

    #[serde(rename = "station_name")]
    pub(crate) station_name: String,

    #[serde(rename = "connection_id")]
    pub(crate) connection_id: String,

    #[serde(rename = "consumer_type")]
    pub(crate) consumer_type: String,

    #[serde(rename = "consumers_group")]
    pub(crate) consumer_group: String,

    #[serde(rename = "max_ack_time_ms")]
    pub(crate) max_ack_time_ms: i32,

    #[serde(rename = "max_msg_deliveries")]
    pub(crate) max_msg_count_for_delivery: i32,

    #[serde(rename = "username")]
    pub(crate) username: String,

    #[serde(rename = "start_consume_from_sequence")]
    pub(crate) start_consume_from_sequence: i32,

    #[serde(rename = "last_messages")]
    pub(crate) last_messages: i32,
}
