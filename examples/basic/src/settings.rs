use std::path::Path;

use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Settings {
    pub memphis_host: String,
    pub memphis_username: String,
    pub memphis_password: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            memphis_host: "localhost:6666".to_string(),
            memphis_username: "root".to_string(),
            memphis_password: "memphis".to_string(),
        }
    }
}

//noinspection DuplicatedCode
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        if !Path::new("settings.json").exists() {
            serde_json::to_writer_pretty(
                std::fs::File::create("settings.json").unwrap(),
                &Settings::default(),
            )
            .unwrap();
        }

        let builder = Config::builder()
            .add_source(File::from_str(
                &serde_json::to_string(&Settings::default()).unwrap(),
                config::FileFormat::Json,
            ))
            .add_source(File::with_name("settings.json"));

        let should_be_settings: Settings = builder.build_cloned()?.try_deserialize().unwrap();
        serde_json::to_writer_pretty(
            std::fs::File::create("settings.json").unwrap(),
            &should_be_settings,
        )
        .unwrap();

        let real_settings = builder
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(" "),
            )
            .build()?;

        real_settings.try_deserialize()
    }
}
