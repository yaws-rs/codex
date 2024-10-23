use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::Read;

fn load_test_data(file_location: &str) -> Vec<u8> {
    let mut file = File::open(file_location).unwrap();
    let mut data: Vec<u8> = vec![];
    file.read_to_end(&mut data).unwrap();
    data
}

#[cfg(feature = "fixed")]
use codex_percent::FixedEncoder;

#[cfg(feature = "vec")]
use codex_percent::VecEncoder;

fn criterion_benchmark(c: &mut Criterion) {
    //*************************************************
    // FixedEncoder no bounds checks
    //*************************************************

    #[cfg(feature = "fixed")]
    c.bench_function("FixedEncoder Poop-U1-3B >> 12B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩";
            let mut e = FixedEncoder::<12>::init();
            let _f = e.encode(black_box(poop)).unwrap();
        })
    });

    #[cfg(feature = "fixed")]
    c.bench_function("FixedEncoder Poop-U3-9B >> 36B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩💩💩";
            let mut e = FixedEncoder::<36>::init();
            let _f = e.encode(black_box(poop)).unwrap();
        })
    });

    #[cfg(feature = "fixed")]
    c.bench_function("FixedEncoder Poop-U6-18B >> 72B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩💩💩💩💩💩";
            let mut e = FixedEncoder::<72>::init();
            let _f = e.encode(black_box(poop)).unwrap();
        })
    });

    //*************************************************
    // VecEncoder without with_capacity() and no re-use
    //*************************************************

    #[cfg(feature = "vec")]
    c.bench_function("VecEncoder no-init-cap Poop-U1-3B >> 12B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩";
            let mut v: Vec<u8> = Vec::new();
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
        })
    });

    #[cfg(feature = "vec")]
    c.bench_function("VecEncoder no-init-cap Poop-U3-9B >> 36B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩💩💩";
            let mut v: Vec<u8> = Vec::new();
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
        })
    });

    #[cfg(feature = "vec")]
    c.bench_function("VecEncoder no-init-cap Poop-U6-18B >> 72B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩💩💩💩💩💩";
            let mut v: Vec<u8> = Vec::new();
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
        })
    });

    //*************************************************
    // VecEncoder but using with_capacity() - no re-use
    //*************************************************

    #[cfg(feature = "vec")]
    c.bench_function("VecEncoder capped Poop-U1-3B >> 12B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩";
            let mut v: Vec<u8> = Vec::with_capacity(12);
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
        })
    });

    #[cfg(feature = "vec")]
    c.bench_function("VecEncoder capped Poop-U3-9B >> 36B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩💩💩";
            let mut v: Vec<u8> = Vec::with_capacity(36);
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
        })
    });

    #[cfg(feature = "vec")]
    c.bench_function("VecEncoder capped Poop-U6-18B >> 72B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩💩💩💩💩💩";
            let mut v: Vec<u8> = Vec::with_capacity(72);
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
        })
    });

    //*************************************************
    // VecEncoder but using with_capacity() - re-use
    //*************************************************

    #[cfg(feature = "vec")]
    c.bench_function("VecEncoder re-use Poop-U1-3B >> 12B Test Cold", |b| {
        let poop = "💩";
        let mut v: Vec<u8> = Vec::with_capacity(12);
        b.iter(|| {
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
            v.clear();
        })
    });

    #[cfg(feature = "vec")]
    c.bench_function("VecEncoder re-use Poop-U3-9B >> 36B Test Cold", |b| {
        let poop = "💩💩💩";
        let mut v: Vec<u8> = Vec::with_capacity(36);
        b.iter(|| {
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
            v.clear();
        })
    });

    #[cfg(feature = "vec")]
    c.bench_function("VecEncoder re-use Poop-U6-18B >> 72B Test Cold", |b| {
        let poop = "💩💩💩💩💩💩";
        let mut v: Vec<u8> = Vec::with_capacity(72);
        b.iter(|| {
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
            v.clear();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
