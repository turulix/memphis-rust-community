use tokio_test::assert_ok;

use memphis_rust_community::consumer::{MemphisConsumer, MemphisConsumerOptions};
use memphis_rust_community::memphis_client::MemphisClient;
use memphis_rust_community::producer::{MemphisProducer, MemphisProducerOptions};
use memphis_rust_community::station::{MemphisStation, MemphisStationsOptions, StorageType};

#[allow(dead_code)]
pub async fn connect_to_memphis() -> MemphisClient {
    pretty_env_logger::init_timed();
    let client = MemphisClient::new("localhost:6666", "root", "memphis").await;

    assert_ok!(&client, "Connecting to Memphis should be possible.");

    client.unwrap()
}

#[allow(dead_code)]
pub async fn create_random_station(client: &MemphisClient) -> MemphisStation {
    let random_station_name = uuid::Uuid::new_v4().to_string();
    eprintln!("random_station_name: {}", random_station_name);

    let station_options = MemphisStationsOptions::new(&random_station_name).with_storage_type(StorageType::Memory);

    let station = client.create_station(station_options).await;

    assert_ok!(&station, "Creating Station should be possible.");

    station.unwrap()
}

#[allow(dead_code)]
pub async fn create_random_consumer(station: &MemphisStation) -> MemphisConsumer {
    let random_consumer_name = uuid::Uuid::new_v4().to_string();

    let consumer_options = MemphisConsumerOptions::new(&random_consumer_name)
        .with_max_ack_time_ms(5000)
        .with_max_msg_deliveries(2);

    let consumer = station.create_consumer(consumer_options).await;

    assert_ok!(&consumer, "Creating Consumer should be possible.");

    consumer.unwrap()
}

#[allow(dead_code)]
pub async fn create_random_producer(station: &MemphisStation) -> MemphisProducer {
    let random_producer_name = uuid::Uuid::new_v4().to_string();

    let producer_options = MemphisProducerOptions::new(&random_producer_name);

    let producer = station.create_producer(producer_options).await;

    assert_ok!(&producer, "Creating Producer should be possible.");

    producer.unwrap()
}

#[allow(dead_code)]
pub async fn create_random_setup() -> (MemphisClient, MemphisStation, MemphisConsumer, MemphisProducer) {
    let client = connect_to_memphis().await;
    let station = create_random_station(&client).await;
    let consumer = create_random_consumer(&station).await;
    let producer = create_random_producer(&station).await;

    (client, station, consumer, producer)
}
