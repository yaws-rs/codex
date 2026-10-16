use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[cfg(all(feature = "decode"))]
use ychunked::{ChunkDecoder, ChunkReceiver};

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg(all(feature = "decode"))]
struct TestReceiver;

#[cfg(all(feature = "decode"))]
impl ChunkReceiver for TestReceiver {
    #[inline]
    fn chunk_add(&mut self, _data: &[u8]) -> bool {
        true
    }
}

#[cfg(feature = "decode")]
fn criterion_benchmark_decode(c: &mut Criterion) {
    //*************************************************
    // Decoder
    //*************************************************
    c.bench_function("Basic", |b| {
        let mut receiver = TestReceiver::default();
        let mut decoder = ChunkDecoder::new();
        let input = b"16\r\nYeahChunkedStuffIs Fun\r\n0\r\n";
        b.iter(|| {
            decoder
                .decode_detached(&mut receiver, black_box(input))
                .unwrap();
        })
    });
}

#[cfg(feature = "decode")]
criterion_group!(benches, criterion_benchmark_decode);

criterion_main!(benches);
