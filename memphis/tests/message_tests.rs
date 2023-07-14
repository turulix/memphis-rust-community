use log::debug;
use memphis_rust_community::consumer::MemphisEvent;
use memphis_rust_community::producer::ComposableMessage;
use std::sync::Arc;
use tokio::sync::Mutex;
mod common;

use common::*;

#[tokio::test]
async fn send_receive_message() {
    let client = connect_to_memphis().await;
    let station = create_random_station(&client).await;

    dbg!(station.get_name());

    let mut consumer = create_random_consumer(&station).await;
    let mut receiver = consumer.consume().await.unwrap();

    let producer = create_random_producer(&station).await;
    let payload = "Hello World!";

    let res = producer
        .produce(ComposableMessage::new().with_payload(payload).with_header("TestHeader", "TestValue"))
        .await;
    assert!(res.is_ok(), "Sending a Message should be possible.");

    let msg = receiver.recv().await.unwrap();
    match msg {
        MemphisEvent::MessageReceived(m) => {
            assert_eq!(m.get_data_as_string().unwrap().as_str(), payload);
            assert_eq!(m.get_headers().clone().unwrap().get("TestHeader").unwrap().as_str(), "TestValue");
            m.ack().await.unwrap();
        }
        _ => panic!("Received Event should be a MessageReceived Event."),
    }
}

#[tokio::test]
async fn message_resend_test() {
    let client = connect_to_memphis().await;
    let station = create_random_station(&client).await;

    dbg!(station.get_name());

    let mut consumer = create_random_consumer(&station).await;
    let mut receiver = consumer.consume().await.unwrap();
    let mut dls_receiver = consumer.consume_dls().await.unwrap();

    let dls_received = Arc::new(Mutex::new(false));
    let dls_received_clone = dls_received.clone();

    let handle = tokio::spawn(async move {
        dls_receiver.recv().await.unwrap();
        debug!("Received Message from DLS");
        *dls_received_clone.lock().await = true;
    });

    let producer = create_random_producer(&station).await;
    let payload = "This should be send twice!";

    let res = producer.produce(ComposableMessage::new().with_payload(payload)).await;

    assert!(res.is_ok(), "Sending a Message should be possible.");

    let msg = receiver.recv().await.unwrap();
    match msg {
        MemphisEvent::MessageReceived(m) => {
            assert_eq!(m.get_data_as_string().unwrap().as_str(), payload);
        }
        _ => panic!("Received Event should be a MessageReceived Event."),
    }
    let msg = receiver.recv().await.unwrap();
    match msg {
        MemphisEvent::MessageReceived(m) => {
            assert_eq!(m.get_data_as_string().unwrap().as_str(), payload);
            m.ack().await.unwrap();
        }
        _ => panic!("Received Event should be a MessageReceived Event."),
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;
    assert!(!*dls_received.lock().await, "The Message should not be received by the DLS");
    handle.abort();
}

//TODO: Test for Messages in DLS once Memphis automatically resends them.
