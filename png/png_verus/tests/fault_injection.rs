//! Allocation behaviour against SPEC P20 (`Req`) and the alloc-delta form: a
//! counting allocator records every allocation the decoder makes during one
//! entry-point call, and can fail the k-th one. Checks, on vector inputs:
//! the physical request list equals `Req` exactly (no other allocation, no
//! reallocation), and failing each request in turn yields `OutOfMemory` with
//! that site and size, raised where SPEC P20 places the reservation.
//!
//! One `#[test]` function: the allocator's state is global.

use png_verus::{AllocSite, Frames, Profile, TryError};
use serde_json::Value;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst};

struct Counting;

static ARMED: AtomicBool = AtomicBool::new(false);
static COUNT: AtomicUsize = AtomicUsize::new(0);
static FAIL_AT: AtomicUsize = AtomicUsize::new(usize::MAX);
static SIZES: [AtomicUsize; 16] = [const { AtomicUsize::new(0) }; 16];

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        if ARMED.load(SeqCst) {
            let i = COUNT.fetch_add(1, SeqCst);
            if i < 16 {
                SIZES[i].store(l.size(), SeqCst);
            }
            if i == FAIL_AT.load(SeqCst) {
                return std::ptr::null_mut();
            }
        }
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        if ARMED.load(SeqCst) {
            // Any reallocation during a call is a failure of the check below.
            let i = COUNT.fetch_add(1, SeqCst);
            if i < 16 {
                SIZES[i].store(usize::MAX, SeqCst);
            }
        }
        unsafe { System.realloc(p, l, n) }
    }
}

#[global_allocator]
static A: Counting = Counting;

fn arm(fail_at: usize) {
    COUNT.store(0, SeqCst);
    FAIL_AT.store(fail_at, SeqCst);
    ARMED.store(true, SeqCst);
}

fn disarm() -> Vec<usize> {
    ARMED.store(false, SeqCst);
    let n = COUNT.load(SeqCst).min(16);
    (0..n).map(|i| SIZES[i].load(SeqCst)).collect()
}

fn hex_decode(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

fn records() -> Vec<Value> {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../spec-frozen/VECTORS.json");
    serde_json::from_str(&std::fs::read_to_string(root).expect("VECTORS.json")).unwrap()
}

fn oom(e: &TryError) -> Option<(AllocSite, usize)> {
    match e {
        TryError::OutOfMemory(o) => Some((o.site(), o.elements())),
        _ => None,
    }
}

/// `R(W,H)` of the IHDR in `b` via the oracle's formula, recomputed here.
fn raw_size(b: &[u8]) -> usize {
    let be = |p: usize| u32::from_be_bytes([b[p], b[p + 1], b[p + 2], b[p + 3]]) as u64;
    let (w, h, d, ct, il) = (be(16), be(20), b[24] as u64, b[25], b[28]);
    let c = match ct { 0 | 3 => 1, 2 => 3, 4 => 2, _ => 4 };
    let row = |pw: u64| (pw * c * d + 7) / 8;
    if il == 0 {
        return (h * (1 + row(w))) as usize;
    }
    let mut r = 0;
    for (x0, y0, dx, dy) in [(0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4), (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2)] {
        let pw = if w > x0 { (w - x0 + dx - 1) / dx } else { 0 };
        let ph = if h > y0 { (h - y0 + dy - 1) / dy } else { 0 };
        if pw > 0 && ph > 0 {
            r += ph * (1 + row(pw));
        }
    }
    r as usize
}

#[test]
fn requests_and_failures() {
    let recs = records();
    let find = |f: &dyn Fn(&Value) -> bool| -> Vec<u8> {
        let r = recs.iter().find(|r| f(r)).expect("vector");
        hex_decode(r["input_hex"].as_str().unwrap())
    };
    let name_is = |n: &'static str| move |r: &Value| r["name"] == n;

    // Still RGBA8 and RGBA16 of an interlaced RGBA image: [(Raw, R), (Output, 4N | 8N)].
    let b = find(&|r: &Value| {
        let n = r["name"].as_str().unwrap();
        n.starts_with("ok/ct6-d16-il1") && r["entry"] == "still"
    });
    let (w, h) = (u32::from_be_bytes(b[16..20].try_into().unwrap()) as usize, u32::from_be_bytes(b[20..24].try_into().unwrap()) as usize);
    let r = raw_size(&b);
    for wide in [false, true] {
        let out = if wide { 8 * w * h } else { 4 * w * h };
        let call = |b: &[u8]| -> Result<(), TryError> {
            if wide { png_verus::try_decode_still16(b).map(|_| ()) } else { png_verus::try_decode_still(b).map(|_| ()) }
        };
        arm(usize::MAX);
        let res = call(&b);
        let got = disarm();
        assert!(res.is_ok());
        assert_eq!(got, vec![r, out], "still Req (wide {wide})");
        arm(0);
        let res = call(&b);
        disarm();
        assert_eq!(oom(&res.unwrap_err()), Some((AllocSite::Raw, r)));
        arm(1);
        let res = call(&b);
        disarm();
        assert_eq!(oom(&res.unwrap_err()), Some((AllocSite::Output, out)));
    }

    // A walk error reserves nothing (PreAlloc): the first byte of the signature changed.
    let mut bad = b.clone();
    bad[0] ^= 1;
    arm(usize::MAX);
    let res = png_verus::try_decode_still(&bad);
    assert!(disarm().is_empty());
    assert!(matches!(res, Err(TryError::Format(png_verus::Error::BadSignature))));

    // Frames with a PREVIOUS disposal at k > 0: Raw, Output in the constructor,
    // Saved at the start of that frame's call (step S), once.
    let fr = recs
        .iter()
        .find(|r| {
            r["entry"] == "frames"
                && r["expected"]["end"] == "iend"
                && r["expected"]["frames"].as_array().map_or(false, |f| f.iter().skip(1).any(|x| x["dispose"] == 2))
        })
        .expect("dispose-previous vector");
    let fb = hex_decode(fr["input_hex"].as_str().unwrap());
    let k_prev = fr["expected"]["frames"].as_array().unwrap().iter().skip(1).position(|x| x["dispose"] == 2).unwrap() + 1;
    let n_frames = fr["expected"]["frames"].as_array().unwrap().len();
    let (cw, ch) = (fr["expected"]["width"].as_u64().unwrap() as usize, fr["expected"]["height"].as_u64().unwrap() as usize);
    arm(usize::MAX);
    let mut d = Frames::try_new(&fb, Profile::Strict).unwrap();
    let mut saved_at = None;
    for k in 0..n_frames {
        let before = COUNT.load(SeqCst);
        assert!(d.try_next().unwrap().is_some());
        if COUNT.load(SeqCst) != before {
            assert!(saved_at.is_none(), "Saved reserved twice");
            saved_at = Some(k);
        }
    }
    assert!(d.try_next().unwrap().is_none());
    let got = disarm();
    assert_eq!(got, vec![raw_size(&fb), 4 * cw * ch, 4 * cw * ch], "frames Req");
    assert_eq!(saved_at, Some(k_prev));
    // Failing Saved: frames before k_prev are returned, then OutOfMemory once, then Ok(None).
    arm(2);
    let mut d = Frames::try_new(&fb, Profile::Strict).unwrap();
    for _ in 0..k_prev {
        assert!(d.try_next().unwrap().is_some());
    }
    let e = d.try_next().err().expect("OutOfMemory");
    assert!(matches!(d.try_next(), Ok(None)));
    disarm();
    assert_eq!(oom(&e), Some((AllocSite::Saved, 4 * cw * ch)));

    // ICC: [(IccWindow, WINDOW_MAX), (Icc, m)]; m = 0 makes no allocator call (D8).
    for (name, m) in [("icc/present", 768usize), ("icc/empty-profile", 0)] {
        let ib = find(&name_is(name));
        arm(usize::MAX);
        let p = png_verus::try_icc_profile(&ib).unwrap().unwrap();
        let got = disarm();
        assert_eq!(p.len(), m);
        let want: Vec<usize> = if m == 0 { vec![32768] } else { vec![32768, m] };
        assert_eq!(got, want, "{name} Req");
        arm(0);
        let e = png_verus::try_icc_profile(&ib).unwrap_err();
        disarm();
        assert_eq!(oom(&e), Some((AllocSite::IccWindow, 32768)));
    }
}
