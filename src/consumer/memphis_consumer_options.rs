#[derive(Debug, Clone)]
pub struct MemphisConsumerOptions {
    pub station_name: Option<String>,
    pub consumer_name: Option<String>,
    pub consumer_group: String,
    pub pull_interval_ms: i32,
    pub batch_size: i32,
    pub batch_max_time_to_wait_ms: i32,
    pub max_ack_time_ms: i32,
    pub max_msg_deliveries: i32,
    pub generate_unique_suffix: bool,
    pub start_consume_from_sequence: i32,
    pub last_messages: i32,
    pub real_name: Option<String>,
}

impl Default for MemphisConsumerOptions {
    fn default() -> Self {
        MemphisConsumerOptions {
            station_name: None,
            consumer_name: None,
            consumer_group: String::from(""),
            pull_interval_ms: 1_000,
            batch_size: 10,
            batch_max_time_to_wait_ms: 5_000,
            max_ack_time_ms: 30_000,
            max_msg_deliveries: 10,
            generate_unique_suffix: false,
            start_consume_from_sequence: 0,
            last_messages: -1,

            real_name: None,
        }
    }
}
