pub struct MemphisProducerOptions {
    pub station_name: String,
    pub producer_name: String,
    pub generate_unique_suffix: bool,
    pub max_ack_time_ms: i32,
}


impl MemphisProducerOptions {
    fn new(station_name: String, producer_name: String, generate_unique_suffix: bool, max_ack_time_ms: Option<i32>) -> Self {
        MemphisProducerOptions {
            station_name,
            producer_name,
            generate_unique_suffix,
            max_ack_time_ms: max_ack_time_ms.unwrap_or(30_000),
        }
    }
}
