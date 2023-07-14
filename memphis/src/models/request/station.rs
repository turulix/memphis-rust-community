use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateStationRequest<'a> {
    pub name: &'a str,
    pub retention_type: &'a str,
    pub retention_value: u32,
    pub storage_type: &'a str,
    pub replicas: u32,
    pub idempotency_window_in_ms: u32,
    pub schema_name: &'a str,
    pub dls_configuration: DlsConfiguration,
    pub username: &'a str,
    pub tiered_storage_enabled: bool,
}

#[derive(Debug, Serialize)]
pub struct DlsConfiguration {
    pub poison: bool,
    #[serde(rename = "Schemaverse")]
    pub schemaverse: bool,
}

#[derive(Debug, Serialize)]
pub struct DestroyStationRequest<'a> {
    pub station_name: &'a str,
    pub username: &'a str,
}
