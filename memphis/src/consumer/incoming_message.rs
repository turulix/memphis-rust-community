use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::string::FromUtf8Error;
use std::sync::Arc;
use std::time::Duration;

use async_nats::jetstream::{AckKind, Message};
use async_nats::HeaderMap;
use log::{error, info};
use tokio::sync::RwLock;
use tokio::task::AbortHandle;
use tokio::time::Instant;

use crate::constants::memphis_constants::MemphisSpecialStation;
use crate::memphis_client::MemphisClient;
use crate::models::request::pm_ack_msg::PmAckMsg;
use crate::RequestError;

#[derive(Clone)]
pub struct MemphisMessage {
    msg: Message,
    memphis_client: MemphisClient,
    consumer_group: String,
    known_messages: Arc<RwLock<HashSet<String>>>,
    known_message_key: String,
    abort_handle: Arc<AbortHandle>,
    pub max_ack_time: Duration,
}

impl MemphisMessage {
    pub(crate) fn new(
        msg: Message,
        memphis_client: MemphisClient,
        consumer_group: String,
        max_ack_time: Duration,
        known_message_key: String,
        known_messages: Arc<RwLock<HashSet<String>>>,
    ) -> Self {
        let msg_clone = msg.clone();
        let abort_handle = tokio::spawn(async move {
            loop {
                let safety_time_ms = max_ack_time.as_millis() as f32 * 0.1f32;
                let safety_time = Duration::from_millis(safety_time_ms.round() as u64);
                tokio::time::sleep_until(Instant::now() + max_ack_time - safety_time).await;
                info!("Sending progress ack");
                if let Err(error) = msg_clone.clone().ack_with(AckKind::Progress).await {
                    error!("Error while sending progress ack: {:?}", error);
                }
            }
        })
        .abort_handle();
        MemphisMessage {
            msg,
            memphis_client,
            consumer_group,
            max_ack_time,
            known_messages,
            known_message_key,
            abort_handle: Arc::new(abort_handle),
        }
    }

    /// Acknowledges the message. Causes the message to be marked as processed and removed from the queue.
    pub async fn ack(&self) -> Result<(), RequestError> {
        self.disable_missed_ack_safety().await;
        let res = self.msg.ack().await;
        return match res {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error while acking message: {:?}", e);
                if let Some(header) = &self.msg.headers {
                    if let Some(memphis_id) = header.get("$memphis_pm_id") {
                        if let Some(cg_group) = header.get("$memphis_pm_cg_name") {
                            let req = PmAckMsg {
                                id: memphis_id.to_string(),
                                consumer_group_name: cg_group.to_string(),
                            };

                            self.memphis_client
                                .send_internal_request(&req, MemphisSpecialStation::PmAcks)
                                .await?;
                        }
                    }
                }
                Err(e.into())
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
        self.disable_missed_ack_safety().await;
        if let Some(headers) = self.get_headers() {
            if let Some(_msg_id) = headers.get("$memphis_pm_id") {
                return match self.msg.ack_with(AckKind::Nak(Some(delay))).await {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                };
            } else if let Some(_cg_name) = headers.get("$memphis_pm_cg_name") {
                return match self.msg.ack_with(AckKind::Nak(Some(delay))).await {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                };
            }
        }
        match self.msg.ack_with(AckKind::Nak(Some(delay))).await {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    /// This function is used to disable the safety mechanism that sends a progress ack to the server
    /// if the message is not acked within the specified time.
    /// This also removes the message from the list of known messages, which have been received but not acked.
    pub async fn disable_missed_ack_safety(&self) {
        self.abort_handle.abort();
        self.known_messages
            .write()
            .await
            .remove(&self.known_message_key);
    }
}

impl Drop for MemphisMessage {
    fn drop(&mut self) {
        self.abort_handle.abort();
        let known_messages = self.known_messages.clone();
        let key = self.known_message_key.clone();
        tokio::spawn(async move {
            known_messages.write().await.remove(&key);
        });
    }
}

impl Debug for MemphisMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data = self
            .get_data_as_string()
            .unwrap_or_else(|_| format!("{:x?}", self.get_data()));

        f.debug_struct("MemphisMessage")
            .field("msg", &data)
            .field("consumer_group", &self.consumer_group)
            .field("max_ack_time_ms", &self.max_ack_time)
            .finish_non_exhaustive()
    }
}
