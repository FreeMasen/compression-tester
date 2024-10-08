use flate2::write::GzEncoder;
use flate2::Compression;
use std::{
    io::{prelude::*, BufReader}, path::Path
};
use base64::prelude::*;

struct GzComparison {
    raw_len: usize,
    gz_len: usize,
    base64_len: usize,
}

fn main() {
    let path = std::env::args().skip(1).next().expect("1 argument is required");
    let gz_default = gzip_encode(&path, Compression::default());
    let gz_best = gzip_encode(&path, Compression::best());
    let gz_fast = gzip_encode(&path, Compression::fast());
    report(gz_default, "default");
    report(gz_best, "best");
    report(gz_fast, "fast");
}

fn report(comp: GzComparison, name: &str) {
    println!("-----{name}-----");
    println!("original  : {}", format_bytes(comp.raw_len));
    println!("gizp      : {}", format_bytes(comp.gz_len));
    println!("gz+base64 : {}", format_bytes(comp.base64_len));
    println!("gz/orig  %: {}", format_percent(comp.gz_len, comp.raw_len));
    println!("b64/gz   %: {}", format_percent(comp.base64_len, comp.gz_len));
    println!("b64/orig %: {}", format_percent(comp.base64_len, comp.raw_len));
}

fn format_bytes(mut b: usize) -> String {
    let mut next_units = vec!["gb", "mb", "kb"];
    let mut units = "b";
    while b > 1024 {
        let Some(u) = next_units.pop() else {
            break;
        };
        b = b / 1024;
        units = u
    }
    format!("{b}{units}")
}

fn format_percent(l: usize, r: usize) -> String {
    format!("{:.2}", l as f32 / r as f32)
}

fn gzip_encode(path: impl AsRef<Path>, compression: Compression) -> GzComparison {
    let path = path.as_ref();
    let mut buf = Vec::new();
    let mut encoder = GzEncoder::new(&mut buf, compression);
    let mut reader = BufReader::new(std::fs::File::open(path).unwrap());
    let mut raw_len = 0;
    loop {
        let mut rb = [0u8;4096];
        let len = reader.read(&mut rb).unwrap();
        if len == 0 {
            break;
        }
        raw_len += len;
        encoder.write_all(&rb[..len]).unwrap();
    }
    drop(encoder);
    let base64 = BASE64_STANDARD.encode(&buf);
    GzComparison {
        raw_len,
        gz_len: buf.len(),
        base64_len: base64.len(),
    }
}
