use log::info;
use memphis_rust_community::consumer::{MemphisConsumerOptions, MemphisMessage};
use memphis_rust_community::producer::{ComposableMessage, MemphisProducerOptions};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::Mutex;
use tokio_test::{assert_err, assert_ok};

mod common;

use common::*;
use memphis_rust_community::station::MemphisStationsOptions;
use memphis_rust_community::station::StorageType::Memory;

#[tokio::test]
async fn send_receive_message() {
    let _ = env_logger::try_init();

    let client = connect_to_memphis().await;
    let station = create_random_station(&client).await;

    let consumer = create_random_consumer(&station).await;
    let mut receiver = consumer.consume().await.unwrap();

    let mut producer = create_random_producer(&station).await;
    let payload = "Hello World!";

    let res = producer
        .produce(
            ComposableMessage::new()
                .with_payload(payload)
                .with_header("TestHeader", "TestValue"),
        )
        .await;
    assert_ok!(res, "Sending a Message should be possible.");

    let msg = receiver.recv().await.unwrap();
    assert_eq!(msg.get_data_as_string().unwrap().as_str(), payload);
    assert_eq!(
        msg.get_headers()
            .clone()
            .unwrap()
            .get("TestHeader")
            .unwrap()
            .as_str(),
        "TestValue"
    );
    msg.ack().await.unwrap();
}

#[tokio::test]
async fn message_resend_test() {
    let _ = env_logger::try_init();

    let client = connect_to_memphis().await;
    let station = create_random_station(&client).await;

    let consumer = create_random_consumer(&station).await;
    let mut receiver = consumer.consume().await.unwrap();
    let mut dls_receiver = consumer.consume_dls().await.unwrap();

    let dls_received = Arc::new(Mutex::new(false));
    let dls_received_clone = dls_received.clone();

    let handle = tokio::spawn(async move {
        dls_receiver.recv().await.unwrap();
        eprintln!("Received Message from DLS");
        *dls_received_clone.lock().await = true;
    });

    let mut producer = create_random_producer(&station).await;
    let payload = "This should be send twice!";

    let res = producer
        .produce(ComposableMessage::new().with_payload(payload))
        .await;
    assert_ok!(res, "Sending a Message should be possible.");

    let msg = receiver.recv().await.unwrap();
    msg.disable_missed_ack_safety().await;
    assert_eq!(msg.get_data_as_string().unwrap().as_str(), payload);

    let msg = receiver.recv().await.unwrap();
    msg.disable_missed_ack_safety().await;

    assert_eq!(msg.get_data_as_string().unwrap().as_str(), payload);
    msg.ack().await.unwrap();

    tokio::time::sleep(Duration::from_secs(7)).await;
    assert!(
        !*dls_received.lock().await,
        "The Message should not be received by the DLS"
    );
    handle.abort();
}

#[tokio::test]
async fn message_delay_test() {
    let _ = env_logger::try_init();

    let (_, _station, consumer, mut producer) = create_random_setup().await;

    let payload = "This should be delayed!";

    producer
        .produce(ComposableMessage::new().with_payload(payload))
        .await
        .unwrap();

    let mut receiver = consumer.consume().await.unwrap();
    let msg = receiver.recv().await.unwrap();
    msg.disable_missed_ack_safety().await;
    assert_ok!(
        msg.delay(Duration::from_secs(15)).await,
        "Delaying a Message should be possible."
    );
    tokio::time::sleep(Duration::from_secs(7)).await;
    let msg = receiver.try_recv();
    match msg {
        Ok(_) => {
            panic!("Received Event should be an Error Event. Got a Message");
        }
        Err(e) => {
            assert_eq!(e, TryRecvError::Empty);
        }
    }
    tokio::time::sleep(Duration::from_secs(10)).await;
    let msg = receiver.try_recv();
    match msg {
        Ok(m) => {
            assert_eq!(m.get_data_as_string().unwrap().as_str(), payload);
            m.ack().await.unwrap();
        }
        Err(e) => {
            panic!(
                "Received Event should be an Error Event. Got an Error: {:?}",
                e
            )
        }
    }
}

#[tokio::test]
async fn max_messages_test_one_partition() {
    let _ = env_logger::try_init();

    let station = create_random_station(&connect_to_memphis().await).await;
    let mut producer = create_random_producer(&station).await;

    let consumer_options = MemphisConsumerOptions::new("consumer")
        .with_max_msg_deliveries(2)
        .with_max_ack_time(Duration::from_secs(120));

    let consumer = station.create_consumer(consumer_options).await.unwrap();

    let now = std::time::Instant::now();

    let message_count = 100000;

    for i in 0..message_count {
        assert_ok!(
            producer
                .produce(
                    ComposableMessage::new()
                        .with_payload(format!("Message {}", i))
                        .with_header("id", format!("{}", i).as_str()),
                )
                .await
        );
    }
    info!(
        "Sending {} Messages took: {:?}",
        message_count,
        now.elapsed()
    );

    let mut receiver = consumer.consume().await.unwrap();

    let now = std::time::Instant::now();
    let mut counter = 0;
    while let Some(msg) = receiver.recv().await {
        assert_eq!(
            msg.get_data_as_string().unwrap().as_str(),
            format!("Message {}", &counter)
        );
        counter += 1;
        msg.ack().await.unwrap();
        if msg
            .get_headers()
            .clone()
            .unwrap()
            .get("id")
            .unwrap()
            .as_str()
            == format!("{}", message_count - 1)
        {
            break;
        }
    }
    if counter != message_count {
        panic!(
            "Not all Messages were received. Only {} of {}",
            counter, message_count
        );
    }
    info!(
        "Receiving {} Messages took: {:?}",
        message_count,
        now.elapsed()
    );
}

#[tokio::test]
async fn max_messages_test_10_partitions() {
    let _ = env_logger::try_init();
    let random_station_name = uuid::Uuid::new_v4().to_string();

    eprintln!("random_station_name: {}", random_station_name);

    let client = connect_to_memphis().await;
    let station = assert_ok!(
        client
            .create_station(
                MemphisStationsOptions::new(&random_station_name)
                    .with_storage_type(Memory)
                    .with_partition_number(10)
            )
            .await
    );

    let consumer_options = MemphisConsumerOptions::new("consumer")
        .with_max_msg_deliveries(2)
        .with_max_ack_time(Duration::from_secs(120));

    let consumer = station.create_consumer(consumer_options).await.unwrap();

    let mut producer = assert_ok!(
        station
            .create_producer(MemphisProducerOptions::new("producer"))
            .await
    );

    let now = std::time::Instant::now();

    let message_count = 100000;

    for i in 0..message_count {
        assert_ok!(
            producer
                .produce(
                    ComposableMessage::new()
                        .with_payload(format!("Message {}", i))
                        .with_header("id", format!("{}", i).as_str()),
                )
                .await
        );
    }
    info!(
        "Sending {} Messages took: {:?}",
        message_count,
        now.elapsed()
    );

    let mut receiver = consumer.consume().await.unwrap();

    let now = std::time::Instant::now();
    let mut vec: Vec<MemphisMessage> = Vec::with_capacity(message_count);
    while let Some(msg) = receiver.recv().await {
        msg.ack().await.unwrap();
        vec.push(msg);
        if vec.len() == message_count {
            break;
        }
    }
    info!(
        "Receiving {} Messages took: {:?}",
        message_count,
        now.elapsed()
    );
}

#[tokio::test]
async fn partition_sending_receiving() {
    let _ = env_logger::try_init();
    let random_station_name = uuid::Uuid::new_v4().to_string();

    eprintln!("random_station_name: {}", random_station_name);

    let client = connect_to_memphis().await;
    let station = assert_ok!(
        client
            .create_station(
                MemphisStationsOptions::new(&random_station_name)
                    .with_storage_type(Memory)
                    .with_partition_number(10)
            )
            .await
    );

    let consumer = assert_ok!(
        station
            .create_consumer(MemphisConsumerOptions::new("consumer"))
            .await
    );

    let mut receiver = assert_ok!(consumer.consume().await);

    let mut producer = assert_ok!(
        station
            .create_producer(MemphisProducerOptions::new("producer"))
            .await
    );

    let payload = "Some message";
    for x in 0..20 {
        let res = producer
            .produce(
                ComposableMessage::new()
                    .with_payload(payload)
                    .with_header("partition", format!("{}", x).as_str()),
            )
            .await;
        assert_ok!(res, "Sending a Message should be possible.");
    }

    tokio::time::sleep(Duration::from_secs(10)).await;

    for _ in 0..20 {
        assert_ok!(receiver.try_recv());
    }

    assert_err!(receiver.try_recv());
}

#[tokio::test]
async fn no_message_duplication() {
    let _ = env_logger::try_init();
    let (_, _station, consumer, mut producer) = create_random_setup().await;

    let mut receiver = consumer.consume().await.unwrap();

    let payload = "Some message";
    assert_ok!(
        producer
            .produce(ComposableMessage::new().with_payload(payload))
            .await
    );

    tokio::time::sleep(Duration::from_secs(1)).await;

    let msg = assert_ok!(receiver.try_recv());
    tokio::time::sleep(Duration::from_secs(7)).await;
    assert_err!(receiver.try_recv());
    assert_eq!(msg.get_data_as_string().unwrap().as_str(), payload);
}

//TODO: Test for Messages in DLS once Memphis automatically resends them.
