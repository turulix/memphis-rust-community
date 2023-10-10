use std::io::Cursor;
use std::sync::Arc;

use log::{error, info};
use murmur3::murmur3_32;

use crate::constants::memphis_constants::MemphisSpecialStation;
use crate::helper::memphis_util::get_internal_name;
use crate::memphis_client::MemphisClient;
use crate::models::request::{CreateStationRequest, DestroyStationRequest, DlsConfiguration};
#[cfg(feature = "schemaverse")]
use crate::schemaverse::schema::SchemaValidator;
use crate::station::memphis_station_options::MemphisStationsOptions;
use crate::RequestError;

static SEED: u32 = 31;

#[derive(Clone)]
pub struct MemphisStation {
    pub(crate) memphis_client: MemphisClient,
    pub(crate) options: Arc<MemphisStationsOptions>,

    #[cfg(feature = "schemaverse")]
    pub(crate) schema: Option<Arc<dyn SchemaValidator>>,
}

impl MemphisStation {
    pub(crate) async fn new(
        client: MemphisClient,
        options: MemphisStationsOptions,
    ) -> Result<Self, RequestError> {
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
            partitions_number: options.partition_number,
        };

        if let Err(e) = client
            .send_internal_request(&req, MemphisSpecialStation::StationCreations)
            .await
        {
            error!("Failed to create station: {}", e);
            return Err(e);
        }

        info!("Created station {}", &options.station_name);

        Ok(Self {
            memphis_client: client,
            options: Arc::new(options),
            #[cfg(feature = "schemaverse")]
            schema: None,
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

    /// Returns the name of the station.
    pub fn get_name(&self) -> &str {
        &self.options.station_name
    }

    /// Returns the internal name of the station. This is the name of the stream used in Jetstream.
    pub fn get_internal_name(&self, partition: Option<u32>) -> String {
        match partition {
            None => get_internal_name(&self.options.station_name),
            Some(partition) => format!(
                "{}${}",
                get_internal_name(&self.options.station_name),
                partition
            )
            .to_string(),
        }
    }

    /// This is the name of the subject used to send messages to the station.
    pub fn get_internal_subject_name(&self, partition: Option<u32>) -> String {
        format!("{}.final", self.get_internal_name(partition))
    }

    pub(crate) fn get_partition_key(&self, key: &str, partition_count: u32) -> u32 {
        murmur3_32(&mut Cursor::new(key), SEED).unwrap_or(0) % partition_count
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
        ///     let client = MemphisClient::new("localhost:6666", "root", "memphis", None).await.unwrap();
        ///
        ///     let station_options = MemphisStationsOptions::new("my-station");
        ///     let station = client.create_station(station_options).await.unwrap();
        ///
        ///     let consumer_options = MemphisConsumerOptions::new("my-consumer").with_generate_unique_suffix(true);
        ///     let mut consumer = station.create_consumer(consumer_options).await.unwrap();
        ///
        ///     let msg_receiver = consumer.consume().await.unwrap();
        /// }
        pub async fn create_consumer(
            &self,
            consumer_options: MemphisConsumerOptions,
        ) -> Result<MemphisConsumer, ConsumerError> {
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
        pub async fn create_producer(
            &self,
            producer_options: MemphisProducerOptions,
        ) -> Result<MemphisProducer, RequestError> {
            MemphisProducer::new(self.clone(), producer_options).await
        }
    }
}
