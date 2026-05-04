use rand;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use aes_gcm::aead::{Aead};
use polygone_common::SessionKey;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SymmetricError {
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
}

pub fn encrypt(key: &SessionKey, plaintext: &[u8], _aad: &[u8]) -> Result<(Vec<u8>, [u8; 12]), SymmetricError> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key.as_slice()));
    let nonce_bytes: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext)
        .map_err(|_| SymmetricError::EncryptionFailed)?;

    Ok((ciphertext, nonce_bytes))
}

pub fn decrypt(key: &SessionKey, ciphertext: &[u8], nonce: &[u8; 12], _aad: &[u8]) -> Result<Vec<u8>, SymmetricError> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key.as_slice()));
    let nonce = Nonce::from_slice(nonce);

    cipher.decrypt(nonce, ciphertext)
        .map_err(|_| SymmetricError::DecryptionFailed)
}
