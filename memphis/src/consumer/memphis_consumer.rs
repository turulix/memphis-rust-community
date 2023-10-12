use std::time::Duration;

use async_nats::jetstream::consumer::PullConsumer;

use async_nats::{Error, Message};
use futures_util::StreamExt;
use log::{debug, error, info, trace, warn};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_util::sync::CancellationToken;

use crate::constants::memphis_constants::{MemphisSpecialStation, MemphisSubscriptions};
use crate::consumer::consumer_error::ConsumerError;
use crate::consumer::memphis_consumer_options::MemphisConsumerOptions;
use crate::consumer::MemphisMessage;
use crate::helper::memphis_util::{get_internal_name, sanitize_name};
use crate::memphis_client::MemphisClient;
use crate::models::request::CreateConsumerRequest;
use crate::models::request::DestroyConsumerRequest;
use crate::models::response::CreateConsumerResponse;
use crate::station::MemphisStation;
use crate::RequestError;

/// The MemphisConsumer is used to consume messages from a Memphis Station.
/// See [MemphisStation::create_consumer] for more information.
pub struct MemphisConsumer {
    station: MemphisStation,
    options: MemphisConsumerOptions,
    cancellation_token: CancellationToken,
    partitions_list: Option<Vec<u32>>,
}

impl MemphisConsumer {
    /// Creates a new MemphisConsumer.
    /// This will also start pinging the consumer, to ensure its availability.
    /// See [MemphisStation::create_consumer] for more information.
    ///
    /// # Arguments
    /// * `station` - The MemphisStation to use.
    /// * `options` - The MemphisConsumerOptions to use.
    pub(crate) async fn new(
        station: MemphisStation,
        mut options: MemphisConsumerOptions,
    ) -> Result<Self, ConsumerError> {
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
            max_ack_time_ms: options.max_ack_time.as_millis() as i32,
            max_msg_deliveries: options.max_msg_deliveries,
            start_consume_from_sequence: options.start_consume_from_sequence,
            last_messages: options.last_messages,
            req_version: 2,
            username: &station.memphis_client.username,
        };

        let res = match station
            .memphis_client
            .send_internal_request(
                &create_consumer_request,
                MemphisSpecialStation::ConsumerCreations,
            )
            .await
        {
            Ok(res) => res,
            Err(e) => {
                error!("Error creating consumer: {}", e.to_string());
                return Err(e.into());
            }
        };

        let res = std::str::from_utf8(&res.payload)
            .map_err(|e| RequestError::MemphisError(e.to_string()))?;

        let cancellation_token = CancellationToken::new();

        let consumer = match serde_json::from_str::<CreateConsumerResponse>(res) {
            Ok(x) => Self {
                station,
                options,
                cancellation_token,
                partitions_list: Some(x.partitions_update.partitions_list),
            },
            Err(e) => {
                if res.is_empty() {
                    Self {
                        station,
                        options,
                        cancellation_token,
                        partitions_list: None,
                    }
                } else {
                    error!("Error creating consumer: {}", e.to_string());
                    return Err(ConsumerError::InvalidResponse(res.to_string()));
                }
            }
        };

        info!("Consumer '{}' created successfully", &consumer.get_name());

        consumer.ping_consumer().await;

        Ok(consumer)
    }

    async fn start_pull_subscription(
        &self,
        partition: Option<u32>,
        sender: UnboundedSender<MemphisMessage>,
    ) -> Result<(), Error> {
        debug!(
            "Starting pull subscription for consumer '{}', Partition: {:?}",
            &self.get_name(),
            partition
        );
        let consumer: PullConsumer = self
            .station
            .memphis_client
            .get_jetstream_context()
            .get_stream(&self.station.get_internal_name(partition))
            .await?
            .get_consumer(&self.get_internal_name())
            .await?;

        let x = consumer
            .stream()
            .max_messages_per_batch(self.options.batch_size)
            .expires(self.options.batch_max_time_to_wait)
            .messages()
            .await;

        let mut stream = match x {
            Ok(s) => s,
            Err(e) => {
                error!("Error while receiving stream from Memphis. {}", e);
                return Err(Error::from(format!(
                    "Error while starting stream for consumer '{}'",
                    &self.get_name()
                )));
            }
        };

        let client_clone = self.station.memphis_client.clone();
        let options_clone = self.options.clone();

        let cancellation_token_clone = self.cancellation_token.clone();
        let known_messages = self.station.known_messages.clone();

        tokio::spawn(async move {
            trace!(
                "Started fetching messages from Memphis. (Consumer: '{}', Group: '{}')",
                &options_clone.consumer_name,
                &options_clone.consumer_group
            );
            loop {
                tokio::select! {
                    msg = stream.next() => {
                        if let Some(msg) = msg {
                            if let Ok(msg) = msg {
                                let sequence = match msg.info() {
                                    Ok(info) => info.stream_sequence,
                                    Err(_e) => {
                                        error!("Error while getting message info.");
                                        0
                                    },
                                };

                                let known_message_key = format!("{}-{}", msg.subject, sequence);

                                if known_messages.read().await.contains(&known_message_key) {
                                    trace!("Message was already received (Subject: {}, Sequence: {})", msg.subject, sequence);
                                    continue;
                                }

                                trace!(
                                    "Message received from Memphis. (Subject: {}, Sequence: {})",
                                    msg.subject,
                                    sequence
                                );

                                known_messages.write().await.insert(known_message_key.clone());

                                let memphis_message = MemphisMessage::new(
                                    msg,
                                    client_clone.clone(),
                                    options_clone.consumer_group.clone(),
                                    options_clone.max_ack_time,
                                    known_message_key,
                                    known_messages.clone(),
                                );

                                if let Err(e) = sender.send(memphis_message) {
                                    error!("Error while sending message to the receiver. {:?}", e);
                                } else {
                                    trace!("Message sent to the receiver. From {:?}", partition);
                                }
                            } else if let Err(e) = msg {
                                error!("Error while receiving messages from Stream. {}", e);
                            }
                        } else {
                            trace!("Consumer '{}' on group '{}' received None message", &options_clone.consumer_name, &options_clone.consumer_group);
                        }
                    }
                    _ = cancellation_token_clone.cancelled() => {
                        trace!("Consumer '{}' on group '{}' was cancelled.", &options_clone.consumer_name, &options_clone.consumer_group);
                        break;
                    }
                }
            }
            drop(sender);
        });
        Ok(())
    }

    /// # Starts consuming messages from Memphis.
    /// This method will spawn a new Tokio task that will start to consume messages from Memphis.
    ///
    /// The messages will be sent to the **message_receiver**.
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
    ///     let client = MemphisClient::new("localhost:6666", "root", "memphis", None).await.unwrap();
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
    pub async fn consume(&self) -> Result<UnboundedReceiver<MemphisMessage>, Error> {
        let (sender, receiver) = unbounded_channel::<MemphisMessage>();
        let cloned_token = self.cancellation_token.clone();
        let cloned_partitions_list = self.partitions_list.clone();

        match cloned_partitions_list {
            None => {
                self.start_pull_subscription(None, sender.clone()).await?;
            }
            Some(list) => {
                for x in list {
                    let res = self.start_pull_subscription(Some(x), sender.clone()).await;
                    if let Err(e) = res {
                        error!(
                            "Error while starting pull subscription. Stopping consumer. {}",
                            e
                        );
                        cloned_token.cancel();
                    }
                }
            }
        }
        drop(sender);
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
    pub async fn consume_dls(&self) -> Result<UnboundedReceiver<Message>, Error> {
        let (s, r) = unbounded_channel::<Message>();
        let subject = format!(
            "{}{}_{}",
            MemphisSubscriptions::DlsPrefix.to_string(),
            &self.station.get_internal_name(None),
            &self.get_internal_name()
        );

        let mut dls_sub = self
            .station
            .memphis_client
            .get_broker_connection()
            .queue_subscribe(subject, self.get_internal_name())
            .await?;

        let cancellation_token = self.cancellation_token.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(message) = dls_sub.next() => {
                        if let Err(e) = s.send(message) {
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

    /// This will stop the consumer, but not destroy it on the server.
    pub fn stop(self) {
        self.cancellation_token.cancel();
    }

    /// Sends a request to destroy/delete this Consumer.
    pub async fn destroy(self) -> Result<(), ConsumerError> {
        self.cancellation_token.cancel();
        let destroy_request = DestroyConsumerRequest {
            consumer_name: &self.options.consumer_name,
            station_name: &self.station.options.station_name,
            username: &self.station.memphis_client.username,
            connection_id: &self.station.memphis_client.connection_id,
            req_version: 1,
        };

        if let Err(e) = self
            .station
            .memphis_client
            .send_internal_request(
                &destroy_request,
                MemphisSpecialStation::ConsumerDestructions,
            )
            .await
        {
            error!("Error destroying consumer. {}", &e);
            return Err(e.into());
        }

        info!("Destroyed consumer {}.", &self.options.consumer_name);

        Ok(())
    }

    pub fn get_name(&self) -> String {
        self.options.consumer_name.clone()
    }

    /// Starts pinging the consumer, to ensure its availability.
    async fn ping_consumer(&self) {
        let cloned_token = self.cancellation_token.clone();
        let cloned_client = self.station.memphis_client.clone();
        let cloned_partitions_data = self.partitions_list.clone();
        let cloned_station = self.station.clone();

        let consumer_name = self.get_name();
        let durable_name = self.get_internal_name();

        let handle = tokio::spawn(async move {
            async fn ping_stream_consumer(
                stream_name: &str,
                consumer_name: &str,
                client: &MemphisClient,
            ) -> Result<(), async_nats::Error> {
                let stream = client
                    .get_jetstream_context()
                    .get_stream(stream_name)
                    .await?;

                let _consumer = stream.consumer_info(&consumer_name).await?;

                Ok(())
            }
            while !cloned_token.is_cancelled() {
                tokio::time::sleep(Duration::from_secs(30)).await;
                match &cloned_partitions_data {
                    None => {
                        let res = ping_stream_consumer(
                            &cloned_station.get_internal_name(None),
                            &durable_name,
                            &cloned_client,
                        )
                        .await;
                        if let Err(e) = res {
                            error!("Error pinging consumer. {}", e);
                            continue;
                        } else {
                            trace!(
                                "Consumer '{}' on station '{}' is still alive.",
                                &consumer_name,
                                &cloned_station.options.station_name
                            )
                        }
                    }
                    Some(data) => {
                        for x in data {
                            let res = ping_stream_consumer(
                                &cloned_station.get_internal_name(Some(*x)),
                                &durable_name,
                                &cloned_client,
                            )
                            .await;
                            if let Err(e) = res {
                                error!("Error pinging consumer. {}", e);
                                continue;
                            } else {
                                trace!(
                                    "Consumer '{}' on station '{}' is still alive.",
                                    &consumer_name,
                                    &cloned_station.options.station_name
                                )
                            }
                        }
                    }
                };
            }
        });

        let cloned_token = self.cancellation_token.clone();
        let cloned_consumer_name = self.get_name();
        tokio::spawn(async move {
            tokio::select! {
                _ = handle => {
                    warn!("Consumer '{}' stopped pinging.", &cloned_consumer_name);
                },
                _ = cloned_token.cancelled() => {
                    trace!("Ping for '{}' was canceled", &cloned_consumer_name);
                }
            }
        });
    }

    /// Get the internal name of the consumer. This is the name of the consumer in Jetstream.
    pub fn get_internal_name(&self) -> String {
        if self.options.consumer_group.is_empty() {
            get_internal_name(&self.options.consumer_name)
        } else {
            get_internal_name(&self.options.consumer_group)
        }
    }
}
