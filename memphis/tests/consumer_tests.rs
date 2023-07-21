mod common;

use common::*;
use memphis_rust_community::consumer::MemphisEvent;
use memphis_rust_community::producer::ComposableMessage;
use std::time::Duration;
use tokio_test::assert_ok;

#[tokio::test]
async fn destroy_consumer() {
    let (_, _, mut consumer, producer) = create_random_setup().await;
    assert_ok!(producer.produce(ComposableMessage::new().with_payload("Works.")).await);

    let mut receiver = assert_ok!(consumer.consume().await);

    tokio::time::sleep(Duration::from_secs(2)).await;
    let msg = assert_ok!(receiver.try_recv());
    if let MemphisEvent::MessageReceived(m) = msg {
        assert_ok!(m.ack().await);
    }

    assert_ok!(consumer.destroy().await);
    match receiver.recv().await {
        None => {}
        Some(_) => {
            panic!("This should be None.")
        }
    };
}
