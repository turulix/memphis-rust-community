use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::helper::memphis_util::get_internal_name;
use crate::schemaverse::schema::SchemaValidator;

pub type BoxedSchemaValidator = Arc<Box<dyn SchemaValidator >>;

pub struct SchemaStore {
    schemas: RwLock<HashMap<String, BoxedSchemaValidator>>
}

impl SchemaStore {
    pub(crate) fn new() -> SchemaStore {
        SchemaStore {
            schemas: Default::default()
        }
    }

    pub async fn get_schema(&self, station_name: &str) -> Option<BoxedSchemaValidator> {
        self.schemas.read().await.get(&get_internal_name(station_name)).cloned()
    }

    pub async fn add_schema(&mut self, station_name: &str, schema: impl SchemaValidator + 'static) {
        self.schemas.write().await.insert(get_internal_name(station_name), Arc::new(Box::new(schema)));
    }
}
