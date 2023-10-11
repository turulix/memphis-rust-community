use anyhow::Error;
use log::{error, info};
use memphis_rust_community::consumer::MemphisConsumerOptions;
use memphis_rust_community::station::MemphisStation;

pub async fn start_consumer(station: &MemphisStation) -> Result<(), Error> {
    let consumer_options = MemphisConsumerOptions::new("log-consumer");
    let mut consumer = station.create_consumer(consumer_options).await?;

    // We need to map the Err here, since async_nats uses a Box Error type...
    let mut receiver = consumer.consume().await.map_err(|e| anyhow::anyhow!(e))?;

    tokio::spawn(async move {
        while let Some(msg) = receiver.recv().await {
            // Do something with the message here.
            info!("Received: {:?}", msg);
            if let Err(e) = msg.ack().await {
                error!("Error while acking message: {:?}", e);
            }
        }
    });

    Ok(())
}
