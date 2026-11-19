//! Differential adapter for `tools/probe_diff.py`: reads input file paths, one
//! per line, from stdin and prints one JSON line per input with the Strict
//! still, still16, frames and ICC results and the RecommendedBeta still and
//! frames results, in the shape `probe_diff.py` builds from `png_oracle.py`
//! and `policy_oracle.py`. Pixels are reported as SHA-256 of the RGBA bytes.

use png_verus::{Frames, Profile, TryError};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::io::BufRead;

fn sha(b: &[u8]) -> String {
    Sha256::digest(b).iter().map(|x| format!("{x:02x}")).collect()
}

fn err(e: &TryError) -> Value {
    json!({ "error": e.name() })
}

fn frames(b: &[u8], p: Profile, policy: bool) -> Value {
    let mut d = match Frames::try_new(b, p) {
        Ok(d) => d,
        Err(e) => return err(&e),
    };
    let info = *d.info();
    let mut out = Vec::new();
    let end;
    loop {
        match d.try_next() {
            Ok(Some(f)) => {
                let mut v = json!({
                    "rect": [f.rect.0, f.rect.1, f.rect.2, f.rect.3],
                    "delay": f.delay.map(|(n, dd)| vec![n, dd]),
                    "dispose": f.dispose, "blend": f.blend, "sha": sha(f.rgba),
                });
                if policy {
                    v["recovered"] = json!(f.recovered);
                }
                out.push(v);
            }
            Ok(None) => {
                end = json!("iend");
                break;
            }
            Err(e) => {
                end = err(&e);
                break;
            }
        }
    }
    json!({ "width": info.width, "height": info.height, "num_plays": info.num_plays, "frames": out, "end": end })
}

fn main() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let path = line.expect("stdin");
        let b = std::fs::read(&path).expect("input file");
        let still = match png_verus::try_decode_still(&b) {
            Ok(s) => json!({ "width": s.width, "height": s.height, "sha": sha(&s.rgba) }),
            Err(e) => err(&e),
        };
        let still16 = match png_verus::try_decode_still16(&b) {
            Ok(s) => json!({ "width": s.width, "height": s.height, "sha": sha(&s.rgba16) }),
            Err(e) => err(&e),
        };
        let icc = match png_verus::try_icc_profile(&b) {
            Ok(None) => json!({ "icc": null }),
            Ok(Some(p)) => json!({ "icc_len": p.len(), "icc_sha256": sha(&p) }),
            Err(e) => err(&e),
        };
        let rb_still = match png_verus::try_decode_still_with_policy(&b, Profile::RecommendedBeta) {
            Ok((s, rec)) => json!({ "width": s.width, "height": s.height, "sha": sha(&s.rgba), "recovered": rec }),
            Err(e) => err(&e),
        };
        let r = json!({
            "path": path,
            "still": still,
            "still16": still16,
            "frames": frames(&b, Profile::Strict, false),
            "icc": icc,
            "rb_still": rb_still,
            "rb_frames": frames(&b, Profile::RecommendedBeta, true),
        });
        println!("{r}");
    }
}
