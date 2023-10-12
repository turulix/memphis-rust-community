use crate::consumer::start_consumer;
use crate::producer::start_producer;
use crate::settings::Settings;
use log::info;
use memphis_rust_community::memphis_client::MemphisClient;
use memphis_rust_community::station::MemphisStationsOptions;
use memphis_rust_community::station::RetentionType::Messages;

mod consumer;
mod producer;
mod settings;
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::try_init()?;

    let settings = Settings::new()?;

    let memphis_client = MemphisClient::new(
        &settings.memphis_host,
        &settings.memphis_username,
        &settings.memphis_password,
        None, // Account ID is only required when using the cloud version of Memphis
    )
    .await?;

    let station_options = MemphisStationsOptions::new("logs")
        .with_retention_type(Messages)
        .with_retention_value(100);
    let station = memphis_client.create_station(station_options).await?;

    let consumer = start_consumer(&station).await?;
    let handle = start_producer(&station).await?;

    info!("Press CTRL-C to stop the application.");
    tokio::signal::ctrl_c().await?;
    handle.abort();
    consumer.stop();

    // Wait for the consumer to finish processing the last message.
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    Ok(())
}
