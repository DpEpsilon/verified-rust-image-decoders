//! GIF / PNG / APNG leniency runner. usage: runner FORMAT:SELECTOR FILE [RGBA_DUMP]
//!
//! One JSON line per input. Every selector feeds composited, top-first, straight
//! (non-premultiplied) RGBA8 canvases into `Acc`; 16-bit samples are reduced to
//! 8 bits by taking the high byte (v >> 8, the libpng `png_set_strip_16` rule) and
//! additionally hashed exactly as big-endian RGBA16 (`rgba16_sha256`) when the
//! decoder exposes 16-bit samples. No gamma, sRGB or ICC transform is applied by
//! any selector: hashes compare raw decoded samples.
use std::io::Cursor;
use glycin_utils::{FrameRequest, InitializationDetails, LoaderImplementation, LocalMemory, MemoryFormat, ProcessError};
use image::{AnimationDecoder, ImageDecoder};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MAX_FRAMES: usize = 64;
const MAX_PIXELS: u64 = 16_000_000;
const MAX_TOTAL: u64 = 64_000_000;

struct Acc { w: u32, h: u32, n: usize, capped: bool, first: Vec<u8>, first_hash: String, seq: Sha256,
             info: Vec<Value>, alpha: bool, rgba16: Option<String>, tail_error: Option<String> }

impl Acc {
    fn new(w: u32, h: u32) -> Result<Acc, Value> {
        if w == 0 || h == 0 || w as u64 * h as u64 > MAX_PIXELS { return Err(err("Dimensions: canvas outside harness limit")) }
        Ok(Acc { w, h, n: 0, capped: false, first: vec![], first_hash: String::new(), seq: Sha256::new(),
                 info: vec![], alpha: false, rgba16: None, tail_error: None })
    }
    /// Returns false when no more frames should be fed.
    fn frame(&mut self, rgba: &[u8], delay_ms: i64, disposal: i32, rect: (i64, i64, i64, i64), blend: i32) -> bool {
        if self.n >= MAX_FRAMES || (self.n as u64 + 1) * self.w as u64 * self.h as u64 > MAX_TOTAL { self.capped = true; return false }
        assert_eq!(rgba.len(), self.w as usize * self.h as usize * 4);
        self.seq.update(rgba);
        if self.n == 0 {
            self.first = rgba.to_vec();
            self.first_hash = format!("{:x}", Sha256::digest(rgba));
            self.alpha = rgba.chunks_exact(4).any(|p| p[3] != 255);
        }
        if self.n < 16 { self.info.push(json!([delay_ms, disposal, rect.0, rect.1, rect.2, rect.3, blend])) }
        self.n += 1;
        self.n < MAX_FRAMES
    }
    fn finish(self, orig: &str, selection: &str, dump: Option<&str>) -> Value {
        if self.n == 0 { return err("NoFrames: decoder produced no frame") }
        if let Some(p) = dump { if let Err(e) = std::fs::write(p, &self.first) { return err(e) } }
        let mut v = json!({"status":"ok","width":self.w,"height":self.h,"frames":self.n,"frames_capped":self.capped,
            "orig_color_type":orig,"alpha_present":self.alpha,"rgba_sha256":self.first_hash,
            "seq_sha256":format!("{:x}",self.seq.finalize()),"frame_info":self.info,"selection":selection});
        if let Some(h) = self.rgba16 { v["rgba16_sha256"] = json!(h) }
        if let Some(t) = self.tail_error { v["tail_error"] = json!(t) }
        v
    }
}

fn err(e: impl std::fmt::Display) -> Value {
    let s = e.to_string();
    json!({"status":"error","error_class":s.split([':', '(', '{']).next().unwrap_or("error").trim().chars().take(100).collect::<String>(),
           "message":s.chars().take(400).collect::<String>()})
}
fn dbg_err(e: impl std::fmt::Debug) -> Value { err(format!("{e:?}")) }
fn glycin_error(e: &ProcessError) -> String {
    match e {
        ProcessError::ExpectedError { .. } => "ExpectedError", ProcessError::InternalError { .. } => "InternalError",
        ProcessError::UnsupportedImageFormat(_) => "UnsupportedImageFormat", ProcessError::ConversionTooLargerError => "ConversionTooLarge",
        ProcessError::OutOfMemory { .. } => "OutOfMemory", ProcessError::NoMoreFrames => "NoMoreFrames", _ => "OtherProcessError",
    }.to_string()
}
fn hi16(v: &[u16]) -> Vec<u8> { v.iter().map(|x| (x >> 8) as u8).collect() }
fn hash16(v: &[u16]) -> String { let mut s = Sha256::new(); for x in v { s.update(x.to_be_bytes()) } format!("{:x}", s.finalize()) }
fn ms(num: u64, den: u64) -> i64 { if den == 0 { -1 } else { ((num * 1000 + den / 2) / den) as i64 } }

/// glycin texture -> RGBA8 (+ exact RGBA16 for 16-bit formats)
fn glycin_rgba(f: &glycin_utils::Frame<LocalMemory>) -> Result<(Vec<u8>, Option<Vec<u16>>), String> {
    let (w, h, stride) = (f.width as usize, f.height as usize, f.stride as usize);
    let mut out = Vec::with_capacity(w * h * 4); let mut out16: Vec<u16> = vec![]; let mut is16 = false;
    let rd = |r: &[u8], i: usize| u16::from_ne_bytes([r[2 * i], r[2 * i + 1]]);
    for row in f.texture.chunks(stride).take(h) {
        match f.memory_format {
            MemoryFormat::R8g8b8a8 => out.extend_from_slice(&row[..w * 4]),
            MemoryFormat::B8g8r8a8 => for p in row[..w * 4].chunks_exact(4) { out.extend([p[2], p[1], p[0], p[3]]) },
            MemoryFormat::R8g8b8 => for p in row[..w * 3].chunks_exact(3) { out.extend([p[0], p[1], p[2], 255]) },
            MemoryFormat::G8 => for &g in &row[..w] { out.extend([g, g, g, 255]) },
            MemoryFormat::G8a8 => for p in row[..w * 2].chunks_exact(2) { out.extend([p[0], p[0], p[0], p[1]]) },
            MemoryFormat::R16g16b16a16 => { is16 = true; for i in 0..w * 4 { out16.push(rd(row, i)) } }
            MemoryFormat::R16g16b16 => { is16 = true; for x in 0..w { out16.extend([rd(row, 3 * x), rd(row, 3 * x + 1), rd(row, 3 * x + 2), 65535]) } }
            MemoryFormat::G16 => { is16 = true; for x in 0..w { let g = rd(row, x); out16.extend([g, g, g, 65535]) } }
            MemoryFormat::G16a16 => { is16 = true; for x in 0..w { let g = rd(row, 2 * x); out16.extend([g, g, g, rd(row, 2 * x + 1)]) } }
            other => return Err(format!("UnsupportedMemoryFormat {other:?}")),
        }
    }
    if is16 { Ok((hi16(&out16), Some(out16))) } else { Ok((out, None)) }
}

fn glycin(bytes: &[u8], mime: &str, dump: Option<&str>) -> Value {
    // A panic inside glycin's animation worker thread reaches us as an error message; report it as a crash.
    let panic = |e: &ProcessError| e.to_string().contains("panicked");
    let (mut loader, _) = match glycin_image_rs::ImgLoader::load::<LocalMemory, _>(Cursor::new(bytes.to_vec()), mime.into(), InitializationDetails::default()) {
        Ok(v) => v, Err(e) => return json!({"status":"error","error_class":glycin_error(&e),"message":e.to_string().chars().take(400).collect::<String>()}) };
    let mut acc: Option<Acc> = None; let mut orig = String::new();
    loop {
        let mut req = FrameRequest::default(); req.loop_animation = false;
        let f = match loader.specific_frame::<LocalMemory>(req) {
            Ok(f) => f,
            Err(ProcessError::NoMoreFrames) if acc.is_some() => break,
            Err(e) => match acc.as_mut() {
                // glycin's animation thread exits after a single-frame image: the next request fails.
                Some(a) => { if panic(&e) { a.tail_error = Some("glycin_worker_panic".into()) } else if a.n > 1 || !matches!(e, ProcessError::InternalError { .. }) { a.tail_error = Some(glycin_error(&e)) } break }
                None if panic(&e) => return json!({"status":"crash","error_class":"glycin_worker_panic","message":e.to_string().chars().take(400).collect::<String>()}),
                None => return json!({"status":"error","error_class":glycin_error(&e),"message":e.to_string().chars().take(400).collect::<String>()}),
            },
        };
        if acc.is_none() { orig = format!("{:?}", f.memory_format); acc = Some(match Acc::new(f.width, f.height) { Ok(a) => a, Err(v) => return v }) }
        let a = acc.as_mut().unwrap();
        if (f.width, f.height) != (a.w, a.h) { a.tail_error = Some("FrameSizeChanged".into()); break }
        let (rgba, r16) = match glycin_rgba(&f) { Ok(x) => x, Err(e) => return err(e) };
        if a.n == 0 { a.rgba16 = r16.as_deref().map(hash16) }
        let d: Option<std::time::Duration> = f.delay.clone().into();
        let n_frame = f.details.n_frame;
        if !a.frame(&rgba, d.map(|d| d.as_millis() as i64).unwrap_or(-1), -1, (0, 0, f.width as i64, f.height as i64), -1) { break }
        if d.is_none() && n_frame.unwrap_or(0) == 0 { break } // still image: no further frames
    }
    acc.unwrap().finish(&orig, "frame-sequence", dump)
}

fn image_frames(frames: image::Frames, orig: &str, sel: &str, dump: Option<&str>) -> Value {
    let mut acc: Option<Acc> = None;
    for fr in frames {
        let fr = match fr { Ok(f) => f, Err(e) => match acc.as_mut() { Some(a) => { a.tail_error = Some(err(e)["error_class"].as_str().unwrap().into()); break } None => return err(e) } };
        let (n, d) = fr.delay().numer_denom_ms();
        let buf = fr.into_buffer();
        if acc.is_none() { acc = Some(match Acc::new(buf.width(), buf.height()) { Ok(a) => a, Err(v) => return v }) }
        let a = acc.as_mut().unwrap();
        if (buf.width(), buf.height()) != (a.w, a.h) { a.tail_error = Some("FrameSizeChanged".into()); break }
        if !a.frame(buf.as_raw(), if d == 0 { -1 } else { ((n as u64 + d as u64 / 2) / d as u64) as i64 }, -1, (0, 0, a.w as i64, a.h as i64), -1) { break }
    }
    match acc { Some(a) => a.finish(orig, sel, dump), None => err("NoFrames: decoder produced no frame") }
}

fn image_static(im: image::DynamicImage, orig: &str, sel: &str, dump: Option<&str>) -> Value {
    let mut a = match Acc::new(im.width(), im.height()) { Ok(a) => a, Err(v) => return v };
    let sixteen = matches!(im.color(), image::ColorType::L16 | image::ColorType::La16 | image::ColorType::Rgb16 | image::ColorType::Rgba16);
    if sixteen { let v = im.to_rgba16().into_raw(); a.rgba16 = Some(hash16(&v)); a.frame(&hi16(&v), -1, -1, (0, 0, a.w as i64, a.h as i64), -1); }
    else { a.frame(im.to_rgba8().as_raw(), -1, -1, (0, 0, a.w as i64, a.h as i64), -1); }
    a.finish(orig, sel, dump)
}

/// GIF89a reference compositor (same policy as runners/acc.h): transparent-black canvas,
/// disposal 2 clears the previous rectangle to transparent, 3 restores, others keep.
struct Comp { canvas: Vec<u8>, saved: Vec<u8>, pdisp: u8, prect: (i64, i64, i64, i64), w: i64, h: i64 }
impl Comp {
    fn new(w: u32, h: u32) -> Comp { let n = w as usize * h as usize * 4; Comp { canvas: vec![0; n], saved: vec![0; n], pdisp: 0, prect: (0, 0, 0, 0), w: w as i64, h: h as i64 } }
    fn dispose_prev(&mut self) {
        match self.pdisp {
            2 => { let (px, py, pw, ph) = self.prect; for y in py.max(0)..(py + ph).min(self.h) { for x in px.max(0)..(px + pw).min(self.w) { let o = ((y * self.w + x) * 4) as usize; self.canvas[o..o + 4].fill(0) } } }
            3 => self.canvas.copy_from_slice(&self.saved),
            _ => {}
        }
    }
    fn draw_indexed(&mut self, idx: &[u8], rect: (i64, i64, i64, i64), pal: &[u8], transparent: Option<u8>, disposal: u8) {
        self.dispose_prev();
        if disposal == 3 { self.saved.copy_from_slice(&self.canvas) }
        let (fx, fy, fw, fh) = rect;
        for y in 0..fh { let cy = fy + y; if cy < 0 || cy >= self.h { continue }
            for x in 0..fw { let cx = fx + x; if cx < 0 || cx >= self.w { continue }
                let i = idx[(y * fw + x) as usize]; if Some(i) == transparent { continue }
                let o = ((cy * self.w + cx) * 4) as usize; let p = i as usize * 3;
                let c = if p + 3 <= pal.len() { [pal[p], pal[p + 1], pal[p + 2], 255] } else { [0, 0, 0, 255] };
                self.canvas[o..o + 4].copy_from_slice(&c);
            } }
        self.pdisp = disposal; self.prect = rect;
    }
}

fn gif_crate(bytes: &[u8], dump: Option<&str>) -> Value {
    let mut opt = gif::DecodeOptions::new(); opt.set_color_output(gif::ColorOutput::Indexed);
    let mut d = match opt.read_info(Cursor::new(bytes)) { Ok(d) => d, Err(e) => return err(e) };
    let mut a = match Acc::new(d.width() as u32, d.height() as u32) { Ok(a) => a, Err(v) => return v };
    let mut c = Comp::new(a.w, a.h); let global = d.global_palette().map(|p| p.to_vec()).unwrap_or_default();
    loop {
        let f = match d.read_next_frame() { Ok(Some(f)) => f, Ok(None) => break,
            Err(e) => { if a.n == 0 { return err(e) } a.tail_error = Some(err(e)["error_class"].as_str().unwrap().into()); break } };
        let pal = f.palette.clone().unwrap_or_else(|| global.clone());
        let rect = (f.left as i64, f.top as i64, f.width as i64, f.height as i64);
        let disp = f.dispose as u8;
        c.draw_indexed(&f.buffer, rect, &pal, f.transparent, disp);
        if !a.frame(&c.canvas, f.delay as i64 * 10, disp as i32, rect, -1) { break }
    }
    a.finish("Indexed8", "harness-compositor", dump)
}

fn gift_dec(bytes: &[u8], dump: Option<&str>) -> Value {
    let mut acc: Option<Acc> = None;
    for step in gift::Decoder::new(Cursor::new(bytes)).into_steps() {
        let step = match step { Ok(s) => s, Err(e) => match acc.as_mut() { Some(a) => { a.tail_error = Some(err(e)["error_class"].as_str().unwrap().into()); break } None => return err(e) } };
        let r = step.raster();
        if acc.is_none() { acc = Some(match Acc::new(r.width(), r.height()) { Ok(a) => a, Err(v) => return v }) }
        let a = acc.as_mut().unwrap();
        if (r.width(), r.height()) != (a.w, a.h) { a.tail_error = Some("FrameSizeChanged".into()); break }
        if !a.frame(r.as_u8_slice(), step.delay_time_cs().map(|d| d as i64 * 10).unwrap_or(-1), -1, (0, 0, a.w as i64, a.h as i64), -1) { break }
    }
    match acc { Some(a) => a.finish("SRgba8", "gift-steps", dump), None => err("NoFrames: decoder produced no frame") }
}

struct Canvas { w: u32, h: u32, buf: Vec<u8> }
impl embedded_graphics::geometry::OriginDimensions for Canvas { fn size(&self) -> embedded_graphics::geometry::Size { embedded_graphics::geometry::Size::new(self.w, self.h) } }
impl embedded_graphics::draw_target::DrawTarget for Canvas {
    type Color = embedded_graphics::pixelcolor::Rgb888; type Error = std::convert::Infallible;
    fn draw_iter<I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>>(&mut self, px: I) -> Result<(), Self::Error> {
        use embedded_graphics::pixelcolor::RgbColor;
        for embedded_graphics::Pixel(p, c) in px { if p.x >= 0 && p.y >= 0 && (p.x as u32) < self.w && (p.y as u32) < self.h {
            let o = (p.y as usize * self.w as usize + p.x as usize) * 4; self.buf[o..o + 4].copy_from_slice(&[c.r(), c.g(), c.b(), 255]) } }
        Ok(())
    }
}
fn tinygif_dec(bytes: &[u8], dump: Option<&str>) -> Value {
    use embedded_graphics::image::ImageDrawable;
    let g = match tinygif::Gif::<embedded_graphics::pixelcolor::Rgb888>::from_slice(bytes) { Ok(g) => g, Err(e) => return dbg_err(e) };
    let mut a = match Acc::new(g.width() as u32, g.height() as u32) { Ok(a) => a, Err(v) => return v };
    let mut c = Canvas { w: a.w, h: a.h, buf: vec![0; a.w as usize * a.h as usize * 4] };
    for f in g.frames() {
        let _ = f.draw(&mut c);
        if !a.frame(&c.buf, f.delay_centis as i64 * 10, -1, (0, 0, a.w as i64, a.h as i64), -1) { break }
    }
    a.finish("Rgb888 (no alpha, frames need a GCE, no disposal)", "tinygif-frames", dump)
}

/// APNG reference compositor: transparent canvas, dispose_op 0 none / 1 background(transparent) /
/// 2 previous (treated as 1 on the first frame), blend_op 0 source / 1 straight-alpha over (8-bit, rounded).
fn png_crate(bytes: &[u8], apng: bool, dump: Option<&str>) -> Value {
    let mut d = png::Decoder::new(Cursor::new(bytes));
    d.set_transformations(png::Transformations::EXPAND);
    let mut r = match d.read_info() { Ok(r) => r, Err(e) => return err(e) };
    let info = r.info(); let (w, h) = (info.width, info.height);
    let orig = format!("{:?}{}", info.color_type, info.bit_depth as u8);
    let actl = info.animation_control.map(|a| a.num_frames);
    let mut a = match Acc::new(w, h) { Ok(a) => a, Err(v) => return v };
    let mut canvas = vec![0u8; w as usize * h as usize * 4]; let mut prev: Option<(png::DisposeOp, (u32, u32, u32, u32))> = None;
    let mut saved = canvas.clone();
    let budget = if apng { actl.unwrap_or(0) as usize + 1 } else { 1 };
    for k in 0..budget.max(1) {
        let size = match r.output_buffer_size() { Some(s) if s <= 1 << 30 => s, _ => return err("LimitsExceeded: output buffer") };
        let mut buf = vec![0; size];
        let out = match r.next_frame(&mut buf) { Ok(o) => o, Err(e) => { if a.n == 0 && (k == 0 || !apng) { return err(e) } if a.n == 0 { return err(e) } a.tail_error = Some(err(e)["error_class"].as_str().unwrap().into()); break } };
        let (fw, fh) = (out.width as usize, out.height as usize); let ls = out.line_size;
        let mut px16: Vec<u16> = Vec::with_capacity(fw * fh * 4); let mut px: Vec<u8> = Vec::with_capacity(fw * fh * 4);
        let s16 = out.bit_depth == png::BitDepth::Sixteen;
        for y in 0..fh { let row = &buf[y * ls..y * ls + ls];
            for x in 0..fw {
                let s = |i: usize| -> u16 { if s16 { u16::from_be_bytes([row[2 * i], row[2 * i + 1]]) } else { row[i] as u16 * 257 } };
                let p = match out.color_type { png::ColorType::Grayscale => { let g = s(x); [g, g, g, 65535] }
                    png::ColorType::GrayscaleAlpha => { let g = s(2 * x); [g, g, g, s(2 * x + 1)] }
                    png::ColorType::Rgb => [s(3 * x), s(3 * x + 1), s(3 * x + 2), 65535],
                    png::ColorType::Rgba => [s(4 * x), s(4 * x + 1), s(4 * x + 2), s(4 * x + 3)],
                    png::ColorType::Indexed => return err("UnexpandedIndexed") };
                px16.extend(p);
            } }
        px.extend(px16.iter().map(|v| (v >> 8) as u8));
        let fc = r.info().frame_control;
        if !apng {
            if s16 { a.rgba16 = Some(hash16(&px16)) }
            a.frame(&px, -1, -1, (0, 0, w as i64, h as i64), -1);
            return a.finish(&orig, "default-image", dump);
        }
        if actl.is_none() { a.frame(&px, -1, -1, (0, 0, w as i64, h as i64), -1); break }
        let Some(fc) = fc else { continue }; // default image outside the animation
        if let Some((op, (x0, y0, pw, ph))) = prev {
            match op { png::DisposeOp::Background => for y in y0..(y0 + ph).min(h) { for x in x0..(x0 + pw).min(w) { let o = (y as usize * w as usize + x as usize) * 4; canvas[o..o + 4].fill(0) } },
                       png::DisposeOp::Previous => canvas.copy_from_slice(&saved), _ => {} }
        }
        let op = if a.n == 0 && fc.dispose_op == png::DisposeOp::Previous { png::DisposeOp::Background } else { fc.dispose_op };
        if op == png::DisposeOp::Previous { saved.copy_from_slice(&canvas) }
        for y in 0..fh { for x in 0..fw {
            let (cx, cy) = (fc.x_offset as usize + x, fc.y_offset as usize + y); if cx >= w as usize || cy >= h as usize { continue }
            let o = (cy * w as usize + cx) * 4; let s = &px[(y * fw + x) * 4..(y * fw + x) * 4 + 4];
            if fc.blend_op == png::BlendOp::Source || s[3] == 255 { canvas[o..o + 4].copy_from_slice(s) }
            else if s[3] != 0 {
                // straight-alpha OVER in f32 with truncation, the arithmetic image-rs uses (Rgba::blend)
                let f = |v: u8| v as f32 / 255.0;
                let (fa, ba) = (f(s[3]), f(canvas[o + 3])); let oa = ba + fa - ba * fa;
                if oa != 0.0 {
                    for ch in 0..3 { canvas[o + ch] = (255.0 * ((f(s[ch]) * fa + f(canvas[o + ch]) * ba * (1.0 - fa)) / oa)) as u8 }
                    canvas[o + 3] = (255.0 * oa) as u8;
                }
            } } }
        prev = Some((op, (fc.x_offset, fc.y_offset, fc.width, fc.height)));
        let den = if fc.delay_den == 0 { 100 } else { fc.delay_den };
        if !a.frame(&canvas, ms(fc.delay_num as u64, den as u64), fc.dispose_op as i32, (fc.x_offset as i64, fc.y_offset as i64, fc.width as i64, fc.height as i64), fc.blend_op as i32) { break }
    }
    a.finish(&orig, if apng { "apng-harness-compositor" } else { "default-image" }, dump)
}

fn lodepng_dec(bytes: &[u8], dump: Option<&str>) -> Value {
    let mut d = lodepng::Decoder::new();
    d.info_raw_mut().colortype = lodepng::ColorType::RGBA; d.info_raw_mut().set_bitdepth(16);
    let im = match d.decode(bytes) { Ok(i) => i, Err(e) => return err(e) };
    let depth = d.info_png().color.bitdepth(); let orig = format!("{:?}{}", d.info_png().color.colortype, depth);
    let lodepng::Image::RGBA16(b) = im else { return err("UnexpectedOutputLayout") };
    // the lodepng crate exposes 16-bit channels as the big-endian byte pair reinterpreted in native order
    let v: Vec<u16> = b.buffer.iter().flat_map(|p| [p.r, p.g, p.b, p.a]).map(u16::from_be).collect();
    let mut a = match Acc::new(b.width as u32, b.height as u32) { Ok(a) => a, Err(v) => return v };
    if depth == 16 { a.rgba16 = Some(hash16(&v)); a.frame(&hi16(&v), -1, -1, (0, 0, a.w as i64, a.h as i64), -1); }
    else {
        // sources up to 8 bits: use lodepng's own RGBA8 conversion (decode32), not the 16-bit path
        match lodepng::decode32(bytes) { Ok(im) => { let v8: Vec<u8> = im.buffer.iter().flat_map(|p| [p.r, p.g, p.b, p.a]).collect(); a.frame(&v8, -1, -1, (0, 0, a.w as i64, a.h as i64), -1); }
                                         Err(e) => return err(e) }
    }
    a.finish(&orig, "default-image", dump)
}

fn minipng_dec(bytes: &[u8], dump: Option<&str>) -> Value {
    let hd = match minipng::decode_png_header(bytes) { Ok(h) => h, Err(e) => return dbg_err(e) };
    if hd.width() as u64 * hd.height() as u64 > MAX_PIXELS { return err("Dimensions: canvas outside harness limit") }
    let mut buf = vec![0; hd.required_bytes_rgba8bpc()];
    let mut im = match minipng::decode_png(bytes, &mut buf) { Ok(i) => i, Err(e) => return dbg_err(e) };
    let orig = format!("{:?}{:?}", im.color_type(), im.bit_depth());
    if let Err(e) = im.convert_to_rgba8bpc() { return dbg_err(e) }
    let mut a = match Acc::new(im.width(), im.height()) { Ok(a) => a, Err(v) => return v };
    a.frame(&im.pixels()[..a.w as usize * a.h as usize * 4], -1, -1, (0, 0, a.w as i64, a.h as i64), -1);
    a.finish(&orig, "default-image", dump)
}

fn zune_dec(bytes: &[u8], dump: Option<&str>) -> Value {
    let mut d = zune_png::PngDecoder::new(zune_core::bytestream::ZCursor::new(bytes));
    if let Err(e) = d.decode_headers() { return dbg_err(e) }
    let (w, h) = d.dimensions().unwrap_or((0, 0));
    if w as u64 * h as u64 > MAX_PIXELS { return err("Dimensions: canvas outside harness limit") }
    let cs = d.colorspace(); let depth = d.depth();
    let res = match d.decode() { Ok(r) => r, Err(e) => return dbg_err(e) };
    let (w, h) = d.dimensions().unwrap_or((w, h));
    let Some(cs) = cs else { return err("NoColorspace") };
    let n = cs.num_components();
    let v16: Vec<u16> = match res { zune_core::result::DecodingResult::U8(v) => v.iter().map(|&x| x as u16 * 257).collect(),
        zune_core::result::DecodingResult::U16(v) => v, _ => return err("UnexpectedFloat") };
    if v16.len() < w * h * n { return err("ShortOutput") }
    let mut rgba: Vec<u16> = Vec::with_capacity(w * h * 4);
    for p in v16.chunks_exact(n).take(w * h) { match n { 1 => rgba.extend([p[0], p[0], p[0], 65535]), 2 => rgba.extend([p[0], p[0], p[0], p[1]]),
        3 => rgba.extend([p[0], p[1], p[2], 65535]), 4 => rgba.extend_from_slice(p), _ => return err("UnexpectedComponents") } }
    let mut a = match Acc::new(w as u32, h as u32) { Ok(a) => a, Err(v) => return v };
    if matches!(depth, Some(zune_core::bit_depth::BitDepth::Sixteen)) { a.rgba16 = Some(hash16(&rgba)) }
    a.frame(&hi16(&rgba), -1, -1, (0, 0, a.w as i64, a.h as i64), -1);
    a.finish(&format!("{cs:?}{depth:?}"), "zune-decode", dump)
}

fn run(which: &str, bytes: &[u8], dump: Option<&str>) -> Value {
    match which {
        "gif:glycin" => glycin(bytes, "image/gif", dump),
        "png:glycin" => glycin(bytes, "image/png", dump),
        "png:glycin-apng" => glycin(bytes, "image/apng", dump),
        "gif:image" => {
            let d = match image::codecs::gif::GifDecoder::new(Cursor::new(bytes)) { Ok(d) => d, Err(e) => return err(e) };
            let orig = format!("{:?}", d.color_type()); image_frames(d.into_frames(), &orig, "frame-sequence", dump)
        }
        "png:image" => {
            let d = match image::codecs::png::PngDecoder::new(Cursor::new(bytes)) { Ok(d) => d, Err(e) => return err(e) };
            let orig = format!("{:?}", d.color_type());
            if d.is_apng().unwrap_or(false) { match d.apng() { Ok(a) => image_frames(a.into_frames(), &orig, "apng-frames", dump), Err(e) => err(e) } }
            else { match image::DynamicImage::from_decoder(d) { Ok(im) => image_static(im, &orig, "default-image", dump), Err(e) => err(e) } }
        }
        "gif:gifcrate" => gif_crate(bytes, dump),
        "gif:gift" => gift_dec(bytes, dump),
        "gif:tinygif" => tinygif_dec(bytes, dump),
        "png:pngcrate" => png_crate(bytes, false, dump),
        "png:pngcrate-apng" => png_crate(bytes, true, dump),
        "png:lodepng" => lodepng_dec(bytes, dump),
        "png:minipng" => minipng_dec(bytes, dump),
        "png:zune" => zune_dec(bytes, dump),
        _ => err("unknown selector"),
    }
}

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let a: Vec<String> = std::env::args().collect();
    let result = std::panic::catch_unwind(|| {
        if a.len() < 3 { return err("usage: runner FORMAT:SELECTOR FILE [RGBA_DUMP]") }
        match std::fs::read(&a[2]) { Ok(b) => run(&a[1], &b, a.get(3).map(String::as_str)), Err(e) => err(e) }
    }).unwrap_or_else(|_| json!({"status":"crash","error_class":"rust_panic"}));
    println!("{result}");
}
