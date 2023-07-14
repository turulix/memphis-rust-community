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
}
