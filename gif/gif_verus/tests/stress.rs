//! Stress and performance test (PLAN.md M5 candidate, DECISIONS.md Q8): a
//! synthetic 60×60 stream with 37,239 frames, the shape of a full-film GIF
//! that fits every cap (134,060,400 frame pixels, 12.5% of
//! `MAX_TOTAL_PIXELS`; 57% of `MAX_FRAMES`). The stream is generated here, with
//! a standard GIF LZW encoder (Clear first, Clear when the table is full), 16
//! colours, a NETSCAPE2.0 loop block and a Graphic Control per frame cycling
//! disposal 0, 1, 2, 3, so `Saved` is reserved.
//!
//! Checked: the iterator yields 37,239 frames and ends at the Trailer; every
//! frame's canvas equals the frame's generated pixels (each frame covers the
//! screen opaquely, so the canvas after frame k is frame k whatever the
//! disposals); the live heap during the decode (input excluded) peaks at
//! `8·W·H + 20,480 = 49,280` bytes, under 64 KiB; zero reallocations. The
//! decode time is printed (run with `--nocapture`).

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::time::Instant;

use gif_verus::Frames;

std::thread_local! {
    static ARMED: Cell<bool> = const { Cell::new(false) };
    static LIVE: Cell<isize> = const { Cell::new(0) };
    static PEAK: Cell<isize> = const { Cell::new(0) };
    static ALLOCS: Cell<usize> = const { Cell::new(0) };
    static REALLOCS: Cell<usize> = const { Cell::new(0) };
}

fn armed() -> bool {
    ARMED.try_with(|a| a.get()).unwrap_or(false)
}

fn add(n: isize) {
    let _ = LIVE.try_with(|l| {
        let v = l.get() + n;
        l.set(v);
        let _ = PEAK.try_with(|p| {
            if v > p.get() {
                p.set(v)
            }
        });
    });
}

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let p = System.alloc(l);
        if !p.is_null() && armed() {
            let _ = ALLOCS.try_with(|c| c.set(c.get() + 1));
            add(l.size() as isize);
        }
        p
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        if armed() {
            add(-(l.size() as isize));
        }
        System.dealloc(p, l)
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        if armed() {
            let _ = REALLOCS.try_with(|c| c.set(c.get() + 1));
        }
        let q = System.realloc(p, l, n);
        if !q.is_null() && armed() {
            add(n as isize - l.size() as isize);
        }
        q
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

const W: usize = 60;
const H: usize = 60;
const FRAMES: usize = 37_239;
const M: u32 = 4;

fn palette() -> [[u8; 3]; 16] {
    let mut p = [[0u8; 3]; 16];
    for (i, e) in p.iter_mut().enumerate() {
        *e = [(i * 17) as u8, (255 - i * 13) as u8, ((i * 71) % 256) as u8];
    }
    p
}

/// Frame k's index raster: moving diagonal bands plus a block pattern.
fn pixel(k: usize, x: usize, y: usize) -> u8 {
    (((x + y + k) / 5 + ((x / 12) ^ (y / 12)) + (k / 7)) % 16) as u8
}

/// Standard GIF LZW (gen_vectors.lzw_codes + widths_for + pack): Clear first,
/// Clear when the table is full, End of Information last.
fn lzw(indices: &[u8], m: u32, out: &mut Vec<u8>) {
    let clear = 1u32 << m;
    let eoi = clear + 1;
    let mut dict = vec![0u16; 4096 * 16]; // dict[code*16 + v] = code of (code's string + v), 0 = none
    let mut codes: Vec<u32> = vec![clear];
    let mut next = eoi + 1;
    let mut w: u32 = indices[0] as u32;
    for &v in &indices[1..] {
        let slot = (w as usize) * 16 + v as usize;
        if dict[slot] != 0 {
            w = dict[slot] as u32;
            continue;
        }
        codes.push(w);
        if next < 4096 {
            dict[slot] = next as u16;
            next += 1;
        } else {
            codes.push(clear);
            dict.iter_mut().for_each(|d| *d = 0);
            next = eoi + 1;
        }
        w = v as u32;
    }
    codes.push(w);
    codes.push(eoi);
    // Reader widths.
    let (mut size, mut width, mut first) = (clear + 2, m + 1, true);
    let (mut acc, mut nb) = (0u64, 0u32);
    let mut data = Vec::new();
    for c in codes {
        acc |= (c as u64) << nb;
        nb += width;
        while nb >= 8 {
            data.push(acc as u8);
            acc >>= 8;
            nb -= 8;
        }
        if c == clear {
            size = clear + 2;
            width = m + 1;
            first = true;
            continue;
        }
        if c == eoi {
            continue;
        }
        if !first && size < 4096 {
            size += 1;
            if size == (1 << width) && width < 12 {
                width += 1;
            }
        }
        first = false;
    }
    if nb > 0 {
        data.push(acc as u8);
    }
    out.push(m as u8);
    for ch in data.chunks(255) {
        out.push(ch.len() as u8);
        out.extend_from_slice(ch);
    }
    out.push(0);
}

fn build() -> Vec<u8> {
    let mut b = b"GIF89a".to_vec();
    b.extend_from_slice(&(W as u16).to_le_bytes());
    b.extend_from_slice(&(H as u16).to_le_bytes());
    b.extend_from_slice(&[0x80 | 3, 0, 0]); // global table, 16 entries
    for e in palette() {
        b.extend_from_slice(&e);
    }
    b.extend_from_slice(&[0x21, 0xFF, 11]);
    b.extend_from_slice(b"NETSCAPE2.0");
    b.extend_from_slice(&[3, 1, 0, 0, 0]); // loop forever
    let mut raster = vec![0u8; W * H];
    for k in 0..FRAMES {
        let disposal = (k % 4) as u8;
        b.extend_from_slice(&[0x21, 0xF9, 4, disposal << 2, 4, 0, 0, 0]); // delay 4 cs
        b.push(0x2C);
        for v in [0u16, 0, W as u16, H as u16] {
            b.extend_from_slice(&v.to_le_bytes());
        }
        b.push(0);
        for y in 0..H {
            for x in 0..W {
                raster[y * W + x] = pixel(k, x, y);
            }
        }
        lzw(&raster, M, &mut b);
    }
    b.push(0x3B);
    b
}

#[test]
fn film_60x60_37239_frames() {
    let t0 = Instant::now();
    let b = build();
    let gen = t0.elapsed();
    assert!(b.len() <= gif_verus::limits::MAX_INPUT, "stream {} bytes exceeds MAX_INPUT", b.len());
    let pal = palette();

    ARMED.with(|a| a.set(true));
    let t1 = Instant::now();
    let mut d = Frames::try_new(&b).expect("constructor");
    assert_eq!(d.info().loop_count, Some(0));
    let mut n = 0usize;
    let mut mismatched = 0usize;
    let mut checksum: u64 = 0;
    loop {
        match d.next_frame() {
            None => break,
            Some(Err(e)) => panic!("frame {n}: {}", e.name()),
            Some(Ok(f)) => {
                assert_eq!(f.disposal as usize, n % 4);
                assert_eq!(f.delay_cs, 4);
                let mut ok = true;
                for y in 0..H {
                    for x in 0..W {
                        let c = pal[pixel(n, x, y) as usize];
                        let o = 4 * (y * W + x);
                        if f.rgba[o..o + 4] != [c[0], c[1], c[2], 255] {
                            ok = false;
                        }
                    }
                }
                checksum = checksum.wrapping_mul(31).wrapping_add(f.rgba[4 * (n % (W * H))] as u64);
                if !ok {
                    mismatched += 1;
                }
                n += 1;
            }
        }
    }
    let dec = t1.elapsed();
    drop(d);
    ARMED.with(|a| a.set(false));
    let peak = PEAK.with(|p| p.get());
    let reallocs = REALLOCS.with(|c| c.get());
    let allocs = ALLOCS.with(|c| c.get());
    println!(
        "stress: {} bytes generated in {:.2?}; decoded {} frames ({} pixels) in {:.2?} \
         ({:.1} Mpixel/s); peak live heap {} bytes in {} allocations, {} reallocs; checksum {:x}",
        b.len(),
        gen,
        n,
        n * W * H,
        dec,
        (n * W * H) as f64 / dec.as_secs_f64() / 1e6,
        peak,
        allocs,
        reallocs,
        checksum
    );
    // Decode-only timing: the same stream without the per-pixel comparison.
    let t2 = Instant::now();
    let mut d = Frames::try_new(&b).expect("constructor");
    let mut n2 = 0usize;
    while let Some(f) = d.next_frame() {
        let f = f.expect("frame");
        checksum ^= f.rgba[0] as u64;
        n2 += 1;
    }
    let dec_only = t2.elapsed();
    println!(
        "stress: decode only (no pixel comparison): {} frames in {:.2?} ({:.1} Mpixel/s, {:.1} µs/frame)",
        n2,
        dec_only,
        (n2 * W * H) as f64 / dec_only.as_secs_f64() / 1e6,
        dec_only.as_secs_f64() * 1e6 / n2 as f64
    );
    assert_eq!(n2, FRAMES);
    assert_eq!(n, FRAMES);
    assert_eq!(mismatched, 0, "frames whose canvas differs from the generated raster");
    assert_eq!(reallocs, 0);
    assert_eq!(allocs, 3, "Canvas, LzwTable, Saved");
    assert_eq!(peak as usize, 8 * W * H + 20_480);
    assert!(peak < 64 * 1024);
}
