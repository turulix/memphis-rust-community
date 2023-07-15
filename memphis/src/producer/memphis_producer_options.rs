pub struct MemphisProducerOptions {
    pub(crate) station_name: String,
    pub producer_name: String,
    pub generate_unique_suffix: bool,
}

impl Default for MemphisProducerOptions {
    fn default() -> Self {
        Self {
            station_name: "Default_Station_Name".to_string(),
            producer_name: "Default_Producer_name".to_string(),
            generate_unique_suffix: true,
        }
    }
}

impl MemphisProducerOptions {
    pub fn new(producer_name: &str) -> Self {
        MemphisProducerOptions {
            producer_name: producer_name.to_string(),
            ..Default::default()
        }
    }

    pub fn with_generate_unique_suffix(mut self, generate_unique_suffix: bool) -> Self {
        self.generate_unique_suffix = generate_unique_suffix;
        self
    }
}
