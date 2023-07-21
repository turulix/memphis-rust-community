use std::sync::Arc;
use std::time::Duration;

use async_nats::jetstream::consumer::PullConsumer;
use async_nats::{Error, Message};
use futures_util::StreamExt;
use log::{debug, error, info, trace, warn};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_util::sync::CancellationToken;

use crate::constants::memphis_constants::{MemphisSpecialStation, MemphisSubscriptions};
use crate::consumer::consumer_error::ConsumerError;
use crate::consumer::event::MemphisEvent;
use crate::consumer::memphis_consumer_options::MemphisConsumerOptions;
use crate::consumer::{get_effective_consumer_name, MemphisMessage};
use crate::helper::memphis_util::{get_internal_name, sanitize_name};
use crate::models::request::CreateConsumerRequest;
use crate::models::request::DestroyConsumerRequest;
use crate::station::MemphisStation;

/// The MemphisConsumer is used to consume messages from a Memphis Station.
/// See [MemphisStation::create_consumer] for more information.
pub struct MemphisConsumer {
    station: MemphisStation,
    options: MemphisConsumerOptions,
    cancellation_token: CancellationToken,
    message_sender: Option<UnboundedSender<MemphisEvent>>,
}

impl MemphisConsumer {
    /// Creates a new MemphisConsumer.
    /// This will also start pinging the consumer, to ensure its availability.
    /// See [MemphisStation::create_consumer] for more information.
    ///
    /// # Arguments
    /// * `station` - The MemphisStation to use.
    /// * `options` - The MemphisConsumerOptions to use.
    pub(crate) async fn new(station: MemphisStation, mut options: MemphisConsumerOptions) -> Result<Self, ConsumerError> {
        sanitize_name(&mut options.consumer_name, options.generate_unique_suffix);

        if options.start_consume_from_sequence <= 0 {
            return Err(ConsumerError::InvalidSequence);
        }

        let create_consumer_request = CreateConsumerRequest {
            consumer_name: &options.consumer_name,
            station_name: &station.options.station_name,
            connection_id: &station.memphis_client.connection_id.to_string(),
            consumer_type: "application",
            consumer_group: &options.consumer_group,
            max_ack_time_ms: options.max_ack_time_ms,
            max_msg_count_for_delivery: options.max_msg_deliveries,
            start_consume_from_sequence: options.start_consume_from_sequence,
            last_messages: options.last_messages,
            username: &station.memphis_client.username,
        };

        if let Err(e) = station
            .memphis_client
            .send_internal_request(&create_consumer_request, MemphisSpecialStation::ConsumerCreations)
            .await
        {
            error!("Error creating consumer: {}", e.to_string());
            return Err(e.into());
        }

        info!("Consumer '{}' created successfully", &options.consumer_name);

        let consumer = Self {
            station,
            options,
            cancellation_token: CancellationToken::new(),
            message_sender: None,
        };

        consumer.ping_consumer();

        Ok(consumer)
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
    /// use memphis_rust_community::consumer::MemphisConsumerOptions;
    /// use memphis_rust_community::station::MemphisStationsOptions;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///
    ///     let client = MemphisClient::new("localhost:6666", "root", "memphis").await.unwrap();
    ///
    ///     let station_options = MemphisStationsOptions::new("test_station");
    ///     let station = client.create_station(station_options).await.unwrap();
    ///
    ///     let consumer_options = MemphisConsumerOptions::new("test_consumer")
    ///         .with_generate_unique_suffix(true);
    ///     let mut consumer = station.create_consumer(consumer_options).await.unwrap();
    ///
    ///     let mut message_receiver = consumer.consume().await.unwrap();
    ///
    ///      tokio::spawn(async move {
    ///          loop{
    ///             let msg = message_receiver.recv().await;
    ///              // Do something with the message
    ///              break;
    ///          }
    ///      });
    ///
    /// }
    /// ```
    pub async fn consume(&mut self) -> Result<UnboundedReceiver<MemphisEvent>, Error> {
        let cloned_token = self.cancellation_token.clone();
        let cloned_client = self.station.memphis_client.clone();
        let cloned_options = self.options.clone();

        let (sender, receiver) = unbounded_channel::<MemphisEvent>();
        self.message_sender = Some(sender.clone());

        // Memphis will create a stream with the name of the station.
        // On this Stream it will create a consumer with the name of the consumer Group.
        // If no consumer group is provided, the consumer name will be used.
        let consumer: PullConsumer = cloned_client
            .get_jetstream_context()
            .get_stream(&self.get_effective_stream_name())
            .await?
            .get_consumer(&get_effective_consumer_name(&self.options))
            .await?;

        tokio::spawn(async move {
            loop {
                let msg_handler = consumer
                    .batch()
                    .max_messages(cloned_options.batch_size)
                    .expires(Duration::from_millis(cloned_options.batch_max_time_to_wait_ms))
                    .messages();

                tokio::select! {
                    _ = cloned_token.cancelled() => {
                        debug!("Consumer '{}' on group '{}' was cancelled.", &cloned_options.consumer_name, &cloned_options.consumer_group);
                        break;
                    },
                    Ok(mut batch) = msg_handler => {
                        while let Some(Ok(msg)) = batch.next().await {
                            trace!(
                                "Message received from Memphis. (Subject: {}, Sequence: {})",
                                msg.subject,
                                match msg.info() {
                                    Ok(info) => info.stream_sequence,
                                    Err(_e) => 0,
                                }
                            );
                            let memphis_message = MemphisMessage::new(
                                msg,
                                cloned_client.clone(),
                                cloned_options.consumer_group.clone(),
                                cloned_options.max_ack_time_ms,
                            );
                            let res = sender.send(MemphisEvent::MessageReceived(memphis_message));
                            if res.is_err() {
                                error!("Error while sending message to the receiver. {:?}", res.err());
                            }
                        }
                    }
                }
            }

            // loop {
            //     if messages.is_err() {
            //         error!("Error while fetching messages from JetStream. {:?}", messages.err());
            //         continue;
            //     }
            //
            //     let mut messages = match messages {
            //         Ok(m) => m,
            //         Err(e) => {
            //             error!("Error while fetching messages from JetStream. {:?}", e);
            //             continue;
            //         }
            //     };
            //     while let Some(Ok(msg)) = messages.next().await {
            //         trace!(
            //             "Message received from Memphis. (Subject: {}, Sequence: {})",
            //             msg.subject,
            //             match msg.info() {
            //                 Ok(info) => info.stream_sequence,
            //                 Err(_e) => 0,
            //             }
            //         );
            //         let memphis_message = MemphisMessage::new(
            //             msg,
            //             cloned_client.clone(),
            //             cloned_options.consumer_group.clone(),
            //             cloned_options.max_ack_time_ms,
            //         );
            //         let res = sender.send(MemphisEvent::MessageReceived(memphis_message));
            //         if res.is_err() {
            //             error!("Error while sending message to the receiver. {:?}", res.err());
            //         }
            //     }
            // }
        });

        debug!(
            "Successfully started consuming messages from Memphis with consumer '{}' on group: '{}'",
            self.options.consumer_name, self.options.consumer_group
        );
        Ok(receiver)
    }

    /// # Starts consuming DLS messages from Memphis.
    /// DLS messages are messages that were not acknowledged in time by the consumer or the Schema Validation failed.
    ///
    /// This method will spawn a new Tokio task that will start to consume DLS messages from Memphis.
    /// The messages will be sent to the receiver.
    ///
    /// # Note
    /// If this method is called multiple times, the messages will be split between the receivers.
    /// This is to ensure that messages are not duplicated.
    ///
    /// # Returns
    /// A Receiver that will receive the DLS messages.
    pub async fn consume_dls(&self) -> Result<UnboundedReceiver<Arc<Message>>, Error> {
        //TODO: Remove Arc once async_nats is updated to >=0.30.0 (https://github.com/nats-io/nats.rs/pull/975)
        let (s, r) = unbounded_channel::<Arc<Message>>();
        let subject = format!(
            "{}{}_{}",
            MemphisSubscriptions::DlsPrefix.to_string(),
            self.get_effective_stream_name(),
            get_effective_consumer_name(&self.options)
        );

        let mut dls_sub = self
            .station
            .memphis_client
            .get_broker_connection()
            .queue_subscribe(subject, get_effective_consumer_name(&self.options))
            .await?;

        let cancellation_token = self.cancellation_token.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(message) = dls_sub.next() => {
                        if let Err(e) = s.send(Arc::new(message)) {
                            error!("Error while sending DLS message to the channel. {}", e);
                        }
                    },
                    _ = cancellation_token.cancelled() => break,
                    else => break
                }
            }
        });
        debug!(
            "Successfully started consuming DLS messages from Memphis with consumer '{}' on group: '{}'",
            self.options.consumer_name, self.options.consumer_group
        );
        Ok(r)
    }

    /// Sends a request to destroy/delete this Consumer.
    pub async fn destroy(self) -> Result<(), ConsumerError> {
        let destroy_request = DestroyConsumerRequest {
            consumer_name: &self.options.consumer_name,
            station_name: &self.station.options.station_name,
            connection_id: &self.station.memphis_client.connection_id,
            username: &self.station.memphis_client.username,
        };

        if let Err(e) = self
            .station
            .memphis_client
            .send_internal_request(&destroy_request, MemphisSpecialStation::ConsumerDestructions)
            .await
        {
            error!("Error destroying consumer. {}", &e);
            return Err(e.into());
        }

        self.cancellation_token.cancel();
        info!("Destroyed consumer {}.", &self.options.consumer_name);

        Ok(())
    }

    pub fn get_name(&self) -> String {
        self.options.consumer_name.clone()
    }

    /// Starts pinging the consumer, to ensure its availability.
    fn ping_consumer(&self) {
        let cloned_token = self.cancellation_token.clone();
        let cloned_options = self.options.clone();
        let cloned_client = self.station.memphis_client.clone();
        let cloned_sender = self.message_sender.clone();

        let consumer_name = self.get_effective_stream_name();
        let options = self.station.options.clone();
        tokio::spawn(async move {
            fn send_message(sender: &Option<UnboundedSender<MemphisEvent>>, event: MemphisEvent, consumer_name: &str) {
                match sender {
                    None => {
                        warn!("Consumer {} tried to send event, without the Sender being initialised", consumer_name);
                    }
                    Some(s) => {
                        let _res = s.send(event);
                    }
                }
            }
            let name = get_effective_consumer_name(&cloned_options);
            while !cloned_token.is_cancelled() {
                let stream = match cloned_client.get_jetstream_context().get_stream(&consumer_name).await {
                    Ok(s) => s,
                    Err(e) => {
                        send_message(&cloned_sender, MemphisEvent::StationUnavailable(Arc::new(e)), name.as_str());
                        error!("Station {} is unavailable. (Ping)", &options.station_name);
                        tokio::time::sleep(Duration::from_secs(30)).await;
                        continue;
                    }
                };

                match stream.consumer_info(get_effective_consumer_name(&cloned_options)).await {
                    Ok(_) => {}
                    Err(e) => {
                        send_message(&cloned_sender, MemphisEvent::ConsumerUnavailable(Arc::new(e)), name.as_str());
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
    fn get_effective_stream_name(&self) -> String {
        get_internal_name(&self.station.options.station_name)
    }
}
