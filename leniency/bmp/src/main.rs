use std::{io::Cursor, path::Path};
use dib_verus::{decode, Mode};
use glycin_utils::{FrameRequest, InitializationDetails, LoaderImplementation, LocalMemory, MemoryFormat, ProcessError};
use image::{ImageDecoder, ImageFormat};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn result(width: u32, height: u32, rgba: Vec<u8>, orig: String, dump: Option<&str>) -> Value {
    if let Some(path) = dump { let _ = std::fs::write(path, &rgba); }
    json!({"status":"ok","width":width,"height":height,"orig_color_type":orig,
           "alpha_present":rgba.chunks_exact(4).any(|p| p[3]!=255),
           "rgba_sha256":format!("{:x}", Sha256::digest(&rgba))})
}
fn error(e: impl std::fmt::Display) -> Value {
    let message = e.to_string();
    json!({"status":"error", "error_class":message.split([':', '(']).next().unwrap_or("error").chars().take(100).collect::<String>(), "message":message.chars().take(400).collect::<String>()})
}
fn glycin_error(e: ProcessError) -> Value {
    let class=match &e {
        ProcessError::ExpectedError{..}=>"ExpectedError",
        ProcessError::InternalError{..}=>"InternalError",
        ProcessError::UnsupportedImageFormat(_)=>"UnsupportedImageFormat",
        ProcessError::ConversionTooLargerError=>"ConversionTooLarge",
        ProcessError::OutOfMemory{..}=>"OutOfMemory",
        ProcessError::NoMoreFrames=>"NoMoreFrames",
        _=>"OtherProcessError",
    };
    json!({"status":"error","error_class":class,"message":e.to_string().chars().take(400).collect::<String>()})
}
fn run(which: &str, bytes: &[u8], dump: Option<&str>) -> Value {
    match which {
        "verified" => match decode(bytes, Mode::BmpFile) {
            Ok(d) => result(d.width,d.height,d.rgba,"RGBA8".into(),dump),
            Err(e) => error(e.name()),
        },
        "image-explicit" | "image-guess" => {
            let format = if which == "image-explicit" { ImageFormat::Bmp }
                         else { match image::guess_format(bytes) { Ok(f)=>f,Err(e)=>return error(e) } };
            let orig = match image::codecs::bmp::BmpDecoder::new(Cursor::new(bytes)) {
                Ok(d)=>format!("{:?}",d.color_type()), Err(e)=>return error(e)
            };
            match image::load_from_memory_with_format(bytes,format) {
                Ok(d) => {let (w,h)=(d.width(),d.height());result(w,h,d.to_rgba8().into_raw(),orig,dump)},
                Err(e)=>error(e),
            }
        },
        "glycin-image-rs" => {
            let (mut loader,_) = match glycin_image_rs::ImgLoader::load::<LocalMemory,_>(Cursor::new(bytes.to_vec()),"image/bmp".into(),InitializationDetails::default()) {
                Ok(v)=>v,Err(e)=>return glycin_error(e)
            };
            let frame=match loader.specific_frame::<LocalMemory>(FrameRequest::default()) {
                Ok(f)=>f,Err(e)=>return glycin_error(e)
            };
            let (w,h)=(frame.width,frame.height);
            let stride=frame.stride as usize;
            let format=frame.memory_format;
            let mut rgba=Vec::with_capacity(w as usize*h as usize*4);
            for row in frame.texture.chunks(stride).take(h as usize) {
                match format {
                    MemoryFormat::R8g8b8a8=>rgba.extend_from_slice(&row[..w as usize*4]),
                    MemoryFormat::B8g8r8a8=>for p in row[..w as usize*4].chunks_exact(4) {rgba.extend([p[2],p[1],p[0],p[3]])},
                    MemoryFormat::R8g8b8=>for p in row[..w as usize*3].chunks_exact(3) {rgba.extend([p[0],p[1],p[2],255])},
                    _=>return error(format!("unsupported output memory format {format:?}")),
                }
            }
            result(w,h,rgba,format!("{format:?}"),dump)
        },
        _=>error("unknown runner"),
    }
}
fn main() {
    let args:Vec<_>=std::env::args().collect();
    let output=std::panic::catch_unwind(|| {
        if args.len()<3 {return error("usage: runner DECODER FILE [RGBA_DUMP]")}
        match std::fs::read(Path::new(&args[2])) {
            Ok(b)=>run(&args[1],&b,args.get(3).map(String::as_str)),
            Err(e)=>error(e)
        }
    }).unwrap_or_else(|_|json!({"status":"crash","error_class":"rust_panic"}));
    println!("{}",output);
}
