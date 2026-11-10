//! Rust decoding columns for the JPEG and TIFF leniency adapters.
//! usage: leniency-jt-runner FORMAT:DECODER FILE [RGBA_DUMP]
//! Every column prints one JSON line; pixels are hashed as straight, top-first RGBA8
//! *before* any EXIF/TIFF orientation; `oriented_*` fields describe the orientation
//! glycin-core would apply (glycin columns only).
use std::io::Cursor;
use glycin_utils::{FrameRequest, FungibleMemory, InitializationDetails, LoaderImplementation, MemoryFormat, MemoryFormatInfo, ProcessError};
use image::ImageDecoder;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const LIMIT: u64 = 16_000_000;

fn ok(w: u32, h: u32, rgba: Vec<u8>, orig: String, extra: Value, dump: Option<&str>) -> Value {
    if w == 0 || h == 0 || w as u64 * h as u64 > LIMIT || rgba.len() != w as usize * h as usize * 4 {
        return err("invalid output length or dimensions");
    }
    if let Some(p) = dump { if let Err(e) = std::fs::write(p, &rgba) { return err(e) } }
    let mut v = json!({"status":"ok","width":w,"height":h,"orig_color_type":orig,
        "alpha_present":rgba.chunks_exact(4).any(|p| p[3] != 255),
        "rgba_sha256":format!("{:x}", Sha256::digest(&rgba))});
    if let (Value::Object(m), Value::Object(e)) = (&mut v, extra) { m.extend(e) }
    v
}
fn err(e: impl std::fmt::Display) -> Value {
    let s = e.to_string();
    json!({"status":"error","error_class":s.split([':', '(', '\n']).next().unwrap_or("error").trim().chars().take(80).collect::<String>(),
           "message":s.chars().take(400).collect::<String>()})
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
    // Drop the source-location prefix ("path/file.rs:L:C: ") so equal causes cluster together.
    let msg = e.to_string();
    let detail = msg.find(".rs:").and_then(|i| msg[i..].find(": ").map(|j| &msg[i + j + 2..])).unwrap_or(&msg);
    let short: String = detail.trim_matches(|c| c == '"' || c == ' ').split(['\n', '(', ',']).next().unwrap_or("").chars().take(60).collect();
    json!({"status":"error","error_class":format!("{class}: {short}"),"message":msg.chars().take(400).collect::<String>()})
}
fn tight(frame: &glycin_utils::Frame<FungibleMemory>) -> Vec<u8> {
    let (w, h, s) = (frame.width as usize, frame.height as usize, frame.stride as usize);
    frame.texture.chunks(s).take(h).flat_map(|r| r[..w * 4].iter().copied()).collect()
}

/// glycin-image-rs loader, then glycin's own `change_memory_format` to R8g8b8a8
/// (its CMYK/16-bit/float/premultiplied conversion); orientation as glycin-core derives it.
fn glycin(mime: &str, bytes: &[u8], dump: Option<&str>) -> Value {
    let (mut loader, details) = match glycin_image_rs::ImgLoader::load::<FungibleMemory, _>(
        Cursor::new(bytes.to_vec()), mime.into(), InitializationDetails::default()) {
        Ok(v) => v, Err(e) => return glycin_error(e) };
    let mut frame = match loader.specific_frame::<FungibleMemory>(FrameRequest::default()) {
        Ok(f) => f, Err(e) => return glycin_error(e) };
    if frame.width == 0 || frame.height == 0 || frame.width as u64 * frame.height as u64 > LIMIT {
        return err("output dimensions outside harness limit");
    }
    let src = frame.memory_format;
    if let Err(e) = glycin_utils::editing::change_memory_format(&mut frame, MemoryFormat::R8g8b8a8) {
        return err(format!("change_memory_format {src:?}: {e}"));
    }
    let (w, h) = (frame.width, frame.height);
    let rgba = tight(&frame);
    let orientation = details.transformation_orientation.or_else(|| details.metadata_exif.as_ref()
        .and_then(|x| gufo_exif::Exif::for_vec(x.to_vec()).ok()).and_then(|x| x.orientation()));
    let o = orientation.unwrap_or(gufo_common::orientation::Orientation::Id);
    let oriented = glycin_utils::editing::change_orientation(frame, o);
    let orgba = tight(&oriented);
    ok(w, h, rgba, format!("{src:?}"),
       json!({"orientation":format!("{o:?}"),"orientation_applied":false,"oriented_width":oriented.width,
              "oriented_height":oriented.height,"oriented_sha256":format!("{:x}", Sha256::digest(&orgba)),
              "bit_depth":src.channel_type().size()*8,"components":src.n_channels()}), dump)
}

fn image_rs(fmt: image::ImageFormat, bytes: &[u8], dump: Option<&str>) -> Value {
    let mut dec: Box<dyn ImageDecoder> = match fmt {
        image::ImageFormat::Jpeg => match image::codecs::jpeg::JpegDecoder::new(Cursor::new(bytes)) { Ok(d) => Box::new(d), Err(e) => return err(e) },
        _ => match image::codecs::tiff::TiffDecoder::new(Cursor::new(bytes)) { Ok(d) => Box::new(d), Err(e) => return err(e) },
    };
    let orig = format!("{:?}", dec.original_color_type());
    let orientation = dec.orientation().map(|o| format!("{o:?}")).unwrap_or_else(|e| format!("error {e}"));
    let (w, h) = dec.dimensions();
    if w as u64 * h as u64 > LIMIT { return err("output dimensions outside harness limit") }
    match image::DynamicImage::from_decoder(dec) {
        Ok(d) => ok(w, h, d.to_rgba8().into_raw(), orig, json!({"orientation":orientation,"orientation_applied":false}), dump),
        Err(e) => err(e),
    }
}

fn jpeg_decoder(bytes: &[u8], dump: Option<&str>) -> Value {
    let mut d = jpeg_decoder::Decoder::new(Cursor::new(bytes));
    let px = match d.decode() { Ok(p) => p, Err(e) => return err(e) };
    let Some(info) = d.info() else { return err("no info after decode") };
    let (w, h) = (info.width as u32, info.height as u32);
    if w as u64 * h as u64 > LIMIT { return err("output dimensions outside harness limit") }
    let n = w as usize * h as usize;
    use jpeg_decoder::PixelFormat::*;
    let need = n * info.pixel_format.pixel_bytes();
    if px.len() < need { return err("short pixel buffer") }
    let mut rgba = Vec::with_capacity(n * 4);
    for i in 0..n {
        match info.pixel_format {
            L8 => { let g = px[i]; rgba.extend([g, g, g, 255]) }
            // jpeg-decoder returns 16-bit samples big-endian; scale with rounding.
            L16 => { let v = u16::from_be_bytes([px[2 * i], px[2 * i + 1]]) as u32; let g = ((v * 255 + 32767) / 65535) as u8; rgba.extend([g, g, g, 255]) }
            RGB24 => rgba.extend([px[3 * i], px[3 * i + 1], px[3 * i + 2], 255]),
            // jpeg-decoder already un-inverts Adobe CMYK: 0 means no ink.
            CMYK32 => { let p = &px[4 * i..4 * i + 4]; let k = 255 - p[3] as u32;
                        for c in 0..3 { rgba.push((((255 - p[c] as u32) * k + 127) / 255) as u8) } rgba.push(255) }
        }
    }
    ok(w, h, rgba, format!("{:?}/{:?}", info.pixel_format, info.coding_process),
       json!({"orientation":"not-parsed","components":info.pixel_format.pixel_bytes(),"bit_depth":if info.pixel_format == L16 {16} else {8}}), dump)
}

/// tiff crate used directly, with the harness's own documented sample normalisation:
/// unsigned n-bit -> round(v*255/(2^n-1)); float clamp(0,1)*255; signed: (v - min) scaled;
/// MinIsWhite is left to the crate; palette via ColorMap>>8; CMYK naive (ink) conversion;
/// YCbCr JFIF full-range; planar images interleaved from all returned planes.
fn tiff_crate(bytes: &[u8], dump: Option<&str>) -> Value {
    use tiff::{decoder::{Decoder, DecodingResult}, tags::Tag, ColorType};
    let mut d = match Decoder::new(Cursor::new(bytes)) { Ok(d) => d, Err(e) => return err(e) };
    let (w, h) = match d.dimensions() { Ok(v) => v, Err(e) => return err(e) };
    if w == 0 || h == 0 || w as u64 * h as u64 > LIMIT { return err("output dimensions outside harness limit") }
    let ct = match d.colortype() { Ok(c) => c, Err(e) => return err(e) };
    let orientation = d.get_tag_u32(Tag::Orientation).ok();
    let colormap = d.get_tag_u16_vec(Tag::ColorMap).ok();
    let mut buf = DecodingResult::U8(vec![]);
    let layout = match d.read_image_to_buffer(&mut buf) { Ok(l) => l, Err(e) => return err(e) };
    let bits = ct.bit_depth() as u32;
    let spp = ct.num_samples() as usize;
    let n = w as usize * h as usize;
    // Samples normalised to 0..=1 (palette: raw index), row-unpacked for sub-byte depths.
    let samples: Vec<f64> = match &buf {
        DecodingResult::U8(v) if bits < 8 => {
            let row_bits = w as usize * spp * bits as usize; let row_bytes = row_bits.div_ceil(8);
            let planes = layout.planes.max(1); let per_plane_spp = if planes > 1 { 1 } else { spp };
            let rb = (w as usize * per_plane_spp * bits as usize).div_ceil(8);
            let _ = row_bytes;
            let mut out = Vec::with_capacity(n * spp);
            for p in 0..planes { for y in 0..h as usize { for s in 0..w as usize * per_plane_spp {
                let bit = s * bits as usize; let byte = p * rb * h as usize + y * rb + bit / 8;
                let Some(&b) = v.get(byte) else { return err("short sub-byte buffer") };
                let val = (b >> (8 - bits as usize - bit % 8)) & ((1u16 << bits) - 1) as u8;
                out.push(if matches!(ct, ColorType::Palette(_)) { val as f64 } else { val as f64 / ((1u32 << bits) - 1) as f64 });
            }}}
            out
        }
        DecodingResult::U8(v) => v.iter().map(|&x| if matches!(ct, ColorType::Palette(_)) { x as f64 } else { x as f64 / 255.0 }).collect(),
        DecodingResult::U16(v) => v.iter().map(|&x| if matches!(ct, ColorType::Palette(_)) { x as f64 } else { x as f64 / ((1u64 << bits.min(16)) - 1) as f64 }).collect(),
        DecodingResult::U32(v) => v.iter().map(|&x| x as f64 / u32::MAX as f64).collect(),
        DecodingResult::U64(v) => v.iter().map(|&x| x as f64 / u64::MAX as f64).collect(),
        DecodingResult::F16(v) => v.iter().map(|x| x.to_f64()).collect(),
        DecodingResult::F32(v) => v.iter().map(|&x| x as f64).collect(),
        DecodingResult::F64(v) => v.clone(),
        DecodingResult::I8(v) => v.iter().map(|&x| (x as f64 + 128.0) / 255.0).collect(),
        DecodingResult::I16(v) => v.iter().map(|&x| (x as f64 + 32768.0) / 65535.0).collect(),
        DecodingResult::I32(v) => v.iter().map(|&x| (x as f64 + 2147483648.0) / 4294967295.0).collect(),
        DecodingResult::I64(v) => v.iter().map(|&x| x as f64 / i64::MAX as f64 * 0.5 + 0.5).collect(),
    };
    let planes = layout.planes.max(1);
    let get = |i: usize, c: usize| -> f64 {
        let idx = if planes > 1 { c * n + i } else { i * spp + c };
        samples.get(idx).copied().unwrap_or(f64::NAN)
    };
    if samples.len() < n * spp && planes == 1 { return err(format!("short buffer {} < {}", samples.len(), n * spp)) }
    if planes > 1 && planes < spp { return err(format!("only {planes} of {spp} planes returned")) }
    let q = |x: f64| -> u8 { if x.is_nan() { 0 } else { (x.clamp(0.0, 1.0) * 255.0).round() as u8 } };
    let mut rgba = Vec::with_capacity(n * 4);
    for i in 0..n {
        let px: [u8; 4] = match ct {
            ColorType::Gray(_) => { let g = q(get(i, 0)); [g, g, g, 255] }
            ColorType::GrayA(_) => { let g = q(get(i, 0)); [g, g, g, q(get(i, 1))] }
            ColorType::RGB(_) => [q(get(i, 0)), q(get(i, 1)), q(get(i, 2)), 255],
            ColorType::RGBA(_) => [q(get(i, 0)), q(get(i, 1)), q(get(i, 2)), q(get(i, 3))],
            ColorType::CMYK(_) | ColorType::CMYKA(_) => {
                let k = 1.0 - get(i, 3);
                let a = if matches!(ct, ColorType::CMYKA(_)) { q(get(i, 4)) } else { 255 };
                [q((1.0 - get(i, 0)) * k), q((1.0 - get(i, 1)) * k), q((1.0 - get(i, 2)) * k), a]
            }
            ColorType::YCbCr(_) => {
                let (y, cb, cr) = (get(i, 0) * 255.0, get(i, 1) * 255.0 - 128.0, get(i, 2) * 255.0 - 128.0);
                [q((y + 1.402 * cr) / 255.0), q((y - 0.344136 * cb - 0.714136 * cr) / 255.0), q((y + 1.772 * cb) / 255.0), 255]
            }
            ColorType::Palette(_) => {
                let Some(map) = &colormap else { return err("palette without ColorMap") };
                let idx = get(i, 0) as usize; let m = map.len() / 3;
                if idx >= m { return err(format!("palette index {idx} >= {m}")) }
                [(map[idx] >> 8) as u8, (map[m + idx] >> 8) as u8, (map[2 * m + idx] >> 8) as u8, 255]
            }
            // Multiband (e.g. MinIsBlack with an extra sample): first sample gray, second alpha;
            // three or more: first three RGB, fourth alpha. A harness convention, documented.
            ColorType::Multiband { num_samples: 2, .. } => { let g = q(get(i, 0)); [g, g, g, q(get(i, 1))] }
            ColorType::Multiband { num_samples, .. } if num_samples >= 3 =>
                [q(get(i, 0)), q(get(i, 1)), q(get(i, 2)), if num_samples >= 4 { q(get(i, 3)) } else { 255 }],
            other => return err(format!("harness normalisation unsupported for {other:?}")),
        };
        rgba.extend(px);
    }
    let mut pages = 1;
    while d.more_images() { if d.next_image().is_err() { break } pages += 1; if pages > 64 { break } }
    ok(w, h, rgba, format!("{ct:?}/{:?}/planes{planes}", layout.sample_format),
       json!({"orientation":orientation,"orientation_applied":false,"pages":pages,"bit_depth":bits,"components":spp}), dump)
}

fn run(which: &str, bytes: &[u8], dump: Option<&str>) -> Value {
    match which {
        "jpeg:glycin" => glycin("image/jpeg", bytes, dump),
        "tiff:glycin" => glycin("image/tiff", bytes, dump),
        "jpeg:image" => image_rs(image::ImageFormat::Jpeg, bytes, dump),
        "tiff:image" => image_rs(image::ImageFormat::Tiff, bytes, dump),
        "jpeg:jpeg-decoder" => jpeg_decoder(bytes, dump),
        "tiff:crate" => tiff_crate(bytes, dump),
        _ => err("unknown runner"),
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    std::panic::set_hook(Box::new(|_| {}));
    let out = std::panic::catch_unwind(|| {
        if args.len() < 3 { return err("usage: runner FORMAT:DECODER FILE [RGBA_DUMP]") }
        match std::fs::read(&args[2]) { Ok(b) => run(&args[1], &b, args.get(3).map(String::as_str)), Err(e) => err(e) }
    }).unwrap_or_else(|_| json!({"status":"crash","error_class":"rust_panic"}));
    println!("{out}");
}
