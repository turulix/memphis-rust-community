use nats::Message;
use crate::memphis_client::MemphisClient;
use crate::models::request::pm_ack_msg::PmAckMsg;

pub struct MemphisMessage {
    msg: Message,
    memphisClient: MemphisClient,
    consumerGroup: String,
    macAckTimeMs: i32,
}

impl MemphisMessage {
    fn new(msg: Message, memphisClient: MemphisClient, consumerGroup: String, macAckTimeMs: i32) -> Self {
        MemphisMessage {
            msg,
            memphisClient,
            consumerGroup,
            macAckTimeMs,
        }
    }

    // fn ack(&self) -> Result<(), ()> {
    //     let res = self.msg.ack();
    //     if res.is_err() {
    //         if self.msg.headers?.contains_key("$memphis_pm_id") {
    //             let msg_to_ack_model = PmAckMsg {
    //                 id: self.msg.headers?["$memphis_pm_id"].to_string(),
    //                 consumer_group_name: self.consumerGroup.clone(),
    //             };
    //             let msg_to_ack_json = serde_json::to_string(&msg_to_ack_model)?;
    //             let msg_to_ack_bytes = msg_to_ack_json.as_bytes();
    //
    //             todo!("Acknowledge the message")
    //         }
    //         todo!("Error while acknowledging the message")
    //     }
    //     return Ok(());
    // }
}
