use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
pub type ArcStationSettings = Arc<StationSettings>;

#[cfg(feature = "schemaverse")]
use crate::schemaverse::schema::SchemaValidator;

pub struct StationSettings {
    pub schemaverse_dls_enabled: bool,

    #[cfg(feature = "schemaverse")]
    pub schema_validator: Option<Box<dyn SchemaValidator>>,
}

#[derive(Default)]
pub struct StationSettingsStore {
    settings: RwLock<HashMap<String, ArcStationSettings>>,
}

impl StationSettingsStore {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn get_settings(&self, station_name: &str) -> Option<ArcStationSettings> {
        self.settings.read().await.get(station_name).cloned()
    }

    pub async fn set_settings(&self, station_name: impl ToString, settings: StationSettings) {
        self.settings.write().await.insert(station_name.to_string(), Arc::new(settings));
    }
}
