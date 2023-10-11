use anyhow::Error;
use log::error;
use memphis_rust_community::producer::{ComposableMessage, MemphisProducerOptions, ProducerError};
use memphis_rust_community::station::MemphisStation;
use serde::{Deserialize, Serialize};
use std::time::Duration;

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
}

pub async fn start_producer(station: &MemphisStation) -> Result<(), Error> {
    let producer_options = MemphisProducerOptions::new("amazing-service");
    let mut producer = station.create_producer(producer_options).await?;

    tokio::spawn(async move {
        let mut counter = 0;
        loop {
            tokio::time::sleep(Duration::from_secs(10)).await;
            let message = LogData {
                level: LogLevel::Info,
                message: format!("Something incredible happened we counted to: {}", counter),
            };
            let json_message = match serde_json::to_string(&message) {
                Ok(x) => x,
                Err(_) => {
                    error!("Error while serializing message: {:?}", message);
                    continue;
                }
            };
            let composable_message = ComposableMessage::new().with_payload(json_message);
            if let Err(e) = producer.produce(composable_message).await {
                error!("Error while producing message: {:?}", e);
            }
            counter += 1;
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    Ok(())
}
