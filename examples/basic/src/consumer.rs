use crate::producer::{LogData, LogLevel};
use anyhow::Error;
use log::{error, info, warn};
use memphis_rust_community::consumer::{MemphisConsumer, MemphisConsumerOptions};
use memphis_rust_community::station::MemphisStation;
use std::time::Duration;

pub async fn start_consumer(station: &MemphisStation) -> Result<MemphisConsumer, Error> {
    let consumer_options = MemphisConsumerOptions::new("log-consumer")
        .with_consumer_group("log-consumer-group")
        .with_max_ack_time(Duration::from_millis(1000));
    let consumer = station.create_consumer(consumer_options).await?;

    // We need to map the Err here, since async_nats uses a Box Error type...
    let mut receiver = consumer.consume().await.map_err(|e| anyhow::anyhow!(e))?;

    tokio::spawn(async move {
        while let Some(msg) = receiver.recv().await {
            // Do something with the message here.
            let json_data = match msg.get_data_as_string() {
                Ok(x) => x,
                Err(e) => {
                    error!("Error while getting data as string: {:?}", e);
                    continue;
                }
            };
            let log_message: LogData = match serde_json::from_str(&json_data) {
                Ok(x) => x,
                Err(e) => {
                    error!("Error while deserializing message: {:?}", e);
                    continue;
                }
            };

            match log_message.level {
                LogLevel::Info => {
                    info!("({}) {}", log_message.date, log_message.message)
                }
                LogLevel::Warning => {
                    warn!("({}) {}", log_message.date, log_message.message)
                }
                LogLevel::Error => {
                    error!("({}) {}", log_message.date, log_message.message)
                }
            }

            if let Err(e) = msg.ack().await {
                error!("Error while acknowledging message: {:?}", e);
            }
        }
        info!("Gracefully stopped consumer.")
    });

    Ok(consumer)
}
