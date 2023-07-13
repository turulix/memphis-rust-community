use std::borrow::Cow;
use std::fmt::Display;
use bytes::Bytes;

use jsonschema::{Draft, JSONSchema, ValidationError};
use thiserror::Error;

use crate::schemaverse::schema::{ErrorData, SchemaValidationError, SchemaValidator};

pub struct JsonSchemaValidator {
    schema: JSONSchema,
}

impl JsonSchemaValidator {
    pub fn new(value: serde_json::Value) -> Result<Self, JsonSchemaError> {
        let schema = JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&value)
            .map_err(|e| JsonSchemaError::SchemaFileError(validation_error_to_owned(e)))?;

        Ok(Self { schema })
    }
}

impl SchemaValidator for JsonSchemaValidator {
    fn validate(&self, message: &Bytes) -> Result<(), SchemaValidationError> {
        let deserialized = serde_json::from_slice(message).map_err(|e| JsonSchemaError::from(e))?;

        if let Err(mut e) = self.schema.validate(&deserialized) {
            let Some(error) = e.next() else {
                return Err(JsonSchemaError::UnknownError.into());
            };

            return Err(JsonSchemaError::ValidationError(validation_error_to_owned(error)).into());
        }

        Ok(())
    }

    fn from_bytes(bytes: &Bytes) -> Result<Self, SchemaValidationError> {
        let deserialized = serde_json::from_slice(bytes).map_err(|e| JsonSchemaError::from(e))?;

        Ok(Self::new(deserialized)?)
    }
}

fn validation_error_to_owned(e: ValidationError) -> ValidationError<'static> {
    ValidationError {
        instance: Cow::Owned(e.instance.into_owned()),
        kind: e.kind,
        instance_path: e.instance_path.to_owned(),
        schema_path: e.schema_path.to_owned(),
    }
}

#[derive(Debug, Error)]
pub enum JsonSchemaError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Error while parsing schema: {0}")]
    SchemaFileError(ValidationError<'static>),

    #[error("Serde Error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Error while validating message: {0}")]
    ValidationError(ValidationError<'static>),

    #[error("Unknown Error")]
    UnknownError,
}

impl ErrorData for JsonSchemaError {}
