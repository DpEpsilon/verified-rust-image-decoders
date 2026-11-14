//! usage: png-policy-probe SELECTOR FILE      -> one JSON line
//! Selectors:
//!   image        image::codecs::png::PngDecoder; APNG (is_apng) through apng().into_frames(),
//!                otherwise DynamicImage::from_decoder(..).to_rgba8() (image's own 16->8 rule);
//!                rgba16_sha256 from to_rgba16() when the source has 16-bit samples.
//!   image-still  the same decoder, never the APNG path (what image::load_from_memory returns).
//!   image-load   image::load_from_memory (format guessing, default Limits: 512 MiB allocation cap).
//!   image-icc    PngDecoder::icc_profile() (Ok(Some(len)) / Ok(None) / Err), then the still decode.
//!   pngcrate     png::Decoder, default limits, Transformations::EXPAND, first next_frame only,
//!                plus the metadata the crate exposes (gamma, chrm, srgb, iccp, text counts, actl).
//! Output: status, w, h, frames, sha (RGBA8 of the first frame), seq (all frames), rgba16, info, error.
use image::{AnimationDecoder, ImageDecoder};
use sha2::{Digest, Sha256};
use std::io::Cursor;

fn esc(s: &str) -> String {
    s.chars().flat_map(|c| match c { '"' => vec!['\\', '"'], '\\' => vec!['\\', '\\'], c if (c as u32) < 32 => vec![' '], c => vec![c] }).collect()
}
fn hex(v: &[u8]) -> String { v.iter().map(|b| format!("{b:02x}")).collect() }
fn sha(v: &[u8]) -> String { format!("{:x}", Sha256::digest(v)) }
fn err(e: impl std::fmt::Display) -> String {
    let s = e.to_string();
    format!("{{\"status\":\"error\",\"error\":\"{}\"}}", esc(&s.chars().take(300).collect::<String>()))
}
fn ok(w: u32, h: u32, frames: &[Vec<u8>], extra: &str) -> String {
    let first = &frames[0];
    let mut seq = Sha256::new();
    for f in frames { seq.update(f) }
    let small = if first.len() <= 1024 { hex(first) } else { String::new() };
    format!("{{\"status\":\"ok\",\"w\":{w},\"h\":{h},\"frames\":{},\"sha\":\"{}\",\"seq\":\"{:x}\",\"rgba\":\"{small}\"{extra}}}",
            frames.len(), sha(first), seq.finalize())
}

fn image_sel(b: &[u8], allow_apng: bool) -> String {
    let d = match image::codecs::png::PngDecoder::new(Cursor::new(b)) { Ok(d) => d, Err(e) => return err(e) };
    let ct = format!("{:?}", d.color_type());
    let apng = allow_apng && d.is_apng().unwrap_or(false);
    if apng {
        let a = match d.apng() { Ok(a) => a, Err(e) => return err(e) };
        let mut frames = vec![]; let mut delays = vec![]; let (mut w, mut h) = (0, 0); let mut tail = String::new();
        for f in a.into_frames() {
            match f {
                Ok(f) => { let (n, dd) = f.delay().numer_denom_ms(); delays.push(format!("[{n},{dd}]"));
                           let buf = f.into_buffer(); w = buf.width(); h = buf.height(); frames.push(buf.into_raw());
                           if frames.len() >= 256 { break } }
                Err(e) => { tail = esc(&e.to_string()); break }
            }
        }
        if frames.is_empty() { return err(if tail.is_empty() { "no frames".into() } else { tail }) }
        let extra = format!(",\"apng\":true,\"color\":\"{ct}\",\"delays_ms\":[{}],\"tail_error\":\"{tail}\"", delays.join(","));
        return ok(w, h, &frames, &extra);
    }
    let im = match image::DynamicImage::from_decoder(d) { Ok(i) => i, Err(e) => return err(e) };
    let r8 = im.to_rgba8();
    let extra = if matches!(im.color(), image::ColorType::L16 | image::ColorType::La16 | image::ColorType::Rgb16 | image::ColorType::Rgba16) {
        let v: Vec<u8> = im.to_rgba16().into_raw().iter().flat_map(|x| x.to_be_bytes()).collect();
        format!(",\"color\":\"{ct}\",\"rgba16\":\"{}\",\"rgba16hex\":\"{}\"", sha(&v), if v.len() <= 1024 { hex(&v) } else { String::new() })
    } else { format!(",\"color\":\"{ct}\"") };
    ok(r8.width(), r8.height(), &[r8.into_raw()], &extra)
}

fn image_load(b: &[u8]) -> String {
    match image::load_from_memory(b) {
        Ok(im) => { let r8 = im.to_rgba8(); ok(r8.width(), r8.height(), &[r8.into_raw()], &format!(",\"color\":\"{:?}\"", im.color())) }
        Err(e) => err(e),
    }
}

fn image_icc(b: &[u8]) -> String {
    let mut d = match image::codecs::png::PngDecoder::new(Cursor::new(b)) { Ok(d) => d, Err(e) => return err(e) };
    let icc = match d.icc_profile() { Ok(Some(p)) => format!("{}", p.len()), Ok(None) => "\"none\"".into(), Err(e) => format!("\"err: {}\"", esc(&e.to_string())) };
    let still = match image::DynamicImage::from_decoder(d) { Ok(im) => { let r8 = im.to_rgba8(); format!("\"ok:{}\"", sha(r8.as_raw())) } Err(e) => format!("\"err: {}\"", esc(&e.to_string())) };
    format!("{{\"status\":\"ok\",\"icc\":{icc},\"still\":{still}}}")
}

fn pngcrate(b: &[u8]) -> String {
    let mut d = png::Decoder::new(Cursor::new(b));
    d.set_transformations(png::Transformations::EXPAND);
    let mut r = match d.read_info() { Ok(r) => r, Err(e) => return err(e) };
    let size = match r.output_buffer_size() { Some(s) if s <= 1 << 28 => s, _ => return err("harness: output buffer too large") };
    let mut buf = vec![0; size];
    let out = match r.next_frame(&mut buf) { Ok(o) => o, Err(e) => return err(e) };
    let info = r.info();
    let meta = format!(",\"info\":{{\"color\":\"{:?}\",\"depth\":{},\"gamma\":\"{:?}\",\"srgb\":\"{:?}\",\"chrm\":{},\"iccp\":{},\"text\":[{},{},{}],\"actl\":\"{:?}\",\"trns\":{},\"line\":{},\"outdepth\":\"{:?}\",\"outcolor\":\"{:?}\",\"raw\":\"{}\"}}",
        info.color_type, info.bit_depth as u8, info.source_gamma, info.srgb, info.source_chromaticities.is_some(),
        info.icc_profile.as_ref().map(|p| p.len() as i64).unwrap_or(-1),
        info.uncompressed_latin1_text.len(), info.compressed_latin1_text.len(), info.utf8_text.len(),
        info.animation_control, info.trns.as_ref().map(|t| t.len() as i64).unwrap_or(-1), out.line_size,
        out.bit_depth, out.color_type, if out.buffer_size() <= 512 { hex(&buf[..out.buffer_size()]) } else { sha(&buf[..out.buffer_size()]) });
    // finish the stream so errors after the image data are visible as tail_error
    let tail = match r.finish() { Ok(()) => String::new(), Err(e) => esc(&e.to_string()) };
    format!("{{\"status\":\"ok\",\"w\":{},\"h\":{}{meta},\"tail_error\":\"{tail}\"}}", out.width, out.height)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 3 { println!("{}", err("usage: png-policy-probe SELECTOR FILE")); return }
    let b = match std::fs::read(&a[2]) { Ok(b) => b, Err(e) => { println!("{}", err(e)); return } };
    let out = std::panic::catch_unwind(|| match a[1].as_str() {
        "image" => image_sel(&b, true),
        "image-still" => image_sel(&b, false),
        "image-load" => image_load(&b),
        "image-icc" => image_icc(&b),
        "pngcrate" => pngcrate(&b),
        s => err(format!("unknown selector {s}")),
    }).unwrap_or_else(|_| err("panic"));
    println!("{out}");
}
