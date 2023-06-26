#[derive(Debug, Clone)]
pub struct MemphisConsumerOptions {
    pub station_name: String,
    pub consumer_name: String,
    pub consumer_group: String,
    pub pull_interval_ms: i32,
    pub batch_size: usize,
    pub batch_max_time_to_wait_ms: u64,
    pub max_ack_time_ms: i32,
    pub max_msg_deliveries: i32,
    pub generate_unique_suffix: bool,
    pub start_consume_from_sequence: i32,
    pub last_messages: i32,
}

impl Default for MemphisConsumerOptions {
    fn default() -> Self {
        MemphisConsumerOptions {
            station_name: String::from("Default_Station_Name"),
            consumer_name: String::from("Default_Consumer_Name"),
            consumer_group: String::from(""),
            pull_interval_ms: 1_000,
            batch_size: 10,
            batch_max_time_to_wait_ms: 5_000,
            max_ack_time_ms: 30_000,
            max_msg_deliveries: 10,
            generate_unique_suffix: false,
            start_consume_from_sequence: 1,
            last_messages: -1,
        }
    }
}

impl MemphisConsumerOptions {
    pub fn new(station_name: &str, consumer_name: &str) -> Self {
        MemphisConsumerOptions {
            station_name: station_name.to_string(),
            consumer_name: consumer_name.to_string(),
            ..Default::default()
        }
    }

    pub fn with_station_name(mut self, station_name: String) -> Self {
        self.station_name = station_name;
        self
    }

    pub fn with_consumer_name(mut self, consumer_name: String) -> Self {
        self.consumer_name = consumer_name;
        self
    }

    pub fn with_consumer_group(mut self, consumer_group: &str) -> Self {
        self.consumer_group = String::from(consumer_group);
        self
    }

    pub fn with_pull_interval_ms(mut self, pull_interval_ms: i32) -> Self {
        self.pull_interval_ms = pull_interval_ms;
        self
    }

    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn with_batch_max_time_to_wait_ms(mut self, batch_max_time_to_wait_ms: u64) -> Self {
        self.batch_max_time_to_wait_ms = batch_max_time_to_wait_ms;
        self
    }

    pub fn with_max_ack_time_ms(mut self, max_ack_time_ms: i32) -> Self {
        self.max_ack_time_ms = max_ack_time_ms;
        self
    }

    pub fn with_max_msg_deliveries(mut self, max_msg_deliveries: i32) -> Self {
        self.max_msg_deliveries = max_msg_deliveries;
        self
    }

    pub fn with_generate_unique_suffix(mut self, generate_unique_suffix: bool) -> Self {
        self.generate_unique_suffix = generate_unique_suffix;
        self
    }

    pub fn with_start_consume_from_sequence(mut self, start_consume_from_sequence: i32) -> Self {
        self.start_consume_from_sequence = start_consume_from_sequence;
        self
    }

    pub fn with_last_messages(mut self, last_messages: i32) -> Self {
        self.last_messages = last_messages;
        self
    }
}
