#![forbid(unsafe_code)]

//! The format-independent, entirely safe half of a loader module.
//!
//! A format plugs in by implementing [`Format`]: static metadata for
//! `fill_info` plus one whole-buffer `decode` function. Everything that touches
//! C (vtable, callbacks, GError, GdkPixbuf) lives in [`crate::ffi`] and is
//! generic over `Format`, so an ICO/TGA/ICNS module is one `impl Format` and
//! one `export_loader!` line.

use std::ffi::CStr;

/// One `GdkPixbufModulePattern`: `prefix`, an optional `mask` that must be
/// exactly as long as `prefix` when present (gdk-pixbuf-query-loaders rejects
/// a module otherwise; `None` means every byte must match), and a relevance in
/// 0..=100.
pub struct Pattern {
    pub prefix: &'static CStr,
    pub mask: Option<&'static CStr>,
    pub relevance: i32,
}

/// A decoded image: `rgba` is exactly `4 * width * height` bytes of
/// non-premultiplied RGBA8, top row first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    /// Whether the GdkPixbuf gets an alpha channel. When `false` the alpha
    /// bytes of `rgba` are dropped, so they must all be 255.
    pub has_alpha: bool,
}

/// Which `GdkPixbufError` code a failure maps to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// GDK_PIXBUF_ERROR_CORRUPT_IMAGE
    Corrupt,
    /// GDK_PIXBUF_ERROR_INSUFFICIENT_MEMORY (also used for the size caps)
    InsufficientMemory,
    /// GDK_PIXBUF_ERROR_FAILED (internal errors, caught panics)
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadError {
    pub kind: ErrorKind,
    /// A `'static` C string. It is never built from image bytes, so reporting a
    /// rejection neither allocates nor can contain a NUL or other
    /// image-controlled text.
    pub message: &'static CStr,
}

impl LoadError {
    pub fn new(kind: ErrorKind, message: &'static CStr) -> Self {
        LoadError { kind, message }
    }
}

/// A file format served by one loader module.
pub trait Format: 'static {
    /// Module name; gdk-pixbuf looks modules up by it for
    /// `gdk_pixbuf_loader_new_with_type` and `gdk_pixbuf_save`.
    const NAME: &'static CStr;
    const DESCRIPTION: &'static CStr;
    /// gettext domain for DESCRIPTION.
    const DOMAIN: &'static CStr;
    const LICENSE: &'static CStr;
    const MIME_TYPES: &'static [&'static CStr];
    const EXTENSIONS: &'static [&'static CStr];
    const SIGNATURE: &'static [Pattern];
    /// Hard cap on accumulated input bytes. A write that would exceed it fails
    /// immediately and nothing beyond it is ever buffered.
    const MAX_INPUT: usize;

    /// Decode a complete file.
    fn decode(bytes: &[u8]) -> Result<Image, LoadError>;
}

/// Byte accumulator behind `load_increment`: grows geometrically but never
/// reserves past `cap`, and reports allocation failure instead of aborting.
#[derive(Debug)]
pub struct Accumulator {
    buf: Vec<u8>,
    cap: usize,
}

impl Accumulator {
    pub fn new(cap: usize) -> Self {
        Accumulator { buf: Vec::new(), cap }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Bytes reserved; never exceeds the cap.
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    pub fn push(&mut self, chunk: &[u8]) -> Result<(), LoadError> {
        let need = self.buf.len().checked_add(chunk.len()).filter(|&n| n <= self.cap);
        let Some(need) = need else {
            return Err(LoadError::new(ErrorKind::InsufficientMemory, c"image exceeds the input size limit"));
        };
        if need > self.buf.capacity() {
            let target = need.max(self.buf.capacity().saturating_mul(2)).max(4096).min(self.cap);
            self.buf.try_reserve_exact(target - self.buf.len()).map_err(|_| {
                LoadError::new(ErrorKind::InsufficientMemory, c"out of memory buffering image data")
            })?;
        }
        self.buf.extend_from_slice(chunk);
        Ok(())
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.buf
    }
}

/// Decode accumulated bytes and check the `Image` invariants the FFI side
/// relies on (dimensions fit a C `int`, buffer length is exactly `4*w*h`,
/// opaque when `has_alpha` is false).
pub fn finish<F: Format>(acc: Accumulator) -> Result<Image, LoadError> {
    if acc.is_empty() {
        return Err(LoadError::new(ErrorKind::Corrupt, c"image file is empty"));
    }
    let bytes = acc.into_inner();
    let img = F::decode(&bytes)?;
    drop(bytes); // release the input before the GdkPixbuf is allocated
    let n = (img.width as usize).checked_mul(img.height as usize).and_then(|n| n.checked_mul(4));
    let ok = img.width >= 1
        && img.height >= 1
        && img.width <= i32::MAX as u32
        && img.height <= i32::MAX as u32
        && n == Some(img.rgba.len())
        && (img.has_alpha || img.rgba.chunks_exact(4).all(|p| p[3] == 255));
    if !ok {
        return Err(LoadError::new(ErrorKind::Failed, c"decoder returned an inconsistent image"));
    }
    Ok(img)
}

/// Copy `img` into a pixbuf's pixel store `dst` (row stride `rowstride`,
/// 4 channels if `img.has_alpha`, else 3). Fails instead of writing out of
/// bounds if `dst` is too short for the geometry.
pub fn copy_into(img: &Image, dst: &mut [u8], rowstride: usize) -> Result<(), LoadError> {
    let nch = if img.has_alpha { 4 } else { 3 };
    let (w, h) = (img.width as usize, img.height as usize);
    let row_bytes = w * nch;
    let needed = rowstride.checked_mul(h - 1).and_then(|v| v.checked_add(row_bytes));
    if rowstride < row_bytes || needed.map_or(true, |n| n > dst.len()) {
        return Err(LoadError::new(ErrorKind::Failed, c"pixbuf geometry does not fit the image"));
    }
    for (y, src_row) in img.rgba.chunks_exact(w * 4).enumerate() {
        let dst_row = &mut dst[y * rowstride..y * rowstride + row_bytes];
        if img.has_alpha {
            dst_row.copy_from_slice(src_row);
        } else {
            for (d, s) in dst_row.chunks_exact_mut(3).zip(src_row.chunks_exact(4)) {
                d.copy_from_slice(&s[..3]);
            }
        }
    }
    Ok(())
}
