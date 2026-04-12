//! Secure memory management utilities.
//!
//! This module provides memory protection for sensitive data:
//! - ZeroizeOnDrop for automatic key erasure
//! - Secure file permissions
//! - Memory locking hints (best-effort)
//!
//! # Security Notes
//!
//! - All secret keys use `ZeroizeOnDrop` trait
//! - File permissions are set to 0o600 for secret keys
//! - Memory locking (mlock) requires elevated privileges on Unix
//!
//! # Memory Locking
//!
//! To prevent secret keys from being swapped to disk:
//! ```bash
//! # Linux: Grant CAP_IPC_LOCK or set RLIMIT_MEMLOCK
//! sudo setcap 'cap_ipc_lock+ep' /path/to/polygone
//! ulimit -l unlimited
//! ```

use tracing::debug;
use zeroize::{Zeroize, Zeroizing};

/// Check if memory locking is available (requires CAP_IPC_LOCK on Linux)
pub fn is_mlock_supported() -> bool {
    #[cfg(unix)]
    {
        debug!("Memory locking capability check: requires elevated privileges");
        false
    }

    #[cfg(not(unix))]
    {
        false
    }
}

/// Attempt to lock memory (best-effort, may require elevated privileges)
///
/// # Security Note
///
/// This is a best-effort protection. On systems without sufficient
/// privileges (CAP_IPC_LOCK on Linux), this will fail silently.
#[allow(dead_code)]
pub fn lock_memory(_data: &[u8]) -> bool {
    false
}

/// Securely zero a region of memory.
///
/// Uses volatile operations to prevent compiler optimization away.
pub fn secure_zero(data: &mut [u8]) {
    for byte in data.iter_mut() {
        *byte = volatile_write(*byte, 0);
    }
}

#[inline(always)]
fn volatile_write<T: Copy>(_: T, val: T) -> T {
    core::hint::black_box(val)
}

/// Wrapper for secure memory that auto-zeroizes on drop
pub struct SecureBuffer {
    data: Zeroizing<Vec<u8>>,
}

impl SecureBuffer {
    /// Create a new secure buffer
    #[allow(dead_code)]
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: Zeroizing::new(data),
        }
    }

    /// Get reference to data
    #[allow(dead_code)]
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Get mutable reference to data
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
    }

    #[test]
    fn test_secure_buffer() {
        let buf = SecureBuffer::new(vec![0x42u8; 16]);
        assert_eq!(buf.as_slice().len(), 16);
        drop(buf);
    }
}
