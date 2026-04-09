use criterion::{black_box, criterion_group, criterion_main, Criterion};
use polygone::crypto::{kem::{generate_keypair, encapsulate, decapsulate}, shamir};

fn bench_kem(c: &mut Criterion) {
    let (pk, sk) = generate_keypair().unwrap();
    
    let mut group = c.benchmark_group("ML-KEM-1024");
    group.bench_function("encapsulate", |b| {
        b.iter(|| encapsulate(black_box(&pk)))
    });
    let (ct, _) = encapsulate(&pk).unwrap();
    group.bench_function("decapsulate", |b| {
        b.iter(|| decapsulate(black_box(&sk), black_box(&ct)))
    });
    group.finish();
}

fn bench_shamir(c: &mut Criterion) {
    let secret = vec![42u8; 32];
    
    let mut group = c.benchmark_group("Shamir");
    group.bench_function("split 4-of-7", |b| {
        b.iter(|| shamir::split(black_box(&secret), 4, 7))
    });
    let frags = shamir::split(&secret, 4, 7).unwrap();
    group.bench_function("reconstruct 4", |b| {
        b.iter(|| shamir::reconstruct(black_box(&frags[..4]), 4))
    });
    group.finish();
}

criterion_group!(benches, bench_kem, bench_shamir);
criterion_main!(benches);
