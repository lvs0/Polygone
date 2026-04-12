//! Secure memory management utilities.
//!
//! This module provides memory protection for sensitive data:
//! - ZeroizeOnDrop for automatic key erasure
//! - Secure file permissions
//!
//! # Security Notes
//!
//! - All secret keys use `ZeroizeOnDrop` trait
//! - File permissions are set to 0o600 for secret keys
//!
//! # Memory Locking
//!
//! Memory locking (mlock) is stubbed because this crate uses `#![forbid(unsafe_code)]`.
//! For production deployments requiring mlock:
//! - Use a wrapper process with CAP_IPC_LOCK
//! - Or compile with custom features
//!
//! ```bash
//! # Linux: Grant CAP_IPC_LOCK or set RLIMIT_MEMLOCK
//! sudo setcap 'cap_ipc_lock+ep' /path/to/polygone
//! ulimit -l unlimited
//! ```

use tracing::debug;
use zeroize::{Zeroize, Zeroizing};

pub fn is_mlock_supported() -> bool {
    debug!("mlock support requires unsafe code; using ZeroizeOnDrop instead");
    false
}

#[allow(dead_code)]
pub fn lock_memory(_data: &[u8]) -> bool {
    false
}

#[allow(dead_code)]
pub fn unlock_memory(_data: &[u8]) -> bool {
    false
}

pub fn secure_zero(data: &mut [u8]) {
    data.zeroize();
}

pub struct SecureBuffer {
    data: Zeroizing<Vec<u8>>,
}

impl SecureBuffer {
    #[allow(dead_code)]
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: Zeroizing::new(data),
        }
    }

    #[allow(dead_code)]
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    #[allow(dead_code)]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl Drop for SecureBuffer {
    fn drop(&mut self) {
        self.data.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_zero() {
        let mut data = vec![0xFFu8; 32];
        secure_zero(&mut data);
        assert!(data.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_secure_buffer() {
        let buf = SecureBuffer::new(vec![0x42u8; 16]);
        assert_eq!(buf.as_slice().len(), 16);
        drop(buf);
    }
}
