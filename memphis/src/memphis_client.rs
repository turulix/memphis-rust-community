use std::sync::Arc;
use std::time::Duration;

use async_nats::connection::State;
use async_nats::jetstream::Context;
use async_nats::{jetstream, Client, ConnectError, ConnectOptions, Event, Message};
use bytes::Bytes;
use log::{error, info};
use serde::Serialize;
use uuid::Uuid;

use crate::constants::memphis_constants::{MemphisNotificationType, MemphisSpecialStation};
use crate::models::request::NotificationRequest;
use crate::request_error::RequestError;
use crate::station::{MemphisStation, MemphisStationsOptions};

/// # Memphis Client
///
/// The Memphis Client is used to connect to Memphis.
///
/// ```rust
/// use memphis_rust_community::memphis_client::MemphisClient;
///
/// #[tokio::main]
/// async fn main() {
///     let client = MemphisClient::new("localhost:6666", "root", "memphis", None).await.unwrap();
/// }
/// ```
#[derive(Clone)]
pub struct MemphisClient {
    jetstream_context: Arc<Context>,
    broker_connection: Arc<Client>,
    pub(crate) username: Arc<String>,
    pub(crate) connection_id: Arc<String>,
}

impl MemphisClient {
    /// Creates a new MemphisClient
    /// # Arguments
    /// * `memphis_host` - The host of the Memphis server
    /// * `memphis_username` - The username of the Memphis user
    /// * `memphis_password` - The password of the Memphis user
    /// * `account_id` - The account id of the Memphis account (Only used in cloud version) use None or "1" for self hosted
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
    ///         "memphis",
    ///         None
    ///     ).await.unwrap();
    /// }
    pub async fn new(
        memphis_host: &str,
        memphis_username: &str,
        memphis_password: &str,
        account_id: Option<&str>,
    ) -> Result<MemphisClient, ConnectError> {
        let uuid = Uuid::new_v4();
        let connection_name = format!("{}::{}", &uuid, memphis_username);

        let account_id = account_id.unwrap_or("1");

        let broker_settings = MemphisClient::create_settings(
            format!("{}${}", memphis_username, account_id).as_str(),
            memphis_password,
            connection_name.clone(),
        );

        let connection = match async_nats::connect_with_options(memphis_host, broker_settings).await
        {
            Ok(c) => c,
            Err(e) => {
                if e.to_string().contains("authorization violation") {
                    let broker_settings = MemphisClient::create_settings(
                        memphis_username,
                        memphis_password,
                        connection_name,
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

        while connection.connection_state() == State::Pending {
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        Ok(MemphisClient {
            jetstream_context: Arc::new(jetstream::new(connection.clone())),
            broker_connection: Arc::new(connection),
            username: Arc::new(memphis_username.to_string()),
            connection_id: Arc::new(uuid.to_string()),
        })
    }

    pub fn is_connected(&self) -> bool {
        match &self.broker_connection.connection_state() {
            State::Pending => false,
            State::Connected => true,
            State::Disconnected => false,
        }
    }

    pub(crate) async fn send_notification(
        &self,
        notification_type: MemphisNotificationType,
        title: &str,
        message: &str,
        code: &str,
    ) -> Result<(), RequestError> {
        let req = NotificationRequest {
            title,
            msg: message,
            msg_type: &notification_type.to_string(),
            code,
        };

        self.send_internal_request(&req, MemphisSpecialStation::Notifications)
            .await?;

        Ok(())
    }

    pub async fn create_station(
        &self,
        station_options: MemphisStationsOptions,
    ) -> Result<MemphisStation, RequestError> {
        MemphisStation::new(self.clone(), station_options).await
    }

    /// Returns the Jetstream Context used behind the scenes, only use this if you know what you are doing
    pub fn get_jetstream_context(&self) -> &Context {
        &self.jetstream_context
    }

    /// Returns the Broker Connection used behind the scenes, only use this if you know what you are doing
    pub fn get_broker_connection(&self) -> &Client {
        &self.broker_connection
    }

    /// Sends a request to a internal/special Memphis Station and handles the errors
    pub(crate) async fn send_internal_request(
        &self,
        request: &impl Serialize,
        request_type: MemphisSpecialStation,
    ) -> Result<Message, RequestError> {
        if !self.is_connected() {
            return Err(RequestError::NotConnected);
        }

        let create_consumer_model_json = serde_json::to_string(&request)?;
        let create_consumer_model_bytes = Bytes::from(create_consumer_model_json);
        let res = self
            .get_broker_connection()
            .request(request_type.to_string(), create_consumer_model_bytes)
            .await
            .map_err(|e| RequestError::NatsError(e.into()))?;

        let error_message = std::str::from_utf8(&res.payload)
            .map_err(|e| RequestError::MemphisError(e.to_string()))?;

        if !error_message.trim().is_empty() {
            let parsed_json: serde_json::Value = serde_json::from_str(error_message)?;
            let raw_object = if let Some(obj) = parsed_json.as_object() {
                obj
            } else {
                return Err(RequestError::MemphisError("Error parsing json".to_string()));
            };
            if let Some((_key, value)) = raw_object.get_key_value("error") {
                let value = value.as_str().unwrap_or("Error was not a string");
                return if value.to_string().trim().is_empty() {
                    Ok(res)
                } else {
                    Err(RequestError::MemphisError(value.to_string()))
                };
            }
        }

        Ok(res)
    }

    fn create_settings(
        memphis_username: &str,
        memphis_password: &str,
        connection_name: String,
    ) -> ConnectOptions {
        ConnectOptions::with_user_and_password(
            memphis_username.to_string(),
            memphis_password.to_string(),
        )
        .event_callback(MemphisClient::event_callback)
        .retry_on_initial_connect()
        .connection_timeout(Duration::from_secs(5))
        .ping_interval(Duration::from_secs(1))
        .request_timeout(Some(Duration::from_secs(5)))
        .name(connection_name)
    }

    async fn event_callback(event: Event) {
        info!("Event: {:?}", event)
    }
}
