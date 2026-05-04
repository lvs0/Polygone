// Secret sharing (Shamir)
// Bouchon minimal pour compilation.

use polygone_common::SessionKey;
use rand::RngCore;

pub fn split_secret(secret: &SessionKey, _threshold: u8, _shares_count: u8) -> Vec<Vec<u8>> {
    // Bouchon
    let mut rng = rand::thread_rng();
    let mut shares = vec![];
    for _ in 0..3 {
        let mut share = [0u8; 32];
        rng.fill_bytes(&mut share);
        shares.push(share.to_vec());
    }
    shares
}

pub fn reconstruct_secret(_shares: Vec<Vec<u8>>) -> Option<SessionKey> {
    // Bouchon
    let mut secret = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut secret);
    Some(SessionKey::new(secret))
}
