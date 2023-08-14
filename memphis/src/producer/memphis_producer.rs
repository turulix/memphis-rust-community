use log::{debug, error, info};

use crate::constants::memphis_constants::{
    MemphisHeaders, MemphisNotificationType, MemphisSpecialStation,
};
use crate::helper::memphis_util::{get_internal_name, sanitize_name};
use crate::helper::partition_iterator::PartitionIterator;
use crate::models::request::{CreateProducerRequest, DestroyProducerRequest};
use crate::models::response::CreateProducerResponse;
use crate::producer::dls_message::{DlsMessage, DlsMessageProducer};
use crate::producer::{ComposableMessage, MemphisProducerOptions, ProducerError};
#[cfg(feature = "schemaverse")]
use crate::schemaverse::schema::SchemaValidationError;
use crate::station::MemphisStation;
use crate::RequestError;

pub struct MemphisProducer {
    station: MemphisStation,
    options: MemphisProducerOptions,
    partitions_iterator: Option<PartitionIterator<u32>>,
}

impl MemphisProducer {
    pub(crate) async fn new(
        station: MemphisStation,
        mut options: MemphisProducerOptions,
    ) -> Result<Self, RequestError> {
        sanitize_name(&mut options.producer_name, options.generate_unique_suffix);

        let req = CreateProducerRequest {
            producer_name: &options.producer_name,
            station_name: &station.options.station_name,
            connection_id: &station.memphis_client.connection_id,
            producer_type: "application",
            req_version: 2,
            username: &station.memphis_client.username,
        };

        let res = match station
            .memphis_client
            .send_internal_request(&req, MemphisSpecialStation::ProducerCreations)
            .await
        {
            Ok(res) => res,
            Err(e) => {
                error!("Failed to create producer: {}", e);
                return Err(e);
            }
        };

        let res = std::str::from_utf8(&res.payload)
            .map_err(|e| RequestError::MemphisError(e.to_string()))?;

        let producer = match serde_json::from_str::<CreateProducerResponse>(res) {
            Ok(x) => {
                let partitions_iterator = if let Some(partitions) = x.partitions_update {
                    Some(PartitionIterator::new(partitions.partitions_list))
                } else {
                    None
                };

                Self {
                    station,
                    options,
                    partitions_iterator,
                }
            }
            Err(e) => {
                if res.is_empty() {
                    Self {
                        station,
                        options,
                        partitions_iterator: None,
                    }
                } else {
                    error!("Error creating producer: {}", e);
                    return Err(RequestError::MemphisError(e.to_string()));
                }
            }
        };
        Ok(producer)
    }

    pub async fn produce(&self, mut message: ComposableMessage) -> Result<(), ProducerError> {
        if message.payload.is_empty() {
            return Err(ProducerError::PayloadEmpty);
        }

        message.headers.insert(
            MemphisHeaders::MemphisProducedBy,
            self.options.producer_name.as_str(),
        );
        message.headers.insert(
            MemphisHeaders::MemphisConnectionId,
            self.station.memphis_client.connection_id.as_str(),
        );

        if let Some(msg_id) = &message.msg_id {
            message
                .headers
                .insert(MemphisHeaders::MessageId, msg_id.as_str());
        }

        #[cfg(feature = "schemaverse")]
        self.validate_message(&message)
            .await
            .map_err(ProducerError::SchemaValidationError)?;

        match &self.partitions_iterator {
            None => {
                if let Err(e) = self
                    .station
                    .memphis_client
                    .get_broker_connection()
                    .publish_with_headers(
                        format!(
                            "{}.final",
                            get_internal_name(&self.station.get_internal_name(None))
                        ),
                        message.headers,
                        message.payload,
                    )
                    .await
                {
                    error!("Could not publish message. {}", e);
                    return Err(ProducerError::RequestError(RequestError::NatsError(
                        e.into(),
                    )));
                }
            }
            Some(iterator) => {
                let partition = if let Some(partition) = iterator.next() {
                    partition
                } else {
                    error!("No partitions available.");
                    return Err(ProducerError::NoPartitionsAvailable);
                };
                if let Err(e) = self
                    .station
                    .memphis_client
                    .get_broker_connection()
                    .publish_with_headers(
                        format!(
                            "{}.final",
                            &self.station.get_internal_name(Some(*partition))
                        ),
                        message.headers,
                        message.payload,
                    )
                    .await
                {
                    error!("Could not publish message. {}", e);
                    return Err(ProducerError::RequestError(RequestError::NatsError(
                        e.into(),
                    )));
                }
            }
        }

        debug!(
            "Producer {} published message into station {}.",
            &self.options.producer_name, &self.station.options.station_name
        );
        Ok(())
    }

    pub async fn destroy(self) -> Result<(), RequestError> {
        let req = DestroyProducerRequest {
            producer_name: &self.options.producer_name,
            station_name: &self.station.options.station_name,
            connection_id: &self.station.memphis_client.connection_id,
            username: &self.station.memphis_client.username,
            req_version: 1,
        };

        if let Err(e) = self
            .station
            .memphis_client
            .send_internal_request(&req, MemphisSpecialStation::ProducerDestructions)
            .await
        {
            error!("Error destroying producer. {}", &e);
            return Err(e);
        }

        info!("Destroyed producer {}.", &self.options.producer_name);

        Ok(())
    }

    pub fn get_name(&self) -> String {
        self.options.producer_name.clone()
    }
}

#[cfg(feature = "schemaverse")]
impl MemphisProducer {
    async fn validate_message(
        &self,
        message: &ComposableMessage,
    ) -> Result<(), SchemaValidationError> {
        let Some(schema_validator) = self.station.schema.clone() else {
            return Ok(());
        };

        if let Err(e) = schema_validator.validate(&message.payload) {
            self.send_notification(&message, &e).await?;

            if self.station.options.send_schema_failed_msg_to_dls {
                self.send_message_to_dls(message, &e).await?;
            }

            return Err(e);
        }

        Ok(())
    }

    async fn send_notification(
        &self,
        message: &ComposableMessage,
        e: &SchemaValidationError,
    ) -> Result<(), SchemaValidationError> {
        self.station
            .memphis_client
            .send_notification(
                MemphisNotificationType::SchemaValidationFailAlert,
                "Schema validation has failed ",
                &format!(
                    "Station {}\nProducer: {}\nError: {}",
                    &self.station.options.station_name, &self.options.producer_name, &e
                ),
                std::str::from_utf8(&message.payload).unwrap_or("no valid utf8 supplied"),
            )
            .await?;
        Ok(())
    }

    async fn send_message_to_dls(
        &self,
        message: &ComposableMessage,
        e: &SchemaValidationError,
    ) -> Result<(), SchemaValidationError> {
        let req = DlsMessage {
            station_name: &self.station.options.station_name,
            producer: DlsMessageProducer {
                name: &self.options.producer_name,
                connection_id: &self.station.memphis_client.connection_id,
            },
            message,
            validation_error: &e.to_string(),
        };

        self.station
            .memphis_client
            .send_internal_request(&req, MemphisSpecialStation::MemphisSchemaverseDls)
            .await?;
        Ok(())
    }
}
