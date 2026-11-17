//! Host-only differential adapter; no host dependency enters the library.
use inflate_verus::{
    inflate_zlib_into, inflate_zlib_measure, limits::*, InflateOptions, SegmentSlices, Segments,
    Status, Window, ZlibTail,
};
use std::io::{self, BufRead, Write};

fn hex(data: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(data.len() * 2);
    for &x in data {
        s.push(DIGITS[usize::from(x >> 4)] as char);
        s.push(DIGITS[usize::from(x & 15)] as char);
    }
    s
}
fn main() {
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 4);
        let limit: usize = fields[0].parse().unwrap();
        let mut options = InflateOptions::STRICT;
        match fields[1] {
            "S" => (),
            "B" => options = InflateOptions::RECOMMENDED_BETA,
            "W" => options.window = Window::Ignore,
            "T" => options.zlib_tail = ZlibTail::IgnoreTail,
            _ => panic!("bad options"),
        }
        let layout: usize = if fields[2].starts_with("p:") {
            3
        } else {
            fields[2].parse().unwrap()
        };
        let data: Vec<u8> = fields[3]
            .as_bytes()
            .chunks_exact(2)
            .map(|x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16).unwrap())
            .collect();
        let mut parts: Vec<&[u8]> = Vec::new();
        if layout == 0 {
            parts.push(&data);
        } else if layout == 3 {
            let mut p = 0;
            for size in fields[2][2..].split(',') {
                let size: usize = size.parse().unwrap();
                parts.push(&data[p..p + size]);
                p += size;
            }
            assert_eq!(p, data.len());
        } else {
            parts.push(&[]);
            let mut p = 0;
            while p < data.len() {
                let stride = if layout == 1 { 1 } else { 1 + (p * 7 + 3) % 23 };
                let end = (p + stride).min(data.len());
                parts.push(&data[p..end]);
                parts.push(&[]);
                p = end;
            }
            parts.push(&[]);
        }
        let mut out = vec![0xa5; limit];
        let mut src = SegmentSlices::new(&parts).unwrap();
        let r = inflate_zlib_into(&mut src, &mut out, options);
        let mut ring = vec![0xcd; WINDOW_MAX];
        let mut src2 = SegmentSlices::new(&parts).unwrap();
        let m = inflate_zlib_measure(&mut src2, &mut ring, limit as u64, options);
        assert_eq!(r, m, "into/measure observation, counters included");
        let k = usize::try_from(r.produced).unwrap();
        assert!(
            out[k..].iter().all(|&x| x == 0xa5),
            "untouched output suffix"
        );
        for j in k.saturating_sub(WINDOW_MAX)..k {
            assert_eq!(ring[j % WINDOW_MAX], out[j], "last-window representation");
        }
        assert!(
            r.core_steps <= C_IN * data.len() as u128 + C_OUT * r.produced as u128 + C_INFL_FIXED,
            "observed core work exceeds manifest bound: {r:?}"
        );
        assert!(r.seg_steps <= C_SEG * (parts.len() as u128 + 1));
        assert!(src.pos() <= data.len() as u64);
        assert!(src.seg_index() <= parts.len());
        let status = match r.status {
            Status::Done => "Done".to_string(),
            Status::Full => "Full".to_string(),
            Status::Err(e) => format!("{e:?}"),
        };
        writeln!(
            stdout,
            "{}\t{}\t{}\t{}\t{}\t{}",
            status,
            k,
            hex(&out[..k]),
            r.core_steps,
            r.seg_steps,
            src.pos()
        )
        .unwrap();
        stdout.flush().unwrap();
    }
}
