//! Shamir Secret Sharing — split secrets across ephemeral nodes.
//!
//! A secret is split into `n` fragments; any `threshold` of them
//! can reconstruct it. No subset smaller than `threshold` leaks
//! *any* information (information-theoretic security).
//!
//! Uses the `sharks` crate for field arithmetic over GF(256).
//!
//! # Security Properties
//!
//! - Information-theoretic: fewer than `threshold` shares reveal NOTHING.
//! - Each share is destined for exactly one ephemeral node.
//! - Share data is zeroed on Drop (via Zeroize).

use polygone_common::{SessionKey, FragmentId};
use sharks::{Share, Sharks};
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// One fragment of a split secret, destined for a single ephemeral node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fragment {
    /// Which share this is (1-indexed).
    pub id: FragmentId,
    /// Raw share bytes (variable length, depends on secret size).
    /// Zeroed on drop for forward secrecy.
    pub data: Vec<u8>,
}

impl Drop for Fragment {
    fn drop(&mut self) {
        // Zero the share data on drop for forward secrecy.
        // We can't derive Zeroize because FragmentId(u8) doesn't implement it,
        // but the important data to zero is the share bytes.
        self.data.zeroize();
    }
}

/// Split a `SessionKey` into `n` fragments requiring `threshold` to reconstruct.
///
/// # Panics
/// Panics if `threshold == 0` or `threshold > n`.
///
/// # Example
/// ```no_run
/// use polygone_common::SessionKey;
/// use polygone_crypto::shamir;
///
/// let key = SessionKey::new([42u8; 32]);
/// let fragments = shamir::split_secret(&key, 4, 7);
/// assert_eq!(fragments.len(), 7);
/// ```
pub fn split_secret(secret: &SessionKey, threshold: u8, n: u8) -> Vec<Fragment> {
    assert!(threshold > 0 && threshold <= n, "invalid threshold/n: t={}, n={}", threshold, n);

    let sharks = Sharks(threshold);
    let dealer = sharks.dealer(secret.as_slice());
    let shares: Vec<Share> = dealer.take(n as usize).collect();

    shares
        .into_iter()
        .enumerate()
        .map(|(i, share)| Fragment {
            id: FragmentId::new(i as u8 + 1),
            data: Vec::from(&share),
        })
        .collect()
}

/// Reconstruct a `SessionKey` from at least `threshold` fragments.
///
/// Returns `None` if:
/// - Fewer than `threshold` fragments are provided.
/// - The `sharks` crate fails to recover the secret.
/// - The recovered secret is not exactly 32 bytes.
pub fn reconstruct_secret(fragments: Vec<Fragment>, threshold: u8) -> Option<SessionKey> {
    if fragments.len() < threshold as usize {
        return None;
    }

    let sharks = Sharks(threshold);
    let shares: Vec<Share> = fragments
        .iter()
        .filter_map(|f| Share::try_from(f.data.as_slice()).ok())
        .collect();

    // Need at least threshold valid shares
    if shares.len() < threshold as usize {
        return None;
    }

    let recovered = sharks.recover(&shares).ok()?;

    if recovered.len() != 32 {
        return None;
    }

    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&recovered);
    Some(SessionKey::new(key_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_and_reconstruct_exact_threshold() {
        let key = SessionKey::new([0xAB; 32]);
        let fragments = split_secret(&key, 4, 7);
        assert_eq!(fragments.len(), 7);

        // Use exactly threshold fragments
        let recovered = reconstruct_secret(fragments[..4].to_vec(), 4);
        assert!(recovered.is_some());
        assert_eq!(recovered.unwrap().as_slice(), &[0xAB; 32]);
    }

    #[test]
    fn reconstruct_with_all_fragments() {
        let key = SessionKey::new([0xCD; 32]);
        let fragments = split_secret(&key, 4, 7);
        let recovered = reconstruct_secret(fragments, 4);
        assert!(recovered.is_some());
        assert_eq!(recovered.unwrap().as_slice(), &[0xCD; 32]);
    }

    #[test]
    fn insufficient_fragments_returns_none() {
        let key = SessionKey::new([0xEF; 32]);
        let fragments = split_secret(&key, 4, 7);
        // Only 3 fragments — below threshold of 4
        let recovered = reconstruct_secret(fragments[..3].to_vec(), 4);
        assert!(recovered.is_none());
    }

    #[test]
    fn fragment_ids_are_one_indexed() {
        let key = SessionKey::new([0x22; 32]);
        let fragments = split_secret(&key, 3, 5);
        for (i, f) in fragments.iter().enumerate() {
            assert_eq!(f.id.as_u8(), (i as u8) + 1);
        }
    }

    #[test]
    fn super_threshold_also_works() {
        // Using more than threshold fragments should also work
        let key = SessionKey::new([0x55; 32]);
        let fragments = split_secret(&key, 3, 7);
        // 5 fragments with threshold 3 — should still work
        let recovered = reconstruct_secret(fragments[..5].to_vec(), 3);
        assert!(recovered.is_some());
        assert_eq!(recovered.unwrap().as_slice(), &[0x55; 32]);
    }

    // ── Additional comprehensive tests ────────────────────────────────────

    fn test_key() -> SessionKey {
        SessionKey::new([0xABu8; 32])
    }

    #[test]
    fn split_produces_exactly_n_fragments() {
        let key = test_key();
        for n in 2..=10 {
            let fragments = split_secret(&key, 2, n);
            assert_eq!(fragments.len() as u8, n, "expected {} fragments, got {}", n, fragments.len());
        }
    }

    #[test]
    fn split_yields_different_fragments() {
        let key = test_key();
        let fragments = split_secret(&key, 3, 7);
        // Compare raw data bytes of each fragment with next one — they should not all be equal
        let all_same = fragments.windows(2).all(|w| w[0].data == w[1].data);
        assert!(!all_same, "all fragments must not have identical data");
    }

    #[test]
    fn threshold_one_any_single_share_reconstructs() {
        let key = test_key();
        let fragments = split_secret(&key, 1, 5);
        for frag in &fragments {
            let recovered = reconstruct_secret(vec![frag.clone()], 1);
            assert!(recovered.is_some());
            assert_eq!(recovered.unwrap().as_slice(), key.as_slice());
        }
    }

    #[test]
    fn threshold_equals_n_all_fragments_needed() {
        let key = SessionKey::new([0x77; 32]);
        let n = 5u8;
        let fragments = split_secret(&key, n, n);

        // n-1 fragments must fail
        let recovered_short = reconstruct_secret(fragments[..(n - 1) as usize].to_vec(), n);
        assert!(recovered_short.is_none(), "n-1 fragments with threshold=n must fail");

        // all n must succeed
        let recovered_all = reconstruct_secret(fragments.clone(), n);
        assert!(recovered_all.is_some());
        assert_eq!(recovered_all.unwrap().as_slice(), key.as_slice());
    }

    #[test]
    fn all_at_threshold_minimum_works() {
        let key = SessionKey::new([0x33; 32]);
        let n = 6u8;
        let t = 3u8;
        let fragments = split_secret(&key, t, n);

        // exactly threshold
        let recovered = reconstruct_secret(fragments[..t as usize].to_vec(), t);
        assert!(recovered.is_some());
        assert_eq!(recovered.unwrap().as_slice(), key.as_slice());
    }

    #[test]
    fn shuffle_fragments_still_reconstructs() {
        let key = SessionKey::new([0x99; 32]);
        let t = 4u8;
        let n = 7u8;
        let fragments = split_secret(&key, t, n);
        let selected: Vec<_> = fragments[..t as usize].iter().cloned().collect();
        let mut shuffled = selected.clone();
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        shuffled.shuffle(&mut rng);

        let recovered = reconstruct_secret(shuffled, t);
        assert!(recovered.is_some(), "reconstruction with shuffled fragments must succeed");
        assert_eq!(recovered.unwrap().as_slice(), key.as_slice());
    }

    #[test]
    fn all_zero_secret_reconstructs() {
        let key = SessionKey::new([0x00; 32]);
        let fragments = split_secret(&key, 3, 5);

        let recovered = reconstruct_secret(fragments[..3].to_vec(), 3);
        assert!(recovered.is_some());
        assert_eq!(recovered.unwrap().as_slice(), &[0x00; 32]);
    }

    #[test]
    fn all_ff_secret_reconstructs() {
        let key = SessionKey::new([0xFF; 32]);
        let fragments = split_secret(&key, 3, 5);

        let recovered = reconstruct_secret(fragments[..3].to_vec(), 3);
        assert!(recovered.is_some());
        assert_eq!(recovered.unwrap().as_slice(), &[0xFF; 32]);
    }

    #[test]
    fn panics_on_zero_threshold() {
        let key = test_key();
        let result = std::panic::catch_unwind(|| split_secret(&key, 0, 5));
        assert!(result.is_err(), "threshold=0 must panic");
    }

    #[test]
    fn panics_on_threshold_exceeding_n() {
        let key = test_key();
        let result = std::panic::catch_unwind(|| split_secret(&key, 5, 3));
        assert!(result.is_err(), "threshold > n must panic");
    }

    #[test]
    fn reconstruct_with_empty_fragment_set_returns_none() {
        let result = reconstruct_secret(vec![], 3);
        assert!(result.is_none(), "empty fragment set with threshold 3 must return None");
    }

    #[test]
    fn reconstruct_with_empty_set_and_threshold_1_returns_none() {
        // Even threshold=1 needs at least 1 fragment; empty vec fails len check.
        let result = reconstruct_secret(vec![], 1);
        assert!(result.is_none());
    }

    #[test]
    fn super_shares_reconstruct_correctly() {
        // Use threshold=2, n=10, pick any 6 out of 10
        let key = SessionKey::new([0xBE; 32]);
        let fragments = split_secret(&key, 2, 10);
        let recovered = reconstruct_secret(fragments[..6].to_vec(), 2);
        assert!(recovered.is_some());
        assert_eq!(recovered.unwrap().as_slice(), key.as_slice());
    }

    #[test]
    fn different_secrets_produce_different_fragments() {
        let key1 = SessionKey::new([0x11; 32]);
        let key2 = SessionKey::new([0x22; 32]);

        let frags1 = split_secret(&key1, 3, 7);
        let frags2 = split_secret(&key2, 3, 7);

        // Corresponding fragment data must differ
        for (f1, f2) in frags1.iter().zip(frags2.iter()) {
            assert_ne!(
                f1.data, f2.data,
                "fragments from different secrets must differ"
            );
        }
    }

    #[test]
    fn fragment_id_preserved_through_serialize_deserialize_roundtrip() {
        use serde_json;

        let key = SessionKey::new([0x44; 32]);
        let fragments = split_secret(&key, 3, 5);
        let frag = &fragments[2];

        let json = serde_json::to_string(frag).expect("serialize");
        let deserialized: Fragment = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(deserialized.id.as_u8(), frag.id.as_u8());
        assert_eq!(deserialized.data, frag.data);
    }

    #[test]
    fn fragment_data_dropped_on_drop_different_shares_differ() {
        // Verify structural property: fragments sharing the same index position
        // but from different secret splits produce different raw data (not trivial
        // identity mapping).
        let key1 = SessionKey::new([0x01; 32]);
        let key2 = SessionKey::new([0x02; 32]);

        let f1 = split_secret(&key1, 2, 3);
        let f2 = split_secret(&key2, 2, 3);

        // Same threshold and n, but different secrets -> different shares
        assert_ne!(f1[1].data, f2[1].data);
    }
}
