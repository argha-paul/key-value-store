use dashmap::DashMap;
use std::sync::Arc;
// use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct KvStore {
    store: Arc<DashMap<String, String>>, // Concurrent HashMap
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            store: Arc::new(DashMap::new()),
        }
    }

    pub fn set(&self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).map(|v| v.value().clone())
    }

    pub fn delete(&self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }
}
