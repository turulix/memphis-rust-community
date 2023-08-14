use serde::Deserialize;

#[derive(Deserialize)]
pub struct SchemaUpdateResponse {
    #[serde(rename = "UpdateType")]
    pub update_type: i64,
    pub init: Init,
}

#[derive(Deserialize)]
pub struct Init {
    pub schema_name: String,
    pub active_version: ActiveVersion,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Deserialize)]
pub struct ActiveVersion {
    pub version_number: i64,
    pub descriptor: String,
    pub schema_content: String,
    pub message_struct_name: String,
}
