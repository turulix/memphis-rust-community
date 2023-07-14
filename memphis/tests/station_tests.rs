use log::info;
use memphis_rust_community::station::{MemphisStationsOptions, StorageType};
mod common;
use common::*;

#[tokio::test]
async fn test_station_creation() {
    let random_station_name = uuid::Uuid::new_v4().to_string();

    let client = connect_to_memphis().await;

    let station_options = MemphisStationsOptions::new(&random_station_name).with_storage_type(StorageType::File);
    let station = client.create_station(station_options).await;
    assert!(station.is_ok(), "Creating Station should be possible.");
    dbg!(station.unwrap().get_name());

    let station_options = MemphisStationsOptions::new(&random_station_name).with_storage_type(StorageType::File);
    let station = client.create_station(station_options).await;
    assert!(station.is_ok(), "Creating Station with the Same Settings should be possible.");

    let station_options = MemphisStationsOptions::new(&random_station_name).with_storage_type(StorageType::Memory);
    let station = client.create_station(station_options).await;
    assert!(station.is_ok(), "Creating Station with different Settings should be possible.");
}
