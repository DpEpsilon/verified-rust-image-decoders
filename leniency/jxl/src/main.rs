#[path = "../../modern/frame.rs"]
mod frame;
use frame::*;
use serde_json::{json, Value};
use std::io::Cursor;

/// jxl-oxide 0.12 (independent of the `jxl` crate): every keyframe via render_frame, f32 -> RGBA8 round(clamp*255),
/// orientation not applied. CMYK is skipped (no conversion).
fn oxide(bytes: &[u8], dump: Option<&str>) -> Value {
    let image = match jxl_oxide::JxlImage::builder().read(Cursor::new(bytes)) { Ok(i) => i, Err(e) => return err(format!("{e}")) };
    let pf = image.pixel_format();
    let hdr = image.image_header();
    let (w, h) = (hdr.size.width, hdr.size.height);
    if w == 0 || h == 0 || w as u64 * h as u64 > MAX_PIXELS { return err("output dimensions outside harness limit") }
    let n = image.num_loaded_keyframes();
    if n == 0 { return err("no frames decoded") }
    let mut frames = vec![];
    let mut later = Value::Null;
    for k in 0..n.min(MAX_FRAMES) {
        let r = match image.render_frame(k) { Ok(r) => r, Err(e) => { if frames.is_empty() { return err(format!("{e}")) } later = json!(e.to_string()); break } };
        let fb = r.image_all_channels();
        let (fw, fh, ch) = (fb.width(), fb.height(), fb.channels());
        let q = |v: f32| -> u8 { let v = if v > 0.0 { v.min(1.0) } else { 0.0 }; (v * 255.0 + 0.5) as u8 };
        let mut rgba = Vec::with_capacity(fw * fh * 4);
        for p in fb.buf().chunks_exact(ch) {
            match (pf, ch) {
                (jxl_oxide::PixelFormat::Gray, 1) => rgba.extend([q(p[0]), q(p[0]), q(p[0]), 255]),
                (jxl_oxide::PixelFormat::Graya, 2) => rgba.extend([q(p[0]), q(p[0]), q(p[0]), q(p[1])]),
                (jxl_oxide::PixelFormat::Rgb, 3) => rgba.extend([q(p[0]), q(p[1]), q(p[2]), 255]),
                (jxl_oxide::PixelFormat::Rgba, 4) => rgba.extend([q(p[0]), q(p[1]), q(p[2]), q(p[3])]),
                _ => return json!({"status":"skip","error_class":"not-applicable","message":format!("pixel format {pf:?} with {ch} channels not converted by runner")}),
            }
        }
        frames.push(Img { w: fw as u32, h: fh as u32, rgba, orig: format!("{pf:?}") });
    }
    ok(&frames, dump, json!({"later_frame_error": later, "orientation": hdr.metadata.orientation}))
}
fn main() {
    main_wrap(|sel, bytes, dump| match sel {
        "jxl:glycin" => glycin_run(bytes, "image/jxl", dump),
        "jxl:oxide" => oxide(bytes, dump),
        _ => err("unknown selector"),
    })
}
