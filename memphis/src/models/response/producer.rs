use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CreateProducerResponse {
    pub(crate) partitions_update: Option<PartitionUpdate>,
    pub(crate) schema_update: SchemaUpdate,
    pub(crate) error: String,
    pub(crate) schemaverse_to_dls: bool,
    pub(crate) send_notification: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SchemaUpdate {
    pub(crate) schema_name: String,
    pub(crate) active_version: ActiveVersion,
    #[serde(rename = "type")]
    pub(crate) type_name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ActiveVersion {
    pub(crate) version_number: u32,
    pub(crate) descriptor: String,
    pub(crate) schema_content: String,
    pub(crate) message_struct_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PartitionUpdate {
    pub(crate) partitions_list: Vec<u32>,
}
