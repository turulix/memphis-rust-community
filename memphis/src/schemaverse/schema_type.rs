#[derive(Debug)]
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
            _ => panic!("unknown SchemaType"),
        }
    }
}
