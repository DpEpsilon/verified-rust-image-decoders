//! PLAN M2's generated >=8 MiB payloads, with and without Clear at a full table.
//! Python's encoder and independent oracle check each stream before Rust sees it.
//! The generated payloads travel over a pipe and are not stored as vector files.
use std::process::Command;
use serde_json::Value;
use lzw_proof::lzw::{lzw_decode, strict};

#[test]
fn generated_eight_mib_payloads_reach_width_twelve_and_full_table() {
    let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tools/stress_stream.py");
    for mode in ["clear", "deferred"] {
        let generated=Command::new("timeout").args(["25m","python3","-I"])
            .arg(&script).arg(mode).output().expect("stress generator");
        assert!(generated.status.success(),"{}",String::from_utf8_lossy(&generated.stderr));
        let newline=generated.stdout.iter().position(|&b|b==b'\n').expect("metadata");
        let meta:Value=serde_json::from_slice(&generated.stdout[..newline]).unwrap();
        let area=meta["area"].as_u64().unwrap() as usize;
        let payload_len=meta["payload"].as_u64().unwrap() as usize;
        assert!(payload_len>=8*1024*1024);
        let expected=&generated.stdout[newline+1..newline+1+area];
        let data=&generated.stdout[newline+1+area..];
        assert_eq!(data.len(),payload_len);
        let mut prefix=vec![u16::MAX;4096];
        let mut suffix=vec![0xee;4096];
        let mut length=vec![u16::MAX;4096];
        let mut out=vec![0;area];
        let result=lzw_decode(8,data,area,&mut prefix,&mut suffix,&mut length,&mut out,strict());
        assert_eq!(result.error,None,"{mode}");
        assert_eq!(result.written,area);
        assert!(!result.e4);
        assert_eq!(result.pos,u128::from(meta["pos"].as_u64().unwrap()));
        assert_eq!(&out,expected,"{mode}");
        eprintln!("{mode}: {payload_len} compressed bytes; {area} indices; width 12; full table");
    }
}
