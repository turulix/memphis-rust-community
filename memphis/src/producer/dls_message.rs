use crate::producer::ComposableMessage;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DlsMessage<'a> {
    pub station_name: &'a str,
    pub producer: DlsMessageProducer<'a>,
    pub message: &'a ComposableMessage,
    pub validation_error: &'a str,
}

#[derive(Debug, Serialize)]
pub struct DlsMessageProducer<'a> {
    pub name: &'a str,
    pub connection_id: &'a str,
}
