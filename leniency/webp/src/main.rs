#[path = "../../modern/frame.rs"]
mod frame;
use frame::*;
use image::{AnimationDecoder, ImageDecoder};
use serde_json::{json, Value};
use std::io::Cursor;

/// `image::codecs::webp::WebPDecoder` (image-webp): static image via `DynamicImage`, animation via `into_frames()`.
fn image_run(bytes: &[u8], dump: Option<&str>) -> Value {
    let d = match image::codecs::webp::WebPDecoder::new(Cursor::new(bytes)) { Ok(d) => d, Err(e) => return err(e) };
    let orig = format!("{:?}", d.color_type());
    if d.has_animation() {
        let mut out = vec![];
        for f in d.into_frames().take(MAX_FRAMES) {
            match f {
                Ok(f) => { let b = f.into_buffer(); out.push(Img { w: b.width(), h: b.height(), rgba: b.into_raw(), orig: orig.clone() }) }
                Err(e) => { if out.is_empty() { return err(e) } return ok(&out, dump, json!({"later_frame_error": e.to_string()})) }
            }
        }
        if out.is_empty() { return err("no frames") }
        ok(&out, dump, json!({}))
    } else {
        match image::DynamicImage::from_decoder(d) {
            Ok(im) => ok(&[Img { w: im.width(), h: im.height(), rgba: im.to_rgba8().into_raw(), orig }], dump, json!({})),
            Err(e) => err(e),
        }
    }
}
fn main() {
    main_wrap(|sel, bytes, dump| match sel {
        "webp:image" => image_run(bytes, dump),
        "webp:glycin" => glycin_run(bytes, "image/webp", dump),
        _ => err("unknown selector"),
    })
}
