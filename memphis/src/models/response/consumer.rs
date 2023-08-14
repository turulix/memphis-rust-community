use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CreateConsumerResponse {
    pub(crate) partitions_update: PartitionUpdate,
    pub(crate) error: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PartitionUpdate {
    pub(crate) partitions_list: Vec<u32>,
}
