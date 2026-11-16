//! Fault-injection tests for the alloc-delta guard on the GIF core (SPEC.md
//! G18; style of `icns/icns_verus/tests/fault_injection.rs`). Falsification
//! checks, not proofs: they exercise the guard's binding of `Err` to
//! `try_reserve_exact`'s `Err` arm at run time on the pinned toolchain.
//!
//! A test-only `#[global_allocator]` wraps `System`. While armed it counts the
//! calling thread's requests, fails the k-th one (returns null), and records the
//! ordered trace of successful allocations (size, alignment), the number of
//! `realloc` calls and the peak of live bytes. State is `thread_local`, and every
//! test holds `SUITE_LOCK`, so other tests' harness allocations neither consume
//! the countdown nor are faulted by it. No allocator callback allocates, and the
//! armed code does not allocate either: frames are summarised into fixed arrays
//! of hashes while armed and compared after disarming.
//!
//! Per call, the logical request list `Req` (G18) is
//! `still: [(Canvas, 4N), (LzwTable, 20,480)]` and
//! `frames: [(Canvas, 4N), (LzwTable, 20,480)] ++ [(Saved, 4N)]` when a
//! disposal-3 frame is reached; empty (`PreAlloc`) when G2–G4 fail. Physically
//! `Canvas` and `Saved` are `4N` bytes at alignment 1 and `LzwTable` one
//! 20,480-byte zero-filled request at alignment 1. The suite checks the
//! physical trace against `Req` one to one, zero reallocations, a peak of at
//! most `8N + 20,480` live bytes, and then fails each reservation in turn: the
//! result must be exactly `OutOfMemory` with that site and element count, no
//! other error, frames before the failure identical to the uninjected run, and
//! the decoder fused.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::sync::Mutex;

use gif_verus::{try_decode_still, AllocSite, Frames, TryError};

static SUITE_LOCK: Mutex<()> = Mutex::new(());

fn lock() -> std::sync::MutexGuard<'static, ()> {
    SUITE_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

const TRACE_CAP: usize = 16;

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
struct Ev {
    size: usize,
    align: usize,
}

#[derive(Clone, Copy)]
struct State {
    armed: bool,
    countdown: usize,
    fired: bool,
    requests: usize,
    reallocs: usize,
    live: isize,
    peak: isize,
    trace: [Ev; TRACE_CAP],
    trace_len: usize,
    overflow: bool,
}

impl State {
    const EMPTY: Self = Self {
        armed: false,
        countdown: 0,
        fired: false,
        requests: 0,
        reallocs: 0,
        live: 0,
        peak: 0,
        trace: [Ev { size: 0, align: 0 }; TRACE_CAP],
        trace_len: 0,
        overflow: false,
    };
}

std::thread_local! { static STATE: Cell<State> = const { Cell::new(State::EMPTY) }; }

fn with_state(f: impl FnOnce(&mut State)) {
    let _ = STATE.try_with(|c| {
        let mut s = c.get();
        f(&mut s);
        c.set(s);
    });
}

fn should_fail() -> bool {
    let mut fail = false;
    with_state(|s| {
        if !s.armed {
            return;
        }
        s.requests += 1;
        if s.countdown > 0 && s.requests == s.countdown {
            s.fired = true;
            fail = true;
        }
    });
    fail
}

fn record(size: usize, align: usize) {
    with_state(|s| {
        if !s.armed {
            return;
        }
        s.live += size as isize;
        if s.live > s.peak {
            s.peak = s.live;
        }
        if s.trace_len < TRACE_CAP {
            s.trace[s.trace_len] = Ev { size, align };
            s.trace_len += 1;
        } else {
            s.overflow = true;
        }
    });
}

fn release(size: usize) {
    with_state(|s| {
        if s.armed {
            s.live -= size as isize;
        }
    });
}

struct FaultAlloc;

unsafe impl GlobalAlloc for FaultAlloc {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        if should_fail() {
            return std::ptr::null_mut();
        }
        let p = System.alloc(l);
        if !p.is_null() {
            record(l.size(), l.align());
        }
        p
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        if should_fail() {
            return std::ptr::null_mut();
        }
        let p = System.alloc_zeroed(l);
        if !p.is_null() {
            record(l.size(), l.align());
        }
        p
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        release(l.size());
        System.dealloc(p, l)
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        with_state(|s| {
            if s.armed {
                s.reallocs += 1;
            }
        });
        if should_fail() {
            return std::ptr::null_mut();
        }
        let q = System.realloc(p, l, n);
        if !q.is_null() {
            release(l.size());
            record(n, l.align());
        }
        q
    }
}

#[global_allocator]
static GLOBAL: FaultAlloc = FaultAlloc;

fn arm(countdown: usize) {
    with_state(|s| {
        *s = State::EMPTY;
        s.armed = true;
        s.countdown = countdown;
    });
}

fn disarm() -> State {
    let mut out = State::EMPTY;
    with_state(|s| {
        s.armed = false;
        out = *s;
    });
    out
}

fn fnv(b: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &x in b {
        h ^= x as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Outcome of one call, recorded without allocating.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum End {
    Trailer,
    Format(&'static str),
    Oom(AllocSite, usize),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Outcome {
    /// Constructor (or still) failed with this.
    ctor: Option<End>,
    frames: [u64; 16],
    nframes: usize,
    end: End,
}

fn end_of(e: &TryError) -> End {
    match e {
        TryError::Format(f) => End::Format(f.name()),
        TryError::OutOfMemory(o) => End::Oom(o.site(), o.elements()),
    }
}

fn run_still(b: &[u8]) -> Outcome {
    let mut o = Outcome { ctor: None, frames: [0; 16], nframes: 0, end: End::Trailer };
    match try_decode_still(b) {
        Ok(s) => {
            o.frames[0] = fnv(&s.rgba);
            o.nframes = 1;
        }
        Err(e) => o.ctor = Some(end_of(&e)),
    }
    o
}

fn run_frames(b: &[u8]) -> Outcome {
    let mut o = Outcome { ctor: None, frames: [0; 16], nframes: 0, end: End::Trailer };
    let mut d = match Frames::try_new(b) {
        Ok(d) => d,
        Err(e) => {
            o.ctor = Some(end_of(&e));
            return o;
        }
    };
    loop {
        match d.next_frame() {
            None => break,
            Some(Err(e)) => {
                o.end = end_of(&e);
                break;
            }
            Some(Ok(f)) => {
                if o.nframes < 16 {
                    o.frames[o.nframes] = fnv(f.rgba);
                }
                o.nframes += 1;
            }
        }
    }
    // Fused / finished: later calls return None.
    assert!(d.next_frame().is_none());
    o
}

// ---- inputs (a tiny encoder for literal-only LZW with Clear before each code)

fn lzw_literals(m: u32, idx: &[u8]) -> Vec<u8> {
    // Clear, then each index as a literal preceded by Clear so the width stays m+1.
    let clear = 1u32 << m;
    let w = m + 1;
    let mut codes = Vec::new();
    for &i in idx {
        codes.push(clear);
        codes.push(i as u32);
    }
    codes.push(clear + 1);
    let mut out = Vec::new();
    let (mut acc, mut n) = (0u64, 0u32);
    for c in codes {
        acc |= (c as u64) << n;
        n += w;
        while n >= 8 {
            out.push(acc as u8);
            acc >>= 8;
            n -= 8;
        }
    }
    if n > 0 {
        out.push(acc as u8);
    }
    out
}

fn image(left: u16, top: u16, w: u16, h: u16, idx: &[u8]) -> Vec<u8> {
    let mut v = vec![0x2C];
    for x in [left, top, w, h] {
        v.extend_from_slice(&x.to_le_bytes());
    }
    v.push(0);
    v.push(2);
    let data = lzw_literals(2, idx);
    for ch in data.chunks(255) {
        v.push(ch.len() as u8);
        v.extend_from_slice(ch);
    }
    v.push(0);
    v
}

fn gce(disposal: u8) -> Vec<u8> {
    vec![0x21, 0xF9, 4, disposal << 2, 5, 0, 0, 0]
}

fn header(w: u16, h: u16) -> Vec<u8> {
    let mut v = b"GIF89a".to_vec();
    v.extend_from_slice(&w.to_le_bytes());
    v.extend_from_slice(&h.to_le_bytes());
    v.extend_from_slice(&[0x81, 0, 0]);
    v.extend_from_slice(&[0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255]);
    v
}

struct Case {
    name: &'static str,
    b: Vec<u8>,
    /// Screen pixels N, or 0 for a header error (PreAlloc).
    n: usize,
    /// The frames entry point reaches a disposal-3 frame.
    saved: bool,
}

fn cases() -> Vec<Case> {
    let mut c = Vec::new();
    let mut b = header(4, 3);
    b.extend(image(0, 0, 4, 3, &[1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0]));
    b.push(0x3B);
    c.push(Case { name: "still-4x3", b, n: 12, saved: false });

    let mut b = header(4, 3);
    b.extend(image(0, 0, 4, 3, &[1; 12]));
    b.extend(gce(3));
    b.extend(image(1, 1, 2, 2, &[2; 4]));
    b.extend(gce(2));
    b.extend(image(0, 0, 2, 1, &[3; 2]));
    b.extend(image(3, 2, 1, 1, &[0]));
    b.push(0x3B);
    c.push(Case { name: "anim-dispose3", b, n: 12, saved: true });

    let mut b = header(4, 3);
    b.extend(gce(3));
    b.extend(image(0, 0, 4, 3, &[1; 12]));
    b.extend(gce(3));
    b.extend(image(0, 0, 4, 3, &[2; 12]));
    b.extend(image(0, 0, 1, 1, &[3]));
    b.extend_from_slice(&[0x2C, 0, 0]); // truncated descriptor
    c.push(Case { name: "dispose3-first-then-truncated", b, n: 12, saved: true });

    let mut b = header(300, 200);
    let px = vec![2u8; 300 * 200];
    b.extend(image(0, 0, 300, 200, &px));
    b.push(0x3B);
    c.push(Case { name: "still-300x200", b, n: 60_000, saved: false });

    let mut b = header(4, 3);
    b[3] = b'9';
    c.push(Case { name: "bad-version", b, n: 0, saved: false });

    let mut b = header(4, 3);
    b.push(0x99); // UnknownBlock in the constructor's prefix walk, after the reservations
    c.push(Case { name: "unknown-block-in-prefix", b, n: 12, saved: false });
    c
}

const LZW_TABLE_BYTES: usize = 20_480;

fn expected_req(case: &Case, frames: bool) -> Vec<(AllocSite, usize, Ev)> {
    if case.n == 0 {
        return Vec::new();
    }
    let n4 = 4 * case.n;
    let mut v = vec![
        (AllocSite::Canvas, n4, Ev { size: n4, align: 1 }),
        (AllocSite::LzwTable, LZW_TABLE_BYTES, Ev { size: LZW_TABLE_BYTES, align: 1 }),
    ];
    if frames && case.saved {
        v.push((AllocSite::Saved, n4, Ev { size: n4, align: 1 }));
    }
    v
}

fn check(case: &Case, frames: bool) {
    let call = |b: &[u8]| if frames { run_frames(b) } else { run_still(b) };
    let req = expected_req(case, frames);

    // Uninjected: physical trace == Req, zero reallocs, peak <= 8N + 20 KiB.
    arm(0);
    let base = call(&case.b);
    let st = disarm();
    assert!(!st.overflow, "{}: trace overflow", case.name);
    assert_eq!(st.reallocs, 0, "{}: realloc", case.name);
    let trace: Vec<Ev> = st.trace[..st.trace_len].to_vec();
    let want: Vec<Ev> = req.iter().map(|r| r.2).collect();
    assert_eq!(trace, want, "{} (frames={frames}): physical trace", case.name);
    assert!(st.peak as usize <= 8 * case.n + LZW_TABLE_BYTES, "{}: peak {}", case.name, st.peak);
    assert!(!matches!(base.ctor, Some(End::Oom(..))) && !matches!(base.end, End::Oom(..)));

    // Fail the k-th reservation for every k the call makes.
    for (k, (site, elems, _)) in req.iter().enumerate() {
        arm(k + 1);
        let got = call(&case.b);
        let st = disarm();
        assert!(st.fired, "{}: countdown {} did not fire", case.name, k + 1);
        assert_eq!(st.reallocs, 0, "{}: realloc under injection", case.name);
        let oom = End::Oom(*site, *elems);
        if !frames || *site != AllocSite::Saved {
            // Still, or a constructor reservation: the call itself fails.
            assert_eq!(got.ctor, Some(oom), "{} (frames={frames}) k={}", case.name, k + 1);
            assert_eq!(got.nframes, 0);
        } else {
            // `Saved` fails inside next_frame: frames before it are identical,
            // then Some(Err(OutOfMemory)) once, then None (checked in run_frames).
            assert_eq!(got.ctor, None);
            assert_eq!(got.end, oom, "{} k={}", case.name, k + 1);
            assert!(got.nframes < base.nframes);
            assert_eq!(got.frames[..got.nframes.min(16)], base.frames[..got.nframes.min(16)]);
        }
    }
    // One past the last reservation: nothing fails, the result is unchanged.
    arm(req.len() + 1);
    let again = call(&case.b);
    let st = disarm();
    assert!(!st.fired);
    assert_eq!(again, base, "{}: result changed", case.name);
}

#[test]
fn still_requests_and_injection() {
    let _g = lock();
    for c in cases() {
        check(&c, false);
    }
}

#[test]
fn frames_requests_and_injection() {
    let _g = lock();
    for c in cases() {
        check(&c, true);
    }
}

#[test]
fn still_never_reserves_saved() {
    // A disposal-3 first frame: Req(still) has no Saved.
    let _g = lock();
    let mut b = header(4, 3);
    b.extend(gce(3));
    b.extend(image(0, 0, 4, 3, &[1; 12]));
    b.push(0x3B);
    arm(0);
    let o = run_still(&b);
    let st = disarm();
    assert_eq!(o.nframes, 1);
    assert_eq!(st.trace_len, 2);
    assert_eq!(st.trace[1].size, LZW_TABLE_BYTES);
}

#[test]
fn saved_reserved_once_per_decoder() {
    // Many disposal-3 frames: exactly one Saved reservation.
    let _g = lock();
    let mut b = header(4, 3);
    for i in 0..20u8 {
        b.extend(gce(3));
        b.extend(image(0, 0, 4, 3, &[i % 4; 12]));
    }
    b.push(0x3B);
    arm(0);
    let o = run_frames(&b);
    let st = disarm();
    assert_eq!(o.nframes, 20);
    assert_eq!(o.end, End::Trailer);
    assert_eq!(st.trace_len, 3, "Canvas, LzwTable, Saved once");
    assert_eq!(st.reallocs, 0);
}
