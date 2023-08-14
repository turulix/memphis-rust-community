#[derive(Debug)]
pub struct MemphisStationsOptions {
    pub station_name: String,
    pub retention_type: RetentionType,
    pub retention_value: u32,
    pub storage_type: StorageType,
    pub replicas: u32,
    pub idempotency_window_ms: u32,
    pub schema_name: String,
    pub send_poison_msg_to_dls: bool,
    pub send_schema_failed_msg_to_dls: bool,
    pub tiered_storage_enabled: bool,
    pub partition_number: u32,
}

#[derive(Debug, Default)]
pub enum RetentionType {
    #[default]
    MessageAgeSec,
    Messages,
    Bytes,
    AckBased,
}

#[derive(Debug, Default)]
pub enum StorageType {
    #[default]
    File,
    Memory,
}

impl MemphisStationsOptions {
    pub fn new(station_name: &str) -> Self {
        MemphisStationsOptions {
            station_name: station_name.to_string(),
            ..Default::default()
        }
    }

    pub fn with_retention_type(mut self, retention_type: RetentionType) -> Self {
        self.retention_type = retention_type;
        self
    }

    pub fn with_retention_value(mut self, retention_value: u32) -> Self {
        self.retention_value = retention_value;
        self
    }

    pub fn with_storage_type(mut self, storage_type: StorageType) -> Self {
        self.storage_type = storage_type;
        self
    }

    pub fn with_replicas(mut self, replicas: u32) -> Self {
        self.replicas = replicas;
        self
    }

    pub fn with_idempotency_window_ms(mut self, idempotency_window_ms: u32) -> Self {
        self.idempotency_window_ms = idempotency_window_ms;
        self
    }

    pub fn with_schema_name(mut self, schema_name: String) -> Self {
        self.schema_name = schema_name;
        self
    }
    pub fn with_send_poison_msg_to_dls(mut self, send_poison_msg_to_dls: bool) -> Self {
        self.send_poison_msg_to_dls = send_poison_msg_to_dls;
        self
    }
    pub fn with_send_schema_failed_msg_to_dls(
        mut self,
        send_schema_failed_msg_to_dls: bool,
    ) -> Self {
        self.send_schema_failed_msg_to_dls = send_schema_failed_msg_to_dls;
        self
    }
    pub fn with_tiered_storage_enabled(mut self, tiered_storage_enabled: bool) -> Self {
        self.tiered_storage_enabled = tiered_storage_enabled;
        self
    }
    pub fn with_partition_number(mut self, partition_number: u32) -> Self {
        self.partition_number = partition_number;
        self
    }
}

impl Default for MemphisStationsOptions {
    fn default() -> Self {
        Self {
            station_name: "".to_string(),
            retention_type: Default::default(),
            retention_value: 604800,
            storage_type: Default::default(),
            replicas: 1,
            idempotency_window_ms: 12000,
            schema_name: "".to_string(),
            send_poison_msg_to_dls: true,
            send_schema_failed_msg_to_dls: true,
            tiered_storage_enabled: false,
            partition_number: 1,
        }
    }
}

impl ToString for RetentionType {
    fn to_string(&self) -> String {
        match self {
            RetentionType::MessageAgeSec => "message_age_sec",
            RetentionType::Messages => "messages",
            RetentionType::Bytes => "bytes",
            RetentionType::AckBased => "ack_based",
        }
        .to_string()
    }
}

impl ToString for StorageType {
    fn to_string(&self) -> String {
        match self {
            StorageType::File => "file",
            StorageType::Memory => "memory",
        }
        .to_string()
    }
}
