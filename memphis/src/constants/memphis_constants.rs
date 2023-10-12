use async_nats::header::IntoHeaderName;
use async_nats::HeaderName;

pub(crate) enum MemphisSpecialStation {
    ProducerCreations,
    ConsumerCreations,
    StationCreations,

    ProducerDestructions,
    ConsumerDestructions,
    StationDestructions,

    #[allow(dead_code)]
    SchemaAttachments,
    #[allow(dead_code)]
    SchemaDetachments,

    #[cfg(feature = "schemaverse")]
    Notifications,
    #[cfg(feature = "schemaverse")]
    MemphisSchemaverseDls,

    PmAcks,
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

            #[cfg(feature = "schemaverse")]
            Self::Notifications => String::from("$memphis_notifications"),

            #[cfg(feature = "schemaverse")]
            Self::MemphisSchemaverseDls => String::from("$memphis_schemaverse_dls"),

            Self::PmAcks => String::from("$memphis_pm_acks"),
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
    #[allow(dead_code)]
    SchemaUpdatesPrefix,
}

impl ToString for MemphisSubscriptions {
    fn to_string(&self) -> String {
        match self {
            Self::DlsPrefix => String::from("$memphis_dls_"),
            Self::SchemaUpdatesPrefix => String::from("$memphis_schema_updates_"),
        }
    }
}

#[cfg(feature = "schemaverse")]
pub enum MemphisNotificationType {
    SchemaValidationFailAlert,
}

#[cfg(feature = "schemaverse")]
impl ToString for MemphisNotificationType {
    fn to_string(&self) -> String {
        match self {
            Self::SchemaValidationFailAlert => String::from("schema_validation_fail_alert"),
        }
    }
}
