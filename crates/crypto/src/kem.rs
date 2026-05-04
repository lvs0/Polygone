// Post-quantum KEM (ML-KEM-1024)
// Bouchon minimal déterministe pour tests unitaires.

use polygone_common::SessionKey;

pub struct PublicKey([u8; 1184]);
pub struct SecretKey([u8; 2400]);

pub fn generate_kem_key_pair() -> (PublicKey, SecretKey) {
    // Bouchon déterministe
    let pk = [1u8; 1184];
    let sk = [2u8; 2400];
    (PublicKey(pk), SecretKey(sk))
}

pub fn encapsulate(_pk: &PublicKey) -> ([u8; 1088], SessionKey) {
    let ciphertext = [3u8; 1088];
    let shared_secret = [0xAAu8; 32];
    (ciphertext, SessionKey::new(shared_secret))
}

pub fn decapsulate(_ct: &[u8; 1088], _sk: &SecretKey) -> SessionKey {
    // Même secret que encapsulate
    let shared_secret = [0xAAu8; 32];
    SessionKey::new(shared_secret)
}
