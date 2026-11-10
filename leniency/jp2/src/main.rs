#[path = "../../modern/frame.rs"]
mod frame;
use frame::*;
use serde_json::{json, Value};

/// glycin-image-rs for `image/jp2` or `image/x-jp2-codestream`. The real dispatcher picks the MIME type from
/// content sniffing; here the codestream magic FF4F FF51 selects `image/x-jp2-codestream`, everything else `image/jp2`.
fn glycin(bytes: &[u8], dump: Option<&str>) -> Value {
    let mime = if bytes.starts_with(&[0xff, 0x4f]) { "image/x-jp2-codestream" } else { "image/jp2" };
    glycin_run(bytes, mime, dump)
}

/// hayro-jpeg2000 called directly (same crate as glycin's path) with `strict` on or off.
fn hayro(bytes: &[u8], strict: bool, dump: Option<&str>) -> Value {
    let mut s = hayro_jpeg2000::DecodeSettings::default();
    s.strict = strict;
    let im = match hayro_jpeg2000::Image::new(bytes, &s) { Ok(i) => i, Err(e) => return err(format!("{e:?}")) };
    let (w, h) = (im.width(), im.height());
    if w == 0 || h == 0 || w as u64 * h as u64 > MAX_PIXELS { return err("output dimensions outside harness limit") }
    let cs = format!("{:?}/alpha={}/depth={}", im.color_space(), im.has_alpha(), im.original_bit_depth());
    let alpha = im.has_alpha();
    let d = match im.decode() { Ok(d) => d, Err(e) => return err(format!("{e:?}")) };
    let n = d.len() / (w as usize * h as usize).max(1);
    if d.len() != w as usize * h as usize * n { return err("decoded length mismatch") }
    let mut rgba = Vec::with_capacity(w as usize * h as usize * 4);
    for p in d.chunks_exact(n) {
        match (n, alpha) {
            (1, _) => rgba.extend([p[0], p[0], p[0], 255]),
            (2, true) => rgba.extend([p[0], p[0], p[0], p[1]]),
            (3, _) => rgba.extend([p[0], p[1], p[2], 255]),
            (4, true) => rgba.extend([p[0], p[1], p[2], p[3]]),
            _ => return err(format!("unsupported channel layout {n} ({cs})")),
        }
    }
    ok(&[Img { w, h, rgba, orig: cs }], dump, json!({}))
}
fn main() {
    main_wrap(|sel, bytes, dump| match sel {
        "jp2:glycin" => glycin(bytes, dump),
        "jp2:hayro" => hayro(bytes, false, dump),
        "jp2:hayro-strict" => hayro(bytes, true, dump),
        _ => err("unknown selector"),
    })
}
