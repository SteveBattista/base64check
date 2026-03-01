use base64check::{trim_trailing_zeros, verify_base64_roundtrip, verify_base64_simple};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::hint::black_box;

fn bench_verify_base64_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("verify_base64_roundtrip");

    // Test different input sizes
    let sizes = [1, 10, 100, 512, 1024];

    for size in &sizes {
        let mut rng = StdRng::seed_from_u64(42); // Fixed seed for consistent benchmarks
        let input: Vec<u8> = (0..*size).map(|_| rng.gen::<u8>()).collect();
        let mut encode_buffer = vec![0u8; size * 4];
        let mut decode_buffer = vec![0u8; size * 4];

        group.bench_with_input(BenchmarkId::new("with_buffers", size), size, |b, _| {
            b.iter(|| {
                verify_base64_roundtrip(
                    black_box(&input),
                    black_box(&mut encode_buffer),
                    black_box(&mut decode_buffer),
                )
                .unwrap()
            })
        });

        group.bench_with_input(BenchmarkId::new("simple_allocation", size), size, |b, _| {
            b.iter(|| verify_base64_simple(black_box(&input)).unwrap())
        });
    }

    group.finish();
}

fn bench_trim_trailing_zeros(c: &mut Criterion) {
    let mut group = c.benchmark_group("trim_trailing_zeros");

    // Test different scenarios
    let test_cases = [
        ("no_zeros", vec![1u8, 2, 3, 4, 5]),
        ("some_zeros", vec![1u8, 2, 3, 0, 0]),
        ("all_zeros", vec![0u8; 100]),
        ("mixed", vec![0u8, 1, 2, 3, 0, 0, 0]),
    ];

    for (name, data) in &test_cases {
        group.bench_with_input(BenchmarkId::new("trim", name), data, |b, data| {
            b.iter(|| trim_trailing_zeros(black_box(data)))
        });
    }

    group.finish();
}

fn bench_large_random_data(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_random_verification");

    // Generate a large dataset
    let mut rng = StdRng::seed_from_u64(123);
    let large_data: Vec<u8> = (0..10_000).map(|_| rng.gen::<u8>()).collect();
    let mut encode_buffer = vec![0u8; 10_000 * 4];
    let mut decode_buffer = vec![0u8; 10_000 * 4];

    group.bench_function("10kb_data", |b| {
        b.iter(|| {
            verify_base64_roundtrip(
                black_box(&large_data),
                black_box(&mut encode_buffer),
                black_box(&mut decode_buffer),
            )
            .unwrap()
        })
    });

    group.finish();
}

fn bench_edge_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_cases");

    // Empty data
    group.bench_function("empty", |b| {
        b.iter(|| verify_base64_simple(black_box(&[])).unwrap())
    });

    // Single byte
    group.bench_function("single_byte", |b| {
        b.iter(|| verify_base64_simple(black_box(&[42u8])).unwrap())
    });

    // All same byte
    let same_bytes = vec![170u8; 1000]; // 0xAA pattern
    group.bench_function("same_bytes", |b| {
        b.iter(|| verify_base64_simple(black_box(&same_bytes)).unwrap())
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_verify_base64_roundtrip,
    bench_trim_trailing_zeros,
    bench_large_random_data,
    bench_edge_cases
);
criterion_main!(benches);
