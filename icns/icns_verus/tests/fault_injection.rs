//! Fault-injection tests for the alloc-delta guard on the ICNS core
//! (THEOREMS-ALLOC-DELTA.md §5; A-ICNS-1..6). These are falsification checks,
//! not proofs: they exercise the guard's binding of `Err` to
//! `try_reserve_exact`'s `Err` arm at run time on the pinned toolchain and
//! allocator.
//!
//! The mechanism is a test-only, unverified `#[global_allocator]` that wraps
//! `System`, arms on a countdown (or on an allocation-size threshold), and
//! returns null on the selected non-zero `alloc`/`alloc_zeroed`/`realloc` of a
//! single decoder call. While armed it also records, for the call:
//!
//! - the ORDERED trace of successful allocations (size, alignment, whether it
//!   was a `realloc`), so the physical trace can be compared one-to-one with the
//!   logical request list;
//! - the live-byte total and its PEAK (the maximum over the call), both updated
//!   only after the underlying allocator returned a non-null pointer, so a
//!   failed request never counts.
//!
//! **Logical versus physical.** The logical request list `L` of a call is the
//! sequence of `(site, elements)` the guard logs. Its zero-element entry (the
//! `EntryList` request of a valid container of 8..=15 bytes, which
//! `try_with_room` logs without calling the allocator) is a logical event with
//! no physical counterpart and is NOT the same thing as `PreAlloc` (an error
//! before the first reservation attempt). The ghost log cannot be read at run
//! time, so the suite recovers `L+` (the requests with k >= 1) from the
//! `OutOfMemory` values the decoder returns under injection at event 1, 2, ...,
//! and checks the successful physical trace against it. The zero-element event
//! and `PreAlloc` are told apart by an independent oracle of R2-R4 (the header
//! checks) that this file implements; the file reports three counts: `PreAlloc`
//! cases, zero-element-request cases and physical-allocation cases.
//!
//! **Serialisation and thread-locality.** Rust runs test functions on several
//! threads, and the allocator callbacks run on the thread that makes the
//! request. The counters, the injector and the live-byte total are therefore
//! `thread_local`, so another test's harness allocations can neither consume the
//! countdown nor be faulted by it. Every test still takes `SUITE_LOCK` for its
//! whole body, so the tests do not interleave their output. Do not add a test to
//! this file that skips the lock. No allocator callback allocates (the trace is a
//! fixed array).
//!
//! ICNS makes up to six physical reservations in one call (seven logical
//! requests with the zero-element one): the entry list
//! (`try_with_room(EntryList, min(4096, (|b|-8)/8))`), three channel buffers
//! (`try_zeroed(Channel(k), n)`), the RGB output (`try_zeroed(RgbOut, 3n)`), the
//! mask copy (`try_copy(MaskData, payload)`) and the combine output
//! (`try_zeroed(CombineOut, 4n)`). A single call reaches a subset of them, so
//! the suite injects a failure at *each* non-zero reservation in turn (the k-th
//! reservation, for every k the call makes) and checks the exact site and size,
//! that no partial output escapes, that no other error appears, and that the
//! result is otherwise identical. It also runs the size-threshold schedules
//! T in {1, 4,096, 65,536, 67,108,864} windowed to each of the call's own
//! request sizes.
//!
//! **The handle.** `icns_verus`'s decoders are methods on `Parsed`, an opaque
//! handle whose only constructor is `try_parse`. For a decode vector the suite
//! parses **outside** the injection window: `check_vector` builds the handle
//! before arming, and `combine_probe` does the same, so the decoder's own
//! reservations are the window's events and the entry list is live at the
//! window's baseline, not one of them. A `Parse` vector parses inside its call
//! (its `Req` *is* the entry-list reservation). The entry list is at most
//! 4,096 * 28 = 114,688 bytes and is live until the call ends; the parsed handle
//! is dropped before the statistics are read, so the live bytes after a decode
//! call are still just the output's buffer.
//!
//! **Peak live bytes.** The peak numbers asserted here (`6*n_c` during the colour
//! decode, `3*n_c + n_m` until the side check, `8n` after a matching check,
//! 131,072 bytes at most, 114,688 for the entry list) are an OWNERSHIP FACT about
//! how the decoder holds its buffers, checked by this test on the pinned std and
//! allocator. They are not a verified clause: no Verus theorem states them
//! (SPEC.md §4).
//!
//! This is a separate std test crate, outside `icns_verus`'s verified core and
//! its `no_std` boundary, so `unsafe impl GlobalAlloc` here is not a proof
//! escape in the core; it is the standard way to intercept the process
//! allocator.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::sync::Mutex;

use icns_verus::alloc_guard::AllocSite;
use icns_verus::icns::{try_parse, Decoded, Entry, Error, Parsed, TryError};

/// Serialises every test in this file. See the module docs.
static SUITE_LOCK: Mutex<()> = Mutex::new(());

fn lock() -> std::sync::MutexGuard<'static, ()> {
    SUITE_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

/// Largest number of successful allocations one call can make (the call makes at
/// most six); the trace is a fixed array because the allocator must not allocate.
const TRACE_CAP: usize = 16;

/// One successful allocator event.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
struct Ev {
    size: usize,
    align: usize,
    realloc: bool,
}
const EV0: Ev = Ev { size: 0, align: 0, realloc: false };

/// Per-thread injector state. A `Cell` keeps the fields together; the allocator
/// callbacks read and write them without a lock (the allocator must not
/// allocate, and a thread-local access does not).
#[derive(Clone, Copy)]
struct State {
    armed: bool,
    countdown: isize,
    /// Set when the countdown reached zero (the k-th request was failed).
    countdown_fired: bool,
    threshold: usize,
    threshold_window: usize,
    threshold_hit: usize,
    allocs: usize,
    reallocs: usize,
    /// Live bytes: incremented only after the underlying allocation succeeded.
    live: isize,
    /// Maximum of `live` since the last `arm`.
    peak: isize,
    /// `live` at the moment the injector failed a request (the failed request
    /// itself is not included).
    live_at_fail: isize,
    /// Bytes already live when the window was armed (the parsed entry list a
    /// decode call inherits). `live`, `peak` and `live_at_fail` are reported
    /// net of it, so the numbers measure the decoder alone.
    baseline_live: isize,
    /// Failures returned by the underlying `System` allocator (not injected).
    sys_failures: usize,
    trace: [Ev; TRACE_CAP],
    trace_len: usize,
    trace_overflow: bool,
    watch_size: usize,
    watch_live: isize,
    watch_align: usize,
    last_size: usize,
    last_align: usize,
    failed_size: usize,
    failed_align: usize,
}
impl State {
    const EMPTY: Self = Self {
        armed: false, countdown: 0, countdown_fired: false, threshold: 0,
        threshold_window: 0, threshold_hit: 0, allocs: 0, reallocs: 0, live: 0,
        peak: 0, live_at_fail: 0, baseline_live: 0, sys_failures: 0, trace: [EV0; TRACE_CAP],
        trace_len: 0, trace_overflow: false,
        watch_size: 0, watch_live: 0, watch_align: 0, last_size: 0, last_align: 0,
        failed_size: 0, failed_align: 0,
    };
}

std::thread_local! { static STATE: Cell<State> = const { Cell::new(State::EMPTY) }; }

/// Read the state, run `f`, and write the (possibly changed) fields back.
fn with_state(f: impl FnOnce(&mut State)) {
    let _ = STATE.try_with(|cell| {
        let mut s = cell.get();
        f(&mut s);
        cell.set(s);
    });
}

struct FaultAlloc;

/// Whether this request should be failed, updating the countdown and the
/// threshold bookkeeping.
fn should_fail(size: usize, align: usize) -> bool {
    let mut fail = false;
    with_state(|s| {
        if !s.armed {
            return;
        }
        s.allocs += 1;
        if s.countdown > 0 && s.allocs as isize == s.countdown {
            s.countdown = 0;
            s.countdown_fired = true;
            s.failed_size = size;
            s.failed_align = align;
            s.live_at_fail = std::cmp::max(0, s.live - s.baseline_live);
            fail = true;
            return;
        }
        if s.threshold > 0 && size >= s.threshold
            && (s.threshold_window == 0 || size == s.threshold_window)
        {
            s.threshold_hit += 1;
            s.failed_size = size;
            s.failed_align = align;
            s.live_at_fail = std::cmp::max(0, s.live - s.baseline_live);
            fail = true;
        }
    });
    fail
}

/// Called only after the underlying allocator returned a non-null pointer.
fn record_success(size: usize, align: usize, realloc: bool) {
    with_state(|s| {
        s.live += size as isize;
        // Net of the bytes live at arm time (a decode call's parsed entry list).
        // Their deallocation can take `live` below the baseline, which nets to
        // zero: the decoder owns nothing of it.
        let net = s.live - s.baseline_live;
        if net > s.peak {
            s.peak = net;
        }
        s.last_size = size;
        s.last_align = align;
        if s.armed {
            if s.trace_len < TRACE_CAP {
                s.trace[s.trace_len] = Ev { size, align, realloc };
                s.trace_len += 1;
            } else {
                s.trace_overflow = true;
            }
        }
        if s.watch_size != 0 && size == s.watch_size {
            s.watch_align = align;
            s.watch_live += 1;
        }
    });
}

fn record_failure(size: usize, align: usize) {
    with_state(|s| {
        s.sys_failures += 1;
        s.failed_size = size;
        s.failed_align = align;
    });
}

fn record_dealloc(size: usize) {
    with_state(|s| {
        s.live -= size as isize;
        if s.watch_size != 0 && size == s.watch_size {
            s.watch_live -= 1;
        }
    });
}

unsafe impl GlobalAlloc for FaultAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() == 0 {
            return System.alloc(layout);
        }
        if should_fail(layout.size(), layout.align()) {
            return core::ptr::null_mut();
        }
        let p = System.alloc(layout);
        if !p.is_null() {
            record_success(layout.size(), layout.align(), false);
        } else {
            record_failure(layout.size(), layout.align());
        }
        p
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if layout.size() == 0 {
            return System.alloc_zeroed(layout);
        }
        if should_fail(layout.size(), layout.align()) {
            return core::ptr::null_mut();
        }
        let p = System.alloc_zeroed(layout);
        if !p.is_null() {
            record_success(layout.size(), layout.align(), false);
        } else {
            record_failure(layout.size(), layout.align());
        }
        p
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        with_state(|s| {
            if s.armed {
                s.reallocs += 1;
            }
        });
        if should_fail(new_size, layout.align()) {
            return core::ptr::null_mut();
        }
        let p = System.realloc(ptr, layout, new_size);
        if !p.is_null() {
            record_dealloc(layout.size());
            record_success(new_size, layout.align(), true);
        } else {
            record_failure(new_size, layout.align());
        }
        p
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if layout.size() != 0 {
            record_dealloc(layout.size());
        }
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static ALLOCATOR: FaultAlloc = FaultAlloc;

/// Set `watch_size` (used by the leak check).
fn set_watch(size: usize) {
    with_state(|s| {
        s.watch_size = size;
        s.watch_live = 0;
        s.watch_align = 0;
    });
}

/// Live bytes of the watched size and the total live bytes, right now.
fn peek_live() -> (isize, isize) {
    let mut out = (0, 0);
    with_state(|s| out = (s.watch_live, s.live));
    out
}

/// Arm the countdown to fail the `k`-th non-zero request; reset the counters.
/// Arm the countdown and reset the counters. Bytes already live (the entry list
/// of a handle parsed before the window) are carried across the reset as
/// `baseline_live`: every live-byte number this window reports is net of them,
/// because SPEC.md §4 assigns the entry list to `try_parse`, not to the decoder.
fn arm(k: usize) {
    with_state(|s| {
        let ws = s.watch_size;
        let live = s.live;
        *s = State::EMPTY;
        s.watch_size = ws;
        s.live = live;
        s.baseline_live = live;
        s.armed = true;
        s.countdown = k as isize;
    });
}

/// `arm(k)` for probes that may not inject: `None` observes.
fn arm_probe(k: Option<usize>) {
    match k {
        Some(k) => arm(k),
        None => arm_observe(),
    }
}

/// Arm with a countdown that never fires: observe the trace and peak of a call
/// that is not failed.
fn arm_observe() {
    arm(usize::MAX >> 2);
}

/// `arm_threshold`, with the bytes already live taken as the baseline. See
/// `arm`.
fn arm_probe_threshold(t: usize, window: usize) {
    arm_threshold(t, window);
}

/// Arm the size-threshold mode for requests of `>= t` bytes, confined to
/// requests of exactly `window` bytes (`0` = every request).
fn arm_threshold(t: usize, window: usize) {
    with_state(|s| {
        let ws = s.watch_size;
        let live = s.live;
        *s = State::EMPTY;
        s.watch_size = ws;
        s.live = live;
        s.baseline_live = live;
        s.armed = true;
        s.threshold = t;
        s.threshold_window = window;
    });
}

/// Disarm and return the counters gathered while armed.
#[derive(Clone, Copy)]
struct Stats {
    countdown_fired: bool,
    allocs: usize,
    reallocs: usize,
    live: isize,
    peak: isize,
    live_at_fail: isize,
    sys_failures: usize,
    trace: [Ev; TRACE_CAP],
    trace_len: usize,
    trace_overflow: bool,
    failed_size: usize,
    failed_align: usize,
    threshold_hit: usize,
}

impl Stats {
    fn trace(&self) -> &[Ev] {
        &self.trace[..self.trace_len]
    }
}

fn disarm() -> Stats {
    let mut out = Stats {
        countdown_fired: false, allocs: 0, reallocs: 0, live: 0, peak: 0,
        live_at_fail: 0, sys_failures: 0, trace: [EV0; TRACE_CAP], trace_len: 0,
        trace_overflow: false, failed_size: 0, failed_align: 0, threshold_hit: 0,
    };
    with_state(|s| {
        out = Stats {
            countdown_fired: s.countdown_fired,
            allocs: s.allocs,
            reallocs: s.reallocs,
            live: std::cmp::max(0, s.live - s.baseline_live),
            peak: s.peak,
            live_at_fail: s.live_at_fail,
            sys_failures: s.sys_failures,
            trace: s.trace,
            trace_len: s.trace_len,
            trace_overflow: s.trace_overflow,
            failed_size: s.failed_size,
            failed_align: s.failed_align,
            threshold_hit: s.threshold_hit,
        };
        s.armed = false;
        s.threshold = 0;
        s.threshold_window = 0;
    });
    out
}

// ---------------------------------------------------------------------------
// Frozen vectors.
// ---------------------------------------------------------------------------

fn nibble(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("non-hex byte in vector"),
    }
}

fn hex_decode(s: &str) -> Vec<u8> {
    assert!(s.len() % 2 == 0, "hex string has even length");
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(s.len() / 2);
    let mut i = 0;
    while i < bytes.len() {
        out.push((nibble(bytes[i]) << 4) | nibble(bytes[i + 1]));
        i += 2;
    }
    out
}

/// One frozen vector, reduced to what the fault suite needs: the input bytes and
/// the operation with its indices. A decode call reaches the decoder only
/// through a `Parsed` handle, and the handle's only constructor is `try_parse`,
/// so a decode vector's handle is parsed **outside** its injection window (see
/// `check_vector`). The entry-list reservation is therefore not part of a decode
/// call's log, exactly as SPEC.md §4 assigns it to `try_parse`; the window sees
/// only the decoder's own sites. The frozen corpus classifies each call by the
/// number of physical events, and for a decode vector those are the decoder's.
#[derive(Clone)]
struct Vector {
    name: String,
    bytes: Vec<u8>,
    op: Op,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    Parse,
    Rgb(u32),
    Mask(u32),
    Combine(Option<u32>, Option<u32>),
}

/// A vector built in the test (not read from VECTORS.json).
fn make_vector(name: &str, bytes: Vec<u8>, op: Op) -> Vector {
    Vector { name: name.to_string(), bytes, op }
}

fn vectors() -> Vec<Vector> {
    use std::path::PathBuf;
    use std::sync::OnceLock;
    use serde_json::Value;
    let path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate has a parent")
        .join("spec-frozen/VECTORS.json");
    static CACHE: OnceLock<Vec<Vector>> = OnceLock::new();
    CACHE
        .get_or_init(|| {
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
            let vectors: Value = serde_json::from_str(&text).expect("VECTORS.json parses");
            let arr = vectors.as_array().expect("top-level array");
            let mut out = Vec::new();
            for v in arr {
                let name = v["name"].as_str().expect("name").to_string();
                let hex = v["input_hex"].as_str().expect("input_hex");
                let repeat = v.get("input_repeat").and_then(|r| r.as_u64()).unwrap_or(1) as usize;
                let unit = hex_decode(hex);
                let mut bytes = Vec::with_capacity(unit.len() * repeat);
                for _ in 0..repeat {
                    bytes.extend_from_slice(&unit);
                }
                let op = match v["operation"].as_str().expect("operation") {
                    "parse" => Op::Parse,
                    "rgb" => Op::Rgb(v["color_index"].as_u64().expect("color_index") as u32),
                    "mask" => Op::Mask(v["mask_index"].as_u64().expect("mask_index") as u32),
                    "combine" => Op::Combine(
                        v.get("color_index").and_then(|x| x.as_u64()).map(|x| x as u32),
                        v.get("mask_index").and_then(|x| x.as_u64()).map(|x| x as u32),
                    ),
                    other => panic!("unexpected operation {other}"),
                };
                out.push(Vector { name, bytes, op });
            }
            out
        })
        .clone()
}

/// The result of one call, in a form two runs can be compared field for field.
/// `Parsed` keeps the entry count, read from the handle without allocating; the
/// entry list's capacity is a fact of the allocator trace alone (the handle no
/// longer reports it: vstd cannot specify `Vec::capacity`, so the core does not
/// expose it). The entry *fields* are compared by the conformance suite, which
/// runs all 111 vectors unarmed; copying them out here would allocate inside an
/// armed window and consume the injector's countdown.
#[derive(PartialEq, Debug)]
enum Outcome {
    Parsed { len: usize },
    Decoded(Decoded),
    Format(Error),
    Oom(AllocSite, usize),
}

/// The outcome of one decoder result.
fn outcome_of(r: Result<Decoded, TryError>) -> Outcome {
    match r {
        Ok(d) => Outcome::Decoded(d),
        Err(TryError::Format(e)) => Outcome::Format(e),
        Err(TryError::OutOfMemory(o)) => Outcome::Oom(o.site(), o.elements()),
    }
}

/// One call of a vector. A `Parse` vector parses inside the call (its `Req` *is*
/// the entry-list reservation). A decode vector is the decoder's call, and
/// SPEC.md §4 assigns the entry list to `try_parse`, so the parse is performed
/// here, before any arming the caller does around this function's tail — but
/// `run` is called both inside and outside armed windows, so it takes the
/// handle when the caller has one and parses otherwise. See `check_vector`,
/// which always passes a handle for a decode vector.
fn run_with(v: &Vector, parsed: Option<&Parsed<'_>>) -> Outcome {
    match v.op {
        Op::Parse => match try_parse(&v.bytes) {
            Ok(p) => Outcome::Parsed { len: p.entries().len() },
            Err(TryError::Format(e)) => Outcome::Format(e),
            Err(TryError::OutOfMemory(o)) => Outcome::Oom(o.site(), o.elements()),
        },
        Op::Rgb(i) => match parsed {
            Some(p) => outcome_of(p.try_decode_rgb(i)),
            None => match try_parse(&v.bytes) {
                Ok(p) => outcome_of(p.try_decode_rgb(i)),
                Err(e) => store_parse_failure(e),
            },
        },
        Op::Mask(i) => match parsed {
            Some(p) => outcome_of(p.try_decode_mask(i)),
            None => match try_parse(&v.bytes) {
                Ok(p) => outcome_of(p.try_decode_mask(i)),
                Err(e) => store_parse_failure(e),
            },
        },
        Op::Combine(ci, mi) => match parsed {
            Some(p) => outcome_of(p.try_combine(ci, mi)),
            None => match try_parse(&v.bytes) {
                Ok(p) => outcome_of(p.try_combine(ci, mi)),
                Err(e) => store_parse_failure(e),
            },
        },
    }
}

/// A parse failure is reported as the decoder's outcome: the decoder never ran.
fn store_parse_failure(e: TryError) -> Outcome {
    match e {
        TryError::Format(e) => Outcome::Format(e),
        TryError::OutOfMemory(o) => Outcome::Oom(o.site(), o.elements()),
    }
}

/// `run_with` without a caller-held handle: the unarmed control, and the shape
/// used by tests that do not care where the parse sits in the window.
fn run(v: &Vector) -> Outcome {
    run_with(v, None)
}

// ---------------------------------------------------------------------------
// Logical requests, the header oracle and the ownership model.
// ---------------------------------------------------------------------------

const MAX_INPUT: usize = 67_108_864;
/// The mandated threshold schedule (THEOREMS-ALLOC-DELTA.md §5).
const THRESHOLDS: [usize; 4] = [1, 4_096, 65_536, 67_108_864];
/// The absolute bound on peak live bytes of a call (128 KiB; SPEC.md §4).
const PEAK_BOUND: usize = 131_072;

/// A logical request with `elements >= 1`, in the order the call made it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Req {
    site: AllocSite,
    elements: usize,
}

impl Req {
    /// The `EntryList` request is `count` elements of `Entry`; every other site
    /// is bytes.
    fn elem_size(&self) -> usize {
        match self.site {
            AllocSite::EntryList => std::mem::size_of::<Entry>(),
            _ => 1,
        }
    }
    fn elem_align(&self) -> usize {
        match self.site {
            AllocSite::EntryList => std::mem::align_of::<Entry>(),
            _ => 1,
        }
    }
    fn bytes(&self) -> usize {
        self.elements * self.elem_size()
    }
}

/// R2-R4 as an independent oracle: the header checks that precede the entry-list
/// reservation. `true` iff the call reaches `try_with_room(EntryList, ..)`.
fn header_ok(b: &[u8]) -> bool {
    b.len() <= MAX_INPUT
        && b.len() >= 8
        && &b[..4] == b"icns"
        && u32::from_be_bytes([b[4], b[5], b[6], b[7]]) as usize == b.len()
}

/// The entry-list request size for a container that passes the header checks.
fn entry_count(b: &[u8]) -> usize {
    std::cmp::min(4096, (b.len() - 8) / 8)
}

/// The three kinds of case (THEOREMS-ALLOC-DELTA.md §5).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Class {
    /// Error before the first reservation attempt: no logical request, no event.
    PreAlloc,
    /// The logical `(EntryList, 0, true)` request: no physical allocation.
    ZeroElement,
    /// At least one physical allocation.
    Physical,
}

/// Classify a call from the oracle and the number of physical events; assert the
/// two agree.
fn classify(v: &Vector, n_phys: usize, obs: &Outcome, found: &[Req]) -> Class {
    match v.op {
        Op::Parse => {
            if header_ok(&v.bytes) {
                let count = entry_count(&v.bytes);
                if count == 0 {
                    assert_eq!(n_phys, 0, "{}: a zero-element request reached the allocator", v.name);
                    match obs {
                        Outcome::Parsed { len, .. } => assert!(*len == 0, "{}", v.name),
                        Outcome::Format(Error::TruncatedEntryHeader) => {}
                        other => panic!("{}: zero-element request gave {other:?}", v.name),
                    }
                    Class::ZeroElement
                } else {
                    assert_eq!(n_phys, 1, "{}: the entry-list reservation is one event", v.name);
                    assert_eq!(found[0], Req { site: AllocSite::EntryList, elements: count },
                        "{}: logical entry-list request", v.name);
                    Class::Physical
                }
            } else {
                assert_eq!(n_phys, 0, "{}: a header error reached the allocator", v.name);
                assert!(matches!(obs, Outcome::Format(_)), "{}: PreAlloc parse succeeded", v.name);
                Class::PreAlloc
            }
        }
        _ => {
            if n_phys == 0 {
                assert!(matches!(obs, Outcome::Format(_)), "{}: PreAlloc decode succeeded", v.name);
                Class::PreAlloc
            } else {
                Class::Physical
            }
        }
    }
}

/// The ownership-fact peak of a call that made `events` successful allocations,
/// in order, `(site, bytes)`: the maximum over the call of the bytes held. The
/// three channel buffers are dropped once the RGB output exists; the RGB output
/// is held until the call returns; the mask copy and the combine output are
/// added as they appear. A failed request is not in `events`, so it counts for
/// nothing.
fn ownership_peak(events: &[(AllocSite, usize)]) -> usize {
    let (mut live, mut peak, mut chan) = (0usize, 0usize, 0usize);
    for &(site, size) in events {
        live += size;
        peak = peak.max(live);
        match site {
            AllocSite::Channel(_) => chan += size,
            AllocSite::RgbOut => {
                live -= chan;
                chan = 0;
            }
            _ => {}
        }
    }
    peak
}

/// The site order and size relations the decoders promise, from the spec's
/// `Req_P`, `Req_R`, `Req_C`, `Req_A` and `Req_J`.
fn check_site_order(v: &Vector, reqs: &[Req]) {
    let sites: Vec<AllocSite> = reqs.iter().map(|r| r.site).collect();
    let full: &[AllocSite] = match v.op {
        Op::Parse => &[AllocSite::EntryList],
        Op::Rgb(_) => &[AllocSite::Channel(0), AllocSite::Channel(1), AllocSite::Channel(2), AllocSite::RgbOut],
        Op::Mask(_) => &[AllocSite::MaskData],
        Op::Combine(..) => &[
            AllocSite::Channel(0), AllocSite::Channel(1), AllocSite::Channel(2),
            AllocSite::RgbOut, AllocSite::MaskData, AllocSite::CombineOut,
        ],
    };
    assert!(sites.len() <= full.len() && sites[..] == full[..sites.len()],
        "{}: request order {sites:?} is not a prefix of {full:?}", v.name);
    if let Some(c0) = reqs.first().filter(|r| matches!(r.site, AllocSite::Channel(_))) {
        let n = c0.elements;
        assert!([256, 1024, 2304, 16384].contains(&n), "{}: channel size {n}", v.name);
        for r in reqs {
            match r.site {
                AllocSite::Channel(_) => assert_eq!(r.elements, n, "{}", v.name),
                AllocSite::RgbOut => assert_eq!(r.elements, 3 * n, "{}: RgbOut is 3n", v.name),
                AllocSite::CombineOut => assert_eq!(r.elements, 4 * n, "{}: CombineOut is 4n", v.name),
                AllocSite::MaskData => {
                    // The combine output exists only when the sides match.
                    if reqs.iter().any(|q| q.site == AllocSite::CombineOut) {
                        assert_eq!(r.elements, n, "{}: matching mask", v.name);
                    }
                }
                _ => {}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// The suite.
// ---------------------------------------------------------------------------

/// Per-schedule suite counters, outside the allocator callbacks.
#[derive(Default)]
struct SuiteCounts {
    cases: usize,
    /// Error before the first reservation attempt.
    prealloc: usize,
    /// Logical zero-element `EntryList` request, no physical allocation.
    zero_element: usize,
    zero_element_names: Vec<String>,
    /// At least one physical allocation.
    allocating: usize,
    max_request: usize,
    max_peak: usize,
    /// Cases where the k-th reservation exists (k = 1..=6).
    reached: [usize; 7],
    /// Cases where the k-th reservation was failed.
    injected: [usize; 7],
    /// Physical events compared with their logical request.
    trace_events: usize,
    /// ... of which the physical size equals the logical request exactly.
    trace_exact: usize,
    /// Cases where some request is at least `THRESHOLDS[i]` bytes.
    thr_reached: [usize; 4],
    /// Cases where a threshold schedule injected a failure.
    thr_injected: [usize; 4],
    thr_runs: usize,
}

/// Check one vector: the control, the observed successful trace and peak, a
/// failure at each reservation in turn, and the threshold schedules windowed to
/// each of the call's own request sizes.
fn check_vector(v: &Vector, c: &mut SuiteCounts) {
    c.cases += 1;
    let baseline = run(v);
    assert!(!matches!(baseline, Outcome::Oom(..)), "{}: control hit OutOfMemory", v.name);

    // A decode vector's armed window must contain the decoder only. `run_with`
    // parses before arming (see `run_with`'s doc), so the parse's entry-list
    // reservation is a baseline of every window, not an event in each.
    let parsed: Option<Parsed<'_>> = match v.op {
        Op::Parse => None,
        _ => match try_parse(&v.bytes) {
            Ok(p) => Some(p),
            Err(_) => None,
        },
    };
    let run = |v: &Vector| run_with(v, parsed.as_ref());

    // (a) No failure injected: the ordered physical trace and the peak.
    arm_probe(None);
    let obs = run(v);
    let ost = disarm();
    assert_eq!(obs, baseline, "{}: result differs from the control", v.name);
    assert!(!ost.trace_overflow, "{}: trace overflow", v.name);
    assert!(!ost.countdown_fired && ost.threshold_hit == 0, "{}: control injected", v.name);
    assert_eq!(ost.sys_failures, 0, "{}: the underlying allocator failed", v.name);
    assert_eq!(ost.reallocs, 0, "{}: a call issued a realloc (A-STD-2)", v.name);
    assert!(ost.trace().iter().all(|e| !e.realloc), "{}: realloc in the trace", v.name);
    let trace: Vec<Ev> = ost.trace().to_vec();
    assert_eq!(ost.allocs, trace.len(), "{}: attempts vs successes", v.name);

    // (b) A failure at each physical reservation in turn. The k-th failure
    // names the k-th logical request with k >= 1 (the zero-element request is
    // not an allocator event and never fires).
    let mut found: Vec<Req> = Vec::new();
    let mut k = 1;
    loop {
        arm_probe(Some(k));
        let r = run(v);
        let st = disarm();
        if !st.countdown_fired {
            // A countdown past the last reservation does not fire and changes
            // nothing.
            assert_eq!(st.allocs, found.len(), "{}: a reservation past the last", v.name);
            assert_eq!(r, baseline, "{}: result differs from the control", v.name);
            break;
        }
        let req = match r {
            Outcome::Oom(site, elements) => Req { site, elements },
            other => panic!("{}: k={k} gave {other:?}, not OutOfMemory", v.name),
        };
        assert_eq!(st.reallocs, 0, "{}: k={k} issued a realloc (A-STD-2)", v.name);
        assert_eq!(st.sys_failures, 0, "{}: k={k}", v.name);
        // The failed physical event is the k-th event of the control run.
        assert!(trace.len() >= k, "{}: k={k} past the control trace", v.name);
        assert_eq!((st.failed_size, st.failed_align), (trace[k - 1].size, trace[k - 1].align),
            "{}: k={k} failed event differs from the control's event", v.name);
        assert_eq!(st.failed_size, req.bytes(), "{}: k={k} ghost vs failed event", v.name);
        assert_eq!(st.failed_align, req.elem_align(), "{}: k={k} reservation alignment", v.name);
        // Successes before the failure are exactly the earlier events.
        assert_eq!(st.trace(), &trace[..k - 1], "{}: k={k} trace prefix", v.name);
        // The failed reservation leaves no live bytes behind and its failed
        // request is not counted: the peak is the peak of the earlier events.
        assert_eq!(st.live, 0, "{}: k={k} retained live bytes after OutOfMemory", v.name);
        let prefix: Vec<(AllocSite, usize)> =
            found.iter().zip(&trace).map(|(q, e)| (q.site, e.size)).collect();
        assert_eq!(st.peak as usize, ownership_peak(&prefix),
            "{}: k={k} peak counts a failed request or a leaked buffer", v.name);
        c.max_request = c.max_request.max(st.failed_size);
        found.push(req);
        k += 1;
        assert!(k <= 8, "{}: more than 8 reservations", v.name);
    }
    let n_phys = found.len();
    assert_eq!(n_phys, trace.len(), "{}: logical L+ and physical trace differ in length", v.name);
    for j in 0..n_phys {
        c.reached[j + 1] += 1;
        c.injected[j + 1] += 1;
    }

    // (c) Success trace versus the logical request list, in order.
    for (req, ev) in found.iter().zip(&trace) {
        assert!(ev.size >= req.bytes(), "{}: {req:?} got a smaller allocation {ev:?}", v.name);
        assert_eq!(ev.align, req.elem_align(), "{}: {req:?} alignment", v.name);
        assert!(!ev.realloc, "{}", v.name);
        c.trace_events += 1;
        c.trace_exact += usize::from(ev.size == req.bytes());
    }
    check_site_order(v, &found);

    // (d) Classes: PreAlloc, zero-element request, physical.
    let class = classify(v, n_phys, &obs, &found);
    match class {
        Class::PreAlloc => c.prealloc += 1,
        Class::ZeroElement => {
            c.zero_element += 1;
            c.zero_element_names.push(v.name.clone());
        }
        Class::Physical => c.allocating += 1,
    }
    if class != Class::Physical {
        // No event at all, under any schedule: a pure size threshold of one
        // byte over every request finds nothing to fail.
        arm_probe_threshold(1, 0);
        let r = run(v);
        let st = disarm();
        assert_eq!((st.allocs, st.threshold_hit, st.trace_len), (0, 0, 0),
            "{}: {class:?} reached the allocator", v.name);
        assert_eq!(r, baseline, "{}", v.name);
    }

    // (e) Capacity and live bytes of the returned buffer, and the peak.
    let held = match &obs {
        Outcome::Parsed { len } => {
            // The entry list is the live buffer, and its size is the reservation
            // the trace recorded (zero for the zero-element request). The length
            // came from the handle, the size from the trace; (e) below checks
            // they agree.
            assert!(*len <= found.first().map_or(0, |f| f.elements), "{}", v.name);
            ost.live as usize
        }
        Outcome::Decoded(d) => {
            let last = *found.last().expect("a successful decode reserved its output");
            let site = match v.op {
                Op::Rgb(_) => AllocSite::RgbOut,
                Op::Mask(_) => AllocSite::MaskData,
                _ => AllocSite::CombineOut,
            };
            assert_eq!(last.site, site, "{}: output site", v.name);
            assert_eq!(d.data.len(), last.elements, "{}: output length", v.name);
            assert!(d.data.capacity() >= last.elements, "{}: output capacity", v.name);
            d.data.capacity()
        }
        _ => 0,
    };
    // `held` is what the call leaves live: for a decode, its output. A `Parse`
    // vector's entry list is also the handle it returns, but the handle is live
    // before the window opens (the parse runs outside it), so it must not be
    // added to the window's net live bytes.
    let want_live = match v.op {
        Op::Parse => 0,
        _ => held,
    };
    assert_eq!(ost.live as usize, want_live, "{}: live bytes after the call are not the result's buffer", v.name);
    let events: Vec<(AllocSite, usize)> =
        found.iter().zip(&trace).map(|(q, e)| (q.site, e.size)).collect();
    assert_eq!(ost.peak as usize, ownership_peak(&events), "{}: peak vs ownership model", v.name);
    assert!(ost.peak as usize >= want_live, "{}: peak below the live result", v.name);
    assert!(ost.peak as usize <= PEAK_BOUND, "{}: peak {} above 128 KiB", v.name, ost.peak);
    c.max_peak = c.max_peak.max(ost.peak as usize);

    // (f) Size-threshold schedules, windowed to each of the call's own request
    // sizes so only the decoder's reservation can be faulted.
    let mut sizes: Vec<usize> = Vec::new();
    for e in &trace {
        if !sizes.contains(&e.size) {
            sizes.push(e.size);
        }
    }
    for (ti, &t) in THRESHOLDS.iter().enumerate() {
        let mut any_reached = false;
        let mut any_injected = false;
        for &s in &sizes {
            let first = trace.iter().position(|e| e.size == s).expect("size from the trace");
            arm_probe_threshold(t, s);
            let r = run(v);
            let st = disarm();
            c.thr_runs += 1;
            assert_eq!(st.reallocs, 0, "{}: threshold {t} issued a realloc", v.name);
            assert_eq!(st.sys_failures, 0, "{}", v.name);
            if s >= t {
                any_reached = true;
                any_injected = true;
                assert_eq!(st.threshold_hit, 1, "{}: threshold {t} window {s}: one failed request", v.name);
                assert_eq!(r, Outcome::Oom(found[first].site, found[first].elements),
                    "{}: threshold {t} window {s}: site/size", v.name);
                assert_eq!((st.failed_size, st.failed_align), (s, found[first].elem_align()), "{}", v.name);
                assert_eq!(st.trace(), &trace[..first], "{}: threshold {t} trace prefix", v.name);
                assert_eq!(st.live, 0, "{}: threshold {t} retained live bytes", v.name);
                let prefix: Vec<(AllocSite, usize)> = events[..first].to_vec();
                assert_eq!(st.peak as usize, ownership_peak(&prefix), "{}: threshold {t} peak", v.name);
            } else {
                assert_eq!(st.threshold_hit, 0, "{}: threshold {t} failed a smaller request ({s})", v.name);
                assert_eq!(r, baseline, "{}: threshold {t} window {s} changed the result", v.name);
                assert_eq!(st.trace(), &trace[..], "{}: threshold {t} changed the trace", v.name);
            }
        }
        c.thr_reached[ti] += usize::from(any_reached);
        c.thr_injected[ti] += usize::from(any_injected);
    }
}

/// Every schedule must have injected a failure at least once across the suite,
/// or be stated unreachable with the reason.
fn assert_suite_exercised(c: &SuiteCounts) {
    println!(
        "fault suite: {} cases = {} PreAlloc (no logical request, no event) + {} zero-element EntryList request (logical event, no physical allocation) + {} physical-allocation",
        c.cases, c.prealloc, c.zero_element, c.allocating
    );
    println!(
        "  largest reservation {} bytes, largest peak live bytes {}",
        c.max_request, c.max_peak
    );
    println!(
        "  physical trace vs logical list: {} events compared, {} exactly the requested size, 0 reallocs",
        c.trace_events, c.trace_exact
    );
    for k in 1..=6 {
        println!("  reservation {k}: reached in {} cases, failed in {}", c.reached[k], c.injected[k]);
    }
    for (i, t) in THRESHOLDS.iter().enumerate() {
        println!("  threshold {t} (windowed): trigger reachable in {} cases, failure injected in {}",
            c.thr_reached[i], c.thr_injected[i]);
    }
    assert!(c.cases > 0);
    assert_eq!(c.prealloc + c.zero_element + c.allocating, c.cases, "classes partition the cases");
    assert!(c.allocating > 0, "no case allocates at all");
    assert!(c.prealloc > 0, "no PreAlloc case in the corpus");
    // The two frozen vectors that reach `try_with_room(EntryList, 0)`.
    for name in ["minimal-empty", "truncated-entry-header"] {
        assert!(c.zero_element_names.iter().any(|n| n == name),
            "{name} was not classified as a zero-element request");
    }
    // The frozen vectors reach at least the entry list, the three channels and
    // the RGB output; the mask copy and the combine output are reached by the
    // mask and combine vectors.
    for k in 1..=6 {
        assert!(c.injected[k] > 0, "reservation {k} never failed in any case");
        assert_eq!(c.reached[k], c.injected[k], "reservation {k}: reached but not failed");
    }
    // Pinned on the frozen corpus (111 vectors): 87/33/33/16/3/2 failures.
    assert_eq!(&c.injected[1..], &[87, 33, 33, 16, 3, 2], "per-reservation failure counts");
    assert_eq!(c.allocating, 87, "physical-allocation cases");
    assert_eq!(c.trace_events, 87 + 33 + 33 + 16 + 3 + 2);
    // Thresholds: 1, 4,096 and 65,536 bytes are reached by the corpus (the
    // 4,096-entry list is 114,688 bytes, `CombineOut` of a 128x128 image 65,536).
    for i in 0..3 {
        assert!(c.thr_injected[i] > 0, "threshold {} never injected a failure", THRESHOLDS[i]);
        assert_eq!(c.thr_reached[i], c.thr_injected[i]);
    }
    // 64 MiB: no ICNS request can reach it. The largest request is the
    // 4,096-entry list (4,096 * 28 = 114,688 bytes); the byte buffers are at most
    // 4 * 128 * 128 = 65,536 bytes. The schedule is a no-failure control here.
    if c.thr_injected[3] == 0 {
        assert!(c.max_request < THRESHOLDS[3]);
        println!("  UNREACHABLE: threshold {}; every ICNS request is at most {} bytes, so the schedule injects nothing and is a no-failure control",
            THRESHOLDS[3], c.max_request);
    }
    assert!(c.max_peak <= PEAK_BOUND);
}

#[test]
fn every_vector_survives_a_failure_at_each_reservation() {
    let _guard = lock();
    let mut c = SuiteCounts::default();
    for v in &vectors() {
        check_vector(v, &mut c);
    }
    assert_suite_exercised(&c);
}

// ---------------------------------------------------------------------------
// Targeted checks on hand-built inputs.
// ---------------------------------------------------------------------------

/// A minimal ICNS container: the 8-byte header plus one entry of `code` with a
/// payload of `payload`.
fn container(code: [u8; 4], payload: &[u8]) -> Vec<u8> {
    icns_file(&[(code, payload.to_vec())])
}

/// An ICNS container of the given `(type, payload)` entries.
fn icns_file(entries: &[([u8; 4], Vec<u8>)]) -> Vec<u8> {
    let total = 8 + entries.iter().map(|(_, p)| 8 + p.len()).sum::<usize>();
    let mut b = Vec::with_capacity(total);
    b.extend_from_slice(b"icns");
    b.extend_from_slice(&(total as u32).to_be_bytes());
    for (code, payload) in entries {
        b.extend_from_slice(code);
        b.extend_from_slice(&((8 + payload.len()) as u32).to_be_bytes());
        b.extend_from_slice(payload);
    }
    b
}

/// The colour type of a legacy side.
fn colour_code(side: usize) -> [u8; 4] {
    match side {
        16 => *b"is32",
        32 => *b"il32",
        48 => *b"ih32",
        128 => *b"it32",
        _ => panic!("no legacy colour type of side {side}"),
    }
}

/// The mask type of a legacy side.
fn mask_code(side: usize) -> [u8; 4] {
    match side {
        16 => *b"s8mk",
        32 => *b"l8mk",
        48 => *b"h8mk",
        128 => *b"t8mk",
        _ => panic!("no legacy mask type of side {side}"),
    }
}

/// An RLE colour payload for a `side` x `side` image: per channel, literal runs
/// of 128 bytes (control byte 127); `it32` carries its four-byte zero prefix.
fn colour_payload(side: usize) -> Vec<u8> {
    let n = side * side;
    let mut payload = Vec::new();
    if side == 128 {
        payload.extend_from_slice(&[0, 0, 0, 0]);
    }
    for ch in 0..3u8 {
        for _ in 0..n / 128 {
            payload.push(127u8);
            payload.extend(std::iter::repeat(0x11u8 + ch).take(128));
        }
    }
    payload
}

/// A colour entry of `colour` side and a mask entry of `mask` side.
fn colour_and_mask(colour: usize, mask: usize) -> Vec<u8> {
    icns_file(&[
        (colour_code(colour), colour_payload(colour)),
        (mask_code(mask), vec![0x7fu8; mask * mask]),
    ])
}

/// A 16x16 `is32` entry whose RLE payload is one literal run of 256 bytes per
/// channel: control byte 127 (128 literals) twice per channel.
fn rgb_is32_16() -> Vec<u8> {
    container(*b"is32", &colour_payload(16))
}

/// A 16x16 `s8mk` entry: 256 alpha bytes.
fn mask_s8mk_16() -> Vec<u8> {
    container(*b"s8mk", &vec![0x7fu8; 256])
}

#[test]
fn entry_layout_matches_the_declaration() {
    let _guard = lock();
    // `global layout Entry is size == 28, align == 4` (sites.rs, gate F2) is a
    // rustc-checked static assertion; this restates it where the test numbers
    // come from.
    #[cfg(target_arch = "x86_64")]
    {
        assert_eq!(std::mem::size_of::<Entry>(), 28);
        assert_eq!(std::mem::align_of::<Entry>(), 4);
        // 4,096 entries * 28 bytes = 114,688 <= 131,072: the entry list fits the
        // 128 KiB bound.
        assert!(4096 * std::mem::size_of::<Entry>() <= PEAK_BOUND);
    }
}

#[test]
fn entry_list_failure_is_exactly_out_of_memory() {
    let _guard = lock();
    let bytes = rgb_is32_16();
    let entries = try_parse(&bytes).expect("control parse");
    assert_eq!(entries.entries().len(), 1);

    // The entry list, `min(4096, (|b|-8)/8)` elements of `Entry`.
    let count = std::cmp::min(4096, (bytes.len() - 8) / 8);
    arm(1);
    let r = try_parse(&bytes);
    let st = disarm();
    let failed_size = st.failed_size;
    let failed_align = st.failed_align;
    match r {
        Err(TryError::OutOfMemory(o)) => {
            assert_eq!(o.site(), AllocSite::EntryList);
            assert_eq!(o.elements(), count);
            assert_eq!(failed_size, count * std::mem::size_of::<Entry>());
            assert_eq!(failed_align, std::mem::align_of::<Entry>());
        }
        Ok(_) => panic!("expected OutOfMemory, the parse succeeded"),
        Err(TryError::Format(e)) => panic!("expected OutOfMemory, got {}", e.name()),
    }
}

#[test]
fn channel_failure_names_the_channel_and_size() {
    let _guard = lock();
    let bytes = rgb_is32_16();
    // The handle is parsed outside every window, so the decoder's four requests
    // are events 1..4: 1..3 = the channels, 4 = RgbOut.
    let p = try_parse(&bytes).expect("control parse");
    for (k, channel) in [(1usize, 0u8), (2, 1), (3, 2)] {
        arm(k);
        let r = p.try_decode_rgb(0);
        disarm();
        match r {
            Err(TryError::OutOfMemory(o)) => {
                assert_eq!(o.site(), AllocSite::Channel(channel), "k={k}");
                assert_eq!(o.elements(), 256, "channel size is n = 16*16");
            }
            Ok(d) => panic!("k={k}: expected OutOfMemory, got a {}x{} decode", d.width, d.height),
            Err(TryError::Format(e)) => panic!("k={k}: expected OutOfMemory, got {}", e.name()),
        }
    }
    // The fourth reservation is the RGB output, `3n = 768` bytes.
    arm(4);
    let r = p.try_decode_rgb(0);
    disarm();
    match r {
        Err(TryError::OutOfMemory(o)) => {
            assert_eq!(o.site(), AllocSite::RgbOut);
            assert_eq!(o.elements(), 768);
        }
        Ok(d) => panic!("expected OutOfMemory, got a {}x{} decode", d.width, d.height),
        Err(TryError::Format(e)) => panic!("expected OutOfMemory, got {}", e.name()),
    }
}

#[test]
fn mask_copy_failure_names_the_site_and_size() {
    let _guard = lock();
    let bytes = mask_s8mk_16();
    let p = try_parse(&bytes).expect("control parse");
    // `try_decode_mask` alone makes one request: the `MaskData` copy.
    arm(1);
    let r = p.try_decode_mask(0);
    disarm();
    match r {
        Err(TryError::OutOfMemory(o)) => {
            assert_eq!(o.site(), AllocSite::MaskData);
            assert_eq!(o.elements(), 256);
        }
        Ok(d) => panic!("expected OutOfMemory, got a {}x{} decode", d.width, d.height),
        Err(TryError::Format(e)) => panic!("expected OutOfMemory, got {}", e.name()),
    }
}

#[test]
fn combine_output_failure_names_the_site_and_size() {
    let _guard = lock();
    // One `is32` colour entry and one `s8mk` mask entry, both 16x16.
    let bytes = colour_and_mask(16, 16);
    let p = try_parse(&bytes).expect("control parse");
    assert_eq!(p.entries().len(), 2);
    // `try_combine` makes six requests: 1..3 = the colour channels, 4 = RgbOut,
    // 5 = MaskData, 6 = CombineOut.
    arm(6);
    let r = p.try_combine(Some(0), Some(1));
    disarm();
    match r {
        Err(TryError::OutOfMemory(o)) => {
            assert_eq!(o.site(), AllocSite::CombineOut);
            assert_eq!(o.elements(), 4 * 256);
        }
        Ok(d) => panic!("expected OutOfMemory, got a {}x{} decode", d.width, d.height),
        Err(TryError::Format(e)) => panic!("expected OutOfMemory, got {}", e.name()),
    }
}

#[test]
fn prealloc_input_never_reaches_the_allocator() {
    let _guard = lock();
    // A bad magic: R3 fails before the entry-list reservation.
    let mut bytes = rgb_is32_16();
    bytes[0] = b'X';
    let control = run(&Vector { name: "bad-magic".into(), bytes: bytes.clone(), op: Op::Parse });
    assert!(matches!(control, Outcome::Format(Error::BadMagic)));

    arm(1);
    let r = try_parse(&bytes);
    let st = disarm();
    assert!(!st.countdown_fired, "a header error reserves nothing");
    assert_eq!(st.allocs, 0, "a header error reaches no allocator");
    match r {
        Err(TryError::Format(e)) => assert_eq!(e, Error::BadMagic),
        Err(TryError::OutOfMemory(_)) => panic!("OutOfMemory displaced a header error"),
        Ok(_) => panic!("a bad-magic input parsed"),
    }
}

#[test]
fn zero_element_request_is_a_logical_event_with_no_allocation() {
    let _guard = lock();
    // An eight-byte valid header: `try_with_room(EntryList, 0)`, a logical
    // request that returns `Ok` without touching the allocator, and not
    // `PreAlloc` (the reservation call is reached).
    let header = icns_file(&[]);
    assert_eq!(header.len(), 8);
    assert!(header_ok(&header) && entry_count(&header) == 0);
    // Twelve bytes: the same logical request, then `TruncatedEntryHeader`.
    let mut short = header.clone();
    short[4..8].copy_from_slice(&12u32.to_be_bytes());
    short.extend_from_slice(b"ABCD");
    assert!(header_ok(&short) && entry_count(&short) == 0);
    for (bytes, want) in [(header, None), (short, Some(Error::TruncatedEntryHeader))] {
        for arming in 0..2 {
            if arming == 0 { arm(1) } else { arm_threshold(1, 0) }
            let r = try_parse(&bytes);
            let st = disarm();
            assert_eq!((st.allocs, st.trace_len, st.countdown_fired, st.threshold_hit), (0, 0, false, 0));
            match (r, want) {
                (Ok(p), None) => {
                    assert!(p.entries().is_empty());
                    assert_eq!((st.allocs, st.trace_len), (0, 0),
                        "a zero-element request allocated");
                }
                (Err(TryError::Format(e)), Some(w)) => assert_eq!(e, w),
                (Err(TryError::Format(e)), None) =>
                    panic!("unexpected format error {}", e.name()),
                (Ok(_), Some(_)) => panic!("a parse that was to fail succeeded"),
                (Err(TryError::OutOfMemory(o)), _) =>
                    panic!("unexpected OutOfMemory at site {:?}, {} elements", o.site(), o.elements()),
            }
        }
    }
}

#[test]
fn a_successful_output_is_dropped_without_leaking() {
    let _guard = lock();
    let bytes = rgb_is32_16();
    let out_size = 3 * 256;
    set_watch(out_size);
    let (watch_before, live_before) = peek_live();
    assert_eq!(watch_before, 0);
    {
        let p = try_parse(&bytes).expect("control parse");
        let d = match p.try_decode_rgb(0) {
            Ok(d) => d,
            Err(e) => panic!("control decode failed: {}", e.name()),
        };
        assert_eq!(d.data.len(), out_size);
        let (watch_live, live_now) = peek_live();
        assert_eq!(watch_live, 1, "the live 3n output is not counted");
        assert!(live_now > live_before);
    }
    let (watch_after, _) = peek_live();
    assert_eq!(watch_after, 0, "dropping the output does not free its 3n buffer");
    set_watch(0);
}

/// The mandated schedule on the 768-byte RGB output: only the one-byte threshold
/// is at or below it; the others leave it alone. The boundary values 256, 768 and
/// 769 pin the comparison itself (`size >= T`).
#[test]
fn threshold_failure_below_the_request_size() {
    let _guard = lock();
    let bytes = rgb_is32_16();
    let mut schedule: Vec<usize> = THRESHOLDS.to_vec();
    schedule.extend([256, 768, 769]);
    let p = try_parse(&bytes).expect("control parse");
    for t in schedule {
        let expect_fail = 768 >= t;
        arm_threshold(t, 768);
        let r = p.try_decode_rgb(0);
        let hit = disarm().threshold_hit;
        if expect_fail {
            assert!(hit > 0, "threshold {t} did not fail the 768-byte request");
            match r {
                Err(TryError::OutOfMemory(o)) => {
                    assert_eq!(o.site(), AllocSite::RgbOut);
                    assert_eq!(o.elements(), 768);
                }
                Ok(d) => panic!(
                    "threshold {t}: expected OutOfMemory, got a {}x{} decode",
                    d.width, d.height,
                ),
                Err(TryError::Format(e)) => panic!(
                    "threshold {t}: expected OutOfMemory, got {}", e.name(),
                ),
            }
        } else {
            assert_eq!(hit, 0, "threshold {t} failed a smaller request");
            assert!(r.is_ok(), "threshold {t}: the 768-byte request should succeed");
        }
    }
}

// ---------------------------------------------------------------------------
// Peak live bytes of `try_combine` (THEOREMS-ALLOC-DELTA.md §5; SPEC.md §4).
// ---------------------------------------------------------------------------

/// One `try_combine(Some(0), Some(1))` call through the handle under `arm(k)`
/// (`None` = observe), returning the result and the allocator's statistics
/// (peak included). The handle is parsed before the window is armed, so the
/// reservation events the window sees are the decoder's own (event 1 is the
/// first colour channel; no `EntryList` event), the entry list is live at the
/// window's baseline, and the numbers are the decoder's own (SPEC.md §4 assigns
/// the list to `try_parse`).
fn combine_probe(bytes: &[u8], k: Option<usize>) -> (Result<Decoded, TryError>, Stats) {
    let p = try_parse(bytes).expect("control parse");
    match k {
        Some(k) => arm(k),
        None => arm_observe(),
    }
    let r = p.try_combine(Some(0), Some(1));
    let st = disarm();
    (r, st)
}

/// Peak live bytes of `try_combine`, per phase, for a colour entry of side `sc`
/// and a mask entry of side `sm` (`n_c = sc^2`, `n_m = sm^2`):
///
/// - during the colour decode, the three channels and the RGB output: `6 n_c`;
/// - from the RGB output's return until the side check, the RGB output and the
///   mask copy: `3 n_c + n_m`;
/// - after a matching check, with the combine output: `8 n`;
/// - mismatched sides: `TypeMismatch` with both buffers live, so the call's peak
///   is `max(6 n_c, 3 n_c + n_m)`, e.g. 16x16 colour with 128x128 mask:
///   `768 + 16,384 = 17,152` bytes;
/// - at most 131,072 bytes (128 KiB) in every case.
///
/// The handle is parsed before the window is armed (`combine_probe`) and is live
/// at the window's baseline, so these remain the decoder's own numbers, exactly
/// as SPEC.md §4 states them (the entry list belongs to `try_parse`). These are
/// an ownership fact about the decoder, measured here on the pinned std and
/// allocator; they are not a verified clause. The request sizes are exact
/// (`try_reserve_exact`), so each measured peak equals its bound; the mandated
/// check is that it does not exceed it.
#[test]
fn combine_peak_live_bytes_follow_the_stated_phases() {
    let _guard = lock();
    let mut checked = 0;
    for (sc, sm) in [
        (16, 16), (32, 32), (48, 48), (128, 128), // matching sides
        (16, 128), (128, 16), (16, 32), (32, 16), (48, 128), (128, 48), // mismatched
    ] {
        let (n_c, n_m) = (sc * sc, sm * sm);
        let bytes = colour_and_mask(sc, sm);
        assert_eq!(try_parse(&bytes).expect("control parse").entries().len(), 2,
            "{sc}x{sc} colour + {sm}x{sm} mask");
        // The handle is parsed outside the window, so the decoder's first
        // reservation is event 1, exactly as in the two-argument API.
        let shift = 0usize;

        // Phase 1, colour decode: the three channels (3 n_c), then the RGB output
        // makes it 6 n_c. Failing the RGB output (event 4) shows the channels
        // alone; failing the mask copy (event 5) shows the whole phase.
        let (r, st) = combine_probe(&bytes, Some(4 + shift));
        assert!(matches!(r, Err(TryError::OutOfMemory(o)) if o.site() == AllocSite::RgbOut));
        assert_eq!(st.peak as usize, 3 * n_c, "three channels before RgbOut ({sc},{sm})");
        assert_eq!(st.live_at_fail as usize, 3 * n_c);
        let (r, st) = combine_probe(&bytes, Some(5 + shift));
        assert!(matches!(r, Err(TryError::OutOfMemory(o)) if o.site() == AllocSite::MaskData));
        assert!(st.peak as usize <= 6 * n_c, "colour-decode peak {} > 6 n_c ({sc},{sm})", st.peak);
        assert_eq!(st.peak as usize, 6 * n_c, "channels + RGB output ({sc},{sm})");
        // After the colour decode only the RGB output (3 n_c) is live.
        assert_eq!(st.live_at_fail as usize, 3 * n_c, "RGB output alone before the mask copy");

        if sc == sm {
            // Phase 2: the mask copy is live beside the RGB output until the
            // side check: 3 n_c + n_m. Failing the combine output (event 6)
            // observes that level at the moment of the request.
            let (r, st) = combine_probe(&bytes, Some(6 + shift));
            assert!(matches!(r, Err(TryError::OutOfMemory(o)) if o.site() == AllocSite::CombineOut));
            assert_eq!(st.live_at_fail as usize, 3 * n_c + n_m, "RGB output + mask copy");
            assert!(st.peak as usize <= std::cmp::max(6 * n_c, 3 * n_c + n_m));
            assert_eq!(st.peak as usize, 6 * n_c);
            // Phase 3: after the matching check the combine output (4 n) joins:
            // 8 n.
            let (r, st) = combine_probe(&bytes, None);
            let d = r.expect("matching sides combine");
            assert_eq!(d.data.len(), 4 * n_c);
            assert!(st.peak as usize <= 8 * n_c, "peak {} > 8 n ({sc},{sm})", st.peak);
            assert_eq!(st.peak as usize, 8 * n_c);
            assert!(st.peak as usize <= PEAK_BOUND);
            assert_eq!(st.live as usize, d.data.capacity(), "only the output stays live");
            if sc == 128 {
                assert_eq!(st.peak as usize, PEAK_BOUND, "128x128 reaches the 128 KiB bound exactly");
            }
        } else {
            // Mismatched sides: `TypeMismatch` with both buffers live.
            let bound = std::cmp::max(6 * n_c, 3 * n_c + n_m);
            let (r, st) = combine_probe(&bytes, None);
            assert!(matches!(r, Err(TryError::Format(Error::TypeMismatch))), "mismatched sides");
            assert!(st.peak as usize <= bound, "mismatched peak {} > {bound} ({sc},{sm})", st.peak);
            assert_eq!(st.peak as usize, bound);
            assert!(st.peak as usize <= PEAK_BOUND);
            assert_eq!(st.live, 0, "TypeMismatch leaked a buffer");
            assert_eq!(st.reallocs, 0);
            // No combine output is requested: five events, not six.
            assert_eq!(st.trace_len, 5, "mismatched sides stop before CombineOut");
            if (sc, sm) == (16, 128) {
                assert_eq!(st.peak as usize, 768 + 16_384);
                assert_eq!(st.peak as usize, 17_152);
            }
        }

        // The generated inputs also run through the full per-vector checks:
        // every failure point, the trace and the threshold schedules.
        let mut c = SuiteCounts::default();
        check_vector(&make_vector("generated-combine", bytes.clone(), Op::Combine(Some(0), Some(1))), &mut c);
        if sc == 128 && sm == 128 {
            // 128x128: CombineOut is 65,536 bytes, so every schedule up to
            // 65,536 is reached and injected.
            assert!(c.thr_injected[..3].iter().all(|&n| n == 1), "128x128 thresholds");
            assert_eq!(c.thr_injected[3], 0, "no request reaches 64 MiB");
        }
        checked += 1;
    }
    assert_eq!(checked, 10);
}

#[test]
fn frozen_vectors_respect_the_peak_bounds() {
    let _guard = lock();
    let mut max_decode = 0usize;
    let mut max_parse = 0usize;
    for v in vectors() {
        arm_observe();
        let r = run(&v);
        let st = disarm();
        let peak = st.peak as usize;
        assert!(peak <= PEAK_BOUND, "{}: peak {peak} above 128 KiB", v.name);
        match v.op {
            Op::Parse => {
                // The entry list is the only buffer: min(4096, (|b|-8)/8) * 28.
                if header_ok(&v.bytes) {
                    assert!(peak <= entry_count(&v.bytes) * std::mem::size_of::<Entry>(), "{}", v.name);
                }
                max_parse = max_parse.max(peak);
            }
            _ => max_decode = max_decode.max(peak),
        }
        drop(r);
    }
    // 4,096 entries * 28 bytes = 114,688 <= 131,072 (the entry list's bound).
    assert!(max_parse <= 4096 * std::mem::size_of::<Entry>());
    assert!(max_parse <= PEAK_BOUND);
    assert!(max_decode <= PEAK_BOUND);
    println!("frozen vectors: largest parse peak {max_parse} bytes, largest decode peak {max_decode} bytes");
}


