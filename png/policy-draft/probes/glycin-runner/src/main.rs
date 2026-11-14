//! usage: png-glycin-probe glycin|glycin-apng FILE -> one JSON line
//!
//! Drives the PNG-only copy of glycin's image-rs loader (`glycin_png.rs`) the way glycin's
//! frame loop does: `ImgLoader::load`, then `specific_frame` with `loop_animation=false` until
//! `NoMoreFrames` (the driver loop is the one of ~/projects/leniency/gif/src/main.rs `glycin`).
//! Output: status, w, h, frames, sha (RGBA8 of frame 0; 16-bit samples rounded as image's
//! to_rgba8 and P13.2: round(v*255/65535)), sha_hi (16-bit samples by v>>8), seq (SHA-256 over all
//! RGBA8 frames), rgba16 (exact RGBA16, big-endian, frame 0), memfmt, delays_ms, icc (byte length
//! of FrameDetails.color_icc_profile, -1 if absent), tail_error.
mod glycin_png;

use glycin_utils::{FrameRequest, InitializationDetails, LoaderImplementation, LocalMemory, MemoryFormat, ProcessError};
use sha2::{Digest, Sha256};
use std::io::Cursor;

const MAX_FRAMES: usize = 256;

fn esc(s: &str) -> String {
    s.chars().flat_map(|c| match c { '"' => vec!['\\', '"'], '\\' => vec!['\\', '\\'], c if (c as u32) < 32 => vec![' '], c => vec![c] }).collect()
}
fn sha(v: &[u8]) -> String { format!("{:x}", Sha256::digest(v)) }
fn class(e: &ProcessError) -> &'static str {
    match e {
        ProcessError::ExpectedError { .. } => "ExpectedError", ProcessError::InternalError { .. } => "InternalError",
        ProcessError::UnsupportedImageFormat(_) => "UnsupportedImageFormat", ProcessError::ConversionTooLargerError => "ConversionTooLarge",
        ProcessError::OutOfMemory { .. } => "OutOfMemory", ProcessError::NoMoreFrames => "NoMoreFrames", _ => "OtherProcessError",
    }
}
fn err(status: &str, c: &str, m: &str) -> String {
    format!("{{\"status\":\"{status}\",\"error\":\"{c}: {}\"}}", esc(&m.chars().take(300).collect::<String>()))
}

/// glycin texture -> (RGBA8 rounded, RGBA8 high byte, exact RGBA16 if 16-bit)
fn rgba(f: &glycin_utils::Frame<LocalMemory>) -> Result<(Vec<u8>, Vec<u8>, Option<Vec<u16>>), String> {
    let (w, h, stride) = (f.width as usize, f.height as usize, f.stride as usize);
    let mut out = Vec::with_capacity(w * h * 4);
    let mut o16: Vec<u16> = vec![];
    let mut is16 = false;
    let rd = |r: &[u8], i: usize| u16::from_ne_bytes([r[2 * i], r[2 * i + 1]]);
    for row in f.texture.chunks(stride).take(h) {
        match f.memory_format {
            MemoryFormat::R8g8b8a8 => out.extend_from_slice(&row[..w * 4]),
            MemoryFormat::R8g8b8 => for p in row[..w * 3].chunks_exact(3) { out.extend([p[0], p[1], p[2], 255]) },
            MemoryFormat::G8 => for &g in &row[..w] { out.extend([g, g, g, 255]) },
            MemoryFormat::G8a8 => for p in row[..w * 2].chunks_exact(2) { out.extend([p[0], p[0], p[0], p[1]]) },
            MemoryFormat::R16g16b16a16 => { is16 = true; for i in 0..w * 4 { o16.push(rd(row, i)) } }
            MemoryFormat::R16g16b16 => { is16 = true; for x in 0..w { o16.extend([rd(row, 3 * x), rd(row, 3 * x + 1), rd(row, 3 * x + 2), 65535]) } }
            MemoryFormat::G16 => { is16 = true; for x in 0..w { let g = rd(row, x); o16.extend([g, g, g, 65535]) } }
            MemoryFormat::G16a16 => { is16 = true; for x in 0..w { let g = rd(row, 2 * x); o16.extend([g, g, g, rd(row, 2 * x + 1)]) } }
            other => return Err(format!("UnsupportedMemoryFormat {other:?}")),
        }
    }
    if is16 {
        let round: Vec<u8> = o16.iter().map(|&v| ((v as u32 * 255 * 2 + 65535) / (2 * 65535)) as u8).collect();
        let hi: Vec<u8> = o16.iter().map(|&v| (v >> 8) as u8).collect();
        Ok((round, hi, Some(o16)))
    } else {
        Ok((out.clone(), out, None))
    }
}

fn glycin(bytes: &[u8], mime: &str) -> String {
    let panicked = |e: &ProcessError| e.to_string().contains("panicked");
    let (mut loader, _) = match glycin_png::ImgLoader::load::<LocalMemory, _>(Cursor::new(bytes.to_vec()), mime.into(), InitializationDetails::default()) {
        Ok(v) => v,
        Err(e) => return err("error", class(&e), &e.to_string()),
    };
    let (mut w, mut h, mut n) = (0u32, 0u32, 0usize);
    let (mut first, mut first_hi, mut r16, mut memfmt, mut icc) = (vec![], vec![], None, String::new(), -1i64);
    let mut seq = Sha256::new();
    let mut delays = vec![];
    let mut tail = String::new();
    loop {
        let mut req = FrameRequest::default();
        req.loop_animation = false;
        let f = match loader.specific_frame::<LocalMemory>(req) {
            Ok(f) => f,
            Err(ProcessError::NoMoreFrames) if n > 0 => break,
            Err(e) => {
                if n == 0 {
                    return err(if panicked(&e) { "crash" } else { "error" }, class(&e), &e.to_string());
                }
                // glycin's worker exits after a one-frame APNG: the next request fails with InternalError.
                if panicked(&e) { tail = "glycin_worker_panic".into() } else if n > 1 || !matches!(e, ProcessError::InternalError { .. }) { tail = format!("{}: {}", class(&e), esc(&e.to_string())) }
                break;
            }
        };
        if n == 0 {
            w = f.width; h = f.height; memfmt = format!("{:?}", f.memory_format);
            icc = f.details.color_icc_profile.as_ref().map(|p| p.len() as i64).unwrap_or(-1);
        }
        if (f.width, f.height) != (w, h) { tail = "FrameSizeChanged".into(); break }
        let (r8, hi, x16) = match rgba(&f) { Ok(x) => x, Err(e) => return err("error", "harness", &e) };
        seq.update(&r8);
        if n == 0 { first = r8; first_hi = hi; r16 = x16 }
        let d: Option<std::time::Duration> = f.delay.clone().into();
        delays.push(d.map(|d| d.as_millis() as i64).unwrap_or(-1));
        n += 1;
        if n >= MAX_FRAMES { break }
        if d.is_none() && f.details.n_frame.unwrap_or(0) == 0 { break } // still image
    }
    let s16 = r16.map(|v| { let mut s = Sha256::new(); for x in v { s.update(x.to_be_bytes()) } format!(",\"rgba16\":\"{:x}\"", s.finalize()) }).unwrap_or_default();
    format!("{{\"status\":\"ok\",\"w\":{w},\"h\":{h},\"frames\":{n},\"sha\":\"{}\",\"sha_hi\":\"{}\",\"seq\":\"{:x}\",\"memfmt\":\"{memfmt}\",\"icc\":{icc},\"delays_ms\":[{}]{s16},\"tail_error\":\"{}\"}}",
            sha(&first), sha(&first_hi), seq.finalize(), delays.iter().take(16).map(|x| x.to_string()).collect::<Vec<_>>().join(","), esc(&tail))
}

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 3 { println!("{}", err("error", "usage", "png-glycin-probe glycin|glycin-apng FILE")); return }
    let b = match std::fs::read(&a[2]) { Ok(b) => b, Err(e) => { println!("{}", err("error", "io", &e.to_string())); return } };
    let out = std::panic::catch_unwind(|| match a[1].as_str() {
        "glycin" => glycin(&b, "image/png"),
        "glycin-apng" => glycin(&b, "image/apng"),
        s => err("error", "usage", &format!("unknown selector {s}")),
    }).unwrap_or_else(|_| err("crash", "panic", "rust panic in loader thread"));
    println!("{out}");
}
