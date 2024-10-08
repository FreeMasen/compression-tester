
use flate2::Compression;
use compression_tester::{gzip_encode, GzComparison};


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
