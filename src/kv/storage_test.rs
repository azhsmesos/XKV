use std::env::current_dir;
use crate::kv::kv_store::KVStore;
use super::errors::{XKVError, Result};

#[test]
fn test_set() -> Result<()> {
    let mut store = KVStore::open(current_dir().unwrap().as_path()).unwrap();
   for i in 0..10 {
       let key = format!("key-{}", i);
       let val = format!("value-{}", i);
       store.set(key.to_owned(), val.to_owned())?;
   }

    Ok(())
}