// EXR runners. Canonical dump: RGBA as little-endian f32, alpha 1.0 when the file has none,
// NaN payloads canonicalised to 0x7fc00000. Hash = SHA-256 of that dump. No tone mapping.
use std::{io::Cursor, path::Path};
use glycin_utils::{FrameRequest, InitializationDetails, LoaderImplementation, LocalMemory, MemoryFormat};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn err(e: impl std::fmt::Display) -> Value {
    let mut s = e.to_string();
    if s.starts_with('/') { if let Some(i) = s.find(".rs:") { s = s[i + 4..].trim_start_matches(|c: char| c.is_ascii_digit() || c == ':').trim_start().to_string(); } }
    json!({"status":"error","error_class":s.chars().take(50).collect::<String>(),"message":s.chars().take(300).collect::<String>()})
}
fn ok(w: u32, h: u32, px: Vec<f32>, orig: String, dump: Option<&str>) -> Value {
    if w == 0 || h == 0 || w as u64 * h as u64 > 4_000_000 || px.len() != w as usize * h as usize * 4 { return err("output dimensions outside harness limit"); }
    let mut bytes = Vec::with_capacity(px.len() * 4);
    for v in &px { let v = if v.is_nan() { f32::from_bits(0x7fc00000) } else { *v }; bytes.extend_from_slice(&v.to_le_bytes()); }
    if let Some(p) = dump { if let Err(e) = std::fs::write(p, &bytes) { return err(e); } }
    json!({"status":"ok","width":w,"height":h,"orig_color_type":orig,"alpha_present":px.chunks_exact(4).any(|p| p[3] != 1.0),"rgba_sha256":format!("{:x}",Sha256::digest(&bytes))})
}
fn half(b: [u8; 2]) -> f32 { exr::prelude::f16::from_ne_bytes(b).to_f32() }

fn glycin(bytes: &[u8], dump: Option<&str>) -> Value {
    let (mut l, _) = match glycin_image_rs::ImgLoader::load::<LocalMemory, _>(Cursor::new(bytes.to_vec()), "image/x-exr".into(), InitializationDetails::default()) { Ok(v) => v, Err(e) => return err(e) };
    let f = match l.specific_frame::<LocalMemory>(FrameRequest::default()) { Ok(f) => f, Err(e) => return err(e) };
    let (w, h) = (f.width as usize, f.height as usize);
    let (ch, sz, is16) = match f.memory_format {
        MemoryFormat::R16g16b16a16Float => (4, 2, true), MemoryFormat::R16g16b16Float => (3, 2, true),
        MemoryFormat::R32g32b32a32Float => (4, 4, false), MemoryFormat::R32g32b32Float => (3, 4, false),
        m => return err(format!("unexpected glycin memory format {m:?}")),
    };
    let stride = f.stride as usize; let mut px = Vec::with_capacity(w * h * 4);
    if w as u64 * h as u64 > 4_000_000 { return err("output dimensions outside harness limit"); }
    for row in f.texture.chunks(stride).take(h) {
        if row.len() < w * ch * sz { return err("short row"); }
        for p in row[..w * ch * sz].chunks_exact(ch * sz) {
            let s = |i: usize| if is16 { half([p[i * 2], p[i * 2 + 1]]) } else { f32::from_ne_bytes([p[i * 4], p[i * 4 + 1], p[i * 4 + 2], p[i * 4 + 3]]) };
            px.extend([s(0), s(1), s(2), if ch == 4 { s(3) } else { 1.0 }]);
        }
    }
    ok(w as u32, h as u32, px, format!("{:?}", f.memory_format), dump)
}
fn image_exr(bytes: &[u8], dump: Option<&str>) -> Value {
    use image::ImageDecoder;
    let orig = match image::codecs::openexr::OpenExrDecoder::new(Cursor::new(bytes)) { Ok(d) => format!("{:?}", d.color_type()), Err(e) => return err(e) };
    match image::load_from_memory_with_format(bytes, image::ImageFormat::OpenExr) {
        Ok(im) => { let r = im.to_rgba32f(); ok(r.width(), r.height(), r.into_raw(), orig, dump) }
        Err(e) => err(e),
    }
}
fn main() {
    std::env::set_var("RAYON_NUM_THREADS", "1"); // the engine caps address space; rayon's default pool cannot start under that cap
    let a: Vec<String> = std::env::args().collect();
    let r = std::panic::catch_unwind(|| {
        if a.len() < 3 { return err("usage: exr-runner SELECTOR FILE [DUMP]"); }
        let b = match std::fs::read(Path::new(&a[2])) { Ok(b) => b, Err(e) => return err(e) };
        match a[1].as_str() { "glycin" => glycin(&b, a.get(3).map(String::as_str)), "image-exr" => image_exr(&b, a.get(3).map(String::as_str)), _ => err("unknown selector") }
    }).unwrap_or_else(|_| json!({"status":"crash","error_class":"rust_panic"}));
    println!("{r}");
}
