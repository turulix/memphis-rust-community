use crate::consumer::memphis_consumer_options::MemphisConsumerOptions;
use crate::core::memphis_message::MemphisMessage;
use crate::helper::memphis_util::get_internal_name;
use crate::memphis_client::MemphisClient;

struct MemphisConsumer {
    memphis_client: MemphisClient,
    options: MemphisConsumerOptions,
    internal_station_name: String,
}


impl MemphisConsumer {
    fn new(memphis_client: MemphisClient, options: MemphisConsumerOptions) -> Self {
        MemphisConsumer {
            memphis_client,
            options: options.clone(),
            //TODO: Do not just Unwrap this.
            internal_station_name: get_internal_name(&options.station_name.unwrap()),
        }
    }

    fn consume_async(&self) {
        todo!()
    }

    fn destroy_async(&self) {
        todo!()
    }

    fn fetch(&self, batch_size: i32, prefetch: bool) {
        todo!()
    }

    fn try_get_and_remove_prefetched_messages(&self, batch_size: i32) -> Option<Vec<MemphisMessage>> {
        todo!()
    }

    fn prefetch(&self) {
        todo!()
    }

    fn ping_consumer(&self) {
        todo!()
    }

    fn stop_consumer(&self) {
        todo!()
    }

    fn fetch_subscription_with_time_out(&self, batch_size: i32) {
        todo!()
    }

    fn consume(&self) {
        todo!()
    }

    fn consume_from_dls(&self) {
        todo!()
    }

    fn is_subscription_active(&self) -> bool {
        todo!()
    }

    fn enqueue_dls_message(&self, memphis_message: MemphisMessage) {
        todo!()
    }

    fn dequeue_dls_messages(&self, messages: &mut Vec<MemphisMessage>) {
        todo!()
    }

    fn dispose(&self) {
        todo!()
    }
}
