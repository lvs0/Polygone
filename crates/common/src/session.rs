use zeroize::{Zeroize};

#[derive(Debug, Clone, Zeroize)]
pub struct SessionKey([u8; 32]);

impl SessionKey {
    pub fn new(key: [u8; 32]) -> Self {
        Self(key)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: [u8; 32],
    pub key: SessionKey,
}

impl Session {
    pub fn new(id: [u8; 32], key: SessionKey) -> Self {
        Self { id, key }
    }
}
