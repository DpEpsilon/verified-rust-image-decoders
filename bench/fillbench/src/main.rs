use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};
use std::time::Instant;

static ON: AtomicBool = AtomicBool::new(false);
static NA: AtomicUsize = AtomicUsize::new(0); // alloc + alloc_zeroed
static NZ: AtomicUsize = AtomicUsize::new(0); // of which alloc_zeroed
static NR: AtomicUsize = AtomicUsize::new(0);
static ND: AtomicUsize = AtomicUsize::new(0);
struct Counting;
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        if ON.load(Relaxed) { NA.fetch_add(1, Relaxed); }
        System.alloc(l)
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        if ON.load(Relaxed) { NA.fetch_add(1, Relaxed); NZ.fetch_add(1, Relaxed); }
        System.alloc_zeroed(l)
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        if ON.load(Relaxed) { NR.fetch_add(1, Relaxed); }
        System.realloc(p, l, n)
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        if ON.load(Relaxed) { ND.fetch_add(1, Relaxed); }
        System.dealloc(p, l)
    }
}
#[global_allocator]
static G: Counting = Counting;

#[repr(C)]
struct Timespec { s: i64, ns: i64 }
#[repr(C)]
struct Timeval { s: i64, us: i64 }
#[repr(C)]
struct Rusage { u: Timeval, st: Timeval, maxrss: i64, rest: [i64; 13] }
extern "C" {
    fn clock_gettime(id: i32, ts: *mut Timespec) -> i32;
    fn getrusage(who: i32, r: *mut Rusage) -> i32;
}
fn cpu_ns() -> u64 {
    let mut t = Timespec { s: 0, ns: 0 };
    unsafe { clock_gettime(2, &mut t) }; // CLOCK_PROCESS_CPUTIME_ID
    t.s as u64 * 1_000_000_000 + t.ns as u64
}
fn maxrss_kib() -> i64 {
    let mut r: Rusage = unsafe { std::mem::zeroed() };
    unsafe { getrusage(0, &mut r) };
    r.maxrss
}

// ---- candidates ----
fn a_push(n: usize) -> Option<Vec<u8>> {
    let mut v: Vec<u8> = Vec::new();
    v.try_reserve_exact(n).ok()?;
    let mut i: usize = 0;
    while i < n { v.push(0u8); i = i + 1; }
    Some(v)
}
fn b_calloc(n: usize) -> Option<Vec<u8>> { Some(vec![0u8; n]) }
fn c_resize(n: usize) -> Option<Vec<u8>> {
    let mut v: Vec<u8> = Vec::new();
    v.try_reserve_exact(n).ok()?;
    v.resize(n, 0);
    Some(v)
}
fn chunked<const C: usize>(n: usize) -> Option<Vec<u8>> {
    let mut v: Vec<u8> = Vec::new();
    v.try_reserve_exact(n).ok()?;
    let z = [0u8; C];
    while v.len() + C <= n { v.extend_from_slice(&z); }
    let rem = n - v.len();
    v.extend_from_slice(&z[..rem]);
    Some(v)
}
fn d4k(n: usize) -> Option<Vec<u8>> { chunked::<4096>(n) }
fn d64k(n: usize) -> Option<Vec<u8>> { chunked::<65536>(n) }
fn e_extend(n: usize) -> Option<Vec<u8>> {
    let mut v: Vec<u8> = Vec::new();
    v.try_reserve_exact(n).ok()?;
    v.extend(core::iter::repeat(0u8).take(n));
    Some(v)
}
fn f_resize_with(n: usize) -> Option<Vec<u8>> {
    let mut v: Vec<u8> = Vec::new();
    v.try_reserve_exact(n).ok()?;
    v.resize_with(n, || 0);
    Some(v)
}
type F = fn(usize) -> Option<Vec<u8>>;
const CANDS: &[(&str, F)] = &[
    ("A push-loop", a_push), ("B vec![0;n]", b_calloc), ("C resize", c_resize),
    ("D1 extend_from_slice 4K", d4k), ("D2 extend_from_slice 64K", d64k),
    ("E extend(repeat.take)", e_extend), ("F resize_with", f_resize_with),
];

fn write_all(v: &mut [u8], seed: u32) {
    let n4 = v.len() / 4;
    for i in 0..n4 {
        let x = (i as u32).wrapping_mul(0x9E3779B1) ^ seed;
        let px = x.to_le_bytes();
        v[4 * i..4 * i + 4].copy_from_slice(&px);
    }
}

// returns (cpu ns/op, wall ns/op)
fn time(f: F, n: usize, reps: usize, write: bool) -> (f64, f64) {
    let c0 = cpu_ns();
    let w0 = Instant::now();
    for r in 0..reps {
        let mut v = black_box(f(black_box(n))).unwrap();
        if write { write_all(&mut v, r as u32); }
        black_box(&v);
        drop(v);
    }
    let w = w0.elapsed().as_nanos() as f64;
    let c = (cpu_ns() - c0) as f64;
    (c / reps as f64, w / reps as f64)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let sizes: [(&str, usize); 4] = [("64MiB", 4 * 4096 * 4096), ("4MiB", 4 * 1024 * 1024),
        ("256KiB", 4 * 256 * 256), ("1KiB", 4 * 16 * 16)];
    match args.get(1).map(|s| s.as_str()) {
        Some("rss") => { // fillbench rss <cand idx> <fill|write>
            let i: usize = args[2].parse().unwrap();
            let write = args[3] == "write";
            let n = sizes[0].1;
            let mut v = CANDS[i].1(n).unwrap();
            if write { write_all(&mut v, 1); }
            black_box(&v);
            println!("{}\t{}\t{}", CANDS[i].0, args[3], maxrss_kib());
        }
        Some("count") => {
            let n = sizes[0].1;
            for (name, f) in CANDS {
                NA.store(0, Relaxed); NZ.store(0, Relaxed); NR.store(0, Relaxed); ND.store(0, Relaxed);
                ON.store(true, Relaxed);
                let v = f(n).unwrap();
                let cap_ok = v.capacity() >= n && v.len() == n;
                let (a, z, r) = (NA.load(Relaxed), NZ.load(Relaxed), NR.load(Relaxed));
                drop(v);
                ON.store(false, Relaxed);
                println!("{}\talloc={} (zeroed={}) realloc={} dealloc={} len==n,cap>=n:{}", name, a, z, r, ND.load(Relaxed), cap_ok);
            }
        }
        _ => { // fillbench time <rounds>
            let rounds: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(7);
            println!("# round\tsize\tcand\tmode\tcpu_ns_per_op\twall_ns_per_op\treps");
            for round in 0..rounds {
                for (sn, n) in sizes {
                    let reps = ((1usize << 31) / n).clamp(20, 50_000);
                    for write in [false, true] {
                        for k in 0..CANDS.len() {
                            // rotate start candidate each round to spread order bias
                            let (name, f) = CANDS[(k + round) % CANDS.len()];
                            let _ = time(f, n, 2, write); // warm-up
                            let (c, w) = time(f, n, reps, write);
                            println!("{}\t{}\t{}\t{}\t{:.0}\t{:.0}\t{}", round, sn, name,
                                if write { "fill+write" } else { "fill" }, c, w, reps);
                        }
                    }
                }
            }
        }
    }
}
