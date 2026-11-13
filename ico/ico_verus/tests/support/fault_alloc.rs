//! Host-only allocation probe for the ICO core (alloc-delta THEOREMS-ALLOC-DELTA
//! §5). The single caller serializes cases; thread-local state excludes unrelated
//! harness threads. No allocator callback allocates.
//!
//! Two reservation sites are exercised: ICO's own `EntryList` reservation in
//! `try_parse`, and DIB's `Output` reservation reached inside `try_decode_entry`
//! (the `Dib(Output)` event). The probe records every `alloc`/`alloc_zeroed`/
//! `realloc` event, injects a null return at a chosen event index or above a
//! chosen size, and counts live bytes only after a successful underlying
//! allocation.
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::ptr;

use ico_verus::alloc_guard::AllocSite;
use ico_verus::ico::{Entry, TryError};
use dib_verus::Decoded;

#[derive(Clone, Copy, Debug, Default)]
pub struct Stats {
    pub attempts: usize,
    pub successes: usize,
    pub failures: usize,
    pub last_size: usize,
    pub last_align: usize,
    pub failed_size: usize,
    pub failed_align: usize,
    pub reallocs: usize,
    pub live: usize,
    pub peak: usize,
    pub overflow: bool,
}

#[derive(Clone, Copy)]
struct State {
    active: bool,
    nth: usize,
    threshold: usize,
    stats: Stats,
    blocks: [(usize, usize); 64],
}

impl State {
    const EMPTY: Self = Self {
        active: false,
        nth: 0,
        threshold: 0,
        stats: Stats {
            attempts: 0,
            successes: 0,
            failures: 0,
            last_size: 0,
            last_align: 0,
            failed_size: 0,
            failed_align: 0,
            reallocs: 0,
            live: 0,
            peak: 0,
            overflow: false,
        },
        blocks: [(0, 0); 64],
    };
    fn remove(&mut self, p: *mut u8) {
        if let Some(i) = self.blocks.iter().position(|&(q, _)| q != 0 && q == p as usize) {
            self.stats.live -= self.blocks[i].1;
            self.blocks[i] = (0, 0);
        }
    }
    fn insert(&mut self, p: *mut u8, size: usize) {
        if let Some(i) = self.blocks.iter().position(|&(q, _)| q == 0) {
            self.blocks[i] = (p as usize, size);
            self.stats.live += size;
            self.stats.peak = self.stats.peak.max(self.stats.live);
        } else {
            self.stats.overflow = true;
        }
    }
}

/// Per-schedule suite counters, outside the allocator callbacks' `STATE`.
const SCHEDULES: [(usize, usize); 8] = [
    (0, 0),
    (1, 0),
    (2, 0),
    (3, 0),
    (0, 1),
    (0, 4096),
    (0, 65536),
    (0, 67108864),
];

#[derive(Clone, Copy)]
struct Suite {
    cases: usize,
    allocating_cases: usize,
    injected: [usize; 8],
    reached: [usize; 8],
    max_request: usize,
    decode_oom: usize,
}

std::thread_local! {
    static SUITE: Cell<Suite> = const { Cell::new(Suite {
        cases: 0, allocating_cases: 0, injected: [0; 8], reached: [0; 8], max_request: 0, decode_oom: 0 }) };
}

/// Suite-level assertions, called once after every `check`. A schedule that
/// never injected a failure would otherwise pass vacuously.
pub fn assert_suite_exercised() {
    let s = SUITE.with(|c| c.get());
    println!(
        "fault suite: {} cases, {} allocate, largest reservation {} bytes",
        s.cases, s.allocating_cases, s.max_request
    );
    for (i, (nth, threshold)) in SCHEDULES.iter().enumerate() {
        println!(
            "  schedule nth={nth} threshold={threshold}: trigger reachable in {} cases, failure injected in {}",
            s.reached[i], s.injected[i]
        );
    }
    assert!(s.cases > 0);
    assert!(s.allocating_cases > 0, "no case allocates at all");
    assert!(s.decode_oom > 0, "no decode case reached DIB's Output reservation under injection");
    println!("  decode cases that returned OutOfMemory at Dib(Output): {}", s.decode_oom);
    assert!(s.injected[0] == 0, "control injected a failure");
    // The ICO decode path makes at most two reservations (DIB's Output, and the
    // entry list on the parse side); the third and later schedules must be
    // reached by allocating cases and inject nothing.
    assert!(s.reached[2] > 0 && s.injected[2] == 0, "allocation-2 schedule not exercised");
    assert!(s.reached[3] > 0 && s.injected[3] == 0, "allocation-3 schedule not exercised");
    // The ICO corpus's largest reservation is the 1,024-entry list (45,056 B);
    // no vector reaches a 64 MiB DIB output, so the 65,536 and 67,108,864
    // thresholds are no-failure controls here. The 1-byte and 4,096-byte
    // thresholds must inject.
    for i in [1, 4, 5] {
        assert!(s.injected[i] > 0, "schedule {:?} never injected a failure", SCHEDULES[i]);
    }
    for i in [6, 7] {
        if s.injected[i] == 0 {
            assert!(
                s.max_request < SCHEDULES[i].1,
                "reservation at or above threshold {:?} seen but no failure injected",
                SCHEDULES[i]
            );
            println!(
                "  UNREACHABLE by this corpus: threshold {}; the largest reservation is {} bytes, so that schedule injects nothing and is a no-failure control here",
                SCHEDULES[i].1, s.max_request
            );
        }
    }
}

std::thread_local! { static STATE: Cell<State> = const { Cell::new(State::EMPTY) }; }
struct Probe;
#[global_allocator]
static ALLOCATOR: Probe = Probe;

fn attempt(size: usize, align: usize, realloc: bool) -> bool {
    STATE
        .try_with(|cell| {
            let mut s = cell.get();
            if !s.active {
                return false;
            }
            s.stats.attempts += 1;
            s.stats.reallocs += usize::from(realloc);
            s.stats.last_size = size;
            s.stats.last_align = align;
            let fail = (s.nth != 0 && s.stats.attempts == s.nth)
                || (s.threshold != 0 && size >= s.threshold);
            if fail {
                s.stats.failures += 1;
                s.stats.failed_size = size;
                s.stats.failed_align = align;
            }
            cell.set(s);
            fail
        })
        .unwrap_or(false)
}

fn completed(p: *mut u8, size: usize, align: usize, old: *mut u8) {
    let _ = STATE.try_with(|cell| {
        let mut s = cell.get();
        if !s.active {
            return;
        }
        if p.is_null() {
            s.stats.failures += 1;
            s.stats.failed_size = size;
            s.stats.failed_align = align;
        } else {
            // A failed underlying allocation never increases live bytes.
            s.remove(old);
            s.stats.successes += 1;
            s.insert(p, size);
        }
        cell.set(s);
    });
}

unsafe impl GlobalAlloc for Probe {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if attempt(layout.size(), layout.align(), false) {
            return ptr::null_mut();
        }
        let p = System.alloc(layout);
        completed(p, layout.size(), layout.align(), ptr::null_mut());
        p
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if attempt(layout.size(), layout.align(), false) {
            return ptr::null_mut();
        }
        let p = System.alloc_zeroed(layout);
        completed(p, layout.size(), layout.align(), ptr::null_mut());
        p
    }
    unsafe fn realloc(&self, p: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if attempt(size, layout.align(), true) {
            return ptr::null_mut();
        }
        let q = System.realloc(p, layout, size);
        completed(q, size, layout.align(), p);
        q
    }
    unsafe fn dealloc(&self, p: *mut u8, layout: Layout) {
        let _ = STATE.try_with(|cell| {
            let mut s = cell.get();
            s.remove(p);
            cell.set(s);
        });
        System.dealloc(p, layout);
    }
}

/// The element type of a site, for the expected `Layout` size.
fn site_elem_size(site: AllocSite) -> usize {
    match site {
        AllocSite::EntryList => core::mem::size_of::<Entry>(),
        AllocSite::Dib(_) => 1,
    }
}

fn equal_parse(a: &Result<Vec<Entry>, TryError>, b: &Result<Vec<Entry>, TryError>) {
    match (a, b) {
        (Ok(x), Ok(y)) => assert_eq!(x, y, "parse result changed without failure"),
        (Err(x), Err(y)) => assert_eq!(x, y, "parse error changed without failure"),
        _ => panic!("parse result variant changed without failure"),
    }
}

fn equal_decode(a: &Result<Decoded, TryError>, b: &Result<Decoded, TryError>) {
    match (a, b) {
        (Ok(x), Ok(y)) => {
            assert_eq!(x.width, y.width);
            assert_eq!(x.height, y.height);
            assert_eq!(x.rle_or_32bpp, y.rle_or_32bpp);
            assert_eq!(x.rgba, y.rgba);
        }
        (Err(x), Err(y)) => assert_eq!(x, y, "decode error changed without failure"),
        _ => panic!("decode result variant changed without failure"),
    }
}

/// Run one case under every schedule. `parse` is the ICO entry point under test;
/// `decode` is the ICO decode entry point (which reaches DIB's `Output`
/// reservation). Both are closures so the caller owns the input.
pub fn check_parse(mut parse: impl FnMut() -> Result<Vec<Entry>, TryError>) {
    let baseline = parse();
    assert!(!matches!(baseline, Err(TryError::OutOfMemory(_))));
    let mut request = None;
    for (nth, threshold) in SCHEDULES {
        STATE.with(|cell| cell.set(State { active: true, nth, threshold, ..State::EMPTY }));
        let result = parse();
        let stats = STATE.with(|cell| {
            let mut s = cell.get();
            s.active = false;
            cell.set(s);
            s.stats
        });
        assert!(!stats.overflow);
        assert_eq!(stats.reallocs, 0, "parse reallocated");
        assert!(stats.attempts <= 1, "extra allocation or retry in parse");
        let idx = SCHEDULES.iter().position(|&x| x == (nth, threshold)).unwrap();
        if nth == 0 && threshold == 0 {
            assert_eq!(stats.failures, 0, "underlying allocator failed in control");
            request = (stats.attempts == 1).then_some(stats.last_size);
            if request.is_some() {
                // ICO's single reservation: `Entry` elements, align 4.
                assert_eq!(stats.last_align, core::mem::align_of::<Entry>(), "reservation alignment");
                if let Ok(es) = &baseline {
                    assert!(stats.last_size >= es.len() * core::mem::size_of::<Entry>());
                }
            }
        }
        let should_fail =
            request.is_some_and(|size| nth == 1 || (threshold != 0 && size >= threshold));
        assert_eq!(stats.failures, usize::from(should_fail));
        if should_fail {
            assert_eq!(stats.failed_align, core::mem::align_of::<Entry>(), "failed event alignment");
        }
        SUITE.with(|c| {
            let mut s = c.get();
            if idx == 0 {
                s.cases += 1;
                s.allocating_cases += usize::from(request.is_some());
            }
            if let Some(size) = request {
                s.max_request = s.max_request.max(size);
            }
            s.reached[idx] += usize::from(match (nth, threshold) {
                (0, 0) => true,
                (n, 0) => request.is_some() && n >= 1,
                (_, t) => request.is_some_and(|size| size >= t),
            });
            s.injected[idx] += usize::from(stats.failures > 0);
            c.set(s);
        });
        assert_eq!(stats.attempts, usize::from(request.is_some()));
        assert_eq!(stats.successes, stats.attempts - stats.failures);
        if let Some(size) = request {
            assert_eq!(stats.last_size, size);
        }
        if should_fail {
            let Err(TryError::OutOfMemory(o)) = result else {
                panic!("failed reservation did not return OutOfMemory");
            };
            assert_eq!(o.site(), AllocSite::EntryList);
            assert_eq!(o.elements(), stats.failed_size / site_elem_size(AllocSite::EntryList));
            assert!(!matches!(result, Ok(_)), "OOM reported success");
            assert_eq!(stats.live, 0, "OOM retained partial output");
            assert_eq!(stats.peak, 0, "failed reservation counted live bytes");
        } else {
            equal_parse(&result, &baseline);
            if let Ok(es) = &result {
                assert_eq!(Some(es.len() * core::mem::size_of::<Entry>()), request);
            } else {
                assert_eq!(stats.live, 0, "format error leaked its buffer");
            }
        }
        drop(result);
        STATE.with(|cell| {
            assert_eq!(cell.get().stats.live, 0, "output drop leaked bytes");
            cell.set(State::EMPTY);
        });
    }
}

/// Run one decode case under every schedule. The decode path makes at most one
/// reservation of its own (DIB's `Output`), so the schedules beyond the first
/// allocation are reached but inject nothing.
pub fn check_decode(mut decode: impl FnMut() -> Result<Decoded, TryError>) {
    let baseline = decode();
    assert!(!matches!(baseline, Err(TryError::OutOfMemory(_))));
    let mut request = None;
    for (nth, threshold) in SCHEDULES {
        STATE.with(|cell| cell.set(State { active: true, nth, threshold, ..State::EMPTY }));
        let result = decode();
        let stats = STATE.with(|cell| {
            let mut s = cell.get();
            s.active = false;
            cell.set(s);
            s.stats
        });
        assert!(!stats.overflow);
        assert_eq!(stats.reallocs, 0, "decode reallocated");
        assert!(stats.attempts <= 1, "extra allocation or retry in decode");
        let idx = SCHEDULES.iter().position(|&x| x == (nth, threshold)).unwrap();
        if nth == 0 && threshold == 0 {
            assert_eq!(stats.failures, 0, "underlying allocator failed in control");
            request = (stats.attempts == 1).then_some(stats.last_size);
            if request.is_some() {
                // DIB's single reservation: u8 elements, align 1.
                assert_eq!(stats.last_align, 1, "reservation alignment");
                if let Ok(d) = &baseline {
                    let n = (d.width as usize) * (d.height as usize);
                    assert!(stats.last_size >= 4 * n, "reservation smaller than 4N");
                }
            }
        }
        let should_fail =
            request.is_some_and(|size| nth == 1 || (threshold != 0 && size >= threshold));
        assert_eq!(stats.failures, usize::from(should_fail));
        if should_fail {
            assert_eq!(stats.failed_align, 1, "failed event alignment");
        }
        SUITE.with(|c| {
            let mut s = c.get();
            if idx == 0 {
                s.cases += 1;
                s.allocating_cases += usize::from(request.is_some());
            }
            if let Some(size) = request {
                s.max_request = s.max_request.max(size);
            }
            s.reached[idx] += usize::from(match (nth, threshold) {
                (0, 0) => true,
                (n, 0) => request.is_some() && n >= 1,
                (_, t) => request.is_some_and(|size| size >= t),
            });
            s.injected[idx] += usize::from(stats.failures > 0);
            c.set(s);
        });
        assert_eq!(stats.attempts, usize::from(request.is_some()));
        assert_eq!(stats.successes, stats.attempts - stats.failures);
        if let Some(size) = request {
            assert_eq!(stats.last_size, size);
        }
        if should_fail {
            let Err(TryError::OutOfMemory(o)) = result else {
                panic!("failed reservation did not return OutOfMemory");
            };
            // The only reservation on the decode path is DIB's `Output`.
            assert_eq!(o.site(), AllocSite::Dib(dib_verus::alloc_guard::AllocSite::Output));
            assert_eq!(o.elements(), stats.failed_size);
            SUITE.with(|c| { let mut s = c.get(); s.decode_oom += 1; c.set(s); });
            assert_eq!(Some(o.elements()), request);
            assert_eq!(stats.live, 0, "OOM retained partial output");
            assert_eq!(stats.peak, 0, "failed reservation counted live bytes");
        } else {
            equal_decode(&result, &baseline);
            if let Ok(d) = &result {
                assert!(d.rgba.capacity() >= d.rgba.len());
                assert_eq!(Some(d.rgba.len()), request);
                assert_eq!(stats.live, d.rgba.capacity());
            } else {
                assert_eq!(stats.live, 0, "format error leaked its buffer");
            }
        }
        drop(result);
        STATE.with(|cell| {
            assert_eq!(cell.get().stats.live, 0, "output drop leaked bytes");
            cell.set(State::EMPTY);
        });
    }
}
