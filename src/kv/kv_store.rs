use std::path::Path;
use crate::kv::storage::{BitcaskStorage, Storage};
use super::errors::Result;

pub struct KVStore {
    storage: Box<dyn Storage>,
}

impl KVStore {
    pub fn open(path: &Path) -> Result<KVStore> {
        let storage = BitcaskStorage::open(path.to_path_buf())?;
        Ok(KVStore {
            storage: Box::new(storage),
        })
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        self.storage.get(key)
    }

    pub fn set(&mut self, key: String, val: String) -> Result<()> {

        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.storage.remove(key)
    }
}