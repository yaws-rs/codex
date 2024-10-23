use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::Read;

fn load_test_data(file_location: &str) -> Vec<u8> {
    let mut file = File::open(file_location).unwrap();
    let mut data: Vec<u8> = vec![];
    file.read_to_end(&mut data).unwrap();
    data
}

use codex_percent::FixedEncoder;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("FixedEncoder Poop-U6-18B >> 72B Test Cold", |b| {
        b.iter(|| {
            let poop = "💩💩💩💩💩💩";
            let mut e = FixedEncoder::<72>::init();
            let _f = e.encode(black_box(poop)).unwrap();
            let _t = core::str::from_utf8(e.cur_block());
        })
    });

    c.bench_function(
        "percent_encoding::utf8_percent_encode Poop-U6-18B >> 72B Test Cold",
        |b| {
            b.iter(|| {
                let poop = "💩💩💩💩💩💩";
                let _e = percent_encoding::utf8_percent_encode(
                    black_box(poop),
                    percent_encoding::NON_ALPHANUMERIC,
                )
                .to_string();
            })
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
