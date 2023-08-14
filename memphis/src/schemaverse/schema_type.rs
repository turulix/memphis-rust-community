use log::error;
use thiserror::Error;

use crate::schemaverse::schema::json::JsonSchemaValidator;
use crate::schemaverse::schema::{SchemaValidationError, SchemaValidator};

#[derive(Debug, Clone)]
pub enum SchemaType {
    #[cfg(feature = "validator_json")]
    Json,
    #[cfg(feature = "validator_graphql")]
    GraphQL,
    #[cfg(feature = "validator_protobuf")]
    Protobuf,
}

impl ToString for SchemaType {
    fn to_string(&self) -> String {
        match self {
            #[cfg(feature = "validator_json")]
            SchemaType::Json => "json".to_string(),
            #[cfg(feature = "validator_graphql")]
            SchemaType::GraphQL => "graphql".to_string(),
            #[cfg(feature = "validator_protobuf")]
            SchemaType::Protobuf => "protobuf".to_string(),
        }
    }
}

impl SchemaType {
    pub fn from_str(data: &str) -> Result<Self, SchemaTypeError> {
        return Ok(match data {
            #[cfg(feature = "validator_json")]
            "json" => Self::Json,

            #[cfg(feature = "validator_graphql")]
            "graphql" => Self::GraphQL,

            #[cfg(feature = "validator_protobuf")]
            "protobuf" => Self::Protobuf,

            _ => return Err(SchemaTypeError::InvalidType(data.to_string())),
        });
    }

    pub(crate) fn create_validator(&self, data: &str) -> Result<Box<dyn SchemaValidator>, SchemaTypeError> {
        return match self {
            #[cfg(feature = "validator_json")]
            SchemaType::Json => Ok(Box::new(JsonSchemaValidator::from_str(&data)?)),

            #[cfg(feature = "validator_graphql")]
            SchemaType::GraphQL => Err(SchemaTypeError::InvalidType(data.to_string())),

            #[cfg(feature = "validator_protobuf")]
            SchemaType::Protobuf => Err(SchemaTypeError::InvalidType(data.to_string())),

            _ => Err(SchemaTypeError::InvalidType(data.to_string())),
        };
    }
}

#[derive(Debug, Error)]
pub enum SchemaTypeError {
    #[error("Type {0} not supported")]
    InvalidType(String),

    #[error("SchemaValidationError: {0}")]
    SchemaValidatorError(#[from] SchemaValidationError),
}
