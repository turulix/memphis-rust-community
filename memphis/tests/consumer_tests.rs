mod common;

use common::*;
use memphis_rust_community::consumer::MemphisConsumerOptions;
use memphis_rust_community::producer::ComposableMessage;
use std::time::Duration;
use tokio_test::assert_ok;

#[tokio::test]
async fn create_consumer() {
    let _ = env_logger::try_init();

    let client = connect_to_memphis().await;
    let station = create_random_station(&client).await;
    let consumer1 = assert_ok!(
        station
            .create_consumer(MemphisConsumerOptions::new("no-group"))
            .await
    );

    let consumer2 = assert_ok!(
        station
            .create_consumer(
                MemphisConsumerOptions::new("a-name").with_consumer_group("group-name")
            )
            .await
    );

    let stream = match client
        .get_jetstream_context()
        .get_stream(station.get_internal_name(Some(1)))
        .await
    {
        Ok(s) => s,
        Err(_e) => {
            assert_ok!(
                client
                    .get_jetstream_context()
                    .get_stream(station.get_internal_name(None))
                    .await
            )
        }
    };

    let consumer1_info = assert_ok!(stream.consumer_info("no-group").await);
    assert!(consumer1_info.name.eq(&consumer1.get_internal_name()));

    let consumer2_info = assert_ok!(stream.consumer_info("group-name").await);
    assert!(consumer2_info.name.eq(&consumer2.get_internal_name()));
}

#[tokio::test]
async fn start_consume() {
    let _ = env_logger::try_init();

    let client = connect_to_memphis().await;
    let station = create_random_station(&client).await;
    let mut consumer1 = assert_ok!(
        station
            .create_consumer(MemphisConsumerOptions::new("no-group"))
            .await
    );
    assert_ok!(consumer1.consume().await);
}

#[tokio::test]
async fn destroy_consumer() {
    let _ = env_logger::try_init();

    let (_, _, mut consumer, mut producer) = create_random_setup().await;
    assert_ok!(
        producer
            .produce(ComposableMessage::new().with_payload("Works."))
            .await
    );

    let mut receiver = assert_ok!(consumer.consume().await);

    tokio::time::sleep(Duration::from_secs(2)).await;
    let msg = assert_ok!(receiver.try_recv());
    assert_ok!(msg.ack().await);

    assert_ok!(consumer.destroy().await);
    match receiver.recv().await {
        None => {}
        Some(_) => {
            panic!("This should be None.")
        }
    };
}
