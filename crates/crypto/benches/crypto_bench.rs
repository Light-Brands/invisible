use criterion::{black_box, criterion_group, criterion_main, Criterion};
use invisible_crypto::{IdentityKey, KeyPair};

fn bench_key_generation(c: &mut Criterion) {
    c.bench_function("keypair_generate", |b| {
        b.iter(|| KeyPair::generate().unwrap())
    });

    c.bench_function("identity_generate", |b| {
        b.iter(|| IdentityKey::generate().unwrap())
    });
}

fn bench_dh(c: &mut Criterion) {
    let kp1 = KeyPair::generate().unwrap();
    let kp2 = KeyPair::generate().unwrap();

    c.bench_function("x25519_dh", |b| {
        b.iter(|| kp1.dh(black_box(kp2.public_key())).unwrap())
    });
}

fn bench_signing(c: &mut Criterion) {
    let identity = IdentityKey::generate().unwrap();
    let message = b"test message";

    c.bench_function("ed25519_sign", |b| {
        b.iter(|| identity.sign(black_box(message)).unwrap())
    });

    let signature = identity.sign(message).unwrap();
    c.bench_function("ed25519_verify", |b| {
        b.iter(|| identity.verify(black_box(message), black_box(&signature)).unwrap())
    });
}

criterion_group!(benches, bench_key_generation, bench_dh, bench_signing);
criterion_main!(benches);
