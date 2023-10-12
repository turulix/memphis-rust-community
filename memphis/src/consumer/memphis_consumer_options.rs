use std::time::Duration;

/// Memphis Consumer Options
///
/// # Example
/// ```rust
/// use memphis_rust_community::consumer::MemphisConsumerOptions;
/// use std::time::Duration;
/// #[tokio::main]
/// async fn main() {
///         let options = MemphisConsumerOptions::new("station_name")
///         .with_consumer_group("consumer_group")
///         .with_generate_unique_suffix(true)
///         .with_pull_interval(Duration::from_secs(1))
///         .with_batch_size(10);
/// }
#[derive(Debug, Clone)]
pub struct MemphisConsumerOptions {
    pub consumer_name: String,
    pub consumer_group: String,
    pub pull_interval: Duration,
    pub batch_size: usize,
    pub batch_max_time_to_wait: Duration,
    pub max_ack_time: Duration,
    pub max_msg_deliveries: i32,
    pub generate_unique_suffix: bool,
    pub start_consume_from_sequence: i32,
    pub last_messages: i32,
}

impl Default for MemphisConsumerOptions {
    fn default() -> Self {
        MemphisConsumerOptions {
            consumer_name: String::from("Default_Consumer_Name"),
            consumer_group: String::from(""),
            pull_interval: Duration::from_secs(1),
            batch_size: 10,
            batch_max_time_to_wait: Duration::from_secs(5),
            max_ack_time: Duration::from_secs(30),
            max_msg_deliveries: 10,
            generate_unique_suffix: false,
            start_consume_from_sequence: 1,
            last_messages: -1,
        }
    }
}

impl MemphisConsumerOptions {
    pub fn new(consumer_name: &str) -> Self {
        MemphisConsumerOptions {
            consumer_name: consumer_name.to_string(),
            ..Default::default()
        }
    }

    pub fn with_consumer_name(mut self, consumer_name: String) -> Self {
        self.consumer_name = consumer_name;
        self
    }

    pub fn with_consumer_group(mut self, consumer_group: &str) -> Self {
        self.consumer_group = String::from(consumer_group);
        self
    }

    pub fn with_pull_interval(mut self, pull_interval: Duration) -> Self {
        self.pull_interval = pull_interval;
        self
    }

    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn with_batch_max_time_to_wait(mut self, batch_max_time_to_wait: Duration) -> Self {
        self.batch_max_time_to_wait = batch_max_time_to_wait;
        self
    }

    pub fn with_max_ack_time(mut self, max_ack_time: Duration) -> Self {
        self.max_ack_time = max_ack_time;
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
