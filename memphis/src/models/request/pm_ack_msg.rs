use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PmAckMsg {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "cg_name")]
    pub consumer_group_name: String,
}
