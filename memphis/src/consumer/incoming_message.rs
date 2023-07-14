use std::fmt::{Debug, Formatter};
use std::string::FromUtf8Error;
use std::sync::Arc;
use std::time::Duration;

use async_nats::jetstream::{AckKind, Message};
use async_nats::{Error, HeaderMap};

use crate::constants::memphis_constants::MemphisSubjects;
use crate::memphis_client::MemphisClient;
use crate::models::request::pm_ack_msg::PmAckMsg;

#[derive(Clone)]
pub struct MemphisMessage {
    msg: Arc<Message>,
    memphis_client: MemphisClient,
    consumer_group: String,
    pub max_ack_time_ms: i32,
}

impl MemphisMessage {
    pub fn new(msg: Message, memphis_client: MemphisClient, consumer_group: String, max_ack_time_ms: i32) -> Self {
        MemphisMessage {
            msg: Arc::new(msg),
            memphis_client,
            consumer_group,
            max_ack_time_ms,
        }
    }

    /// Acknowledges the message. Causes the message to be marked as processed and removed from the queue.
    pub async fn ack(&self) -> Result<(), Error> {
        let res = self.msg.ack().await;
        return match res {
            Ok(_) => Ok(()),
            Err(e) => {
                if let Some(header) = &self.msg.headers {
                    if let Some(memphis_id) = header.get("$memphis_pm_id") {
                        let msg_to_ack_model = PmAckMsg {
                            id: memphis_id.to_string(),
                            consumer_group_name: self.consumer_group.clone(),
                        };
                        let msg_to_ack_json = serde_json::to_string(&msg_to_ack_model)?;
                        let msg_to_ack_bytes = bytes::Bytes::from(msg_to_ack_json);
                        self.memphis_client
                            .get_broker_connection()
                            .publish(MemphisSubjects::PmResendAckSubj.to_string(), msg_to_ack_bytes)
                            .await?;
                    }
                }
                Err(e)
            }
        };
    }

    /// Get the payload of the underlying NATS message.
    pub fn get_data(&self) -> &bytes::Bytes {
        &self.msg.payload
    }

    pub fn get_data_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.msg.payload.to_vec())
    }

    /// Get the headers of the underlying NATS message.
    pub fn get_headers(&self) -> &Option<HeaderMap> {
        &self.msg.headers
    }

    /// Delay the message for the specified duration.
    ///
    /// # Arguments
    ///
    /// * `delay` - The duration to delay the message.
    pub async fn delay(&self, delay: Duration) -> Result<(), ()> {
        if let Some(headers) = self.get_headers() {
            if let Some(_msg_id) = headers.get("$memphis_pm_id") {
                return match self.msg.ack_with(AckKind::Nak(Some(delay))).await {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                };
            }
            if let Some(_cg_name) = headers.get("$memphis_pm_cg_name") {
                return match self.msg.ack_with(AckKind::Nak(Some(delay))).await {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                };
            }
        }
        Err(())
    }
}

impl Debug for MemphisMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data = self.get_data_as_string().unwrap_or_else(|_| format!("{:x?}", self.get_data()));

        f.debug_struct("MemphisMessage")
            .field("msg", &data)
            .field("consumer_group", &self.consumer_group)
            .field("max_ack_time_ms", &self.max_ack_time_ms)
            .finish_non_exhaustive()
    }
}
