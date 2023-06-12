use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PmAckMsg {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "cg_name")]
    pub consumer_group_name: String,
}
