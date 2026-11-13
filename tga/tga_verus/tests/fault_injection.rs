//! Fault-injection tests for the alloc-delta guard (THEOREMS-ALLOC-DELTA.md §5;
//! A-TGA-1 clause 2, A-TGA-2). These are falsification checks, not proofs: they
//! exercise the guard's binding of `Err` to `try_reserve_exact`'s `Err` arm at
//! run time on the pinned toolchain and allocator.
//!
//! The mechanism is a test-only, unverified `#[global_allocator]` that wraps
//! `System`, arms on a countdown (or on an allocation-size threshold), and
//! returns null on the selected non-zero `alloc`/`alloc_zeroed`/`realloc` of a
//! single decoder call.
//!
//! **Serialisation.** The counters, the injector and the live-byte total are
//! process-global, and Rust runs test functions concurrently by default, so
//! every test here takes `SUITE_LOCK` for its whole body. A test that does not
//! hold the lock never arms the injector, so a parallel test's allocations
//! cannot be faulted. Do not add a test to this file that skips the lock.
//!
//! TGA has exactly one reservation site (`try_zeroed` on `AllocSite::Output`,
//! `4N` bytes), so a *later*-reservation case does not exist: `k = 1` is the
//! reservation and `k >= 2` must never be reached. The other `SPEC.md` §5 modes
//! apply: the size-threshold mode (fail the request of at least `T` bytes, with
//! the window confined to the decoder's own `4N` request so the harness's
//! allocations are never faulted), the no-`realloc` check, the
//! physical-event-versus-ghost-request layout check, the `PreAlloc` (no-event)
//! check, the drop-without-leak check, and a sweep of every frozen vector plus a
//! corpus sample under an injected reservation failure.
//!
//! This is a separate std test crate, outside `tga_verus`'s verified core and
//! its `no_std` boundary, so `unsafe impl GlobalAlloc` here is not a proof
//! escape in the core; it is the standard way to intercept the process
//! allocator.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard};

use tga_verus::alloc_guard::AllocSite;
use tga_verus::policy::Profile;
use tga_verus::tga::{try_decode_with_policy, Decoded, Error, TryError};

/// Serialises every test in this file. See the module docs.
static SUITE_LOCK: Mutex<()> = Mutex::new(());

/// `0` disarms the countdown; `k > 0` makes the k-th non-zero `alloc` of the
/// armed window return null, after which the countdown disarms itself.
static COUNTDOWN: AtomicIsize = AtomicIsize::new(0);
/// Non-zero `alloc`/`alloc_zeroed`/`realloc` calls since the last arm.
static ALLOCS: AtomicUsize = AtomicUsize::new(0);
/// `realloc` calls since the last arm (A-STD-2: must stay zero in a decode).
static REALLOCS: AtomicUsize = AtomicUsize::new(0);
/// `0` disables the threshold; `T > 0` fails every request of `>= T` bytes that
/// also matches `THRESHOLD_WINDOW` (see below).
static THRESHOLD: AtomicUsize = AtomicUsize::new(0);
/// When non-zero, the threshold mode fails only requests of exactly this size.
/// The harness runs its own tests (and its own bookkeeping allocations) in the
/// same process, so a pure size threshold can catch one and abort the process;
/// arming the window to the decoder's known reservation size (`4N`) confines a
/// threshold failure to that reservation. `0` means a pure threshold.
static THRESHOLD_WINDOW: AtomicUsize = AtomicUsize::new(0);
/// Live bytes: a successful `alloc`/`realloc` adds, `dealloc` subtracts. A
/// failed request adds nothing. Process-global, so it also counts the harness's
/// own allocations; the leak check uses the size-window counter below instead.
static LIVE: AtomicIsize = AtomicIsize::new(0);
/// The size the leak check watches: `0` disables it. The decode's output is the
/// only allocation of exactly this size during the measured window, so counting
/// blocks of this size is immune to harness noise.
static WATCH_SIZE: AtomicUsize = AtomicUsize::new(0);
/// Live blocks of exactly `WATCH_SIZE` bytes.
static WATCH_LIVE: AtomicIsize = AtomicIsize::new(0);
/// Size and alignment of the last non-zero allocation that reached `System`
/// while the injector was armed (the physical event).
static LAST_SIZE: AtomicUsize = AtomicUsize::new(0);
static LAST_ALIGN: AtomicUsize = AtomicUsize::new(0);
/// Set once a threshold-mode request has been failed, to tell "no request of
/// this size occurred" apart from "one did and was failed".
static THRESHOLD_HIT: AtomicUsize = AtomicUsize::new(0);
/// Size and alignment of the last request the injector failed (a failed request
/// never reaches `System`, so `LAST_*` cannot show it).
static FAILED_SIZE: AtomicUsize = AtomicUsize::new(0);
static FAILED_ALIGN: AtomicUsize = AtomicUsize::new(0);
/// Alignment of the last successful block of exactly `WATCH_SIZE` bytes. Unlike
/// `LAST_ALIGN`, harness allocations of other sizes cannot overwrite it.
static WATCH_ALIGN: AtomicUsize = AtomicUsize::new(0);

/// Alignment of the request currently being served, read by `watch`.
static PENDING_ALIGN: AtomicUsize = AtomicUsize::new(0);

fn lock() -> MutexGuard<'static, ()> {
    SUITE_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

struct FaultAlloc;

/// Track a block of `size` bytes entering or leaving the live set.
fn watch(size: usize, delta: isize) {
    if size != 0 && size == WATCH_SIZE.load(Ordering::SeqCst) {
        if delta > 0 {
            // Callers pass the layout alignment through `WATCH_ALIGN_PENDING`.
            WATCH_ALIGN.store(PENDING_ALIGN.load(Ordering::SeqCst), Ordering::SeqCst);
        }
        WATCH_LIVE.fetch_add(delta, Ordering::SeqCst);
    }
}

/// Whether this request should be failed, updating `COUNTDOWN`/`THRESHOLD_HIT`.
fn should_fail(size: usize, align: usize) -> bool {
    let n = ALLOCS.fetch_add(1, Ordering::SeqCst) + 1;
    let k = COUNTDOWN.load(Ordering::SeqCst);
    if k > 0 && n as isize == k {
        COUNTDOWN.store(0, Ordering::SeqCst);
        FAILED_SIZE.store(size, Ordering::SeqCst);
        FAILED_ALIGN.store(align, Ordering::SeqCst);
        return true;
    }
    let t = THRESHOLD.load(Ordering::SeqCst);
    if t > 0 && size >= t {
        let w = THRESHOLD_WINDOW.load(Ordering::SeqCst);
        if w == 0 || size == w {
            THRESHOLD_HIT.fetch_add(1, Ordering::SeqCst);
            FAILED_SIZE.store(size, Ordering::SeqCst);
            FAILED_ALIGN.store(align, Ordering::SeqCst);
            return true;
        }
    }
    false
}

unsafe impl GlobalAlloc for FaultAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() == 0 {
            return System.alloc(layout);
        }
        if should_fail(layout.size(), layout.align()) {
            return core::ptr::null_mut();
        }
        LAST_SIZE.store(layout.size(), Ordering::SeqCst);
        LAST_ALIGN.store(layout.align(), Ordering::SeqCst);
        PENDING_ALIGN.store(layout.align(), Ordering::SeqCst);
        let p = System.alloc(layout);
        if !p.is_null() {
            LIVE.fetch_add(layout.size() as isize, Ordering::SeqCst);
            watch(layout.size(), 1);
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
        LAST_SIZE.store(layout.size(), Ordering::SeqCst);
        LAST_ALIGN.store(layout.align(), Ordering::SeqCst);
        PENDING_ALIGN.store(layout.align(), Ordering::SeqCst);
        let p = System.alloc_zeroed(layout);
        if !p.is_null() {
            LIVE.fetch_add(layout.size() as isize, Ordering::SeqCst);
            watch(layout.size(), 1);
        }
        p
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        REALLOCS.fetch_add(1, Ordering::SeqCst);
        if should_fail(new_size, layout.align()) {
            return core::ptr::null_mut();
        }
        let p = System.realloc(ptr, layout, new_size);
        if !p.is_null() {
            LIVE.fetch_add(new_size as isize - layout.size() as isize, Ordering::SeqCst);
            watch(layout.size(), -1);
            PENDING_ALIGN.store(layout.align(), Ordering::SeqCst);
            watch(new_size, 1);
        }
        p
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if layout.size() != 0 {
            LIVE.fetch_sub(layout.size() as isize, Ordering::SeqCst);
            watch(layout.size(), -1);
        }
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static ALLOCATOR: FaultAlloc = FaultAlloc;

/// Arm the countdown to fail the `k`-th non-zero request; reset the counters.
fn arm(k: usize) {
    ALLOCS.store(0, Ordering::SeqCst);
    REALLOCS.store(0, Ordering::SeqCst);
    THRESHOLD.store(0, Ordering::SeqCst);
    THRESHOLD_WINDOW.store(0, Ordering::SeqCst);
    THRESHOLD_HIT.store(0, Ordering::SeqCst);
    COUNTDOWN.store(k as isize, Ordering::SeqCst);
}

/// Arm the size-threshold mode for requests of `>= t` bytes, confined to
/// requests of exactly `window` bytes (the decoder's reservation size).
fn arm_threshold(t: usize, window: usize) {
    ALLOCS.store(0, Ordering::SeqCst);
    REALLOCS.store(0, Ordering::SeqCst);
    COUNTDOWN.store(0, Ordering::SeqCst);
    THRESHOLD_HIT.store(0, Ordering::SeqCst);
    THRESHOLD_WINDOW.store(window, Ordering::SeqCst);
    THRESHOLD.store(t, Ordering::SeqCst);
}

fn disarm() {
    COUNTDOWN.store(0, Ordering::SeqCst);
    THRESHOLD.store(0, Ordering::SeqCst);
    THRESHOLD_WINDOW.store(0, Ordering::SeqCst);
}

/// A one-pixel uncompressed true-colour TGA: 18-byte header (`image_type = 2`,
/// width = height = 1, `pixel_depth = 24`, top-down descriptor) plus one BGR
/// pixel. `N = 1`, so the reservation is exactly 4 bytes.
fn one_pixel_tga() -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&[
        0, // id_length
        0, // color_map_flag
        2, // image_type: uncompressed true-colour
        0, 0, 0, 0, 0, // map_first, map_length, map_entry_depth
        0, 0, // x_origin
        0, 0, // y_origin
        1, 0, // width = 1
        1, 0, // height = 1
        24,   // pixel_depth
        0x20, // descriptor: top-left origin
    ]);
    b.extend_from_slice(&[1, 2, 3]); // one BGR pixel
    b
}

/// A `side`x`side` uncompressed true-colour TGA of one constant BGR pixel, so
/// `N = side^2` and the reservation is `4*N` bytes (for the threshold mode).
fn square_tga(side: u16) -> Vec<u8> {
    let mut b = vec![
        0u8, // id_length
        0,   // color_map_flag
        2,   // image_type: uncompressed true-colour
        0, 0, 0, 0, 0, // map_first, map_length, map_entry_depth
        0, 0, // x_origin
        0, 0, // y_origin
    ];
    b.extend_from_slice(&side.to_le_bytes());
    b.extend_from_slice(&side.to_le_bytes());
    b.push(24); // pixel_depth
    b.push(0x20); // descriptor: top-left origin
    let n = side as usize * side as usize;
    for _ in 0..n {
        b.extend_from_slice(&[9, 8, 7]); // one BGR pixel per sample
    }
    b
}

/// A header error: a 17-byte input is `TruncatedHeader`, and `MH` rejects it
/// before any reservation (`PreAlloc`), so no allocator call is made.
fn truncated_header() -> Vec<u8> {
    vec![0u8; 17]
}

fn decode_strict(bytes: &[u8]) -> (Result<Decoded, TryError>, bool) {
    try_decode_with_policy(bytes, Profile::Strict)
}

fn expect_oom(r: Result<Decoded, TryError>, flag: bool, elements: usize) {
    match r {
        Err(TryError::OutOfMemory(o)) => {
            assert_eq!(o.site(), AllocSite::Output);
            assert_eq!(o.elements(), elements, "the failed request is exactly 4N");
        }
        Ok(d) => panic!("fault injection did not fail: decoded {} bytes", d.rgba.len()),
        Err(TryError::Format(e)) => panic!("fault injection produced format error {}", e.name()),
    }
    assert!(!flag, "an OutOfMemory result reports no recovery");
}

/// The 169 frozen vectors, as `(name, bytes)`.
fn conformance_vectors() -> Vec<(String, Vec<u8>)> {
    use std::path::PathBuf;
    use std::sync::OnceLock;
    use serde_json::Value;
    let path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate has a parent")
        .join("spec-frozen/VECTORS.json");
    // Cache the whole set: two of the vectors expand to tens of megabytes
    // (`input-at-64m`, `input-over-64m`), so re-reading them per test is the
    // dominant cost. The cache is immutable data, safe to share across the
    // serialised tests.
    static CACHE: OnceLock<Vec<(String, Vec<u8>)>> = OnceLock::new();
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
                let unit: Vec<u8> = hex
                    .as_bytes()
                    .chunks_exact(2)
                    .map(|p| {
                        let d = |c: u8| match c {
                            b'0'..=b'9' => c - b'0',
                            b'a'..=b'f' => c - b'a' + 10,
                            b'A'..=b'F' => c - b'A' + 10,
                            _ => panic!("non-hex byte"),
                        };
                        16 * d(p[0]) + d(p[1])
                    })
                    .collect();
                let mut bytes = Vec::with_capacity(unit.len() * repeat);
                for _ in 0..repeat {
                    bytes.extend_from_slice(&unit);
                }
                out.push((name, bytes));
            }
            out
        })
        .clone()
}

/// Every vector with the reservation failed must be exactly `OutOfMemory`,
/// unless its header is rejected (`PreAlloc`), in which case the format error
/// must not have touched the allocator.
fn run_vectors_under_fault() -> (usize, usize) {
    let vectors = conformance_vectors();
    let mut oom = 0usize;
    let mut prealloc = 0usize;
    for (name, bytes) in &vectors {
        arm(1);
        let (r, flag) = decode_strict(bytes);
        let reached = COUNTDOWN.load(Ordering::SeqCst) == 0;
        let n_alloc = ALLOCS.load(Ordering::SeqCst);
        let reallocs = REALLOCS.load(Ordering::SeqCst);
        disarm();
        assert_eq!(reallocs, 0, "{name}: a decode issued a realloc (A-STD-2)");
        match r {
            Err(TryError::OutOfMemory(o)) => {
                assert!(reached, "{name}: OutOfMemory without a null-returning event");
                assert_eq!(o.site(), AllocSite::Output, "{name}: wrong site");
                assert!(!flag, "{name}: OutOfMemory reports no recovery");
                assert_eq!(n_alloc, 1, "{name}: OutOfMemory needs exactly one request");
                oom += 1;
            }
            Err(TryError::Format(_)) => {
                // PreAlloc: the header was rejected before the reservation, so
                // the injector stayed armed and no allocation occurred.
                assert!(!reached, "{name}: a format error consumed the reservation");
                assert_eq!(n_alloc, 0, "{name}: a PreAlloc error reached the allocator");
                assert!(!flag, "{name}: a format error reports no recovery");
                prealloc += 1;
            }
            Ok(_) => panic!("{name}: decoded under fault injection"),
        }
    }
    (oom, prealloc)
}

/// Sample corpus inputs (read in place, never copied into the repo) and inject
/// the reservation failure in each.
fn run_corpus_sample_under_fault(limit: usize) -> usize {
    corpus_sample(limit).into_iter().map(|(name, bytes)| corpus_one(&name, &bytes)).sum()
}

/// Read up to `limit` corpus inputs in place (never copied into the repo).
fn corpus_sample(limit: usize) -> Vec<(String, Vec<u8>)> {
    use std::path::PathBuf;
    let manifest = PathBuf::from("/home/ws/corpora/tga/MANIFEST.tsv");
    let text = match std::fs::read_to_string(&manifest) {
        Ok(t) => t,
        Err(_) => return Vec::new(), // corpus absent in this sandbox: nothing to check
    };
    let base = manifest.parent().expect("manifest parent");
    let mut out = Vec::new();
    for line in text.lines().skip(1).filter(|l| !l.is_empty()) {
        if out.len() >= limit {
            break;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() != 9 {
            continue;
        }
        if let Ok(b) = std::fs::read(base.join(PathBuf::from(cols[0]))) {
            out.push((cols[0].to_string(), b));
        }
    }
    out
}

/// One corpus input with the reservation failed; returns 1 (the input counted).
fn corpus_one(name: &str, bytes: &[u8]) -> usize {
    arm(1);
    let (r, flag) = decode_strict(bytes);
    let reached = COUNTDOWN.load(Ordering::SeqCst) == 0;
    let reallocs = REALLOCS.load(Ordering::SeqCst);
    disarm();
    assert_eq!(reallocs, 0, "{name}: realloc during decode");
    match r {
        Err(TryError::OutOfMemory(o)) => {
            assert!(reached, "{name}: OutOfMemory without a null-returning event");
            assert_eq!(o.site(), AllocSite::Output);
            assert!(!flag);
        }
        Err(TryError::Format(_)) => {
            assert!(!reached, "{name}: format error consumed the reservation");
        }
        Ok(_) => panic!("{name}: decoded under fault injection"),
    }
    1
}

#[test]
fn fault_at_the_reservation_is_exactly_out_of_memory() {
    let _guard = lock();
    let bytes = one_pixel_tga();

    // Control: the input decodes without fault injection.
    let (ok, flag) = decode_strict(&bytes);
    assert!(ok.is_ok(), "control decode failed: {:?}", ok.err().map(|e| e.name()));
    let control = ok.unwrap();
    assert!(!flag);
    assert_eq!(control.rgba.len(), 4, "N = 1 gives a 4-byte output");

    // Fault the first non-zero reservation: it must be the one `(Output, 4N)`.
    WATCH_SIZE.store(4, Ordering::SeqCst);
    WATCH_LIVE.store(0, Ordering::SeqCst);
    arm(1);
    let (r, flag) = decode_strict(&bytes);
    let reallocs = REALLOCS.load(Ordering::SeqCst);
    let live_4 = WATCH_LIVE.load(Ordering::SeqCst);
    disarm();
    expect_oom(r, flag, 4);
    assert_eq!(ALLOCS.load(Ordering::SeqCst), 1, "exactly one reservation, so k = 1 is it");
    assert_eq!(reallocs, 0, "no realloc during a decode (A-STD-2)");
    assert_eq!(live_4, 0, "the failed reservation left no 4-byte block live");
    WATCH_SIZE.store(0, Ordering::SeqCst);
}

#[test]
fn physical_event_matches_the_ghost_request() {
    let _guard = lock();
    // N = 4 (2x2), so the reservation is 16 bytes of u8, alignment 1.
    let bytes = square_tga(2);
    let n = 4usize;

    // Read the physical layout of the successful reservation. The site's element
    // type is `u8`, so the expected alignment is 1 and the size is at least `4N`
    // (equal on the pinned std). A failed request reaches no `System::alloc`, so
    // read the event on the success path.
    LAST_SIZE.store(0, Ordering::SeqCst);
    LAST_ALIGN.store(0, Ordering::SeqCst);
    let (ok, _) = decode_strict(&bytes);
    let size = LAST_SIZE.load(Ordering::SeqCst);
    let align = LAST_ALIGN.load(Ordering::SeqCst);
    disarm();
    assert!(ok.is_ok(), "control decode failed");
    assert_eq!(align, 1, "the output site is u8, so the request alignment is 1");
    assert!(
        size >= 4 * n,
        "physical request {size} < 4N = {}: the ghost request is larger than the event",
        4 * n,
    );
    assert_eq!(size, 4 * n, "on the pinned std the request is exactly 4N");

    // The same input under fault injection: the ghost request names 4N, and the
    // failed call makes no further request.
    arm(1);
    let (r, flag) = decode_strict(&bytes);
    disarm();
    expect_oom(r, flag, 4 * n);
    assert_eq!(ALLOCS.load(Ordering::SeqCst), 1, "one request for the whole decode");
}

#[test]
fn a_successful_output_is_dropped_without_leaking() {
    let _guard = lock();
    let bytes = square_tga(8); // N = 64, output 256 bytes
    let out_size = 4 * 64;
    // Watch only 256-byte blocks: the decode's output is the only allocation of
    // exactly that size during this window, so harness allocations cannot move
    // the counter.
    WATCH_SIZE.store(out_size, Ordering::SeqCst);
    WATCH_LIVE.store(0, Ordering::SeqCst);
    {
        let (r, flag) = decode_strict(&bytes);
        let d = match r {
            Ok(d) => d,
            Err(e) => panic!("control decode failed: {}", e.name()),
        };
        assert!(!flag);
        assert_eq!(d.rgba.len(), out_size);
        // While `d` is alive exactly one 256-byte block is live.
        assert_eq!(
            WATCH_LIVE.load(Ordering::SeqCst),
            1,
            "the live 4N output is not counted",
        );
    }
    assert_eq!(
        WATCH_LIVE.load(Ordering::SeqCst),
        0,
        "dropping the decoded output does not free its 4N buffer",
    );
    // The process-wide `LIVE` total is not compared here: it also counts allocations
    // made by libtest's own threads, so it drifts by a few hundred bytes at random.
    // The exact-size watch above is the leak check.
    WATCH_SIZE.store(0, Ordering::SeqCst);
}

#[test]
fn no_reservation_after_the_first() {
    let _guard = lock();
    let bytes = one_pixel_tga();
    let (control, control_flag) = decode_strict(&bytes);
    assert!(control.is_ok());

    // k = 2 must never be reached: the call makes exactly one reservation.
    arm(2);
    let (r, flag) = decode_strict(&bytes);
    let still_armed = COUNTDOWN.load(Ordering::SeqCst);
    disarm();
    assert_eq!(still_armed, 2, "the injector was never triggered");
    assert_eq!(ALLOCS.load(Ordering::SeqCst), 1, "exactly one allocation");
    assert_eq!(flag, control_flag);
    match (r, control) {
        (Ok(a), Ok(b)) => {
            assert_eq!(a.width, b.width);
            assert_eq!(a.height, b.height);
            assert_eq!(a.rgba, b.rgba);
        }
        (Err(TryError::Format(a)), Err(TryError::Format(b))) => assert_eq!(a, b),
        other => panic!("results differ from the control: {:?}", other),
    }
}

#[test]
fn threshold_failure_below_the_request_size() {
    let _guard = lock();
    // N = 128*128 = 16384, so the reservation is 4N = 65,536 bytes.
    let bytes = square_tga(128);
    let n = 128usize * 128 * 4;

    // Control: a 65,536-byte request succeeds when no threshold is armed.
    let (control, _) = decode_strict(&bytes);
    assert!(control.is_ok(), "control decode failed");

    // Thresholds at or below 4N fail the reservation; a threshold above it
    // leaves it alone. The window confines each mode to a 65,536-byte request,
    // so the harness's own allocations cannot be faulted.
    for (t, expect_fail) in [(1usize, true), (4096, true), (65536, true), (65537, false)] {
        arm_threshold(t, n);
        let (r, flag) = decode_strict(&bytes);
        let hit = THRESHOLD_HIT.load(Ordering::SeqCst);
        disarm();
        if expect_fail {
            assert!(hit > 0, "threshold {t} did not fail the {n}-byte request");
            expect_oom(r, flag, n);
        } else {
            assert_eq!(hit, 0, "threshold {t} failed a request smaller than it");
            assert!(r.is_ok(), "threshold {t}: the {n}-byte request should succeed");
        }
    }
}

#[test]
fn prealloc_input_never_reaches_the_allocator() {
    let _guard = lock();
    let bytes = truncated_header();

    // Control error, no fault.
    let (control, flag) = decode_strict(&bytes);
    assert!(!flag);
    let control_err = match control {
        Err(TryError::Format(e)) => e,
        other => panic!("expected a format error, got {:?}", other.map(|d| d.width)),
    };

    // Arm the injector and confirm it is never triggered and the error is the
    // same: `PreAlloc` means no reservation, so no `OutOfMemory` can appear.
    arm(1);
    let (r, flag) = decode_strict(&bytes);
    let still_armed = COUNTDOWN.load(Ordering::SeqCst);
    let n_alloc = ALLOCS.load(Ordering::SeqCst);
    disarm();
    assert_eq!(still_armed, 1, "a header error reserves nothing");
    assert_eq!(n_alloc, 0, "a header error reaches no allocator");
    assert!(!flag);
    match r {
        Err(TryError::Format(e)) => assert_eq!(e, control_err),
        Err(TryError::OutOfMemory(_)) => panic!("OutOfMemory displaced a header error"),
        Ok(_) => panic!("a truncated header decoded"),
    }
    let _: Error = control_err;
}

#[test]
fn every_conformance_vector_survives_a_failed_reservation() {
    let _guard = lock();
    let (oom, prealloc) = run_vectors_under_fault();
    println!("fault injection over 169 vectors: {oom} OutOfMemory, {prealloc} PreAlloc");
    assert_eq!(oom + prealloc, 169, "every vector is one or the other");
}

#[test]
fn corpus_sample_survives_a_failed_reservation() {
    let _guard = lock();
    let checked = run_corpus_sample_under_fault(12);
    if checked == 0 {
        println!("corpus absent; corpus fault injection skipped");
    } else {
        println!("fault injection over {checked} corpus inputs: all OutOfMemory or PreAlloc");
    }
}

/// Schedules exercised by the suite test: allocation 1, allocation 2 (never
/// fires: the decoder makes one reservation), and size thresholds.
const SCHEDULES: [&str; 6] = ["nth=1", "nth=2", "thr=1", "thr=4096", "thr=65536", "thr=67108864"];
const MIB64: usize = 67_108_864;

#[derive(Default)]
struct SuiteCounts {
    inputs: usize,
    allocating: usize,
    max_request: usize,
    /// Cases where the schedule's trigger was reachable (a reservation of at
    /// least the threshold; any reservation for the nth schedules).
    reached: [usize; 6],
    /// Cases where the schedule injected a failure.
    injected: [usize; 6],
}

/// Run every schedule against one input. The request size `S` (the single
/// reservation, 4N) is learned from the `nth=1` failure; threshold schedules
/// are windowed to exactly `S` so only the decoder's reservation is faulted.
/// Asserts alignment == 1 (failed and, where the decode succeeds, physical) and
/// physical size >= 4*width*height.
fn exercise_schedules(name: &str, bytes: &[u8], c: &mut SuiteCounts) {
    c.inputs += 1;
    // nth=1.
    arm(1);
    let (r, flag) = decode_strict(bytes);
    let fired = COUNTDOWN.load(Ordering::SeqCst) == 0;
    let (fsize, falign) = (FAILED_SIZE.load(Ordering::SeqCst), FAILED_ALIGN.load(Ordering::SeqCst));
    disarm();
    let request = match r {
        Err(TryError::OutOfMemory(o)) => {
            assert!(fired && !flag, "{name}: nth=1 OOM without an injected failure");
            assert_eq!(o.elements(), fsize, "{name}: ghost request vs failed event");
            assert_eq!(falign, 1, "{name}: failed event alignment is not 1");
            assert_eq!(fsize % 4, 0, "{name}: reservation is not a multiple of 4");
            Some(fsize)
        }
        Err(TryError::Format(_)) => {
            assert!(!fired, "{name}: format error consumed the reservation");
            None
        }
        Ok(_) => panic!("{name}: decoded under fault injection"),
    };
    c.reached[0] += usize::from(request.is_some());
    c.injected[0] += usize::from(fired);
    let Some(size) = request else { return };
    c.allocating += 1;
    c.max_request = c.max_request.max(size);

    // nth=2: reached (an allocating case) but can never fire.
    arm(2);
    let (r, _) = decode_strict(bytes);
    let still_armed = COUNTDOWN.load(Ordering::SeqCst);
    let allocs = ALLOCS.load(Ordering::SeqCst);
    disarm();
    assert_eq!(still_armed, 2, "{name}: allocation 2 fired");
    assert_eq!(allocs, 1, "{name}: more than one reservation");
    assert!(!matches!(r, Err(TryError::OutOfMemory(_))), "{name}: OOM without injection");
    c.reached[1] += 1;

    // Thresholds, windowed to the decoder's reservation.
    for (i, t) in [(2usize, 1usize), (3, 4096), (4, 65536), (5, MIB64)] {
        let expect = size >= t;
        c.reached[i] += usize::from(expect);
        arm_threshold(t, size);
        let (r, flag) = decode_strict(bytes);
        let hit = THRESHOLD_HIT.load(Ordering::SeqCst);
        let (fsize, falign) = (FAILED_SIZE.load(Ordering::SeqCst), FAILED_ALIGN.load(Ordering::SeqCst));
        disarm();
        if expect {
            assert!(hit > 0, "{name}: threshold {t} did not fail the {size}-byte request");
            assert_eq!((fsize, falign), (size, 1), "{name}: threshold failed event");
            assert!(matches!(r, Err(TryError::OutOfMemory(_))) && !flag, "{name}: threshold {t}");
            c.injected[i] += 1;
        } else {
            assert_eq!(hit, 0, "{name}: threshold {t} failed a smaller request");
            assert!(!matches!(r, Err(TryError::OutOfMemory(_))), "{name}: threshold {t}");
        }
    }

    // Control (no injection): physical reservation alignment 1, size >= 4N.
    WATCH_SIZE.store(size, Ordering::SeqCst);
    WATCH_ALIGN.store(0, Ordering::SeqCst);
    let (r, _) = decode_strict(bytes);
    let align = WATCH_ALIGN.load(Ordering::SeqCst);
    WATCH_SIZE.store(0, Ordering::SeqCst);
    assert_eq!(align, core::mem::align_of::<u8>(), "{name}: reservation alignment");
    assert_eq!(align, 1, "{name}: reservation alignment");
    if let Ok(d) = &r {
        let n = d.width as usize * d.height as usize;
        assert!(size >= 4 * n, "{name}: reservation {size} < 4N = {}", 4 * n);
    }
}

/// Every schedule must have injected a failure at least once across the suite;
/// otherwise it passed vacuously. Prints per-schedule counts.
fn assert_suite_exercised(c: &SuiteCounts) {
    println!(
        "fault suite: {} inputs, {} allocate, largest reservation {} bytes",
        c.inputs, c.allocating, c.max_request
    );
    for (i, s) in SCHEDULES.iter().enumerate() {
        println!("  schedule {s}: trigger reachable in {} cases, failure injected in {}", c.reached[i], c.injected[i]);
    }
    assert!(c.inputs > 0 && c.allocating > 0, "no case allocates at all");
    assert!(c.injected[0] > 0, "nth=1 never injected a failure");
    assert!(c.reached[1] > 0 && c.injected[1] == 0, "allocation-2 schedule not exercised or fired");
    for i in [2, 3, 4] {
        assert!(c.injected[i] > 0, "schedule {} never injected a failure", SCHEDULES[i]);
    }
    if c.injected[5] == 0 {
        assert!(c.max_request < MIB64, "64 MiB reservation seen but no failure injected");
        println!("  UNREACHABLE by this input set: threshold 67108864; largest reservation is {} bytes", c.max_request);
    }
}

#[test]
fn every_schedule_is_exercised_across_the_suite() {
    let _guard = lock();
    let mut c = SuiteCounts::default();
    for (name, bytes) in &conformance_vectors() {
        exercise_schedules(name, bytes, &mut c);
    }
    // Corpus sample: read in place, never copied into the repo.
    for (name, bytes) in &corpus_sample(12) {
        exercise_schedules(name, bytes, &mut c);
    }
    assert_suite_exercised(&c);
}
