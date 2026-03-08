use std::sync::Arc;

use aes_gcm::{Aes256Gcm, Key};
use anyhow::anyhow;
use keyring::Entry;
use rand::RngCore;

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
    /// initializes the secure storage and opens the db
    pub fn new(path: &str) -> Result<Arc<Self>> {
        let storage = storage::Storage::new(path)?;
        let key: Key<Aes256Gcm> = Self::get_or_generate_master_key()?;
        Ok(Arc::new(Self { storage, key }))
    }

    /// sets a key for a new value
    pub fn set(&self, key: &str, value: &[u8]) -> error::Result<()> {
        let cipher_text = core::encrypt(value, &self.key)?;
        self.storage.set(key, &cipher_text)?;
        Ok(())
    }

    /// gets a value for a key
    pub fn get(&self, key: &str) -> Result<Vec<u8>> {
        let value = self
            .storage
            .get(key)?
            .ok_or_else(|| anyhow!("Key not found"))?;
        Ok(core::decrypt(&value, &self.key.to_vec())?)
    }

    /// deletes a key for a value
    pub fn delete(&self, key: &str) -> Result<()> {
        self.storage.delete(key)?;
        Ok(())
    }

    /// clears all storage for key-value pairs
    pub fn clear_storage(&self) -> Result<()> {
        self.storage.clear_storage()?;
        Ok(())
    }
}

impl SecureStorage {
    fn get_or_generate_master_key() -> Result<Key<Aes256Gcm>> {
        let entry =
            Entry::new("secure_storage", "secure_storage").map_err(|e| anyhow!("{:?}", e))?;
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
