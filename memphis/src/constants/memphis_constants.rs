use async_nats::header::IntoHeaderName;
use async_nats::HeaderName;

pub(crate) enum MemphisSpecialStation {
    ProducerCreations,
    ConsumerCreations,
    StationCreations,

    ProducerDestructions,
    ConsumerDestructions,
    StationDestructions,

    SchemaAttachments,
    SchemaDetachments,

    Notifications,

    MemphisSchemaverseDls,
}

impl ToString for MemphisSpecialStation {
    fn to_string(&self) -> String {
        match self {
            Self::ProducerCreations => String::from("$memphis_producer_creations"),
            Self::ConsumerCreations => String::from("$memphis_consumer_creations"),
            Self::StationCreations => String::from("$memphis_station_creations"),

            Self::ProducerDestructions => String::from("$memphis_producer_destructions"),
            Self::ConsumerDestructions => String::from("$memphis_consumer_destructions"),
            Self::StationDestructions => String::from("$memphis_station_destructions"),

            Self::SchemaAttachments => String::from("$memphis_schema_attachments"),
            Self::SchemaDetachments => String::from("$memphis_schema_detachments"),

            Self::Notifications => String::from("$memphis_notifications"),

            Self::MemphisSchemaverseDls => String::from("$memphis_schemaverse_dls"),
        }
    }
}

pub(crate) enum MemphisHeaders {
    MessageId,
    MemphisProducedBy,
    MemphisConnectionId,
}

impl MemphisHeaders {
    pub fn as_str(&self) -> &str {
        match self {
            Self::MessageId => "msg-id",
            Self::MemphisProducedBy => "$memphis_producedBy",
            Self::MemphisConnectionId => "$memphis_connectionId",
        }
    }
}

impl IntoHeaderName for MemphisHeaders {
    fn into_header_name(self) -> HeaderName {
        self.as_str().into_header_name()
    }
}

pub(crate) enum MemphisSubscriptions {
    DlsPrefix,
}

impl ToString for MemphisSubscriptions {
    fn to_string(&self) -> String {
        match self {
            MemphisSubscriptions::DlsPrefix => String::from("$memphis_dls_"),
        }
    }
}

#[allow(dead_code)]
pub(crate) enum MemphisSubjects {
    PmResendAckSubj,
    MemphisSchemaUpdate,
    SdkClientsUpdate,
    MemphisSchemaVerseDls,
}

impl ToString for MemphisSubjects {
    fn to_string(&self) -> String {
        match self {
            Self::PmResendAckSubj => String::from("$memphis_pm_acks"),
            Self::MemphisSchemaUpdate => String::from("$memphis_schema_updates_"),
            Self::SdkClientsUpdate => String::from("$memphis_sdk_clients_updates"),
            Self::MemphisSchemaVerseDls => String::from("$memphis_schemaverse_dls"),
        }
    }
}

pub enum MemphisNotificationType {
    SchemaValidationFailAlert,
}

impl ToString for MemphisNotificationType {
    fn to_string(&self) -> String {
        match self {
            Self::SchemaValidationFailAlert => String::from("schema_validation_fail_alert"),
        }
    }
}