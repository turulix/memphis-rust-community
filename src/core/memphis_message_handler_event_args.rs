use nats::jetstream::JetStream;
use crate::core::memphis_message::MemphisMessage;

struct MemphisMessageHandlerEventArgs {
    message_list: Vec<MemphisMessage>,
    context: Option<JetStream>,
}

impl MemphisMessageHandlerEventArgs {
    fn new(message_list: Vec<MemphisMessage>, context: Option<JetStream>) -> Self {
        MemphisMessageHandlerEventArgs {
            message_list,
            context,
        }
    }

    fn get_message_list(&self) -> &Vec<MemphisMessage> {
        &self.message_list
    }

    fn get_context(&self) -> &Option<JetStream> {
        &self.context
    }
}
