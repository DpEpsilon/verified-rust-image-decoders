//! In-process decode benchmark. Usage:
//!   bench DECODER check LISTFILE
//!   bench DECODER time  LISTFILE [PASSES] [WARMUP]
//! LISTFILE holds one path per line. All files are read into memory first, so
//! timing excludes file I/O. A "pass" decodes every listed file once.
use std::{hint::black_box, io::Cursor, time::Instant};
use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};
use image::ImageFormat;
use sha2::{Digest, Sha256};

struct Img { w: u32, h: u32, rgba: Vec<u8> }
type R = Result<Img, String>;

fn via_image(b: &[u8], f: ImageFormat) -> R {
    let im = image::load_from_memory_with_format(b, f).map_err(|e| e.to_string())?;
    let (w, h) = (im.width(), im.height());
    Ok(Img { w, h, rgba: im.into_rgba8().into_raw() })
}
fn bmp_verified(b: &[u8]) -> R {
    dib_verus::decode(b, dib_verus::Mode::BmpFile).map(|d| Img { w: d.width, h: d.height, rgba: d.rgba }).map_err(|e| e.name().into())
}
fn tga_verified(b: &[u8]) -> R {
    tga_verus::tga::decode(b).map(|d| Img { w: d.width, h: d.height, rgba: d.rgba }).map_err(|e| e.name().into())
}
fn tga_tiny(b: &[u8]) -> R {
    let im = tinytga::Tga::<Rgb888>::from_slice(b).map_err(|e| format!("{e:?}"))?;
    let s = im.as_raw().size();
    let (w, h) = (s.width, s.height);
    if w == 0 || h == 0 || w as u64 * h as u64 > 16_800_000 { return Err("dims".into()); }
    let mut rgba = vec![0u8; w as usize * h as usize * 4];
    let mut n = 0usize;
    for embedded_graphics::Pixel(p, c) in im.pixels() {
        if p.x < 0 || p.y < 0 || p.x as u32 >= w || p.y as u32 >= h { return Err("oob".into()); }
        let at = (p.y as usize * w as usize + p.x as usize) * 4;
        rgba[at..at + 4].copy_from_slice(&[c.r(), c.g(), c.b(), 255]);
        n += 1;
    }
    if n != w as usize * h as usize { return Err("truncated".into()); }
    Ok(Img { w, h, rgba })
}
fn icns_verified(b: &[u8]) -> R {
    let entries = icns_verus::parse(b).map_err(|e| e.name().to_string())?;
    let codes: [([u8; 4], [u8; 4]); 4] = [(*b"it32", *b"t8mk"), (*b"ih32", *b"h8mk"), (*b"il32", *b"l8mk"), (*b"is32", *b"s8mk")];
    for (c, m) in codes {
        let ci = entries.iter().position(|e| e.type_code() == c);
        let mi = entries.iter().position(|e| e.type_code() == m);
        if let (Some(ci), Some(mi)) = (ci, mi) {
            return icns_verus::combine(b, &entries, Some(ci as u32), Some(mi as u32))
                .map(|d| Img { w: d.width, h: d.height, rgba: d.data }).map_err(|e| e.name().into());
        }
    }
    Err("NoLegacyPair".into())
}
fn icns_crate(b: &[u8]) -> R {
    let fam = icns::IconFamily::read(Cursor::new(b)).map_err(|e| e.to_string())?;
    let t = fam.available_icons().into_iter().max_by_key(|t| (t.pixel_width().max(t.pixel_height()), t.pixel_width())).ok_or("NoCandidate")?;
    let im = fam.get_icon_with_type(t).map_err(|e| e.to_string())?;
    let d = im.convert_to(icns::PixelFormat::RGBA);
    Ok(Img { w: d.width(), h: d.height(), rgba: d.data().to_vec() })
}
fn ico_verified(b: &[u8]) -> R {
    let entries = ico_verus::ico::parse(b).map_err(|e| e.name().to_string())?;
    let side = |v: u16| if v == 0 { 256u64 } else { v as u64 };
    let i = entries.iter().enumerate().max_by_key(|(_, e)| (side(e.width) * side(e.height), e.bit_count.unwrap_or(0))).map(|(i, _)| i).ok_or("NoEntry")?;
    ico_verus::ico::decode_entry(b, &entries, i as u32).map(|d| Img { w: d.width, h: d.height, rgba: d.rgba }).map_err(|e| e.name().to_string())
}
fn ico_crate(b: &[u8]) -> R {
    let dir = ico::IconDir::read(Cursor::new(b)).map_err(|e| e.to_string())?;
    let es = dir.entries();
    let i = es.iter().enumerate().max_by_key(|(_, e)| ((e.width() as u64) * (e.height() as u64), e.bits_per_pixel())).map(|(i, _)| i).ok_or("NoEntry")?;
    let im = es[i].decode().map_err(|e| e.to_string())?;
    Ok(Img { w: im.width(), h: im.height(), rgba: im.rgba_data().to_vec() })
}
fn pick(name: &str) -> Option<fn(&[u8]) -> R> {
    Some(match name {
        "bmp-verified" => bmp_verified,
        "bmp-image" => |b| via_image(b, ImageFormat::Bmp),
        "tga-verified" => tga_verified,
        "tga-image" => |b| via_image(b, ImageFormat::Tga),
        "tga-tinytga" => tga_tiny,
        "icns-verified" => icns_verified,
        "icns-crate" => icns_crate,
        "ico-verified" => ico_verified,
        "ico-crate" => ico_crate,
        "ico-image" => |b| via_image(b, ImageFormat::Ico),
        "none" => |_| Ok(Img { w: 1, h: 1, rgba: vec![0; 4] }),
        _ => return None,
    })
}
#[repr(C)]
struct Rusage { times: [i64; 4], maxrss: i64, rest: [i64; 13] }
#[repr(C)]
struct Timespec { sec: i64, nsec: i64 }
extern "C" { fn getrusage(who: i32, usage: *mut Rusage) -> i32; fn clock_gettime(clk: i32, ts: *mut Timespec) -> i32; }
/// Process CPU time (CLOCK_PROCESS_CPUTIME_ID = 2) in ns.
fn cpu_ns() -> u128 {
    let mut t = Timespec { sec: 0, nsec: 0 };
    unsafe { clock_gettime(2, &mut t) };
    t.sec as u128 * 1_000_000_000 + t.nsec as u128
}
/// Peak resident set size so far (getrusage ru_maxrss, KiB on Linux); /proc VmHWM reads 0 in this sandbox.
fn status_kb(_key: &str) -> u64 {
    let mut r = Rusage { times: [0; 4], maxrss: 0, rest: [0; 13] };
    if unsafe { getrusage(0, &mut r) } == 0 { r.maxrss as u64 } else { 0 }
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 4 { eprintln!("usage: bench DECODER check|time LIST [PASSES] [WARMUP]"); std::process::exit(2); }
    let Some(f) = pick(&a[1]) else { eprintln!("unknown decoder"); std::process::exit(2) };
    let paths: Vec<String> = std::fs::read_to_string(&a[3]).unwrap().lines().filter(|l| !l.is_empty()).map(String::from).collect();
    let files: Vec<Vec<u8>> = paths.iter().map(|p| std::fs::read(p).unwrap()).collect();
    if a[2] == "check" {
        std::panic::set_hook(Box::new(|_| {}));
        for (p, b) in paths.iter().zip(&files) {
            match std::panic::catch_unwind(|| f(b)) {
                Ok(Ok(im)) if im.rgba.len() as u64 == im.w as u64 * im.h as u64 * 4 =>
                    println!("C\t{p}\tok\t{}\t{}\t{:x}", im.w, im.h, Sha256::digest(&im.rgba)),
                Ok(Ok(_)) => println!("C\t{p}\terr\t0\t0\tbad-length"),
                Ok(Err(e)) => println!("C\t{p}\terr\t0\t0\t{}", e.replace(['\t', '\n'], " ")),
                Err(_) => println!("C\t{p}\tcrash\t0\t0\tpanic"),
            }
        }
        return;
    }
    let passes: usize = a.get(4).and_then(|s| s.parse().ok()).unwrap_or(7);
    let warm: usize = a.get(5).and_then(|s| s.parse().ok()).unwrap_or(2);
    let baseline = status_kb("VmHWM:");
    let mut pixels = 0u64;
    for w in 0..warm {
        for b in &files {
            let im = f(b).expect("decode failed in time mode");
            if w == 0 { pixels += im.w as u64 * im.h as u64; }
            black_box(&im);
        }
    }
    // Calibrate: repeat the file list so one pass lasts >= ~100 ms (the CPU clock here has 10 ms granularity).
    let t = Instant::now();
    for b in &files { black_box(f(black_box(b)).ok()); }
    let reps = ((100_000_000f64 / t.elapsed().as_nanos().max(1) as f64).ceil() as u64).max(1);
    pixels *= reps;
    let mut ns = Vec::new();
    let mut cpu = Vec::new();
    let (tc0, tw0) = (cpu_ns(), Instant::now());
    for _ in 0..passes {
        let c0 = cpu_ns();
        let t = Instant::now();
        for _ in 0..reps { for b in &files { black_box(f(black_box(b)).ok()); } }
        ns.push(t.elapsed().as_nanos());
        cpu.push(cpu_ns() - c0);
    }
    let total_cpu = cpu_ns() - tc0; let _ = tw0;
    let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
    let c: Vec<String> = cpu.iter().map(|n| n.to_string()).collect();
    println!("T\t{}\t{}\t{}\t{}\t{}\t{}\t{}", s.join(","), c.join(","), pixels, files.len(), baseline, status_kb("VmHWM:"), total_cpu);
}
