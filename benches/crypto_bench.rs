use criterion::{black_box, criterion_group, criterion_main, Criterion};
use polygone::crypto::{
    kem::{generate_keypair, encapsulate, decapsulate},
    symmetric::{SessionKey, EncryptedPayload},
    shamir,
};
use polygone::protocol::Session;
use std::time::Duration;

pub fn bench_primitives(c: &mut Criterion) {
    let (pk, sk) = generate_keypair().unwrap();
    let (ct, ss) = encapsulate(&pk).unwrap();
    let secret = vec![42u8; 32];
    let key_bytes = [0u8; 32];
    let key = SessionKey::from_bytes(key_bytes);

    // 1. ML-KEM-1024
    let mut group = c.benchmark_group("ML-KEM-1024");
    group.bench_function("encapsulate", |b| b.iter(|| encapsulate(black_box(&pk))));
    group.bench_function("decapsulate", |b| b.iter(|| decapsulate(black_box(&sk), black_box(&ct))));
    group.finish();

    // 2. BLAKE3 (Derivation)
    c.bench_function("BLAKE3 (KDF)", |b| b.iter(|| {
        blake3::derive_key("polygone test", black_box(&[0u8; 32]))
    }));

    // 3. AES-256-GCM
    c.bench_function("AES-GCM-256 (Encrypt 1KB)", |b| {
        let msg = vec![0u8; 1024];
        b.iter(|| key.encrypt(black_box(&msg)))
    });

    // 4. Shamir (4-of-7)
    let mut group = c.benchmark_group("Shamir");
    group.bench_function("split 4-of-7 (32B)", |b| b.iter(|| shamir::split(black_box(&secret), 4, 7)));
    let frags = shamir::split(&secret, 4, 7).unwrap();
    group.bench_function("reconstruct 4-of-7", |b| b.iter(|| shamir::reconstruct(black_box(&frags[..4]), 4)));
    group.finish();
}

pub fn bench_lifecycle(c: &mut Criterion) {
    let bob_kp = generate_keypair().unwrap();
    let bob_pk = bob_kp.0.clone();
    let msg = b"L'information n'existe pas. Elle traverse.";

    c.bench_function("Session Lifecycle (Alice Send)", |b| {
        b.iter(|| {
            let (mut alice, _) = Session::new_initiator(black_box(&bob_pk)).unwrap();
            alice.establish(None).unwrap();
            alice.send(black_box(msg)).unwrap()
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = bench_primitives, bench_lifecycle
}
criterion_main!(benches);
