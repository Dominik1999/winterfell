// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use math::fields::f64::{self, BaseElement};
use rand_utils::rand_value;
use winter_crypto::{
    hashers::{Rp64_256, Sha3_256},
    Hasher, ElementHasher,
};

type Sha3 = Sha3_256<f64::BaseElement>;
type Sha3Digest = <Sha3 as Hasher>::Digest;
type Rp64_256Digest = <Rp64_256 as Hasher>::Digest;

fn sha3_2to1(c: &mut Criterion) {
    let v: [Sha3Digest; 2] = [Sha3::hash(&[1u8]), Sha3::hash(&[2u8])];
    c.bench_function("SHA3 2-to-1 hashing (cached)", |bench| {
        bench.iter(|| Sha3::merge(black_box(&v)))
    });

    c.bench_function("SHA3 2-to-1 hashing (random)", |b| {
        b.iter_batched(
            || {
                [
                    Sha3::hash(&rand_value::<u64>().to_le_bytes()),
                    Sha3::hash(&rand_value::<u64>().to_le_bytes()),
                ]
            },
            |state| Sha3::merge(&state),
            BatchSize::SmallInput,
        )
    });
}

fn sha3_sequential(c: &mut Criterion) {
    let v: [BaseElement; 100] = (0..100)
        .into_iter()
        .map(BaseElement::new)
        .collect::<Vec<_>>()
        .try_into()
        .expect("should not fail");
    c.bench_function("SHA3 sequential hashing (cached)", |bench| {
        bench.iter(|| Sha3::hash_elements(black_box(&v)))
    });

    c.bench_function("SHA3 sequential hashing (random)", |bench| {
        bench.iter_batched(
            || {
                let v: [BaseElement; 100] = (0..100)
                    .into_iter()
                    .map(|_| BaseElement::new(rand_value()))
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("should not fail");
                v
            },
            |state| Sha3::hash_elements(&state),
            BatchSize::SmallInput,
        )
    });
}

fn rescue256_2to1(c: &mut Criterion) {
    let v: [Rp64_256Digest; 2] = [Rp64_256::hash(&[1u8]), Rp64_256::hash(&[2u8])];
    c.bench_function("Rescue prime 2-to-1 hashing (cached)", |bench| {
        bench.iter(|| Rp64_256::merge(black_box(&v)))
    });

    c.bench_function("Rescue prime 2-to-1 hashing (random)", |b| {
        b.iter_batched(
            || {
                [
                    Rp64_256::hash(&rand_value::<u64>().to_le_bytes()),
                    Rp64_256::hash(&rand_value::<u64>().to_le_bytes()),
                ]
            },
            |state| Rp64_256::merge(&state),
            BatchSize::SmallInput,
        )
    });
}

fn rescue256_sequential(c: &mut Criterion) {
    let v: [BaseElement; 100] = (0..100)
        .into_iter()
        .map(BaseElement::new)
        .collect::<Vec<BaseElement>>()
        .try_into()
        .expect("should not fail");
    c.bench_function("Rescue prime sequential hashing (cached)", |bench| {
        bench.iter(|| Rp64_256::hash_elements(black_box(&v)))
    });

    c.bench_function("Rescue prime sequential hashing (random)", |bench| {
        bench.iter_batched(
            || {
                let v: [BaseElement; 100] = (0..100)
                    .into_iter()
                    .map(|_| BaseElement::new(rand_value()))
                    .collect::<Vec<BaseElement>>()
                    .try_into()
                    .expect("should not fail");
                v
            },
            |state| Rp64_256::hash_elements(&state),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(hash_group, sha3_2to1, sha3_sequential, rescue256_2to1, rescue256_sequential);
criterion_main!(hash_group);