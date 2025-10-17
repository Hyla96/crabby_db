use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

pub static MEMORY_DATABASE: LazyLock<MemoryDatabase> = LazyLock::new(|| MemoryDatabase::new());

pub struct MemoryDatabase {
    data: RwLock<HashMap<Vec<u8>, Vec<u8>>>,
}

impl MemoryDatabase {
    fn new() -> Self {
        MemoryDatabase {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub fn set(&self, key: &[u8], value: &[u8]) {
        let mut data = self.data.write().unwrap();
        data.insert(key.to_vec(), value.to_vec());
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let data = self.data.read().unwrap();
        data.get(key).cloned()
    }
}
