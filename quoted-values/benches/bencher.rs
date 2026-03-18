use core::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

#[cfg(feature = "decode")]
use quoted_values::BackslashRemoval;

#[cfg(feature = "decode")]
fn criterion_benchmark_decode(c: &mut Criterion) {
    c.bench_function("BackslashesRemoval FOOBAR\\\"", |b| {
        let mut bytes = "FOOBAR\\\"".to_string().into_bytes();

        b.iter(|| {
            let _skipped = BackslashRemoval::in_place(black_box(&mut bytes)).unwrap();
        })
    });

    c.bench_function("BackslashesRemoval FOOBAR\\\" x2", |b| {
        let mut bytes = "FOOBAR\\\"FOOBAR\\\"".to_string().into_bytes();

        b.iter(|| {
            let _skipped = BackslashRemoval::in_place(black_box(&mut bytes)).unwrap();
        })
    });

    c.bench_function("BackslashesRemoval FOOBAR\\\" x3", |b| {
        let mut bytes = "FOOBAR\\\"FOOBAR\\\"FOOBAR\\\"".to_string().into_bytes();

        b.iter(|| {
            let _skipped = BackslashRemoval::in_place(black_box(&mut bytes)).unwrap();
        })
    });
}

#[cfg(feature = "decode")]
criterion_group!(benches, criterion_benchmark_decode);

criterion_main!(benches);
