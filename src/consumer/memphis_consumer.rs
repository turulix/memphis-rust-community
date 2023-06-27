use crate::consumer::memphis_consumer_options::MemphisConsumerOptions;
use crate::core::memphis_message::MemphisMessage;
use crate::core::memphis_message_handler::MemphisEvent;
use crate::helper::memphis_util::get_internal_name;
use crate::memphis_client::MemphisClient;
use async_nats::jetstream::consumer::PullConsumer;
use async_nats::Error;
use futures_util::StreamExt;
use log::{error, trace};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio_util::sync::CancellationToken;

/// The MemphisConsumer is used to consume messages from a Memphis Station.
/// See [MemphisClient::create_consumer](crate::memphis_client::MemphisClient::create_consumer) for more information.
pub struct MemphisConsumer {
    memphis_client: MemphisClient,
    options: Arc<MemphisConsumerOptions>,
    cancellation_token: CancellationToken,
    /// This will be equivalent to the consumer name, if no Consumer Group is provided.
    /// If a Consumer Group is provided, this will be the Consumer Group name.
    real_name: String,
    /// The receiver for the MemphisEvents.
    /// This is used to communicate with the MemphisConsumer.
    /// The MemphisConsumer will send events to this receiver.
    pub message_receiver: Receiver<MemphisEvent>,
    message_sender: Sender<MemphisEvent>,
}

impl MemphisConsumer {
    /// Creates a new MemphisConsumer.
    /// This will also start pinging the consumer, to ensure its availability.
    /// See [MemphisClient::create_consumer](MemphisClient::create_consumer) for more information.
    ///
    /// # Arguments
    /// * `memphis_client` - The MemphisClient to use.
    /// * `options` - The MemphisConsumerOptions to use.
    /// * `real_name` - The real name of the consumer. This will be equivalent to the consumer name, if no Consumer Group is provided.
    /// If a Consumer Group is provided, this will be the Consumer Group name.
    pub(crate) fn new(
        memphis_client: MemphisClient,
        options: MemphisConsumerOptions,
        real_name: String,
    ) -> Self {
        let (s, r) = channel(100);
        let consumer = MemphisConsumer {
            memphis_client,
            options: Arc::new(options),
            cancellation_token: CancellationToken::new(),
            message_receiver: r,
            message_sender: s,
            real_name,
        };

        consumer.ping_consumer();

        consumer
    }
    /// Starts pinging the consumer, to ensure its availability.
    fn ping_consumer(&self) {
        let cloned_token = self.cancellation_token.clone();
        let cloned_options = self.options.clone();
        let cloned_client = self.memphis_client.clone();
        let cloned_sender = self.message_sender.clone();
        let cloned_real_name = self.real_name.clone();
        tokio::spawn(async move {
            fn send_message(sender: &Sender<MemphisEvent>, event: MemphisEvent) {
                let _res = sender.send(event);
            }

            while !cloned_token.is_cancelled() {
                let stream = match cloned_client
                    .get_jetstream_context()
                    .get_stream(&cloned_options.station_name.clone())
                    .await
                {
                    Ok(s) => s,
                    Err(e) => {
                        send_message(
                            &cloned_sender,
                            MemphisEvent::StationUnavailable(Arc::new(e)),
                        );
                        error!(
                            "Station {} is unavailable. (Ping)",
                            &cloned_options.station_name.clone()
                        );
                        tokio::time::sleep(Duration::from_secs(30)).await;
                        continue;
                    }
                };

                match stream.consumer_info(&cloned_real_name).await {
                    Ok(_) => {}
                    Err(e) => {
                        send_message(
                            &cloned_sender,
                            MemphisEvent::ConsumerUnavailable(Arc::new(e)),
                        );
                        error!(
                            "Consumer '{}' on group '{}' is unavailable. (Ping)",
                            &cloned_options.consumer_name, &cloned_options.consumer_group
                        );
                        tokio::time::sleep(Duration::from_secs(30)).await;
                        continue;
                    }
                }

                trace!(
                    "Consumer '{}' on group '{}' is alive. (Ping)",
                    &cloned_options.consumer_name,
                    &cloned_options.consumer_group
                );
                tokio::time::sleep(Duration::from_secs(30)).await;
            }
        });
    }

    /// # Starts consuming messages from Memphis.
    /// This method will spawn a new Tokio task that will start to consume messages from Memphis.
    ///
    /// The messages will be sent to the **message_receiver**.
    ///
    /// * If the station is not available, the [StationUnavailable](MemphisEvent::StationUnavailable) event will be sent.
    /// * If the consumer is not available, the [ConsumerUnavailable](MemphisEvent::ConsumerUnavailable) event will be sent.
    ///
    /// # Example
    /// ```rust
    /// use memphis_rust_community::memphis_client::MemphisClient;
    /// use memphis_rust_community::consumer::memphis_consumer_options::MemphisConsumerOptions;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = MemphisClient::new("localhost:6666", "root", "memphis").await.unwrap();
    ///     let consumer_options = MemphisConsumerOptions::new("my-station", "my-consumer")
    ///         .with_generate_unique_suffix(true);
    ///     let mut consumer = client.create_consumer(consumer_options).await.unwrap();
    ///
    ///     consumer.consume().await.unwrap();
    ///     tokio::spawn(async move {
    ///         loop{
    ///             let msg = consumer.message_receiver.recv().await;
    ///             // Do something with the message
    ///             break;
    ///         }
    ///     });
    /// }
    /// ```
    pub async fn consume(&self) -> Result<(), Error> {
        let cloned_token = self.cancellation_token.clone();
        let cloned_client = self.memphis_client.clone();
        let cloned_options = self.options.clone();
        let cloned_sender = self.message_sender.clone();

        let consumer: PullConsumer = cloned_client
            .get_jetstream_context()
            .get_stream(get_internal_name(&cloned_options.station_name))
            .await?
            .get_consumer(get_internal_name(&self.real_name).as_str())
            .await?;

        tokio::spawn(async move {
            while !cloned_token.is_cancelled() {
                let messages = consumer
                    .batch()
                    .max_messages(cloned_options.batch_size)
                    .expires(Duration::from_millis(
                        cloned_options.batch_max_time_to_wait_ms,
                    ))
                    .messages()
                    .await;

                if messages.is_err() {
                    error!(
                        "Error while fetching messages from JetStream. {}",
                        messages.err().unwrap()
                    );
                    continue;
                }

                let mut messages = messages.unwrap();
                while let Some(Ok(msg)) = messages.next().await {
                    trace!(
                        "Message received from Memphis. (Subject: {}, Sequence: {})",
                        msg.subject,
                        msg.info().expect("NONE").stream_sequence
                    );
                    let memphis_message = MemphisMessage::new(
                        msg,
                        cloned_client.clone(),
                        cloned_options.consumer_group.clone(),
                        cloned_options.max_ack_time_ms,
                    );
                    let _res = cloned_sender.send(MemphisEvent::MessageReceived(memphis_message));
                }
            }
        });

        trace!("Successfully started consuming messages from Memphis with consumer '{}' on group: '{}'", self.options.consumer_name, self.options.consumer_group);
        Ok(())
    }
}
