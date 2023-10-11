use crate::consumer::start_consumer;
use crate::settings::Settings;
use log::info;
use memphis_rust_community::memphis_client::MemphisClient;
use memphis_rust_community::station::MemphisStationsOptions;

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

    let station_options = MemphisStationsOptions::new("logs");
    let station = memphis_client.create_station(station_options).await?;

    start_consumer(&station).await?;

    info!("Press CTRL-C to stop the application.");
    tokio::signal::ctrl_c().await?;

    Ok(())
}
