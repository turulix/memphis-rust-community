use log::debug;
use memphis_rust_community::station::{MemphisStationsOptions, StorageType};
use tokio_test::assert_ok;
mod common;
use common::*;

#[tokio::test]
async fn test_station_creation() {
    let random_station_name = uuid::Uuid::new_v4().to_string();
    debug!("random_station_name: {}", random_station_name);

    let client = connect_to_memphis().await;

    let station_options =
        MemphisStationsOptions::new(&random_station_name).with_storage_type(StorageType::File);
    let station = client.create_station(station_options).await;
    assert_ok!(&station, "Creating Station should be possible.");

    let station_options =
        MemphisStationsOptions::new(&random_station_name).with_storage_type(StorageType::File);
    let station = client.create_station(station_options).await;
    assert_ok!(
        station,
        "Creating Station with same Settings should be possible."
    );

    let station_options =
        MemphisStationsOptions::new(&random_station_name).with_storage_type(StorageType::Memory);
    let station = client.create_station(station_options).await;
    assert_ok!(
        station,
        "Creating Station with different Settings should be possible."
    );
}
