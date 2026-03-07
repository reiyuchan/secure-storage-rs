use anyhow::Result;
use sled::Db;
use uniffi::deps::anyhow::Ok;

pub struct Storage {
    pub db: Db,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let db = sled::open("secure_storage_db")?;
        Ok(Self { db })
    }

    pub fn set(&self, key: &str, value: &[u8]) -> Result<()> {
        let tree = self.db.open_tree("data")?;
        tree.insert(key, value)?;
        tree.flush()?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let tree = self.db.open_tree("data")?;

        Ok(tree.get(key)?.map(|v| v.to_vec()))
    }

    pub fn delete(&self, key: &str) -> Result<()> {
        self.db.remove(key)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn clear_storage(&self) -> Result<()> {
        self.db.clear()?;
        self.db.flush()?;
        Ok(())
    }
}
