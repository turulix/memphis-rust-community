use crate::constants::memphis_constants::MemphisStations;
use crate::consumer::create_consumer_error::CreateConsumerError;
use crate::consumer::memphis_consumer::MemphisConsumer;
use crate::consumer::memphis_consumer_options::MemphisConsumerOptions;
use crate::helper::memphis_util::get_unique_key;
use crate::models::request::create_consumer_request::CreateConsumerRequest;
use async_nats::connection::State;
use async_nats::jetstream::Context;
use async_nats::{jetstream, Client, ConnectError, ConnectOptions};
use bytes::Bytes;
use log::{error, trace};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

/// # Memphis Client
///
/// The Memphis Client is used to connect to Memphis.
///
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
///     // Start consuming messages
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
#[derive(Clone)]
pub struct MemphisClient {
    jetstream_context: Arc<Context>,
    broker_connection: Arc<Client>,
    username: Arc<String>,
    connection_id: Arc<String>,
}

impl MemphisClient {
    /// Creates a new MemphisClient
    /// # Arguments
    /// * `memphis_host` - The host of the Memphis server
    /// * `memphis_username` - The username of the Memphis user
    /// * `memphis_password` - The password of the Memphis user
    ///
    /// # Example
    /// ```rust
    /// use memphis_rust_community::memphis_client::MemphisClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = MemphisClient::new(
    ///         "localhost:6666",
    ///         "root",
    ///         "memphis"
    ///     ).await.unwrap();
    /// }
    pub async fn new(
        memphis_host: &str,
        memphis_username: &str,
        memphis_password: &str,
    ) -> Result<MemphisClient, ConnectError> {
        let uuid = Uuid::new_v4();
        let name = format!("{}::{}", &uuid, memphis_username);

        // TODO: Replace 1 with account_id
        let broker_settings = MemphisClient::create_settings(
            format!("{}${}", memphis_username, 1).as_str(),
            memphis_password,
            name.clone(),
        );

        let connection = match async_nats::connect_with_options(memphis_host, broker_settings).await
        {
            Ok(c) => c,
            Err(e) => {
                if e.to_string().contains("authorization violation") {
                    let broker_settings = MemphisClient::create_settings(
                        memphis_username,
                        memphis_password,
                        name.clone(),
                    );
                    let connection =
                        async_nats::connect_with_options(memphis_host, broker_settings).await;
                    match connection {
                        Ok(c) => c,
                        Err(e) => {
                            return Err(e);
                        }
                    }
                } else {
                    return Err(e);
                }
            }
        };

        Ok(MemphisClient {
            jetstream_context: Arc::new(jetstream::new(connection.clone())),
            broker_connection: Arc::new(connection),
            username: Arc::new(name),
            connection_id: Arc::new(uuid.to_string()),
        })
    }

    pub async fn is_connected(&self) -> bool {
        match &self.broker_connection.connection_state() {
            State::Pending => false,
            State::Connected => true,
            State::Disconnected => false,
        }
    }

    fn create_settings(
        memphis_username: &str,
        memphis_password: &str,
        name: String,
    ) -> ConnectOptions {
        ConnectOptions::with_user_and_password(
            memphis_username.to_string(),
            memphis_password.to_string(),
        )
        .flush_interval(Duration::from_millis(100))
        .connection_timeout(Duration::from_secs(5))
        .ping_interval(Duration::from_secs(1))
        .request_timeout(Some(Duration::from_secs(5)))
        .name(name)
    }

    pub(crate) fn get_jetstream_context(&self) -> &Context {
        &self.jetstream_context
    }

    pub(crate) fn get_broker_connection(&self) -> &Client {
        &self.broker_connection
    }

    /// Creates a consumer for the given station and returns a MemphisConsumer
    /// You need to call **consume()** on the MemphisConsumer to start consuming messages.
    /// # Arguments
    /// * `consumer_options` - [MemphisConsumerOptions](MemphisConsumerOptions)
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
    ///
    ///     let mut consumer = client.create_consumer(consumer_options).await.unwrap();
    ///     // Start consuming messages
    ///     consumer.consume().await.unwrap();
    /// }
    pub async fn create_consumer(
        &self,
        mut consumer_options: MemphisConsumerOptions,
    ) -> Result<MemphisConsumer, CreateConsumerError> {
        if !self.is_connected().await {
            error!("Tried to create consumer without being connected to Memphis");
            return Err(CreateConsumerError::NotConnected);
        }

        consumer_options.consumer_name = consumer_options.consumer_name.to_lowercase();
        if consumer_options.generate_unique_suffix {
            consumer_options.consumer_name =
                format!("{}_{}", consumer_options.consumer_name, get_unique_key(8));
        }
        let real_name = if consumer_options.consumer_group.is_empty() {
            &consumer_options.consumer_name
        } else {
            &consumer_options.consumer_group
        };

        if consumer_options.start_consume_from_sequence <= 0 {
            return Err(CreateConsumerError::InvalidSequence);
        }

        let create_consumer_request = CreateConsumerRequest {
            consumer_name: consumer_options.clone().consumer_name,
            station_name: consumer_options.clone().station_name,
            connection_id: self.connection_id.to_string(),
            consumer_type: "application".to_string(),
            consumer_group: consumer_options.clone().consumer_group,
            max_ack_time_ms: consumer_options.clone().max_ack_time_ms,
            max_msg_count_for_delivery: consumer_options.clone().max_msg_deliveries,
            start_consume_from_sequence: consumer_options.clone().start_consume_from_sequence,
            last_messages: consumer_options.clone().last_messages,
            username: self.username.to_string(),
        };

        let create_consumer_model_json = serde_json::to_string(&create_consumer_request).unwrap();
        let create_consumer_model_bytes = Bytes::from(create_consumer_model_json);
        let res = self
            .broker_connection
            .request(
                MemphisStations::ConsumerCreations.to_string(),
                create_consumer_model_bytes,
            )
            .await;

        let res = match res {
            Ok(m) => m,
            Err(e) => {
                error!("Error creating consumer: {}", e.to_string());
                return Err(CreateConsumerError::NatsError(e));
            }
        };

        let error_message = match std::str::from_utf8(&res.payload) {
            Ok(s) => s,
            Err(e) => {
                error!("Error creating consumer: {}", e.to_string());
                return Err(CreateConsumerError::MemphisError(e.to_string()));
            }
        };

        if !error_message.trim().is_empty() {
            error!(
                "Error creating consumer '({})': {}",
                &consumer_options.consumer_name, error_message
            );
            return Err(CreateConsumerError::MemphisError(error_message.to_string()));
        }

        trace!(
            "Consumer '{}' created successfully",
            &consumer_options.consumer_name.clone()
        );
        Ok(MemphisConsumer::new(
            self.clone(),
            consumer_options.clone(),
            real_name.clone(),
        ))
    }
}
