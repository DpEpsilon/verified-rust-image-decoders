//! Policy-phase probe runner for ICNS and ICO incumbents (spec work only).
//! Usage: policy-probe SELECTOR FILE...   -> one JSON line per file.
//! Selectors:
//!   icns-strict-parse | icns-strict-largest | icns-strict-pair:C,M
//!   icns-crate-read | icns-crate-largest | icns-crate-type:OSTY
//!   ico-strict-parse | ico-strict-entry:I | ico-strict-largest
//!   ico-crate-read | ico-crate-entry:I | ico-crate-largest
//!   ico-image
//! Successful decodes print w, h, sha256 (first 16 hex) and, for at most 64
//! pixels, the full RGBA hex.
use sha2::{Digest, Sha256};
use std::io::Cursor;

fn esc(s: &str) -> String {
    s.chars().flat_map(|c| match c { '"' => vec!['\\', '"'], '\\' => vec!['\\', '\\'], c if (c as u32) < 32 => vec![' '], c => vec![c] }).collect()
}
fn ok(w: u32, h: u32, rgba: &[u8], sel: &str) -> String {
    let hash = format!("{:x}", Sha256::digest(rgba));
    let hex = if rgba.len() <= 256 { rgba.iter().map(|b| format!("{b:02x}")).collect::<String>() } else { String::new() };
    format!("{{\"status\":\"ok\",\"w\":{w},\"h\":{h},\"sha\":\"{}\",\"sel\":\"{}\",\"rgba\":\"{hex}\"}}", &hash[..16], esc(sel))
}
fn err(e: impl std::fmt::Display) -> String {
    format!("{{\"status\":\"error\",\"error\":\"{}\"}}", esc(&e.to_string()))
}
fn info(s: String) -> String {
    format!("{{\"status\":\"ok\",\"info\":\"{}\"}}", esc(&s))
}

/// The parsed handle, or its error: `try_parse` is the only constructor.
fn icns_parsed(b: &[u8]) -> Result<icns_verus::Parsed<'_>, String> {
    match icns_verus::try_parse(b) {
        Ok(p) => Ok(p),
        Err(icns_verus::TryError::Format(e)) => Err(e.name().to_string()),
        Err(icns_verus::TryError::OutOfMemory(_)) => Err("OOM".into()),
    }
}
fn icns_pair(p: &icns_verus::Parsed<'_>, c: usize, m: usize, sel: &str) -> String {
    match p.try_combine(Some(c as u32), Some(m as u32)) {
        Ok(d) => ok(d.width, d.height, &d.data, sel),
        Err(icns_verus::TryError::Format(e)) => err(e.name()),
        Err(_) => err("OOM"),
    }
}
fn code(e: &icns_verus::Entry) -> [u8; 4] {
    let h = |x: u8| if x.is_ascii_digit() { x - b'0' } else { x - b'a' + 10 };
    let t = e.type_hex;
    [h(t[0]) * 16 + h(t[1]), h(t[2]) * 16 + h(t[3]), h(t[4]) * 16 + h(t[5]), h(t[6]) * 16 + h(t[7])]
}

fn run(sel: &str, b: &[u8]) -> String {
    let (name, arg) = sel.split_once(':').unwrap_or((sel, ""));
    match name {
        "icns-strict-parse" => match icns_parsed(b) {
            Ok(p) => info(p.entries().iter().map(|e| format!("{}[{},{})", String::from_utf8_lossy(&code(e)), e.payload[0], e.payload[1])).collect::<Vec<_>>().join(" ")),
            Err(e) => err(e),
        },
        "icns-strict-largest" => {
            let p = match icns_parsed(b) { Ok(v) => v, Err(e) => return err(e) };
            for (c, m) in [(*b"it32", *b"t8mk"), (*b"ih32", *b"h8mk"), (*b"il32", *b"l8mk"), (*b"is32", *b"s8mk")] {
                let ci = p.entries().iter().position(|e| code(e) == c);
                let mi = p.entries().iter().position(|e| code(e) == m);
                if let (Some(ci), Some(mi)) = (ci, mi) {
                    return icns_pair(&p, ci, mi, &format!("{}:{}", ci, mi));
                }
            }
            err("NoLegacyPair")
        }
        "icns-strict-pair" => {
            let p = match icns_parsed(b) { Ok(v) => v, Err(e) => return err(e) };
            let (c, m) = arg.split_once(',').unwrap();
            icns_pair(&p, c.parse().unwrap(), m.parse().unwrap(), arg)
        }
        "icns-crate-read" => match icns::IconFamily::read(Cursor::new(b)) {
            Ok(f) => info(format!("{} elements; available {:?}", f.elements.len(), f.available_icons())),
            Err(e) => err(e),
        },
        "icns-crate-largest" | "icns-crate-type" => {
            let f = match icns::IconFamily::read(Cursor::new(b)) { Ok(f) => f, Err(e) => return err(e) };
            let t = if name == "icns-crate-largest" {
                match f.available_icons().into_iter().max_by_key(|t| (t.pixel_width().max(t.pixel_height()), t.pixel_width())) { Some(t) => t, None => return err("NoCandidate") }
            } else {
                let o: [u8; 4] = arg.as_bytes().try_into().unwrap();
                match icns::IconType::from_ostype(icns::OSType(o)) { Some(t) => t, None => return err("unknown type") }
            };
            match f.get_icon_with_type(t) {
                Ok(im) => { let d = im.convert_to(icns::PixelFormat::RGBA); ok(d.width(), d.height(), d.data(), &format!("{t:?}")) }
                Err(e) => err(e),
            }
        }
        "ico-strict-parse" | "ico-strict-entry" | "ico-strict-largest" => {
            use ico_verus::ico::TryError as T;
            let es = match ico_verus::ico::try_parse(b) { Ok(v) => v, Err(T::Format(e)) => return err(format!("{e:?}")), Err(_) => return err("OOM") };
            if name == "ico-strict-parse" {
                return info(es.iter().map(|e| format!("{:?}", e.status)).collect::<Vec<_>>().join(" "));
            }
            let i = if name == "ico-strict-entry" { arg.parse().unwrap() } else {
                let side = |v: u16| if v == 0 { 256u64 } else { v as u64 };
                match es.iter().enumerate().max_by_key(|(_, e)| (side(e.width) * side(e.height), e.bit_count.unwrap_or(0))) { Some((i, _)) => i, None => return err("NoEntry") }
            };
            match ico_verus::ico::try_decode_entry(b, &es, i as u32) {
                Ok(d) => ok(d.width, d.height, &d.rgba, &format!("entry:{i}")),
                Err(T::Format(e)) => err(format!("{e:?}")),
                Err(_) => err("OOM"),
            }
        }
        "ico-crate-read" => match ico::IconDir::read(Cursor::new(b)) {
            Ok(d) => info(d.entries().iter().map(|e| format!("{}x{}@{}{}", e.width(), e.height(), e.bits_per_pixel(), if e.is_png() { "png" } else { "" })).collect::<Vec<_>>().join(" ")),
            Err(e) => err(e),
        },
        "ico-crate-entry" | "ico-crate-largest" => {
            let d = match ico::IconDir::read(Cursor::new(b)) { Ok(d) => d, Err(e) => return err(e) };
            let es = d.entries();
            let i = if name == "ico-crate-entry" { arg.parse().unwrap() } else {
                match es.iter().enumerate().max_by_key(|(_, e)| ((e.width() as u64) * (e.height() as u64), e.bits_per_pixel())) { Some((i, _)) => i, None => return err("NoEntry") }
            };
            if i >= es.len() { return err("NoEntry"); }
            match es[i].decode() { Ok(im) => ok(im.width(), im.height(), im.rgba_data(), &format!("entry:{i}")), Err(e) => err(e) }
        }
        "ico-image" => match image::load_from_memory_with_format(b, image::ImageFormat::Ico) {
            Ok(im) => { let (w, h) = (im.width(), im.height()); ok(w, h, &im.into_rgba8().into_raw(), "library") }
            Err(e) => err(e),
        },
        _ => err("unknown selector"),
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    for p in &a[2..] {
        let line = match std::fs::read(p) {
            Ok(b) => std::panic::catch_unwind(|| run(&a[1], &b)).unwrap_or_else(|_| "{\"status\":\"crash\",\"error\":\"panic\"}".into()),
            Err(e) => err(e),
        };
        println!("{{\"file\":\"{}\",\"r\":{}}}", esc(p), line);
    }
}
