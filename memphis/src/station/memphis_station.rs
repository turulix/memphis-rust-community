use std::sync::Arc;

use futures_util::StreamExt;
use log::{error, info};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

use crate::constants::memphis_constants::MemphisSpecialStation;
use crate::memphis_client::MemphisClient;
use crate::models::request::{CreateStationRequest, DestroyStationRequest, DlsConfiguration};
#[cfg(feature = "schemaverse")]
use crate::schemaverse::{schema::SchemaValidator, SchemaType};
use crate::station::memphis_station_options::MemphisStationsOptions;
use crate::RequestError;

#[derive(Clone)]
pub struct MemphisStation {
    pub(crate) memphis_client: MemphisClient,
    pub(crate) options: Arc<MemphisStationsOptions>,

    #[cfg(feature = "schemaverse")]
    pub(crate) schema: Arc<RwLock<Option<SchemaverseHolding>>>,

    cancel_token: CancellationToken,
}

#[cfg(feature = "schemaverse")]
pub(crate) struct SchemaverseHolding {
    pub schema: Box<dyn SchemaValidator>,
    pub schema_type: SchemaType,
    pub schema_name: String,
}

impl MemphisStation {
    pub(crate) async fn new(client: MemphisClient, options: MemphisStationsOptions) -> Result<Self, RequestError> {
        let req = CreateStationRequest {
            name: &options.station_name,
            retention_type: &options.retention_type.to_string(),
            retention_value: options.retention_value,
            storage_type: &options.storage_type.to_string(),
            replicas: options.replicas,
            idempotency_window_in_ms: options.idempotency_window_ms,
            schema_name: &options.schema_name,
            dls_configuration: DlsConfiguration {
                poison: options.send_poison_msg_to_dls,
                schemaverse: options.send_schema_failed_msg_to_dls,
            },
            username: &client.username,
            tiered_storage_enabled: options.tiered_storage_enabled,
        };

        if let Err(e) = client.send_internal_request(&req, MemphisSpecialStation::StationCreations).await {
            error!("Failed to create station: {}", e);
            return Err(e);
        }

        info!("Created station {}", &options.station_name);

        let station = Self {
            memphis_client: client,
            options: Arc::new(options),
            #[cfg(feature = "schemaverse")]
            schema: Arc::new(RwLock::new(None)),
            cancel_token: CancellationToken::new(),
        };

        #[cfg(feature = "schemaverse")]
        station.subscribe_schema_updates().await?;

        Ok(station)
    }

    pub async fn destroy(self) -> Result<(), RequestError> {
        let req = DestroyStationRequest {
            station_name: &self.options.station_name,
            username: &self.memphis_client.username,
        };

        self.memphis_client
            .send_internal_request(&req, MemphisSpecialStation::StationDestructions)
            .await?;

        info!("Destroyed station {}", self.options.station_name);

        Ok(())
    }

    pub fn get_name(&self) -> &str {
        &self.options.station_name
    }
}

#[cfg(feature = "consumers")]
mod consumers {
    use crate::consumer::{ConsumerError, MemphisConsumer, MemphisConsumerOptions};
    use crate::station::MemphisStation;

    impl MemphisStation {
        /// Creates a consumer for the given station and returns a MemphisConsumer
        /// You need to call **consume()** on the MemphisConsumer to start consuming messages.
        /// # Arguments
        /// * `consumer_options` - [MemphisConsumerOptions](MemphisConsumerOptions)
        ///
        /// # Example
        /// ```rust
        /// use memphis_rust_community::memphis_client::MemphisClient;
        /// use memphis_rust_community::consumer::MemphisConsumerOptions;
        ///
        /// #[tokio::main]
        /// async fn main() {
        ///     use memphis_rust_community::station::MemphisStationsOptions;
        ///     let client = MemphisClient::new("localhost:6666", "root", "memphis").await.unwrap();
        ///
        ///     let station_options = MemphisStationsOptions::new("my-station");
        ///     let station = client.create_station(station_options).await.unwrap();
        ///
        ///     let consumer_options = MemphisConsumerOptions::new("my-consumer").with_generate_unique_suffix(true);
        ///     let mut consumer = station.create_consumer(consumer_options).await.unwrap();
        ///
        ///     let msg_receiver = consumer.consume().await.unwrap();
        /// }
        pub async fn create_consumer(&self, consumer_options: MemphisConsumerOptions) -> Result<MemphisConsumer, ConsumerError> {
            MemphisConsumer::new(self.clone(), consumer_options).await
        }
    }
}

#[cfg(feature = "producers")]
mod producer {
    use crate::producer::{MemphisProducer, MemphisProducerOptions};
    use crate::station::MemphisStation;
    use crate::RequestError;

    impl MemphisStation {
        pub async fn create_producer(&self, producer_options: MemphisProducerOptions) -> Result<MemphisProducer, RequestError> {
            MemphisProducer::new(self.clone(), producer_options).await
        }
    }
}

#[cfg(feature = "schemaverse")]
mod schemaverse {
    use async_nats::Message;
    use futures_util::StreamExt;
    use log::{error, info, trace};

    use crate::constants::memphis_constants::MemphisSubscriptions;
    use crate::models::schemaverse_schema_update::SchemaUpdateResponse;
    use crate::schemaverse::SchemaType;
    use crate::station::{MemphisStation, SchemaverseHolding};

    impl MemphisStation {
        pub async fn get_schema_type(&self) -> Option<SchemaType> {
            self.schema.read().await.as_ref().map(|v| v.schema_type.clone())
        }

        pub(crate) async fn subscribe_schema_updates(&self) -> Result<(), async_nats::Error> {
            let subscription_name = format!("{}{}", &MemphisSubscriptions::SchemaUpdatesPrefix.to_string(), &self.options.station_name);
            let mut subscriber = self.memphis_client.get_broker_connection().subscribe(subscription_name.clone()).await?;

            let cloned_self = self.clone();

            trace!("Start listening for schema changes at {}", &subscription_name);
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        Some(msg) = subscriber.next() => {
                            let msg: Message = msg;
                            let r: SchemaUpdateResponse = match serde_json::from_slice(&msg.payload) {
                                Ok(v) => v,
                                Err(e) => {
                                    error!("Error while receiving SchemaUpdate: {}\nMessage was: {}", &e, String::from_utf8_lossy(&msg.payload));
                                    continue;
                                }
                            };

                            let schema_type = match SchemaType::from_str(&r.init.r#type) {
                                Ok(v) => v,
                                Err(e) => {
                                    error!("Error while receiving SchemaUpdate: {}\n", &e);
                                    continue;
                                }
                            };

                            let schema = match schema_type.create_validator(&r.init.active_version.schema_content) {
                                Ok(v) => v,
                                Err(e) => {
                                    error!("Error while receiving SchemaUpdate: {}\n", &e);
                                    continue;
                                }
                            };

                            let new = SchemaverseHolding {
                                schema,
                                schema_type,
                                schema_name: r.init.schema_name
                            };

                            {
                                let mut guard = cloned_self.schema.write().await;
                                *guard = Some(new);
                            }

                            info!("Updated schema for station {}", &cloned_self.options.station_name);
                        },
                        _ = cloned_self.cancel_token.cancelled() => break,
                        else => break
                    }
                }
            });

            Ok(())
        }
    }
}
