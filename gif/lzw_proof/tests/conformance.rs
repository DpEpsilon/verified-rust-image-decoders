//! Executable conformance against independent oracle observations.
//! Every layer runs the verified stack-free decoder, not a Rust model mirror.
use std::fs;
use std::path::PathBuf;
use serde_json::Value;
use lzw_proof::lzw::{lzw_decode, Params};

fn root() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")) }
fn load(name: &str) -> Vec<Value> {
    let p = if name == "witnesses.json" { root().join("tests").join(name) }
        else { root().join("tests/expected").join(name) };
    serde_json::from_str(&fs::read_to_string(p).expect("test records")).expect("JSON")
}
fn bytes(v: &Value) -> Vec<u8> {
    v.as_array().expect("byte array").iter().map(|v| v.as_u64().unwrap() as u8).collect()
}
fn params(r: &Value) -> Params {
    if let Some(p) = r.get("params") {
        Params {m_lo:p["m_lo"].as_u64().unwrap() as u8, m_hi:p["m_hi"].as_u64().unwrap() as u8,
            size1_only_one_pixel:p["size1_only_one_pixel"].as_bool().unwrap(),
            checked_literals:p["checked_literals"].as_bool().unwrap(),
            deferred_clear:p["deferred_clear"].as_bool().unwrap(),
            stop_at_raster:p["stop_at_raster"].as_bool().unwrap()}
    } else if r["profile"].as_str()==Some("RecommendedBeta")
        || (r.get("profile").is_none() && r["m"].as_u64()==Some(1)) {
        lzw_proof::lzw::recommended_beta()
    } else { lzw_proof::lzw::strict() }
}

fn check_records(recs: &[Value]) {
    // Reuse deliberately stale storage across frames and minimum code sizes.
    // Literal and unreadable entries must not need zeroing or reinitialisation.
    let mut prefix=vec![u16::MAX;4096];
    let mut suffix=vec![0xee;4096];
    let mut length=vec![u16::MAX;4096];
    for (i,r) in recs.iter().enumerate() {
        let m=r["m"].as_u64().unwrap() as u8;
        let a=r["a"].as_u64().unwrap() as usize;
        let d=bytes(&r["data"]);
        let mut out=vec![0xcc;a+5];
        let got=lzw_decode(m,&d,a,&mut prefix,&mut suffix,&mut length,&mut out,params(r));
        assert_eq!(&out[..got.written], bytes(&r["expected"]), "record {i}: {}", r["name"]);
        assert_eq!(got.error.map(|e|format!("{e:?}")),r["error"].as_str().map(str::to_owned),
            "record {i}: {}",r["name"]);
        if let Some(pos)=r["pos"].as_u64().or_else(||r["mirror_pos"].as_u64()) {
            assert_eq!(got.pos,u128::from(pos),"record {i} bit position");
        }
        if let Some(e4)=r["e4"].as_bool() {assert_eq!(got.e4,e4,"record {i} E4");}
        assert!(out[a..].iter().all(|&v|v==0xcc),"write beyond raster in record {i}");
    }
}

#[test]
fn exec_vs_expected_random_streams() {
    let recs=load("random.json"); assert_eq!(recs.len(),200); check_records(&recs);
}
#[test]
fn exec_vs_expected_vectors() {
    let recs=load("vectors.json"); assert_eq!(recs.len(),47); check_records(&recs);
}
#[test]
fn witness_gate_matches() {
    let recs=load("witnesses.json"); assert_eq!(recs.len(),12); check_records(&recs);
}
#[test]
fn exec_vs_oracle_all_policy_switches() {
    let recs=load("policy.json"); assert!(recs.len()>10_000); check_records(&recs);
}
