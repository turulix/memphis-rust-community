use async_nats::jetstream::context::PublishError;
use thiserror::Error;

#[cfg(feature = "schemaverse")]
use crate::schemaverse::schema::SchemaValidationError;

#[derive(Error, Debug)]
pub enum ProducerError {
    #[error("NatsPublishError: {0}")]
    NatsPublishError(#[from] PublishError),

    #[error("The payload is empty.")]
    PayloadEmpty,

    /// The payload does not match the schema.
    #[cfg(feature = "schemaverse")]
    #[error("SchemaValidationError: {0}")]
    SchemaValidationError(SchemaValidationError),

    /// The partition provided is not valid.
    /// This is returned when a partition is provided, but is not in the list of valid partitions.
    #[error("Partition '{0}' not valid")]
    PartitionNotValid(u32),

    /// In newer versions of Memphis, a partition is required for all messages.
    /// This is returned when a partition is not provided, but required by Memphis.
    #[error("PartitionRequired")]
    PartitionRequired,

    #[error("PartitionUnavailable")]
    PartitionUnavailable,
}
