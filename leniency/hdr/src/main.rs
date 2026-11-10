// Radiance HDR leniency runner. usage: runner SELECTOR FILE [F32_DUMP]
// selectors: image (HdrDecoder::new, strict), image-nonstrict (new_nonstrict), glycin, radiant, hdrldr
// Output: decoded linear float RGB exactly as the library returns it (no tone mapping, no reorientation,
// no exposure/colorcorr applied by us). rgba_sha256 = SHA-256 over little-endian f32 bit patterns of R,G,B per pixel.
// The optional dump file receives those raw f32 LE samples (3 per pixel).
use std::{io::{BufReader, Cursor}, path::Path};
use glycin_utils::{FrameRequest, InitializationDetails, LoaderImplementation, LocalMemory, MemoryFormat, ProcessError};
use image::ImageDecoder;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MAX_PIXELS: u64 = 16_000_000;

fn ok(w: u32, h: u32, f: Vec<f32>, orig: String, dump: Option<&str>) -> Value {
    if w == 0 || h == 0 || w as u64 * h as u64 > MAX_PIXELS || f.len() != w as usize * h as usize * 3 {
        return err_class("HarnessLimit", "invalid output length or dimensions (zero, >16M pixels, or length mismatch)");
    }
    let mut bytes = Vec::with_capacity(f.len() * 4);
    for x in &f { bytes.extend_from_slice(&x.to_le_bytes()); }
    if let Some(p) = dump { if let Err(e) = std::fs::write(p, &bytes) { return err(e); } }
    json!({"status":"ok","width":w,"height":h,"orig_color_type":orig,"channels":3,
           "alpha_present":false,"nan_inf":f.iter().filter(|x| !x.is_finite()).count(),
           "rgba_sha256":format!("{:x}",Sha256::digest(&bytes))})
}
fn err_class(c: &str, m: &str) -> Value { json!({"status":"error","error_class":c,"message":m}) }
fn err(e: impl std::fmt::Display) -> Value {
    let s = e.to_string();
    json!({"status":"error","error_class":s.split([':','(']).next().unwrap_or("error").trim().chars().take(100).collect::<String>(),
           "message":s.chars().take(300).collect::<String>()})
}
fn glycin_error(e: ProcessError) -> Value {
    let class = match &e {
        ProcessError::ExpectedError { .. } => "ExpectedError", ProcessError::InternalError { .. } => "InternalError",
        ProcessError::UnsupportedImageFormat(_) => "UnsupportedImageFormat", ProcessError::ConversionTooLargerError => "ConversionTooLarge",
        ProcessError::OutOfMemory { .. } => "OutOfMemory", ProcessError::NoMoreFrames => "NoMoreFrames", _ => "OtherProcessError" };
    json!({"status":"error","error_class":class,"message":e.to_string().chars().take(300).collect::<String>()})
}
fn run(which: &str, bytes: &[u8], dump: Option<&str>) -> Value {
    match which {
        "image" | "image-nonstrict" => {
            let d = if which == "image" { image::codecs::hdr::HdrDecoder::new(Cursor::new(bytes)) } else { image::codecs::hdr::HdrDecoder::new_nonstrict(Cursor::new(bytes)) };
            let d = match d { Ok(d) => d, Err(e) => return err(e) };
            let (w, h) = d.dimensions();
            if w as u64 * h as u64 > MAX_PIXELS { return err_class("HarnessLimit", "dimensions over harness limit"); }
            let orig = format!("{:?}", d.color_type());
            let mut buf = vec![0u8; d.total_bytes() as usize];
            if let Err(e) = d.read_image(&mut buf) { return err(e); }
            let f: Vec<f32> = buf.chunks_exact(4).map(|c| f32::from_ne_bytes([c[0], c[1], c[2], c[3]])).collect();
            ok(w, h, f, orig, dump)
        }
        "glycin" => {
            let (mut loader, _) = match glycin_image_rs::ImgLoader::load::<LocalMemory, _>(Cursor::new(bytes.to_vec()), "image/vnd.radiance".into(), InitializationDetails::default()) { Ok(v) => v, Err(e) => return glycin_error(e) };
            let frame = match loader.specific_frame::<LocalMemory>(FrameRequest::default()) { Ok(f) => f, Err(e) => return glycin_error(e) };
            let (w, h) = (frame.width, frame.height);
            if w == 0 || h == 0 || w as u64 * h as u64 > MAX_PIXELS { return err_class("HarnessLimit", "output dimensions outside harness limit"); }
            let fmt = frame.memory_format; let stride = frame.stride as usize;
            let (pix, keep) = match fmt { MemoryFormat::R32g32b32Float => (12, 3), MemoryFormat::R32g32b32a32Float => (16, 3), _ => return err(format!("unsupported output memory format {fmt:?}")) };
            let mut f = Vec::with_capacity(w as usize * h as usize * 3);
            for row in frame.texture.chunks(stride).take(h as usize) {
                if row.len() < w as usize * pix { return err("short row"); }
                for p in row[..w as usize * pix].chunks_exact(pix) { for c in 0..keep { f.push(f32::from_ne_bytes([p[c*4], p[c*4+1], p[c*4+2], p[c*4+3]])); } }
            }
            ok(w, h, f, format!("{fmt:?} cicp={:?}", frame.details.color_cicp), dump)
        }
        "radiant" => match radiant::load(BufReader::new(Cursor::new(bytes))) {
            Ok(im) => ok(im.width as u32, im.height as u32, im.data.iter().flat_map(|p| [p.r, p.g, p.b]).collect(), "Rgb<f32> (radiant)".into(), dump),
            Err(e) => err(e) },
        "hdrldr" => match hdrldr::load(Cursor::new(bytes)) {
            Ok(im) => ok(im.width as u32, im.height as u32, im.data.iter().flat_map(|p| [p.r, p.g, p.b]).collect(), "RGB<f32> (hdrldr)".into(), dump),
            Err(e) => err(format!("{e:?}")) },
        _ => err("unknown selector"),
    }
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let result = std::panic::catch_unwind(|| {
        if a.len() < 3 { return err("usage: runner SELECTOR FILE [DUMP]"); }
        match std::fs::read(Path::new(&a[2])) { Ok(b) => run(&a[1], &b, a.get(3).map(String::as_str)), Err(e) => err(e) }
    }).unwrap_or_else(|_| json!({"status":"crash","error_class":"rust_panic"}));
    println!("{result}");
}
