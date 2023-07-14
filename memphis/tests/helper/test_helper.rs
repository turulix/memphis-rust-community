use memphis_rust_community::consumer::{MemphisConsumer, MemphisConsumerOptions};
use memphis_rust_community::memphis_client::MemphisClient;
use memphis_rust_community::producer::{MemphisProducer, MemphisProducerOptions};
use memphis_rust_community::station::{MemphisStation, MemphisStationsOptions, StorageType};

pub async fn connect_to_memphis() -> MemphisClient {
    let client = MemphisClient::new("localhost:6666", "root", "memphis").await;
    assert!(client.is_ok(), "Connecting to Memphis should be possible.");
    client.unwrap()
}

pub async fn create_random_station(client: &MemphisClient) -> MemphisStation {
    let random_station_name = uuid::Uuid::new_v4().to_string();

    let station_options = MemphisStationsOptions::new(&random_station_name).with_storage_type(StorageType::Memory);
    let station = client.create_station(station_options).await;
    assert!(station.is_ok(), "Creating Station should be possible.");
    station.unwrap()
}

pub async fn create_random_consumer(station: &MemphisStation) -> MemphisConsumer {
    let random_consumer_name = uuid::Uuid::new_v4().to_string();
    let consumer_options = MemphisConsumerOptions::new(&random_consumer_name)
        .with_max_ack_time_ms(5000)
        .with_max_msg_deliveries(2);

    let consumer = station.create_consumer(consumer_options).await;
    assert!(consumer.is_ok(), "Creating Consumer should be possible.");

    consumer.unwrap()
}

pub async fn create_random_producer(station: &MemphisStation) -> MemphisProducer {
    let random_producer_name = uuid::Uuid::new_v4().to_string();

    let producer_options = MemphisProducerOptions::new(&random_producer_name);
    let producer = station.create_producer(producer_options).await;
    assert!(producer.is_ok(), "Creating Producer should be possible.");

    producer.unwrap()
}
