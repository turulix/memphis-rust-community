#[derive(Debug)]
pub enum SchemaType {
    Json,
    GraphQL,
    Protobuf
}

impl ToString for SchemaType {
    fn to_string(&self) -> String {
        match self {
            SchemaType::Json => "json".to_string(),
            SchemaType::GraphQL => "graphql".to_string(),
            SchemaType::Protobuf => "protobuf".to_string()
        }
    }
}

