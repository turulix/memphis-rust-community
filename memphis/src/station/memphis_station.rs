use log::{error, info};

use crate::constants::memphis_constants::MemphisSpecialStation;
use crate::memphis_client::MemphisClient;
use crate::models::request::{CreateStationRequest, DestroyStationRequest, DlsConfiguration};
use crate::station::memphis_station_options::MemphisStationsOptions;
use crate::RequestError;

pub struct MemphisStation {
    memphis_client: MemphisClient,
    options: MemphisStationsOptions,
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

        Ok(Self {
            memphis_client: client,
            options,
        })
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
    use crate::station::{MemphisStation, MemphisStationsOptions};

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
        pub async fn create_consumer(&self, mut consumer_options: MemphisConsumerOptions) -> Result<MemphisConsumer, ConsumerError> {
            consumer_options.station_name = self.options.station_name.clone();
            MemphisConsumer::new(self.memphis_client.clone(), consumer_options).await
        }
    }
}

#[cfg(feature = "producers")]
mod producer {
    use crate::producer::{MemphisProducer, MemphisProducerOptions};
    use crate::station::MemphisStation;
    use crate::RequestError;

    impl MemphisStation {
        pub async fn create_producer(&self, mut producer_options: MemphisProducerOptions) -> Result<MemphisProducer, RequestError> {
            producer_options.station_name = self.options.station_name.clone();
            MemphisProducer::new(self.memphis_client.clone(), producer_options).await
        }
    }
}
