// DDS leniency runner. usage: runner SELECTOR FILE [RGBA_DUMP]
// selectors: image, glycin, image-dds, ddsfile-bcdec, ddsfile-texpresso
// Output: first mip level, array layer 0, volume slice 0, straight RGBA8, top-first.
use std::{io::Cursor, path::Path};
use glycin_utils::{FrameRequest, InitializationDetails, LoaderImplementation, LocalMemory, MemoryFormat, ProcessError};
use image::ImageDecoder;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MAX_PIXELS: u64 = 16_000_000;

fn ok(w: u32, h: u32, rgba: Vec<u8>, orig: String, extra: Value, dump: Option<&str>) -> Value {
    if w == 0 || h == 0 || w as u64 * h as u64 > MAX_PIXELS || rgba.len() != w as usize * h as usize * 4 {
        return err_class("HarnessLimit", "invalid output length or dimensions");
    }
    if let Some(p) = dump { if let Err(e) = std::fs::write(p, &rgba) { return err(e); } }
    let mut v = json!({"status":"ok","width":w,"height":h,"orig_color_type":orig,
        "alpha_present":rgba.chunks_exact(4).any(|p| p[3] != 255),
        "rgba_sha256":format!("{:x}",Sha256::digest(&rgba))});
    if let (Some(o), Some(e)) = (v.as_object_mut(), extra.as_object()) { for (k, x) in e { o.insert(k.clone(), x.clone()); } }
    v
}
fn err_class(c: &str, m: &str) -> Value { json!({"status":"error","error_class":c,"message":m}) }
fn err(e: impl std::fmt::Display) -> Value {
    let s = e.to_string();
    json!({"status":"error","error_class":s.split([':','(']).next().unwrap_or("error").trim().chars().take(100).collect::<String>(),
           "message":s.chars().take(300).collect::<String>()})
}
fn glycin_error(e: ProcessError) -> Value {
    let class = match &e {
        ProcessError::ExpectedError { .. } => "ExpectedError",
        ProcessError::InternalError { .. } => "InternalError",
        ProcessError::UnsupportedImageFormat(_) => "UnsupportedImageFormat",
        ProcessError::ConversionTooLargerError => "ConversionTooLarge",
        ProcessError::OutOfMemory { .. } => "OutOfMemory",
        ProcessError::NoMoreFrames => "NoMoreFrames",
        _ => "OtherProcessError",
    };
    json!({"status":"error","error_class":class,"message":e.to_string().chars().take(300).collect::<String>()})
}
fn frame_result(frame: glycin_utils::Frame<LocalMemory>, extra: Value, dump: Option<&str>) -> Value {
    let (w, h) = (frame.width, frame.height);
    if w == 0 || h == 0 || w as u64 * h as u64 > MAX_PIXELS { return err_class("HarnessLimit", "output dimensions outside harness limit"); }
    let stride = frame.stride as usize; let fmt = frame.memory_format;
    let mut rgba = Vec::with_capacity(w as usize * h as usize * 4);
    for row in frame.texture.chunks(stride).take(h as usize) {
        match fmt {
            MemoryFormat::R8g8b8a8 if row.len() >= w as usize * 4 => rgba.extend_from_slice(&row[..w as usize * 4]),
            MemoryFormat::B8g8r8a8 if row.len() >= w as usize * 4 => for p in row[..w as usize * 4].chunks_exact(4) { rgba.extend([p[2], p[1], p[0], p[3]]) },
            MemoryFormat::R8g8b8 if row.len() >= w as usize * 3 => for p in row[..w as usize * 3].chunks_exact(3) { rgba.extend([p[0], p[1], p[2], 255]) },
            MemoryFormat::G8 if row.len() >= w as usize => for &g in &row[..w as usize] { rgba.extend([g, g, g, 255]) },
            MemoryFormat::G8a8 if row.len() >= w as usize * 2 => for p in row[..w as usize * 2].chunks_exact(2) { rgba.extend([p[0], p[0], p[0], p[1]]) },
            _ => return err(format!("unsupported output memory format {fmt:?}")),
        }
    }
    ok(w, h, rgba, format!("{fmt:?}"), extra, dump)
}

// ---------- ddsfile-based glue (our code: format mapping, mask expansion, layout) ----------
use ddsfile::{Dds, DxgiFormat, PixelFormatFlags, Caps2, MiscFlag};

enum Kind { Bc(u8), Mask { bits: u32, r: u32, g: u32, b: u32, a: u32, lum: bool, alpha_only: bool } }

fn mask(bits: u32, r: u32, g: u32, b: u32, a: u32) -> Kind { Kind::Mask { bits, r, g, b, a, lum: false, alpha_only: false } }

fn kind_of(d: &Dds) -> Result<(Kind, String), String> {
    if let Some(h10) = &d.header10 {
        use DxgiFormat::*;
        let f = h10.dxgi_format;
        let k = match f {
            BC1_Typeless | BC1_UNorm | BC1_UNorm_sRGB => Kind::Bc(1),
            BC2_Typeless | BC2_UNorm | BC2_UNorm_sRGB => Kind::Bc(2),
            BC3_Typeless | BC3_UNorm | BC3_UNorm_sRGB => Kind::Bc(3),
            BC4_Typeless | BC4_UNorm => Kind::Bc(4),
            BC5_Typeless | BC5_UNorm => Kind::Bc(5),
            BC7_Typeless | BC7_UNorm | BC7_UNorm_sRGB => Kind::Bc(7),
            R8G8B8A8_UNorm | R8G8B8A8_UNorm_sRGB | R8G8B8A8_Typeless => mask(32, 0xff, 0xff00, 0xff0000, 0xff000000),
            B8G8R8A8_UNorm | B8G8R8A8_UNorm_sRGB | B8G8R8A8_Typeless => mask(32, 0xff0000, 0xff00, 0xff, 0xff000000),
            B8G8R8X8_UNorm | B8G8R8X8_UNorm_sRGB | B8G8R8X8_Typeless => mask(32, 0xff0000, 0xff00, 0xff, 0),
            R8_UNorm | R8_Typeless => Kind::Mask { bits: 8, r: 0xff, g: 0, b: 0, a: 0, lum: true, alpha_only: false },
            A8_UNorm => Kind::Mask { bits: 8, r: 0, g: 0, b: 0, a: 0xff, lum: false, alpha_only: true },
            _ => return Err(format!("UnsupportedDxgi: {f:?}")),
        };
        return Ok((k, format!("DX10:{f:?}")));
    }
    let spf = &d.header.spf;
    if spf.flags.contains(PixelFormatFlags::FOURCC) {
        let cc = spf.fourcc.as_ref().map(|c| c.0.to_le_bytes()).unwrap_or([0; 4]);
        let name = String::from_utf8_lossy(&cc).to_string();
        let k = match &cc { b"DXT1" => Kind::Bc(1), b"DXT2" | b"DXT3" => Kind::Bc(2), b"DXT4" | b"DXT5" => Kind::Bc(3),
            b"ATI1" | b"BC4U" => Kind::Bc(4), b"ATI2" | b"BC5U" => Kind::Bc(5),
            _ => return Err(format!("UnsupportedFourCC: {name:?}")) };
        return Ok((k, format!("FourCC:{name}")));
    }
    let bits = spf.rgb_bit_count.unwrap_or(0);
    let (r, g, b, a) = (spf.r_bit_mask.unwrap_or(0), spf.g_bit_mask.unwrap_or(0), spf.b_bit_mask.unwrap_or(0), spf.a_bit_mask.unwrap_or(0));
    if !matches!(bits, 8 | 16 | 24 | 32) { return Err(format!("UnsupportedBitCount: {bits}")); }
    let has_a = spf.flags.contains(PixelFormatFlags::ALPHA_PIXELS);
    let desc = format!("Mask{bits}:{r:x}/{g:x}/{b:x}/{a:x}");
    if spf.flags.contains(PixelFormatFlags::RGB) {
        return Ok((mask(bits, r, g, b, if has_a { a } else { 0 }), desc));
    }
    if spf.flags.contains(PixelFormatFlags::LUMINANCE) {
        return Ok((Kind::Mask { bits, r, g: 0, b: 0, a: if has_a { a } else { 0 }, lum: true, alpha_only: false }, desc));
    }
    if spf.flags.contains(PixelFormatFlags::ALPHA) {
        return Ok((Kind::Mask { bits, r: 0, g: 0, b: 0, a, lum: false, alpha_only: true }, desc));
    }
    Err(format!("UnsupportedPixelFormatFlags: {:#x}", spf.flags.bits()))
}

fn chan(v: u32, m: u32) -> Option<u8> {
    if m == 0 { return None; }
    let sh = m.trailing_zeros(); let max = m >> sh;
    if max == 0 || (max & (max + 1)) != 0 { return Some(((v & m) >> sh).min(255) as u8); } // non-contiguous: raw shifted value
    Some((((v & m) >> sh) as u64 * 255 / max as u64) as u8) // truncating scale to 8 bits
}

fn dds_extra(d: &Dds) -> Value {
    let cube = d.header.caps2.contains(Caps2::CUBEMAP) || matches!(&d.header10, Some(h) if h.misc_flag.contains(MiscFlag::TEXTURECUBE));
    json!({"mips":d.get_num_mipmap_levels(),"layers":d.get_num_array_layers(),"cube":cube,"depth":d.get_depth()})
}

fn ddsfile_decode(bytes: &[u8], bc: &str, dump: Option<&str>) -> Value {
    let dds = match Dds::read(Cursor::new(bytes)) { Ok(d) => d, Err(e) => return err(format!("DdsfileRead: {e}")) };
    let (w, h) = (dds.get_width(), dds.get_height());
    let extra = dds_extra(&dds);
    if w == 0 || h == 0 { return err_class("ZeroDimension", "zero width or height"); }
    if w as u64 * h as u64 > MAX_PIXELS { return err_class("HarnessLimit", "dimensions over harness limit"); }
    let (kind, desc) = match kind_of(&dds) { Ok(k) => k, Err(e) => return err(e) };
    if bc == "texpresso" && !matches!(kind, Kind::Bc(1..=5)) { return json!({"status":"na","error_class":"UnsupportedByCandidate","message":format!("texpresso column covers BC1-BC5 only: {desc}")}); }
    let data = match dds.get_data(0) { Ok(d) => d, Err(e) => return err(format!("DdsfileLayout: {e}")) };
    let (wu, hu) = (w as usize, h as usize);
    let mut rgba = vec![0u8; wu * hu * 4];
    match kind {
        Kind::Bc(n) => {
            let bs = if matches!(n, 1 | 4) { 8 } else { 16 };
            let (bw, bh) = ((wu + 3) / 4, (hu + 3) / 4);
            if data.len() < bw * bh * bs { return err_class("Truncated", "mip level 0 data shorter than block count"); }
            if bc == "texpresso" {
                let f = match n { 1 => texpresso::Format::Bc1, 2 => texpresso::Format::Bc2, 3 => texpresso::Format::Bc3, 4 => texpresso::Format::Bc4, _ => texpresso::Format::Bc5 };
                f.decompress(&data[..bw * bh * bs], wu, hu, &mut rgba);
            } else {
                for by in 0..bh { for bx in 0..bw {
                    let blk = &data[(by * bw + bx) * bs..][..bs];
                    let mut out = [0u8; 64];
                    match n {
                        1 => bcdec_rs::bc1(blk, &mut out, 16), 2 => bcdec_rs::bc2(blk, &mut out, 16),
                        3 => bcdec_rs::bc3(blk, &mut out, 16), 7 => bcdec_rs::bc7(blk, &mut out, 16),
                        4 => { let mut t = [0u8; 16]; bcdec_rs::bc4(blk, &mut t, 4, false); for i in 0..16 { out[i*4..i*4+4].copy_from_slice(&[t[i], 0, 0, 255]); } }
                        _ => { let mut t = [0u8; 32]; bcdec_rs::bc5(blk, &mut t, 8, false); for i in 0..16 { out[i*4..i*4+4].copy_from_slice(&[t[i*2], t[i*2+1], 0, 255]); } }
                    }
                    for y in 0..4 { for x in 0..4 {
                        let (px, py) = (bx * 4 + x, by * 4 + y);
                        if px < wu && py < hu { rgba[(py * wu + px) * 4..][..4].copy_from_slice(&out[(y * 4 + x) * 4..][..4]); }
                    } }
                } }
            }
        }
        Kind::Mask { bits, r, g, b, a, lum, alpha_only } => {
            let bpp = (bits / 8) as usize;
            let pitch = match dds.get_pitch() { Some(p) => p as usize, None => wu * bpp };
            if pitch < wu * bpp { return err_class("PitchTooSmall", "pitch smaller than a row of pixels"); }
            if data.len() < pitch * (hu - 1) + wu * bpp { return err_class("Truncated", "mip level 0 data shorter than pitch*height"); }
            for y in 0..hu { for x in 0..wu {
                let px = &data[y * pitch + x * bpp..][..bpp];
                let mut v = 0u32; for (i, &c) in px.iter().enumerate() { v |= (c as u32) << (8 * i); }
                let o = &mut rgba[(y * wu + x) * 4..][..4];
                let av = chan(v, a).unwrap_or(255);
                if alpha_only { o.copy_from_slice(&[0, 0, 0, av]); }
                else if lum { let l = chan(v, r).unwrap_or(0); o.copy_from_slice(&[l, l, l, av]); }
                else { o.copy_from_slice(&[chan(v, r).unwrap_or(0), chan(v, g).unwrap_or(0), chan(v, b).unwrap_or(0), av]); }
            } }
        }
    }
    ok(w, h, rgba, desc, extra, dump)
}

fn image_dds_decode(bytes: &[u8], dump: Option<&str>) -> Value {
    use image_dds::{Surface, SurfaceRgba8};
    let dds = match Dds::read(Cursor::new(bytes)) { Ok(d) => d, Err(e) => return err(format!("DdsfileRead: {e}")) };
    let extra = dds_extra(&dds);
    let s = match Surface::from_dds(&dds) { Ok(s) => s, Err(e) => return err(format!("Surface: {e}")) };
    let (w, h) = (s.width, s.height);
    if w == 0 || h == 0 { return err_class("ZeroDimension", "zero width or height"); }
    if w as u64 * h as u64 * s.depth.max(1) as u64 > MAX_PIXELS { return err_class("HarnessLimit", "dimensions over harness limit"); }
    let fmt = format!("{:?}", s.image_format);
    let out: SurfaceRgba8<Vec<u8>> = match s.decode_layers_mipmaps_rgba8(0..1, 0..1) { Ok(o) => o, Err(e) => return err(format!("Decode: {e}")) };
    let n = w as usize * h as usize * 4;
    if out.data.len() < n { return err_class("ShortOutput", "decoded surface shorter than one slice"); }
    ok(w, h, out.data[..n].to_vec(), fmt, extra, dump)
}

fn run(which: &str, bytes: &[u8], dump: Option<&str>) -> Value {
    let policy = json!({"policy":"first-mip-only"});
    match which {
        "image" => {
            let d = match image::codecs::dds::DdsDecoder::new(Cursor::new(bytes)) { Ok(d) => d, Err(e) => return err(e) };
            let (w, h) = d.dimensions();
            if w as u64 * h as u64 > MAX_PIXELS { return err_class("HarnessLimit", "dimensions over harness limit"); }
            let orig = format!("{:?}", d.color_type());
            match image::DynamicImage::from_decoder(d) { Ok(im) => ok(im.width(), im.height(), im.to_rgba8().into_raw(), orig, policy, dump), Err(e) => err(e) }
        }
        "glycin" => {
            let (mut loader, _) = match glycin_image_rs::ImgLoader::load::<LocalMemory, _>(Cursor::new(bytes.to_vec()), "image/vnd.ms-dds".into(), InitializationDetails::default()) { Ok(v) => v, Err(e) => return glycin_error(e) };
            match loader.specific_frame::<LocalMemory>(FrameRequest::default()) { Ok(f) => frame_result(f, policy, dump), Err(e) => glycin_error(e) }
        }
        "image-dds" => image_dds_decode(bytes, dump),
        "ddsfile-bcdec" => ddsfile_decode(bytes, "bcdec", dump),
        "ddsfile-texpresso" => ddsfile_decode(bytes, "texpresso", dump),
        _ => err("unknown selector"),
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let result = std::panic::catch_unwind(|| {
        if a.len() < 3 { return err("usage: runner SELECTOR FILE [RGBA_DUMP]"); }
        match std::fs::read(Path::new(&a[2])) { Ok(b) => run(&a[1], &b, a.get(3).map(String::as_str)), Err(e) => err(e) }
    }).unwrap_or_else(|_| json!({"status":"crash","error_class":"rust_panic"}));
    println!("{result}");
}
