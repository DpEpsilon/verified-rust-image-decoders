//! Read the manifest and corpus in place; write only aggregate counts to stdout.
//!
//! Checks each corpus file against both manifest columns: its byte length
//! (column 8) and its SHA-256 (column 2). A digest mismatch panics, naming the
//! file, so a swapped or edited corpus file cannot pass as the measured one.
//! The corpus is read only; no bytes are copied into the repository.
use std::collections::BTreeMap;
use std::path::{Component, PathBuf};
use tga_verus::policy::Profile;
use tga_verus::tga::decode_with_policy;

/// SHA-256 of `data`, lowercase hex. Standard FIPS 180-4, no dependencies.
fn sha256_hex(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let bitlen = (data.len() as u64).wrapping_mul(8);
    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bitlen.to_be_bytes());
    let mut w = [0u32; 64];
    for chunk in msg.chunks_exact(64) {
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[4 * i],
                chunk[4 * i + 1],
                chunk[4 * i + 2],
                chunk[4 * i + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    let mut s = String::with_capacity(64);
    for v in h {
        s.push_str(&format!("{:08x}", v));
    }
    s
}

/// FIPS 180-4's two published short vectors plus the one-million-`a` vector:
/// a wrong digest implementation fails here, before it can pass a corpus file.
fn sha256_self_test() {
    let million_a = vec![b'a'; 1_000_000];
    let cases: [(&[u8], &str); 4] = [
        (b"", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
        (b"abc", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
        (
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        ),
        (
            &million_a[..],
            "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0",
        ),
    ];
    for (data, want) in cases {
        assert_eq!(sha256_hex(data), want, "SHA-256 self-test failed");
    }
}

fn main() {
    sha256_self_test();
    let manifest = PathBuf::from(std::env::args_os().nth(1).expect("MANIFEST.tsv argument"));
    let base = manifest.parent().expect("manifest parent");
    let text = std::fs::read_to_string(&manifest).expect("read manifest");
    assert!(text.starts_with("path\tsha256\t"));
    let profiles = [
        ("Strict", Profile::Strict),
        ("Recommended", Profile::Recommended),
        ("GdkCompat-2.42.12", Profile::GdkCompat),
        ("ImageCrateCompat-0.25.10", Profile::ImageCrateCompat),
    ];
    let mut accepted = [0usize; 4];
    let mut recovered = [0usize; 4];
    let mut errors: [BTreeMap<&str, usize>; 4] = std::array::from_fn(|_| BTreeMap::new());
    let mut available = 0usize;
    let mut missing = 0usize;
    let mut rows = 0usize;
    for line in text.lines().skip(1).filter(|line| !line.is_empty()) {
        rows += 1;
        let columns: Vec<_> = line.split('\t').collect();
        assert_eq!(columns.len(), 9);
        let path = PathBuf::from(columns[0]);
        assert!(path.components().all(|part| matches!(part, Component::Normal(_))));
        assert!(path.starts_with("files"));
        let bytes = match std::fs::read(base.join(path)) {
            Ok(bytes) => bytes,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => { missing += 1; continue; }
            Err(e) => panic!("corpus read: {e}"),
        };
        assert_eq!(bytes.len(), columns[7].parse::<usize>().unwrap());
        let digest = sha256_hex(&bytes);
        assert_eq!(
            digest,
            columns[1],
            "SHA-256 mismatch for {} (manifest column 2)",
            columns[0],
        );
        available += 1;
        for (i, (_, profile)) in profiles.iter().enumerate() {
            let (result, flag) = decode_with_policy(&bytes, *profile);
            match result {
                Ok(_) => { accepted[i] += 1; recovered[i] += usize::from(flag); }
                Err(e) => { assert!(!flag); *errors[i].entry(e.name()).or_default() += 1; }
            }
        }
    }
    println!("manifest={} rows={rows} available={available} missing={missing} sha256=verified",
        manifest.display());
    for (i, (name, _)) in profiles.iter().enumerate() {
        println!("{name}: accepted={}/{} recovered={} rejected={} errors={:?}",
            accepted[i], available, recovered[i], available - accepted[i], errors[i]);
    }
}
