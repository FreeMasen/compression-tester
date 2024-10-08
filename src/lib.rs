use flate2::write::GzEncoder;
use flate2::Compression;
use std::{
    io::{prelude::*, BufReader}, path::Path
};
use base64::prelude::*;

pub struct GzComparison {
    pub raw_len: usize,
    pub gz_len: usize,
    pub base64_len: usize,
}


pub fn gzip_encode(path: impl AsRef<Path>, compression: Compression) -> GzComparison {
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
