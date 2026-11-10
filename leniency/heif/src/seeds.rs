// Deterministic synthetic AVIF seeds with the ravif/rav1e encoder (pure Rust, no asm). Usage: avif-seeds OUTDIR
use imgref::Img;
use ravif::{AlphaColorMode, BitDepth, ColorModel, Encoder};
use rgb::{RGB8, RGBA8};

fn px(x: usize, y: usize, w: usize, h: usize, alpha: bool) -> RGBA8 {
    let r = (x * 255 / w.max(1)) as u8; let g = (y * 255 / h.max(1)) as u8; let b = (((x * 7 + y * 13) % 256) as u8) ^ 0x55;
    let a = if alpha { 255 - ((x + y) * 255 / (w + h).max(1)) as u8 } else { 255 };
    RGBA8::new(r, g, b, a)
}
fn main() {
    let dir = std::env::args().nth(1).expect("outdir");
    std::fs::create_dir_all(&dir).unwrap();
    let mut save = |name: &str, r: Result<ravif::EncodedImage, ravif::Error>| match r {
        Ok(e) => std::fs::write(format!("{dir}/{name}.avif"), e.avif_file).unwrap(),
        Err(e) => eprintln!("skip {name}: {e}"),
    };
    let base = |q: f32| Encoder::new().with_quality(q).with_speed(10).with_num_threads(Some(1));
    for (name, w, h, alpha) in [("rgb-17x13", 17, 13, false), ("rgba-17x13", 17, 13, true), ("rgb-1x1", 1, 1, false), ("rgba-2x2", 2, 2, true),
                                ("rgb-33x20", 33, 20, false), ("rgba-64x48", 64, 48, true), ("rgb-8x64", 8, 64, false)] {
        let pix: Vec<RGBA8> = (0..w * h).map(|i| px(i % w, i / w, w, h, alpha)).collect();
        if alpha {
            save(&format!("ycbcr-8bit-{name}"), base(85.0).with_bit_depth(BitDepth::Eight).encode_rgba(Img::new(&pix[..], w, h)));
        } else {
            let rgb: Vec<RGB8> = pix.iter().map(|p| RGB8::new(p.r, p.g, p.b)).collect();
            save(&format!("ycbcr-8bit-{name}"), base(85.0).with_bit_depth(BitDepth::Eight).encode_rgb(Img::new(&rgb[..], w, h)));
        }
    }
    let (w, h) = (24, 16);
    let pix: Vec<RGBA8> = (0..w * h).map(|i| px(i % w, i / w, w, h, true)).collect();
    let rgb: Vec<RGB8> = pix.iter().map(|p| RGB8::new(p.r, p.g, p.b)).collect();
    save("ycbcr-10bit-rgb-24x16", base(85.0).with_bit_depth(BitDepth::Ten).encode_rgb(Img::new(&rgb[..], w, h)));
    save("ycbcr-10bit-rgba-24x16", base(85.0).with_bit_depth(BitDepth::Ten).encode_rgba(Img::new(&pix[..], w, h)));
    save("identity-rgb-24x16", base(85.0).with_bit_depth(BitDepth::Eight).with_internal_color_model(ColorModel::RGB).encode_rgb(Img::new(&rgb[..], w, h)));
    save("identity-rgba-24x16", base(85.0).with_bit_depth(BitDepth::Eight).with_internal_color_model(ColorModel::RGB).encode_rgba(Img::new(&pix[..], w, h)));
    save("lowq-rgb-24x16", base(10.0).with_bit_depth(BitDepth::Eight).encode_rgb(Img::new(&rgb[..], w, h)));
    save("highq-rgb-24x16", base(100.0).with_bit_depth(BitDepth::Eight).encode_rgb(Img::new(&rgb[..], w, h)));
    save("alpha-premultiplied-24x16", base(85.0).with_bit_depth(BitDepth::Eight).with_alpha_color_mode(AlphaColorMode::Premultiplied).encode_rgba(Img::new(&pix[..], w, h)));
    save("alpha-clean-24x16", base(85.0).with_bit_depth(BitDepth::Eight).with_alpha_color_mode(AlphaColorMode::UnassociatedClean).encode_rgba(Img::new(&pix[..], w, h)));
    save("exif-rgb-24x16", base(85.0).with_bit_depth(BitDepth::Eight).with_exif(vec![0u8, 0, 0, 0, b'M', b'M', 0, 42, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0]).encode_rgb(Img::new(&rgb[..], w, h)));
    // raw planes: matrix coefficient / range variants (plane triples are interpreted by the decoder as YCbCr/GBR)
    let planes: Vec<[u8; 3]> = pix.iter().map(|p| [p.g, p.b, p.r]).collect();
    for (n, m) in [("bt601", ravif::MatrixCoefficients::BT601), ("bt709", ravif::MatrixCoefficients::BT709), ("bt2020ncl", ravif::MatrixCoefficients::BT2020NCL)] {
        for (rn, r) in [("full", ravif::PixelRange::Full), ("limited", ravif::PixelRange::Limited)] {
            save(&format!("planes-{n}-{rn}-24x16"), base(85.0).encode_raw_planes_8_bit(w, h, planes.clone(), None::<[u8; 0]>, r, m));
        }
    }
}
