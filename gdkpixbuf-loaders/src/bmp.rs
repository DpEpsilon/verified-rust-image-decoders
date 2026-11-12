#![forbid(unsafe_code)]

//! BMP format: `dib_verus::decode_with_policy(bytes, BmpMode::BmpFile,
//! BmpProfile::GdkCompat)`, the verified core under POLICY-SPEC's measured
//! gdk-pixbuf 2.42.12 profile. No other profile is reachable from this module.

use std::ffi::CStr;

use dib_verus::{decode_with_policy, BmpMode, BmpProfile, Error};

use crate::shim::{ErrorKind, Format, Image, LoadError, Pattern};

/// The one profile this module decodes with.
pub const PROFILE: BmpProfile = BmpProfile::GdkCompat;

pub struct Bmp;

impl Format for Bmp {
    // Same name as the stock module so gdk_pixbuf_loader_new_with_type("bmp")
    // resolves to this module when it is listed last in the cache (see README).
    const NAME: &'static CStr = c"bmp";
    const DESCRIPTION: &'static CStr = c"BMP (verified core, GdkCompat)";
    const DOMAIN: &'static CStr = c"gdkpixbuf-loaders-verified";
    const LICENSE: &'static CStr = c"MIT";
    const MIME_TYPES: &'static [&'static CStr] = &[c"image/bmp", c"image/x-bmp", c"image/x-MS-bmp"];
    const EXTENSIONS: &'static [&'static CStr] = &[c"bmp"];
    // No mask: both bytes must match exactly (the stock entry's `"BM" "" 100`).
    const SIGNATURE: &'static [Pattern] = &[Pattern { prefix: c"BM", mask: None, relevance: 100 }];
    /// The core's own R1 input cap (64 MiB); larger files are `FileTooLarge`
    /// in the core, so buffering more could never succeed.
    const MAX_INPUT: usize = dib_verus::dib::MAX_INPUT;

    fn decode(bytes: &[u8]) -> Result<Image, LoadError> {
        // The recovery flag (`.1`) says the profile repaired or blanked data;
        // gdk-pixbuf has no channel for a warning, so it is not surfaced.
        match decode_with_policy(bytes, BmpMode::BmpFile, PROFILE).0 {
            Ok(d) => {
                // Stock gdk-pixbuf decides has_alpha from the header, not from
                // pixels: bit depth 32 or RLE4/RLE8 compression. The core
                // computes that rule as `rle_or_32bpp`. shim::finish checks that
                // every alpha byte is 255 when it is false and returns a FAILED
                // GError (no panic) if some accepted input broke that.
                let has_alpha = d.rle_or_32bpp;
                Ok(Image { width: d.width, height: d.height, rgba: d.rgba, has_alpha })
            }
            Err(e) => Err(map_error(e)),
        }
    }
}

fn map_error(e: Error) -> LoadError {
    let kind = match e {
        Error::FileTooLarge | Error::OutputTooLarge => ErrorKind::InsufficientMemory,
        _ => ErrorKind::Corrupt,
    };
    // The message is one of a fixed set of `'static` spellings of the core's
    // R15 error (`e.name()`), so rejecting an image never allocates and cannot
    // abort the host on allocation failure. The earlier `format!` versions did
    // allocate; the numeric limits are therefore absent from the text — the
    // README documents the 64 MiB caps instead.
    let msg: &'static CStr = match e {
        Error::FileTooLarge => c"BMP image rejected: FileTooLarge",
        Error::TruncatedFileHeader => c"BMP image rejected: TruncatedFileHeader",
        Error::BadMagic => c"BMP image rejected: BadMagic",
        Error::InvalidFileSize => c"BMP image rejected: InvalidFileSize",
        Error::InvalidPixelOffset => c"BMP image rejected: InvalidPixelOffset",
        Error::TruncatedDibHeader => c"BMP image rejected: TruncatedDibHeader",
        Error::UnsupportedHeaderSize => c"BMP image rejected: UnsupportedHeaderSize",
        Error::TruncatedHeader => c"BMP image rejected: TruncatedHeader",
        Error::InvalidPlanes => c"BMP image rejected: InvalidPlanes",
        Error::InvalidDimensions => c"BMP image rejected: InvalidDimensions",
        Error::UnsupportedPixelDepth => c"BMP image rejected: UnsupportedPixelDepth",
        Error::UnsupportedCompression => c"BMP image rejected: UnsupportedCompression",
        Error::InvalidCompressionDepth => c"BMP image rejected: InvalidCompressionDepth",
        Error::UnsupportedOrientation => c"BMP image rejected: UnsupportedOrientation",
        Error::InvalidColorCount => c"BMP image rejected: InvalidColorCount",
        Error::OutputTooLarge => c"BMP image rejected: OutputTooLarge",
        Error::TruncatedMasks => c"BMP image rejected: TruncatedMasks",
        Error::InvalidBitfields => c"BMP image rejected: InvalidBitfields",
        Error::TruncatedPalette => c"BMP image rejected: TruncatedPalette",
        Error::TruncatedPixels => c"BMP image rejected: TruncatedPixels",
        Error::PaletteIndexOutOfRange => c"BMP image rejected: PaletteIndexOutOfRange",
        Error::RleTruncated => c"BMP image rejected: RleTruncated",
        Error::RleBounds => c"BMP image rejected: RleBounds",
        Error::InvalidImageSize => c"BMP image rejected: InvalidImageSize",
        Error::TruncatedAndMask => c"BMP image rejected: TruncatedAndMask",
        Error::TrailingBytes => c"BMP image rejected: TrailingBytes",
    };
    LoadError::new(kind, msg)
}
