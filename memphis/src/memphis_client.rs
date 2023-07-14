use std::sync::Arc;
use std::time::Duration;

use async_nats::connection::State;
use async_nats::jetstream::Context;
use async_nats::{jetstream, Client, ConnectError, ConnectOptions, Message};
use bytes::Bytes;
use serde::Serialize;
use uuid::Uuid;

use crate::constants::memphis_constants::{MemphisNotificationType, MemphisSpecialStation};
use crate::models::request::NotificationRequest;
use crate::request_error::RequestError;
use crate::station::{MemphisStation, MemphisStationsOptions};
use crate::station_settings::StationSettingsStore;

/// # Memphis Client
///
/// The Memphis Client is used to connect to Memphis.
///
/// ```rust
/// use memphis_rust_community::memphis_client::MemphisClient;
///
/// #[tokio::main]
/// async fn main() {
///     let client = MemphisClient::new("localhost:6666", "root", "memphis").await.unwrap();
/// }
/// ```
#[derive(Clone)]
pub struct MemphisClient {
    jetstream_context: Arc<Context>,
    broker_connection: Arc<Client>,
    pub(crate) username: Arc<String>,
    pub(crate) connection_id: Arc<String>,
    pub(crate) station_settings: Arc<StationSettingsStore>,
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
    pub async fn new(memphis_host: &str, memphis_username: &str, memphis_password: &str) -> Result<MemphisClient, ConnectError> {
        let uuid = Uuid::new_v4();
        let connection_name = format!("{}::{}", &uuid, memphis_username);

        // TODO: Replace 1 with account_id
        let broker_settings = MemphisClient::create_settings(format!("{}${}", memphis_username, 1).as_str(), memphis_password, connection_name.clone());

        let connection = match async_nats::connect_with_options(memphis_host, broker_settings).await {
            Ok(c) => c,
            Err(e) => {
                if e.to_string().contains("authorization violation") {
                    let broker_settings = MemphisClient::create_settings(memphis_username, memphis_password, connection_name);
                    let connection = async_nats::connect_with_options(memphis_host, broker_settings).await;
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
            username: Arc::new(memphis_username.to_string()),
            connection_id: Arc::new(uuid.to_string()),
            station_settings: Arc::new(StationSettingsStore::new()),
        })
    }

    pub fn is_connected(&self) -> bool {
        match &self.broker_connection.connection_state() {
            State::Pending => false,
            State::Connected => true,
            State::Disconnected => false,
        }
    }

    pub async fn send_notification(&self, notification_type: MemphisNotificationType, title: &str, message: &str, code: &str) -> Result<(), RequestError> {
        let req = NotificationRequest {
            title,
            msg: message,
            msg_type: &notification_type.to_string(),
            code,
        };

        self.send_internal_request(&req, MemphisSpecialStation::Notifications).await?;

        Ok(())
    }

    pub async fn create_station(&self, station_options: MemphisStationsOptions) -> Result<MemphisStation, RequestError> {
        MemphisStation::new(self.clone(), station_options).await
    }

    pub(crate) fn get_jetstream_context(&self) -> &Context {
        &self.jetstream_context
    }

    pub(crate) fn get_broker_connection(&self) -> &Client {
        &self.broker_connection
    }

    /// Sends a request to a internal/special Memphis Station and handles the errors
    pub(crate) async fn send_internal_request(&self, request: &impl Serialize, request_type: MemphisSpecialStation) -> Result<Message, RequestError> {
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

        let error_message = std::str::from_utf8(&res.payload).map_err(|e| RequestError::MemphisError(e.to_string()))?;

        if !error_message.trim().is_empty() {
            return Err(RequestError::MemphisError(error_message.to_string()));
        }

        Ok(res)
    }

    fn create_settings(memphis_username: &str, memphis_password: &str, name: String) -> ConnectOptions {
        ConnectOptions::with_user_and_password(memphis_username.to_string(), memphis_password.to_string())
            .flush_interval(Duration::from_millis(100))
            .connection_timeout(Duration::from_secs(5))
            .ping_interval(Duration::from_secs(1))
            .request_timeout(Some(Duration::from_secs(5)))
            .name(name)
    }
}
