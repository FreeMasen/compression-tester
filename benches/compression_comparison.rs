use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use compression_tester::gzip_encode_full;
use criterion::{black_box, Criterion};
use flate2::Compression;

pub fn default_compression(c: &mut Criterion) {
    let payload = std::fs::read("./payload.json").unwrap();
    let mut group = c.benchmark_group("default");
    group.bench_function("default_compression", |b| {
        b.iter(|| black_box(gzip_encode_full(&payload, Compression::default())))
    });
    group.bench_function("best_compression", |b| {
        b.iter(|| black_box(gzip_encode_full(&payload, Compression::best())))
    });
    group.bench_function("fast_compression", |b| {
        b.iter(|| black_box(gzip_encode_full(&payload, Compression::best())))
    });
    group.finish();
}
criterion::criterion_group!(benches, default_compression);
criterion::criterion_main!(benches);
