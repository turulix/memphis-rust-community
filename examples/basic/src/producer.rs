use anyhow::Error;
use chrono::{DateTime, Utc};
use log::error;
use memphis_rust_community::producer::{ComposableMessage, MemphisProducerOptions};
use memphis_rust_community::station::MemphisStation;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::task::JoinHandle;

#[derive(Serialize, Deserialize, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogData {
    pub level: LogLevel,
    pub message: String,
    pub date: DateTime<Utc>,
}

pub async fn start_producer(station: &MemphisStation) -> Result<JoinHandle<()>, Error> {
    let producer_options =
        MemphisProducerOptions::new("amazing-service").with_generate_unique_suffix(true);
    let mut producer = station.create_producer(producer_options).await?;

    let handle = tokio::spawn(async move {
        let mut counter = 0;
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            // Some different types of messages.
            let level = if counter % 5 == 0 {
                LogLevel::Warning
            } else if counter % 3 == 0 {
                LogLevel::Error
            } else {
                LogLevel::Info
            };
            let message = LogData {
                level,
                message: format!("Something incredible happened we counted to: {}", counter),
                date: Utc::now(),
            };
            let json_message = match serde_json::to_string(&message) {
                Ok(x) => x,
                Err(_) => {
                    error!("Error while serializing message: {:?}", message);
                    continue;
                }
            };
            let composable_message = ComposableMessage::new()
                .with_payload(json_message)
                .with_msg_id(counter.to_string());
            match producer.produce(composable_message).await {
                Ok(x) => {
                    if let Err(e) = x.await {
                        error!("Error while awaiting ack: {:?}", e);
                        continue;
                    };
                }
                Err(e) => {
                    error!("Error while producing message: {:?}", e);
                    continue;
                }
            }
            counter += 1;
        }
    });

    Ok(handle)
}
