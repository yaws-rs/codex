use criterion::{black_box, criterion_group, criterion_main, Criterion};

/*
use std::fs::File;
use std::io::Read;

fn load_test_data(file_location: &str) -> Vec<u8> {
    let mut file = File::open(file_location).unwrap();
    let mut data: Vec<u8> = vec![];
    file.read_to_end(&mut data).unwrap();
    data
}*/

#[cfg(all(feature = "encode", feature = "fixed"))]
use codex_percent::FixedEncoder;

#[cfg(all(feature = "decode", feature = "fixed"))]
use codex_percent::FixedDecoder;

#[cfg(all(feature = "encode", feature = "vec"))]
use codex_percent::VecEncoder;

#[cfg(all(feature = "decode", feature = "vec"))]
use codex_percent::VecDecoder;

#[cfg(feature = "decode")]
fn criterion_benchmark_decode(c: &mut Criterion) {
    //*************************************************
    // FixedDecoder
    //*************************************************

    #[cfg(feature = "fixed")]
    c.bench_function("FixedDecoder Poop-U1-4B", |b| {
        b.iter(|| {
            let poop = "%F0%9F%92%A9";
            let mut d = FixedDecoder::<4>::init();
            let _f = d.decode(black_box(poop)).unwrap();
        })
    });

    #[cfg(feature = "fixed")]
    c.bench_function("FixedDecoder Poop-U3-12B", |b| {
        b.iter(|| {
            let poop = "%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9";
            let mut d = FixedDecoder::<12>::init();
            let _f = d.decode(black_box(poop)).unwrap();
        })
    });

    #[cfg(feature = "fixed")]
    c.bench_function("FixedDecoder Poop-U6-24B", |b| {
        b.iter(|| {
            let poop = "%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9";
            let mut d = FixedDecoder::<24>::init();
            let _f = d.decode(black_box(poop)).unwrap();
        })
    });

    #[cfg(feature = "vec")]
    c.bench_function("VecDecoder Poop-U6-24B straightline", |b| {
        b.iter(|| {
            let poop = "%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9";
            let mut d: Vec<u8> = Vec::new();
            let _f = VecDecoder::decode(black_box(poop), &mut d).unwrap();
        })
    });

    #[cfg(feature = "vec")]
    c.bench_function("VecDecoder Poop-U6-24B capped, re-use", |b| {
        let mut d: Vec<u8> = Vec::with_capacity(24);
        b.iter(|| {
            let poop = "%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9";
            let _f = VecDecoder::decode(black_box(poop), &mut d).unwrap();
            d.clear();
        })
    });
}

#[cfg(feature = "encode")]
fn criterion_benchmark_encode(c: &mut Criterion) {
    //*************************************************
    // FixedEncoder no bounds checks
    //*************************************************

    #[cfg(feature = "fixed")]
    c.bench_function("FixedEncoder Poop-U1 -> 12B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩";
            let mut e = FixedEncoder::<12>::init();
            let _f = e.encode(black_box(poop)).unwrap();
        })
    });

    #[cfg(feature = "fixed")]
    c.bench_function("FixedEncoder Poop-U3 -> 36B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩💩💩";
            let mut e = FixedEncoder::<36>::init();
            let _f = e.encode(black_box(poop)).unwrap();
        })
    });

    #[cfg(feature = "fixed")]
    c.bench_function("FixedEncoder Poop-U6 -> 72B Test Cold", |b| {
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
    c.bench_function("VecEncoder no-init-cap Poop-U1 -> 12B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩";
            let mut v: Vec<u8> = Vec::new();
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
        })
    });

    #[cfg(feature = "vec")]
    c.bench_function("VecEncoder no-init-cap Poop-U3 -> 36B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩💩💩";
            let mut v: Vec<u8> = Vec::new();
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
        })
    });

    #[cfg(feature = "vec")]
    c.bench_function("VecEncoder no-init-cap Poop-U6 -> 72B Test Cold", |b| {
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
    c.bench_function("VecEncoder capped Poop-U1 -> 12B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩";
            let mut v: Vec<u8> = Vec::with_capacity(12);
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
        })
    });

    #[cfg(feature = "vec")]
    c.bench_function("VecEncoder capped Poop-U3 -> 36B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩💩💩";
            let mut v: Vec<u8> = Vec::with_capacity(36);
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
        })
    });

    #[cfg(feature = "vec")]
    c.bench_function("VecEncoder capped Poop-U6 -> 72B Test Cold", |b| {
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
    c.bench_function("VecEncoder re-use Poop-U1 -> 12B Test Cold", |b| {
        let poop = "💩";
        let mut v: Vec<u8> = Vec::with_capacity(12);
        b.iter(|| {
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
            v.clear();
        })
    });

    #[cfg(feature = "vec")]
    c.bench_function("VecEncoder re-use Poop-U3 -> 36B Test Cold", |b| {
        let poop = "💩💩💩";
        let mut v: Vec<u8> = Vec::with_capacity(36);
        b.iter(|| {
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
            v.clear();
        })
    });

    #[cfg(feature = "vec")]
    c.bench_function("VecEncoder re-use Poop-U6 -> 72B Test Cold", |b| {
        let poop = "💩💩💩💩💩💩";
        let mut v: Vec<u8> = Vec::with_capacity(72);
        b.iter(|| {
            let _f = VecEncoder::encode(black_box(poop), &mut v).unwrap();
            v.clear();
        })
    });
}

#[cfg(feature = "encode")]
criterion_group!(benches, criterion_benchmark_encode);

#[cfg(feature = "decode")]
criterion_group!(benches, criterion_benchmark_decode);

criterion_main!(benches);
