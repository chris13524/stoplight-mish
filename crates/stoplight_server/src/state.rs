use {
    alloy::primitives::B256,
    serde_json::Value,
    std::{collections::HashMap, sync::Arc},
    tokio::sync::RwLock,
};

#[derive(Clone)]
pub struct AppState {
    pub object_store: HashMap<B256, Value>,
    pub property_store: Arc<RwLock<HashMap<B256, B256>>>,
}
