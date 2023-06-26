#[allow(dead_code)]
pub(crate) enum MemphisStations {
    ProducerCreations,
    ConsumerCreations,
    StationCreations,

    ProducerDestructions,
    ConsumerDestructions,

    SchemaAttachments,
    SchemaDetachments,

    Notifications,

    StationDestruction,
}

impl ToString for MemphisStations {
    fn to_string(&self) -> String {
        match self {
            MemphisStations::ProducerCreations => String::from("$memphis_producer_creations"),
            MemphisStations::ConsumerCreations => String::from("$memphis_consumer_creations"),
            MemphisStations::StationCreations => String::from("$memphis_station_creations"),

            MemphisStations::ProducerDestructions => String::from("$memphis_producer_destructions"),
            MemphisStations::ConsumerDestructions => String::from("$memphis_consumer_destructions"),

            MemphisStations::SchemaAttachments => String::from("$memphis_schema_attachments"),
            MemphisStations::SchemaDetachments => String::from("$memphis_schema_detachments"),

            MemphisStations::Notifications => String::from("$memphis_notifications"),

            MemphisStations::StationDestruction => String::from("$memphis_station_destructions"),
        }
    }
}

#[allow(dead_code)]
pub(crate) enum MemphisHeaders {
    MessageId,
    MemphisProducedBy,
    MemphisConnectionId,
}

impl ToString for MemphisHeaders {
    fn to_string(&self) -> String {
        match self {
            MemphisHeaders::MessageId => String::from("msg-id"),
            MemphisHeaders::MemphisProducedBy => String::from("$memphis_producedBy"),
            MemphisHeaders::MemphisConnectionId => String::from("$memphis_connectionId"),
        }
    }
}

#[allow(dead_code)]
pub(crate) enum MemphisSubscriptions {
    DlsPrefix,
}

impl ToString for MemphisSubscriptions {
    fn to_string(&self) -> String {
        match self {
            MemphisSubscriptions::DlsPrefix => String::from("$memphis_dlq_"),
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
            MemphisSubjects::PmResendAckSubj => String::from("$memphis_pm_acks"),
            MemphisSubjects::MemphisSchemaUpdate => String::from("$memphis_schema_updates_"),
            MemphisSubjects::SdkClientsUpdate => String::from("$memphis_sdk_clients_updates"),
            MemphisSubjects::MemphisSchemaVerseDls => String::from("$memphis_schemaverse_dls"),
        }
    }
}

#[allow(dead_code)]
pub(crate) enum MemphisSchemaTypes {
    None,
    Json,
    GraphQl,
    ProtoBuf,
}

impl ToString for MemphisSchemaTypes {
    fn to_string(&self) -> String {
        match self {
            MemphisSchemaTypes::None => String::from(""),
            MemphisSchemaTypes::Json => String::from("json"),
            MemphisSchemaTypes::GraphQl => String::from("graphql"),
            MemphisSchemaTypes::ProtoBuf => String::from("protobuf"),
        }
    }
}

#[allow(dead_code)]
pub(crate) enum MemphisSdkClientUpdateTypes {
    SendNotification,
    SchemaVerseToDls,
    RemoveStation,
}

impl ToString for MemphisSdkClientUpdateTypes {
    fn to_string(&self) -> String {
        match self {
            MemphisSdkClientUpdateTypes::SendNotification => String::from("send_notification"),
            MemphisSdkClientUpdateTypes::SchemaVerseToDls => String::from("schemaverse_to_dls"),
            MemphisSdkClientUpdateTypes::RemoveStation => String::from("remove_station"),
        }
    }
}

#[allow(dead_code)]
pub(crate) enum MemphisGlobalVariables {
    GlobalAccountName,
}

impl ToString for MemphisGlobalVariables {
    fn to_string(&self) -> String {
        match self {
            MemphisGlobalVariables::GlobalAccountName => String::from("$memphis"),
        }
    }
}
