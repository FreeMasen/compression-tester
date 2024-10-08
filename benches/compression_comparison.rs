use std::{fs::File, io::{BufReader, Read}, path::Path};

use compression_tester::gzip_encode;
use criterion::{black_box, Criterion};
use flate2::Compression;

fn read_file_into_drain(p: impl AsRef<Path>) {
    let mut reader = BufReader::new(File::open(p.as_ref()).unwrap());
    loop {
        let mut rb = [0u8;4096];
        let len = reader.read(&mut rb).unwrap();
        if len == 0 {
            break;
        }
        black_box(rb);
    }
}

pub fn default_compression(c: &mut Criterion) {
    let mut group = c.benchmark_group("default");
    group.bench_function("baseline-no-compression", |b| {
        b.iter(|| read_file_into_drain("./payload.json"));
    });
    group.bench_function("default_compression", |b| {
        b.iter(|| black_box(gzip_encode("./payload.json", Compression::default())))
    });
    group.bench_function("best_compression", |b| {
        b.iter(|| black_box(gzip_encode("./payload.json", Compression::best())))
    });
    group.bench_function("fast_compression", |b| {
        b.iter(|| black_box(gzip_encode("./payload.json", Compression::best())))
    });
    group.finish();
}
criterion::criterion_group!(benches, default_compression);
criterion::criterion_main!(benches);
