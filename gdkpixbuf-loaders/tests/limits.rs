//! Truncated, empty and oversized input, at the safe shim layer and through
//! the extern "C" entry points gdk-pixbuf calls. A per-thread counting
//! allocator checks that oversized declarations never allocate unboundedly.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::ffi::{c_int, c_uint, c_void};
use std::ptr;

use pixbufloader_verified_bmp::bmp::Bmp;
use pixbufloader_verified_bmp::ffi::{self, GError, GdkPixbuf, GdkPixbufAnimation};
use pixbufloader_verified_bmp::shim::{self, Accumulator, ErrorKind, Format, Image, LoadError, Pattern};

// ---- per-thread peak-allocation counter ---------------------------------

struct Counting;
thread_local! {
    static CUR: Cell<usize> = const { Cell::new(0) };
    static PEAK: Cell<usize> = const { Cell::new(0) };
    /// Test-injected allocation failure: while non-zero, this thread's
    /// allocations of at least this many bytes return null.
    static FAIL_AT_OR_ABOVE: Cell<usize> = const { Cell::new(0) };
}
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        if FAIL_AT_OR_ABOVE.try_with(|f| f.get() != 0 && l.size() >= f.get()).unwrap_or(false) {
            return ptr::null_mut();
        }
        let _ = CUR.try_with(|c| {
            c.set(c.get() + l.size());
            let _ = PEAK.try_with(|p| p.set(p.get().max(c.get())));
        });
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        let _ = CUR.try_with(|c| c.set(c.get().saturating_sub(l.size())));
        unsafe { System.dealloc(p, l) }
    }
}
#[global_allocator]
static A: Counting = Counting;

/// Peak bytes allocated by this thread while running `f`, above the level at entry.
fn peak_during<R>(f: impl FnOnce() -> R) -> (R, usize) {
    let base = CUR.with(|c| c.get());
    PEAK.with(|p| p.set(base));
    let r = f();
    (r, PEAK.with(|p| p.get()) - base)
}

// ---- inputs ----------------------------------------------------------------

fn le16(v: &mut Vec<u8>, x: u16) {
    v.extend_from_slice(&x.to_le_bytes())
}
fn le32(v: &mut Vec<u8>, x: u32) {
    v.extend_from_slice(&x.to_le_bytes())
}

/// BITMAPFILEHEADER + BITMAPINFOHEADER (40 bytes), BI_RGB, no palette.
fn header(file_size: u32, w: i32, h: i32, bpp: u16) -> Vec<u8> {
    let mut v = b"BM".to_vec();
    le32(&mut v, file_size);
    le32(&mut v, 0);
    le32(&mut v, 54);
    le32(&mut v, 40);
    le32(&mut v, w as u32);
    le32(&mut v, h as u32);
    le16(&mut v, 1);
    le16(&mut v, bpp);
    for _ in 0..6 {
        le32(&mut v, 0); // compression, image size, ppm x/y, colours used/important
    }
    v
}

/// A valid 2x2 24-bit image (bottom-up rows of 6 bytes + 2 pad bytes).
fn tiny_bmp() -> Vec<u8> {
    let mut v = header(70, 2, 2, 24);
    v.extend_from_slice(&[0, 0, 255, 0, 255, 0, 0, 0]); // bottom row: red, green
    v.extend_from_slice(&[255, 0, 0, 255, 255, 255, 0, 0]); // top row: blue, white
    v
}

fn decode_all(bytes: &[u8]) -> Result<Image, LoadError> {
    let mut acc = Accumulator::new(Bmp::MAX_INPUT);
    acc.push(bytes)?;
    shim::finish::<Bmp>(acc)
}

// ---- safe-layer tests --------------------------------------------------------

#[test]
fn tiny_image_decodes() {
    let img = decode_all(&tiny_bmp()).unwrap();
    assert_eq!((img.width, img.height, img.has_alpha), (2, 2, false));
    assert_eq!(&img.rgba[..8], &[0, 0, 255, 255, 255, 255, 255, 255]); // top row first
}

#[test]
fn empty_input_is_corrupt() {
    let e = shim::finish::<Bmp>(Accumulator::new(Bmp::MAX_INPUT)).unwrap_err();
    assert_eq!(e.kind, ErrorKind::Corrupt);
}

/// Expected RGBA of the 2x2 image when only `n` bytes (54 <= n <= 70) of
/// `tiny_bmp()` exist. Stock gdk-pixbuf (and GdkCompat's short_pixels ZeroFill)
/// keeps a row only if its whole 8-byte stride, padding included, is present;
/// every other row is zero-filled (black, alpha 255 in the 24 bpp output).
fn expected_rgba_for_prefix(n: usize) -> Vec<u8> {
    let full = tiny_bmp();
    let mut out = Vec::new();
    for y in 0..2 {
        let file_row = 1 - y; // bottom-up storage: file row 0 is the bottom image row
        let present = n >= 54 + 8 * (file_row + 1);
        for x in 0..2 {
            let o = 54 + file_row * 8 + x * 3;
            if present {
                out.extend_from_slice(&[full[o + 2], full[o + 1], full[o], 255]);
            } else {
                out.extend_from_slice(&[0, 0, 0, 255]);
            }
        }
    }
    out
}

/// GdkCompat matches stock gdk-pixbuf: a prefix shorter than the 54-byte
/// headers is an error; any longer prefix decodes, with incomplete rows
/// zero-filled. (Checked against stock gdk-pixbuf 2.42.12 on all 70 prefixes.)
#[test]
fn truncation_matches_stock_gdk_pixbuf() {
    let full = tiny_bmp();
    for n in 1..54 {
        let e = decode_all(&full[..n]).unwrap_err();
        assert_eq!(e.kind, ErrorKind::Corrupt, "prefix of {n} bytes: {}", e.message.to_string_lossy());
    }
    for n in 54..=full.len() {
        let img = decode_all(&full[..n])
            .unwrap_or_else(|e| panic!("prefix of {n} bytes: {}", e.message.to_string_lossy()));
        assert_eq!((img.width, img.height, img.has_alpha), (2, 2, false), "prefix of {n} bytes");
        assert_eq!(img.rgba, expected_rgba_for_prefix(n), "prefix of {n} bytes");
    }
}

#[test]
fn huge_declared_file_size_errors_without_allocating() {
    // bfSize claims 64 MiB + 1; dimensions claim an 8192x8192 32-bit image
    // (256 MiB of RGBA). The file is 54 bytes.
    let bytes = header(64 * 1024 * 1024 + 1, 8192, 8192, 32);
    let (r, peak) = peak_during(|| decode_all(&bytes));
    let e = r.unwrap_err();
    assert!(e.message.to_string_lossy().contains("BMP image rejected"), "{}", e.message.to_string_lossy());
    assert!(peak < 64 * 1024, "peak allocation {peak} bytes");
}

#[test]
fn huge_declared_output_errors_without_allocating() {
    // Consistent bfSize, but 8192x8192 at 32 bpp needs 256 MiB of output.
    let bytes = header(54, 8192, 8192, 32);
    let (r, peak) = peak_during(|| decode_all(&bytes));
    let e = r.unwrap_err();
    assert_eq!(e.kind, ErrorKind::InsufficientMemory, "{}", e.message.to_string_lossy());
    assert!(peak < 64 * 1024, "peak allocation {peak} bytes");
}

#[test]
fn input_beyond_cap_is_refused_and_buffer_stays_capped() {
    const MIB: usize = 1024 * 1024;
    let chunk = vec![0u8; MIB];
    let mut acc = Accumulator::new(Bmp::MAX_INPUT);
    let (err, peak) = peak_during(|| {
        let mut pushed = 0;
        loop {
            match acc.push(&chunk) {
                Ok(()) => pushed += MIB,
                Err(e) => break (pushed, e),
            }
        }
    });
    let (pushed, e) = err;
    assert_eq!(pushed, Bmp::MAX_INPUT);
    assert_eq!(e.kind, ErrorKind::InsufficientMemory);
    assert!(acc.capacity() <= Bmp::MAX_INPUT);
    // This counter models realloc as alloc-new + free-old, so the last growth
    // step (32 MiB -> 64 MiB) shows a transient 96 MiB; nothing beyond that.
    assert!(peak <= Bmp::MAX_INPUT + Bmp::MAX_INPUT / 2, "peak allocation {peak} bytes");
    // A 1-byte write past the cap is refused too.
    assert!(acc.push(&[0]).is_err());
}

// ---- through the extern "C" entry points --------------------------------------

extern "C" {
    fn g_error_free(e: *mut GError);
    fn gdk_pixbuf_error_quark() -> u32;
    fn gdk_pixbuf_get_width(p: *const GdkPixbuf) -> c_int;
}

#[derive(Default)]
struct Calls {
    log: String,
    size: (c_int, c_int),
    pixbuf_width: c_int,
}

unsafe extern "C" fn on_size(w: *mut c_int, h: *mut c_int, ud: *mut c_void) {
    let c = unsafe { &mut *(ud as *mut Calls) };
    c.log.push('S');
    c.size = unsafe { (*w, *h) };
}
unsafe extern "C" fn on_prepared(p: *mut GdkPixbuf, _a: *mut GdkPixbufAnimation, ud: *mut c_void) {
    let c = unsafe { &mut *(ud as *mut Calls) };
    c.log.push('P');
    c.pixbuf_width = unsafe { gdk_pixbuf_get_width(p) };
}
unsafe extern "C" fn on_updated(_p: *mut GdkPixbuf, x: c_int, y: c_int, w: c_int, h: c_int, ud: *mut c_void) {
    let c = unsafe { &mut *(ud as *mut Calls) };
    c.log.push_str(&format!("U{x},{y},{w},{h}"));
}

/// Run begin/increment(chunked)/stop; returns (ok, GError code if any, calls).
fn run_ffi<F: Format>(bytes: &[u8], chunk: usize) -> (bool, Option<(u32, c_int)>, Calls) {
    let mut calls = Calls::default();
    let mut err: *mut GError = ptr::null_mut();
    unsafe {
        let ctx = ffi::begin_load::<F>(
            Some(on_size),
            Some(on_prepared),
            Some(on_updated),
            &mut calls as *mut Calls as *mut c_void,
            &mut err,
        );
        assert!(!ctx.is_null());
        let mut ok = true;
        for c in bytes.chunks(chunk.max(1)) {
            if ffi::load_increment::<F>(ctx, c.as_ptr(), c.len() as c_uint, &mut err) == 0 {
                ok = false;
                break;
            }
        }
        let mut err2: *mut GError = ptr::null_mut();
        let stopped = ffi::stop_load::<F>(ctx, if ok { &mut err } else { &mut err2 }) != 0;
        if !err2.is_null() {
            g_error_free(err2);
        }
        let code = (!err.is_null()).then(|| ((*err).domain, (*err).code));
        if !err.is_null() {
            g_error_free(err);
        }
        (ok && stopped, code, calls)
    }
}

#[test]
fn ffi_callbacks_in_order_for_any_chunking() {
    for chunk in [1, 7, 70] {
        let (ok, code, calls) = run_ffi::<Bmp>(&tiny_bmp(), chunk);
        assert!(ok && code.is_none(), "chunk {chunk}: {code:?}");
        assert_eq!(calls.log, "SPU0,0,2,2");
        assert_eq!((calls.size, calls.pixbuf_width), ((2, 2), 2));
    }
}

#[test]
fn ffi_empty_and_short_set_corrupt_gerror() {
    let quark = unsafe { gdk_pixbuf_error_quark() };
    for bytes in [&[][..], &tiny_bmp()[..30], &tiny_bmp()[..53]] {
        let (ok, code, calls) = run_ffi::<Bmp>(bytes, 7);
        assert!(!ok);
        assert_eq!(code, Some((quark, 0)), "GDK_PIXBUF_ERROR_CORRUPT_IMAGE");
        assert!(calls.log.is_empty(), "no callbacks on error");
    }
}

/// Prefixes of 54..=69 bytes decode (zero-filled rows) like stock.
#[test]
fn ffi_truncated_pixel_data_decodes() {
    for n in [54, 69] {
        let (ok, code, calls) = run_ffi::<Bmp>(&tiny_bmp()[..n], 7);
        assert!(ok && code.is_none(), "prefix {n}: {code:?}");
        assert_eq!(calls.log, "SPU0,0,2,2");
    }
}

/// A format whose decoder panics: the panic must come back as a GError.
struct Panicky;
impl Format for Panicky {
    const NAME: &'static std::ffi::CStr = c"panicky";
    const DESCRIPTION: &'static std::ffi::CStr = c"test";
    const DOMAIN: &'static std::ffi::CStr = c"test";
    const LICENSE: &'static std::ffi::CStr = c"MIT";
    const MIME_TYPES: &'static [&'static std::ffi::CStr] = &[];
    const EXTENSIONS: &'static [&'static std::ffi::CStr] = &[];
    const SIGNATURE: &'static [Pattern] = &[];
    const MAX_INPUT: usize = 1024;
    fn decode(_: &[u8]) -> Result<Image, LoadError> {
        panic!("decoder bug")
    }
}

/// A decoder that claims no alpha channel but returns a transparent pixel: the
/// shim guard must turn that into a FAILED GError, not a panic or a bad copy.
struct LiesAboutAlpha;
impl Format for LiesAboutAlpha {
    const NAME: &'static std::ffi::CStr = c"liar";
    const DESCRIPTION: &'static std::ffi::CStr = c"test";
    const DOMAIN: &'static std::ffi::CStr = c"test";
    const LICENSE: &'static std::ffi::CStr = c"MIT";
    const MIME_TYPES: &'static [&'static std::ffi::CStr] = &[];
    const EXTENSIONS: &'static [&'static std::ffi::CStr] = &[];
    const SIGNATURE: &'static [Pattern] = &[];
    const MAX_INPUT: usize = 1024;
    fn decode(_: &[u8]) -> Result<Image, LoadError> {
        Ok(Image { width: 1, height: 1, rgba: vec![1, 2, 3, 0], has_alpha: false })
    }
}

#[test]
fn has_alpha_guard_gives_failed_gerror() {
    let quark = unsafe { gdk_pixbuf_error_quark() };
    let (ok, code, calls) = run_ffi::<LiesAboutAlpha>(b"x", 1);
    assert!(!ok);
    assert_eq!(code, Some((quark, 5)), "GDK_PIXBUF_ERROR_FAILED");
    assert!(calls.log.is_empty());
}

#[test]
fn ffi_panic_becomes_failed_gerror() {
    let quark = unsafe { gdk_pixbuf_error_quark() };
    let (ok, code, calls) = run_ffi::<Panicky>(b"anything", 3);
    assert!(!ok);
    assert_eq!(code, Some((quark, 5)), "GDK_PIXBUF_ERROR_FAILED");
    assert!(calls.log.is_empty());
}

/// Run `f` with every allocation of at least `min` bytes on this thread failing.
fn failing_at_or_above<R>(min: usize, f: impl FnOnce() -> R) -> R {
    FAIL_AT_OR_ABOVE.with(|c| c.set(min));
    let r = f();
    FAIL_AT_OR_ABOVE.with(|c| c.set(0));
    r
}

/// A 4096x4096 24-bit file that is only a header: GdkCompat accepts the
/// truncation (zero-filled rows), so the core must reserve 64 MiB of RGBA.
fn big_output_header() -> Vec<u8> {
    header(54, 4096, 4096, 24)
}

#[test]
fn injected_core_out_of_memory_maps_to_insufficient_memory() {
    let bytes = big_output_header();
    // Control: with the allocator healthy the same input decodes.
    let img = decode_all(&bytes).expect("control decode");
    assert_eq!((img.width, img.height), (4096, 4096));
    drop(img);
    // The 54-byte input buffer is far below 1 MiB; only the core's output
    // reservation (64 MiB) reaches the failing threshold.
    let e = failing_at_or_above(1 << 20, || decode_all(&bytes)).unwrap_err();
    assert_eq!(e.kind, ErrorKind::InsufficientMemory);
    assert_eq!(e.message, c"BMP image rejected: OutOfMemory");
}

#[test]
fn injected_core_out_of_memory_sets_gerror_insufficient_memory() {
    let quark = unsafe { gdk_pixbuf_error_quark() };
    let bytes = big_output_header();
    let (ok, code, calls) = failing_at_or_above(1 << 20, || run_ffi::<Bmp>(&bytes, 54));
    assert!(!ok);
    assert_eq!(code, Some((quark, 1)), "GDK_PIXBUF_ERROR_INSUFFICIENT_MEMORY");
    assert!(calls.log.is_empty(), "no callback may fire on failure: {}", calls.log);
}

#[test]
fn ffi_cap_overflow_sets_insufficient_memory() {
    let quark = unsafe { gdk_pixbuf_error_quark() };
    let (ok, code, _) = run_ffi::<Panicky>(&[0u8; 1025], 1025); // cap is 1024
    assert!(!ok);
    assert_eq!(code, Some((quark, 1)), "GDK_PIXBUF_ERROR_INSUFFICIENT_MEMORY");
}

// ---- reentrancy, at the extern "C" boundary --------------------------------
//
// The full close/write/close-then-write matrix through a real GdkPixbufLoader
// (with AddressSanitizer) is tests/reentrancy.rs. These two tests exercise the
// same module-state machine directly, so they run in every `cargo test` even
// where ASAN is unavailable.

/// State shared with the reentrancy callbacks: which signal triggers the nested
/// call, what the nested call did, and the context the outer frame owns.
struct Reenter {
    /// Trigger signal: 0 size_prepared, 1 area_prepared, 2 area_updated.
    trigger: u8,
    /// 0 = nested stop_load (close), 1 = nested load_increment (write).
    action: u8,
    fired: bool,
    outer_context: *mut c_void,
    nested_close_rc: Option<bool>,
    nested_write_rc: Option<bool>,
    nested_write_code: Option<c_int>,
}

impl Reenter {
    fn new(trigger: u8, action: u8) -> Self {
        Reenter {
            trigger,
            action,
            fired: false,
            outer_context: ptr::null_mut(),
            nested_close_rc: None,
            nested_write_rc: None,
            nested_write_code: None,
        }
    }

    /// Run the nested call once, when `signal` is the trigger.
    unsafe fn maybe(&mut self, signal: u8) {
        if self.fired || self.trigger != signal {
            return;
        }
        self.fired = true;
        let mut err: *mut GError = ptr::null_mut();
        if self.action == 0 {
            // Nested stop_load: must not free the context the outer call owns.
            let rc = unsafe { ffi::stop_load::<Bmp>(self.outer_context, &mut err) };
            self.nested_close_rc = Some(rc != 0);
        } else {
            const EXTRA: &[u8] = b"BMmore bytes fed from a callback";
            // Nested load_increment: must be refused, not mutate or alias.
            let rc = unsafe {
                ffi::load_increment::<Bmp>(self.outer_context, EXTRA.as_ptr(), EXTRA.len() as c_uint, &mut err)
            };
            self.nested_write_rc = Some(rc != 0);
            if !err.is_null() {
                self.nested_write_code = Some(unsafe { (*err).code });
                unsafe { g_error_free(err) };
            }
        }
    }
}

unsafe extern "C" fn reenter_size(_w: *mut c_int, _h: *mut c_int, ud: *mut c_void) {
    unsafe { (*(ud as *mut Reenter)).maybe(0) };
}
unsafe extern "C" fn reenter_prepared(_p: *mut GdkPixbuf, _a: *mut GdkPixbufAnimation, ud: *mut c_void) {
    unsafe { (*(ud as *mut Reenter)).maybe(1) };
}
unsafe extern "C" fn reenter_updated(
    _p: *mut GdkPixbuf,
    _x: c_int,
    _y: c_int,
    _w: c_int,
    _h: c_int,
    ud: *mut c_void,
) {
    unsafe { (*(ud as *mut Reenter)).maybe(2) };
}

/// begin, write the whole tiny BMP, then stop. Returns whether the outer load
/// succeeded; the nested call's effect is recorded in `reenter`.
fn run_reenter(reenter: &mut Reenter) -> bool {
    let bmp = tiny_bmp();
    let mut err: *mut GError = ptr::null_mut();
    unsafe {
        let ctx = ffi::begin_load::<Bmp>(
            Some(reenter_size as ffi::SizeFunc),
            Some(reenter_prepared as ffi::PreparedFunc),
            Some(reenter_updated as ffi::UpdatedFunc),
            reenter as *mut Reenter as *mut c_void,
            &mut err,
        );
        assert!(!ctx.is_null());
        reenter.outer_context = ctx;
        let wrote = ffi::load_increment::<Bmp>(ctx, bmp.as_ptr(), bmp.len() as c_uint, &mut err) != 0;
        let mut err2: *mut GError = ptr::null_mut();
        let stopped = ffi::stop_load::<Bmp>(ctx, &mut err2) != 0;
        if !err2.is_null() {
            g_error_free(err2);
        }
        if !err.is_null() {
            g_error_free(err);
        }
        wrote && stopped
    }
}

/// A nested stop_load (a handler calls close) must not free the context twice;
/// it returns success, and the outer stop_load still completes.
#[test]
fn nested_stop_load_does_not_double_free() {
    for trigger in [0u8, 1, 2] {
        let mut r = Reenter::new(trigger, 0);
        assert!(run_reenter(&mut r), "trigger {trigger}: outer load failed");
        assert!(r.fired, "trigger {trigger}: handler did not fire");
        assert_eq!(r.nested_close_rc, Some(true), "trigger {trigger}: nested stop_load must succeed");
    }
}

/// A nested load_increment (a handler calls write) must be refused with
/// GDK_PIXBUF_ERROR_FAILED and must not touch the outer accumulation.
#[test]
fn nested_load_increment_is_refused() {
    for trigger in [0u8, 1, 2] {
        let mut r = Reenter::new(trigger, 1);
        assert!(run_reenter(&mut r), "trigger {trigger}: outer load failed");
        assert!(r.fired, "trigger {trigger}: handler did not fire");
        assert_eq!(r.nested_write_rc, Some(false), "trigger {trigger}: nested write must be refused");
        assert_eq!(r.nested_write_code, Some(5), "trigger {trigger}: expected GDK_PIXBUF_ERROR_FAILED");
    }
}

#[test]
fn fill_info_allocates_nothing_and_exposes_static_tables() {
    use std::ffi::CStr;
    let mut info: ffi::GdkPixbufFormat = unsafe { std::mem::zeroed() };
    let (_, peak) = peak_during(|| unsafe { pixbufloader_verified_bmp::fill_info(&mut info) });
    assert_eq!(peak, 0, "fill_info allocated");
    unsafe {
        assert_eq!(CStr::from_ptr(info.name), c"bmp");
        let sig = std::slice::from_raw_parts(info.signature, 2);
        assert_eq!(CStr::from_ptr(sig[0].prefix), c"BM");
        assert!(sig[0].mask.is_null());
        assert_eq!(sig[0].relevance, 100);
        assert!(sig[1].prefix.is_null());
        let mimes: Vec<&CStr> = (0..).map(|i| *info.mime_types.add(i)).take_while(|p| !p.is_null()).map(|p| CStr::from_ptr(p)).collect();
        assert_eq!(mimes, [c"image/bmp", c"image/x-bmp", c"image/x-MS-bmp"]);
        let exts: Vec<&CStr> = (0..).map(|i| *info.extensions.add(i)).take_while(|p| !p.is_null()).map(|p| CStr::from_ptr(p)).collect();
        assert_eq!(exts, [c"bmp"]);
    }
}
