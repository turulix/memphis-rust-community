use std::time::Duration;

use tokio_test::assert_ok;

use common::*;
use memphis_rust_community::producer::{ComposableMessage, MemphisProducerOptions};
use memphis_rust_community::station::{MemphisStationsOptions, StorageType};

mod common;

#[tokio::test]
async fn test_station_creation() {
    let random_station_name = uuid::Uuid::new_v4().to_string();
    eprintln!("random_station_name: {}", random_station_name);

    let client = connect_to_memphis().await;

    let station_options = MemphisStationsOptions::new(&random_station_name).with_storage_type(StorageType::File);
    let station = client.create_station(station_options).await;
    assert_ok!(&station, "Creating Station should be possible.");

    let station_options = MemphisStationsOptions::new(&random_station_name).with_storage_type(StorageType::File);
    let station = client.create_station(station_options).await;
    assert_ok!(station, "Creating Station with same Settings should be possible.");

    let station_options = MemphisStationsOptions::new(&random_station_name).with_storage_type(StorageType::Memory);
    let station = client.create_station(station_options).await;
    assert_ok!(station, "Creating Station with different Settings should be possible.");
}

#[tokio::test]
async fn aa() {
    let client = connect_to_memphis().await;

    let station = client.create_station(MemphisStationsOptions::new("test")).await.unwrap();

    tokio::time::sleep(Duration::from_secs(15)).await;

    let producer = station.create_producer(MemphisProducerOptions::new("chello")).await.unwrap();

    producer.produce(ComposableMessage::new().with_payload("hello")).await.unwrap();
}
