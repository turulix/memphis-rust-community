use log::{debug, error, info};

use crate::constants::memphis_constants::MemphisSpecialStation;
use crate::helper::memphis_util::{get_internal_name, sanitize_name};
use crate::memphis_client::MemphisClient;
use crate::models::request::{CreateProducerRequest, DestroyProducerRequest};
use crate::producer::{ComposableMessage, MemphisProducerOptions, ProducerError};
use crate::RequestError;

pub struct MemphisProducer {
    memphis_client: MemphisClient,
    options: MemphisProducerOptions,
}

impl MemphisProducer {
    pub(crate) async fn new(client: MemphisClient, mut options: MemphisProducerOptions) -> Result<Self, RequestError> {
        sanitize_name(&mut options.producer_name, options.generate_unique_suffix);

        let req = CreateProducerRequest {
            producer_name: &options.producer_name,
            station_name: &options.station_name,
            producer_type: "application",
            connection_id: &client.connection_id,
            username: &client.username,
        };

        if let Err(e) = client.send_internal_request(&req, MemphisSpecialStation::ProducerCreations).await {
            error!("Error creating producer. {}", &e);
            return Err(e);
        }

        Ok(Self {
            memphis_client: client,
            options,
        })
    }

    pub async fn produce(&self, mut message: ComposableMessage) -> Result<(), ProducerError> {
        if message.payload.is_empty() {
            return Err(ProducerError::PayloadEmpty);
        }

        message.headers.insert("$memphis_producedBy", self.options.producer_name.as_str());
        message.headers.insert("$memphis_connectionId", self.memphis_client.connection_id.as_str());

        if let Some(msg_id) = message.msg_id {
            message.headers.insert("msg-id", msg_id.as_str());
        }

        if let Err(e) = self
            .memphis_client
            .get_broker_connection()
            .publish_with_headers(
                format!("{}.final", get_internal_name(&self.options.station_name)),
                message.headers,
                message.payload,
            )
            .await
        {
            error!("Could not publish message. {}", e);
            return Err(ProducerError::RequestError(RequestError::NatsError(e.into())));
        }

        debug!(
            "Producer {} published message into station {}.",
            &self.options.producer_name, &self.options.station_name
        );
        Ok(())
    }

    pub async fn destroy(self) -> Result<(), RequestError> {
        let req = DestroyProducerRequest {
            producer_name: &self.options.producer_name,
            station_name: &self.options.station_name,
            connection_id: &self.memphis_client.connection_id,
            username: &self.memphis_client.username,
        };

        if let Err(e) = self
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
}
