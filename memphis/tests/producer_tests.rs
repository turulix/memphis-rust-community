use crate::common::{connect_to_memphis, create_random_producer, create_random_station};

mod common;

#[tokio::test]
async fn create_producer() {
    let _ = env_logger::try_init();
    let client = connect_to_memphis().await;
    let station = create_random_station(&client).await;
    let _producer = create_random_producer(&station).await;
}
