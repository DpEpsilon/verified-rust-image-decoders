//! Reference output for tests/compare.py: run the BMP `Format::decode` (the
//! verified core under the GdkCompat profile; `has_alpha` is the core's
//! `rle_or_32bpp`, stock gdk-pixbuf's rule) directly on each file, without gdk-pixbuf.
//! Usage: core_dump OUT_DIR FILE...   Prints one JSON line per file and writes
//! the RGBA bytes of accepted files to OUT_DIR/<basename>.rgba.

use pixbufloader_verified_bmp::bmp::Bmp;
use pixbufloader_verified_bmp::shim::Format;
use std::path::Path;

fn main() {
    let mut args = std::env::args().skip(1);
    let out = args.next().expect("usage: core_dump OUT_DIR FILE...");
    for f in args {
        let bytes = std::fs::read(&f).expect("read input");
        let name = Path::new(&f).file_name().unwrap().to_string_lossy().into_owned();
        match Bmp::decode(&bytes) {
            Ok(img) => {
                std::fs::write(Path::new(&out).join(format!("{name}.rgba")), &img.rgba).unwrap();
                println!(
                    "{{\"file\":\"{name}\",\"status\":\"ok\",\"width\":{},\"height\":{},\"has_alpha\":{}}}",
                    img.width, img.height, img.has_alpha
                );
            }
            Err(e) => println!("{{\"file\":\"{name}\",\"status\":\"error\",\"message\":\"{}\"}}", e.message.to_string_lossy()),
        }
    }
}
