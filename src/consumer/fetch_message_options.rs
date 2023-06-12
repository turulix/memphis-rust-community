struct FetchMessageOptions {
    pub consumer_name: Option<String>,
    pub station_name: Option<String>,
    pub consumer_group: Option<String>,
    pub batch_size: i32,
    pub batch_max_time_to_wait_ms: i32,
    pub max_ack_time_ms: i32,
    pub max_msg_deliveries: i32,
    pub generate_unique_suffix: Option<bool>,
    pub start_consume_from_sequence: i32,
    pub last_messages: i32,
    pub prefetch: Option<bool>,
}

impl Default for FetchMessageOptions {
    fn default() -> Self {
        FetchMessageOptions {
            consumer_name: None,
            station_name: None,
            consumer_group: None,
            batch_size: 1,
            batch_max_time_to_wait_ms: 5_000,
            max_ack_time_ms: 30_000,
            max_msg_deliveries: 10,
            generate_unique_suffix: None,
            start_consume_from_sequence: 0,
            last_messages: -1,
            prefetch: None,
        }
    }
}
