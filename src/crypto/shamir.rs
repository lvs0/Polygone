//! Shamir Secret Sharing — split secrets across ephemeral nodes.
//!
//! A secret is split into `n` fragments; any `threshold` of them
//! can reconstruct it. No subset smaller than `threshold` leaks
//! *any* information (information-theoretic security).
//!
//! Used to distribute the symmetric session key across N ephemeral
//! nodes. No single node ever holds a reconstructable secret.

use sharks::{Share, Sharks};
use serde::{Deserialize, Serialize};
use crate::{PolygoneError, Result};

/// Opaque identifier for a fragment (1-indexed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FragmentId(pub u8);

/// One fragment of a split secret, destined for a single ephemeral node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fragment {
    /// Which share this is.
    pub id: FragmentId,
    /// Raw share bytes (variable length, depends on secret size).
    pub data: Vec<u8>,
}

/// Split `secret` into `n` fragments requiring `threshold` to reconstruct.
///
/// # Panics
/// Panics if `threshold == 0` or `threshold > n`.
pub fn split(secret: &[u8], threshold: u8, n: u8) -> Result<Vec<Fragment>> {
    assert!(threshold > 0 && threshold <= n, "invalid threshold/n");
    let sharks = Sharks(threshold);
    let dealer = sharks.dealer(secret);
    let shares: Vec<Share> = dealer.take(n as usize).collect();
    Ok(shares
        .into_iter()
        .enumerate()
        .map(|(i, share)| Fragment {
            id: FragmentId(i as u8 + 1),
            data: Vec::from(&share),
        })
        .collect())
}

/// Reconstruct a secret from at least `threshold` fragments.
pub fn reconstruct(fragments: &[Fragment], threshold: u8) -> Result<Vec<u8>> {
    if fragments.len() < threshold as usize {
        return Err(PolygoneError::ShamirError(format!(
            "need {} fragments, got {}",
            threshold,
            fragments.len()
        )));
    }
    let sharks = Sharks(threshold);
    let shares: Vec<Share> = fragments
        .iter()
        .map(|f| Share::try_from(f.data.as_slice()))
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|e| PolygoneError::ShamirError(e.to_string()))?;
    sharks
        .recover(&shares)
        .map_err(|e| PolygoneError::ShamirError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_and_reconstruct_exact_threshold() {
        let secret = b"post-quantum-polygone-key-32byte";
        let frags = split(secret, 3, 5).unwrap();
        assert_eq!(frags.len(), 5);
        // Use exactly threshold fragments
        let recovered = reconstruct(&frags[..3], 3).unwrap();
        assert_eq!(recovered, secret);
    }

    #[test]
    fn reconstruct_with_all_fragments() {
        let secret = b"all-five-reconstruct-test-secret";
        let frags = split(secret, 3, 5).unwrap();
        let recovered = reconstruct(&frags, 3).unwrap();
        assert_eq!(recovered, secret);
    }

    #[test]
    fn insufficient_fragments_returns_error() {
        let secret = b"should-fail-reconstruction-here!";
        let frags = split(secret, 3, 5).unwrap();
        assert!(reconstruct(&frags[..2], 3).is_err());
    }
}
