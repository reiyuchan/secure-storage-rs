use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use anyhow::{Result, anyhow};
use rand::RngCore;
use zeroize::Zeroize;

/// encrypt the plain text and insert into the storage
pub fn encrypt(plaintext: &[u8], key_bytes: &[u8]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let mut ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| anyhow!("encryption failed {:?}", e))?;

    // Prepend nonce for storage
    let mut output = nonce_bytes.to_vec();
    output.append(&mut ciphertext);

    Ok(output)
}

/// decrypt the cipher text in storage and return result
pub fn decrypt(ciphertext_with_nonce: &[u8], key_bytes: &[u8]) -> Result<Vec<u8>> {
    if ciphertext_with_nonce.len() < 32 {
        return Err(anyhow!("ciphertext is too short"));
    }

    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    let (nonce_bytes, ciphertext) = ciphertext_with_nonce.split_at(32);
    let nonce = Nonce::from_slice(nonce_bytes);

    Ok(cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow!("decryption failed {:?}", e))?)
}

/// Securely clear sensitive key bytes from memory
pub fn zeroize(key_bytes: &mut [u8]) {
    key_bytes.zeroize();
}
