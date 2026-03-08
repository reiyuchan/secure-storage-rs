use std::sync::Arc;

use aes_gcm::{Aes256Gcm, Key};
use anyhow::anyhow;
use keyring::Entry;
use rand::RngCore;
use sled::Db;

use crate::{
    core,
    error::{self, Result},
    storage,
};

#[derive(uniffi::Object)]
pub struct SecureStorage {
    storage: storage::Storage,
    key: Key<Aes256Gcm>,
}

#[uniffi::export]
impl SecureStorage {
    #[uniffi::constructor]
    pub fn new() -> Result<Arc<Self>> {
        let storage = storage::Storage::new()?;
        let key: Key<Aes256Gcm> = Self::get_or_generate_master_key(&storage.db)?;
        Ok(Arc::new(Self { storage, key }))
    }

    pub fn set(&self, key: &str, value: &[u8]) -> error::Result<()> {
        let cipher_text = core::encrypt(value, &self.key)?;
        self.storage.set(key, &cipher_text)?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Vec<u8>> {
        let value = self
            .storage
            .get(key)?
            .ok_or_else(|| anyhow!("Key not found"))?;
        Ok(core::decrypt(&value, &self.key.to_vec())?)
    }

    pub fn delete(&self, key: &str) -> Result<()> {
        self.storage.delete(key)?;
        Ok(())
    }

    pub fn clear_storage(&self) -> Result<()> {
        self.storage.clear_storage()?;
        Ok(())
    }
}

impl SecureStorage {
    fn get_or_generate_master_key(db: &Db) -> Result<Key<Aes256Gcm>> {
        let entry = Entry::new("my-service", "my-name").map_err(|e| anyhow!("{:?}", e))?;
        let key = match entry.get_secret() {
            Ok(key) if !key.is_empty() => key,
            _ => {
                let mut new_key = [0u8; 32];
                rand::thread_rng().fill_bytes(&mut new_key);
                entry.set_secret(&new_key).map_err(|e| anyhow!("{:?}", e))?;

                new_key.to_vec().into()
            }
        };

        Ok(Key::<Aes256Gcm>::from_slice(&key).clone())
    }
}

impl Drop for SecureStorage {
    fn drop(&mut self) {
        core::zeroize(&mut self.key);
    }
}
