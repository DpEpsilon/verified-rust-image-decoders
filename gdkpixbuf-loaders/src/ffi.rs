//! The whole unsafe surface of the loader: hand-written declarations of the
//! gdk-pixbuf module ABI and the five library functions the module calls, and
//! the generic `extern "C"` entry points gdk-pixbuf calls.
//!
//! The declarations were written against the installed 2.42.12 headers
//! (`gdk-pixbuf/gdk-pixbuf-io.h`, `gdk-pixbuf-core.h`, `glib/gerror.h`); no
//! gdk-pixbuf source was consulted or copied. glib-sys/gdk-pixbuf-sys are not
//! in the offline cargo cache, hence the hand-written subset.
//!
//! Every entry point runs its Rust body under `catch_unwind`; a panic becomes a
//! `GDK_PIXBUF_ERROR_FAILED` GError and never unwinds into C.

#![allow(non_camel_case_types)]

use std::alloc::{alloc as raw_alloc, dealloc as raw_dealloc, Layout};
use std::ffi::{c_char, c_int, c_uint, c_void, CStr};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;

use crate::shim::{self, Accumulator, ErrorKind, Format, Image, LoadError};

pub type gboolean = c_int;
pub type gpointer = *mut c_void;
pub type GQuark = u32;

const TRUE: gboolean = 1;
const FALSE: gboolean = 0;

/// `GdkColorspace`: only GDK_COLORSPACE_RGB (= 0) exists.
const GDK_COLORSPACE_RGB: c_int = 0;
/// `GdkPixbufError` values (gdk-pixbuf-core.h enum order).
const GDK_PIXBUF_ERROR_CORRUPT_IMAGE: c_int = 0;
const GDK_PIXBUF_ERROR_INSUFFICIENT_MEMORY: c_int = 1;
const GDK_PIXBUF_ERROR_FAILED: c_int = 5;
/// `GdkPixbufFormatFlags::GDK_PIXBUF_FORMAT_THREADSAFE`. The module keeps no
/// global state, so gdk-pixbuf need not serialize calls into it.
const GDK_PIXBUF_FORMAT_THREADSAFE: u32 = 1 << 2;

#[repr(C)]
pub struct GError {
    pub domain: GQuark,
    pub code: c_int,
    pub message: *mut c_char,
}

#[repr(C)]
pub struct GdkPixbuf {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct GdkPixbufAnimation {
    _opaque: [u8; 0],
}

pub type SizeFunc = unsafe extern "C" fn(width: *mut c_int, height: *mut c_int, user_data: gpointer);
pub type PreparedFunc =
    unsafe extern "C" fn(pixbuf: *mut GdkPixbuf, anim: *mut GdkPixbufAnimation, user_data: gpointer);
pub type UpdatedFunc = unsafe extern "C" fn(
    pixbuf: *mut GdkPixbuf,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    user_data: gpointer,
);
pub type BeginLoadFunc = unsafe extern "C" fn(
    Option<SizeFunc>,
    Option<PreparedFunc>,
    Option<UpdatedFunc>,
    gpointer,
    *mut *mut GError,
) -> gpointer;
pub type StopLoadFunc = unsafe extern "C" fn(gpointer, *mut *mut GError) -> gboolean;
pub type IncrementLoadFunc =
    unsafe extern "C" fn(gpointer, *const u8, c_uint, *mut *mut GError) -> gboolean;
/// Slots this module leaves NULL; only their pointer size matters.
pub type UnusedFunc = Option<unsafe extern "C" fn()>;

#[repr(C)]
pub struct GdkPixbufModulePattern {
    pub prefix: *mut c_char,
    pub mask: *mut c_char,
    pub relevance: c_int,
}

#[repr(C)]
pub struct GdkPixbufFormat {
    pub name: *mut c_char,
    pub signature: *mut GdkPixbufModulePattern,
    pub domain: *mut c_char,
    pub description: *mut c_char,
    pub mime_types: *mut *mut c_char,
    pub extensions: *mut *mut c_char,
    pub flags: u32,
    pub disabled: gboolean,
    pub license: *mut c_char,
}

/// `struct _GdkPixbufModule`, field for field.
#[repr(C)]
pub struct GdkPixbufModule {
    pub module_name: *mut c_char,
    pub module_path: *mut c_char,
    pub module: gpointer,
    pub info: *mut GdkPixbufFormat,
    pub load: UnusedFunc,
    pub load_xpm_data: UnusedFunc,
    pub begin_load: Option<BeginLoadFunc>,
    pub stop_load: Option<StopLoadFunc>,
    pub load_increment: Option<IncrementLoadFunc>,
    pub load_animation: UnusedFunc,
    pub save: UnusedFunc,
    pub save_to_callback: UnusedFunc,
    pub is_save_option_supported: UnusedFunc,
    pub _reserved1: UnusedFunc,
    pub _reserved2: UnusedFunc,
    pub _reserved3: UnusedFunc,
    pub _reserved4: UnusedFunc,
}

extern "C" {
    fn g_set_error_literal(err: *mut *mut GError, domain: GQuark, code: c_int, message: *const c_char);
    fn gdk_pixbuf_error_quark() -> GQuark;
    fn gdk_pixbuf_new(
        colorspace: c_int,
        has_alpha: gboolean,
        bits_per_sample: c_int,
        width: c_int,
        height: c_int,
    ) -> *mut GdkPixbuf;
    fn gdk_pixbuf_get_pixels_with_length(pixbuf: *const GdkPixbuf, length: *mut c_uint) -> *mut u8;
    fn gdk_pixbuf_get_rowstride(pixbuf: *const GdkPixbuf) -> c_int;
    fn g_object_unref(object: gpointer);
}

/// Lifecycle states of a load context, kept in a plain `u8` beside the context
/// storage so that a *nested* module entry point (one entered from a C callback
/// that an outer `stop_load`/`load_increment` is still inside) can read and
/// change it without borrowing the `Context` the outer frame holds. The C
/// callbacks gdk-pixbuf passes us are reentrancy-safe only in this specific
/// way, and gdk-pixbuf itself refuses to `write` after `closed` and to re-enter
/// `stop_load` after `closed`; we mirror that.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum LoadState {
    /// Between `begin_load` and the first `stop_load`; the only state in which
    /// `load_increment` accepts data.
    Active = 0,
    /// An outermost `stop_load` is running its callbacks; its context is valid
    /// and will be freed exactly once when it returns. A nested `stop_load`
    /// returns success without freeing; a nested `load_increment` is refused.
    Stopping = 1,
    /// The context has been freed; this byte (inside the allocation) must never
    /// be read again.
    Dead = 2,
}

impl LoadState {
    fn from_u8(v: u8) -> LoadState {
        match v {
            0 => LoadState::Active,
            1 => LoadState::Stopping,
            _ => LoadState::Dead,
        }
    }
}

/// The per-load state handed to gdk-pixbuf as the opaque `context`.
///
/// One allocation holds a `u8` state byte followed by the `Context`, the byte
/// at the allocation's start and `Context` at the first aligned offset after it
/// (see `ctx_ptr`). The byte is deliberate: a *nested* entry point (one entered
/// from a callback that an outer `stop_load` is still inside) reads and writes
/// it through a raw pointer without forming any reference to `Context`, which
/// the outer frame may be borrowing. Because the state is not a field of
/// `Context`, `Context` needs no interior mutability at all: it is borrowed
/// mutably only while its owning frame holds the matching state, and never
/// while a nested frame runs.
struct Context {
    size_func: Option<SizeFunc>,
    prepared_func: Option<PreparedFunc>,
    updated_func: Option<UpdatedFunc>,
    user_data: gpointer,
    acc: Accumulator,
    /// Set when a `load_increment` failed; `stop_load` reports it again.
    failed: Option<LoadError>,
}

/// The byte layout of one allocation: a `u8` state, then a `Context`.
#[repr(C)]
struct ContextAlloc {
    state: u8,
    ctx: Context,
}

/// Bytes to reserve before the `Context` so the state byte is addressable and
/// `Context` keeps its natural alignment.
const STATE_PAD: usize = align_of::<Context>() - 1;
const CTX_ALLOC_SIZE: usize = 1 + STATE_PAD + size_of::<Context>();
const CTX_LAYOUT: Layout = match Layout::from_size_align(CTX_ALLOC_SIZE, align_of::<Context>()) {
    Ok(l) => l,
    Err(_) => panic!("invalid context layout"),
};

/// The layout `ContextAlloc` actually has must match the arithmetic above (a
/// `u8` then padding to `align_of::<Context>()` then `Context`), or `ctx_ptr`
/// would point at the wrong bytes. Checked at compile time.
const _: () = assert!(size_of::<ContextAlloc>() == CTX_ALLOC_SIZE);
const _: () = assert!(align_of::<ContextAlloc>() == align_of::<Context>());

/// The state byte's address: the allocation start.
#[inline]
fn state_ptr(alloc: *mut u8) -> *mut u8 {
    alloc
}

/// The `Context`'s address: aligned up past the byte and its padding. `1 +
/// STATE_PAD` is a multiple of `align_of::<Context>()`, and the allocation is
/// aligned to `align_of::<Context>()`, so the sum is a valid `Context` address.
#[inline]
fn ctx_ptr(alloc: *mut u8) -> *mut Context {
    unsafe { alloc.add(1 + STATE_PAD) }.cast()
}

/// Read the ticking state of a load, without borrowing the context.
///
/// SAFETY (each caller): `alloc` is the pointer `begin_load` returned and
/// neither this read nor the state byte is `Dead` here, so the byte is live.
#[inline]
unsafe fn load_state(alloc: *const u8) -> LoadState {
    LoadState::from_u8(unsafe { *state_ptr(alloc.cast_mut()) })
}

/// Move the state byte to `s`; the caller must be the one that owns the
/// transition (see each entry point).
///
/// SAFETY (each caller): as `load_state`.
#[inline]
unsafe fn store_state(alloc: *mut u8, s: LoadState) {
    unsafe { *state_ptr(alloc) = s as u8 }
}

/// Allocate a zeroed context and set the state byte to `Active`.
///
/// All-bits-zero is the error layout for every `Context` field (`Option::None`,
/// empty `Accumulator`), so zeroing the block yields a valid context; setting
/// the state byte afterwards is the only other step and no `Context` value is
/// constructed in Rust before the allocation exists.
///
/// SAFETY: the returned pointer is the sole owner of `CTX_LAYOUT` and has the
/// layout `ctx_ptr`/`state_ptr` expect. Returns null when the allocator refuses
/// (no abort), which `begin_load` turns into `GDK_PIXBUF_ERROR_INSUFFICIENT_MEMORY`.
unsafe fn alloc_context() -> *mut u8 {
    // SAFETY: CTX_ALLOC_SIZE is at least 1, so CTX_LAYOUT has a non-zero size.
    let p = unsafe { raw_alloc(CTX_LAYOUT) };
    if p.is_null() {
        return p;
    }
    // SAFETY: p has at least 1 + STATE_PAD + size_of::<Context>() writable
    // bytes; zero them, then set the live state byte. `Context`'s all-zero
    // image is valid (Option::None, Vec::new); the `_` const asserts above
    // checked the layout arithmetic.
    unsafe {
        ptr::write_bytes(p, 0, CTX_ALLOC_SIZE);
        store_state(p, LoadState::Active);
    }
    p
}

/// Free a context allocation. The caller must have set the state byte to `Dead`
/// and must not touch the allocation afterwards.
///
/// SAFETY (each caller): `alloc` came from `alloc_context`, and no `&`/`&mut`
/// `Context` into it is live across this call.
unsafe fn free_context(alloc: *mut u8) {
    unsafe {
        // Drop the accumulated buffer and any stored message (Vec fields hold
        // heap allocations) before returning the block itself.
        ptr::drop_in_place(ctx_ptr(alloc));
        raw_dealloc(alloc, CTX_LAYOUT);
    }
}

fn panic_error() -> LoadError {
    LoadError::new(ErrorKind::Failed, c"internal error in image loader (caught panic)")
}

/// Store `e` in `*error` unless the caller passed NULL or it is already set.
///
/// The message is the `'static` `e.message` (see `shim::LoadError`), so this
/// function does not allocate: it hands gdk-pixbuf the pointer to a string that
/// lives for the program's lifetime, and `g_set_error_literal` copies it. No
/// image byte ever reaches this path, and no `format!` is used.
unsafe fn set_error(error: *mut *mut GError, e: &LoadError) {
    // SAFETY: gdk-pixbuf passes either NULL or a valid GError** (the
    // GError-convention "return location"); we read `*error` only after the
    // NULL check, and `e.message` is a 'static CStr, so its pointer is valid
    // and NUL-terminated for the whole call (g_set_error_literal copies it).
    unsafe {
        if error.is_null() || !(*error).is_null() {
            return;
        }
        let code = match e.kind {
            ErrorKind::Corrupt => GDK_PIXBUF_ERROR_CORRUPT_IMAGE,
            ErrorKind::InsufficientMemory => GDK_PIXBUF_ERROR_INSUFFICIENT_MEMORY,
            ErrorKind::Failed => GDK_PIXBUF_ERROR_FAILED,
        };
        g_set_error_literal(error, gdk_pixbuf_error_quark(), code, e.message.as_ptr());
    }
}

/// Owns one reference to a GdkPixbuf.
struct PixbufRef(*mut GdkPixbuf);
impl Drop for PixbufRef {
    fn drop(&mut self) {
        // SAFETY: self.0 is the non-NULL pixbuf returned by gdk_pixbuf_new, to
        // which this guard owns the single reference it was created with.
        unsafe { g_object_unref(self.0.cast()) }
    }
}

/// size_func, then (unless it requested 0x0) prepared_func with a filled
/// pixbuf, then one updated_func covering the whole image.
///
/// The callbacks and `user_data` are passed in by value, not read from a
/// `&Context`, because each callback can re-enter the loader (`stop_load` or
/// `load_increment` per the gdk-pixbuf contract). `stop_load` puts the context
/// in `Stopping` before it calls here and holds no context reference while a
/// callback runs, so no nested call can free, mutate or alias anything this
/// function reaches.
unsafe fn emit(
    img: Image,
    size_func: Option<SizeFunc>,
    prepared_func: Option<PreparedFunc>,
    updated_func: Option<UpdatedFunc>,
    user_data: gpointer,
) -> Result<(), LoadError> {
    // shim::finish guarantees 1 <= width, height <= i32::MAX.
    let (w, h) = (img.width as c_int, img.height as c_int);
    if let Some(size_func) = size_func {
        let (mut rw, mut rh) = (w, h);
        // SAFETY: size_func and user_data are exactly what gdk-pixbuf passed to
        // begin_load; the two pointers are to live locals.
        unsafe { size_func(&mut rw, &mut rh, user_data) };
        // The size_func handler may have re-entered the loader. We hold no
        // context reference, so nothing here is invalidated by that; the values
        // we still need (`img`, the callback pointers) are all copied out.
        if rw == 0 || rh == 0 {
            // gdk-pixbuf-io.h: a 0 size means "closing soon, allocate nothing".
            return Ok(());
        }
        // Any other requested size is ignored; GdkPixbufLoader scales afterwards.
    }
    // SAFETY: plain constructor call with validated, positive dimensions;
    // returns NULL on allocation failure, which is checked.
    let raw = unsafe { gdk_pixbuf_new(GDK_COLORSPACE_RGB, img.has_alpha as gboolean, 8, w, h) };
    if raw.is_null() {
        return Err(LoadError::new(
            ErrorKind::InsufficientMemory,
            c"not enough memory to allocate the image",
        ));
    }
    let pixbuf = PixbufRef(raw);
    let mut len: c_uint = 0;
    // SAFETY: pixbuf.0 is a live pixbuf we own; gdk_pixbuf_new-created
    // pixbufs have a mutable pixel store of exactly `len` bytes, valid while
    // the pixbuf lives and not aliased by anyone else until prepared_func runs
    // (we finish writing before that). Rowstride is non-negative by contract;
    // copy_into rejects any geometry that does not fit `len`.
    unsafe {
        let px = gdk_pixbuf_get_pixels_with_length(pixbuf.0, &mut len);
        let stride = gdk_pixbuf_get_rowstride(pixbuf.0);
        if px.is_null() || stride < 0 {
            return Err(LoadError::new(ErrorKind::Failed, c"pixbuf has no pixel store"));
        }
        let dst = std::slice::from_raw_parts_mut(px, len as usize);
        shim::copy_into(&img, dst, stride as usize)?;
    }
    drop(img);
    // SAFETY: callbacks and user_data as given to begin_load; the pixbuf is
    // valid for the duration of the calls (the loader takes its own ref in
    // prepared_func; we drop ours afterwards); anim may be NULL per the header.
    // No reference to the context is alive across either call.
    unsafe {
        if let Some(prepared) = prepared_func {
            prepared(pixbuf.0, ptr::null_mut(), user_data);
        }
        if let Some(updated) = updated_func {
            updated(pixbuf.0, 0, 0, w, h, user_data);
        }
    }
    Ok(())
}

pub unsafe extern "C" fn begin_load<F: Format>(
    size_func: Option<SizeFunc>,
    prepared_func: Option<PreparedFunc>,
    updated_func: Option<UpdatedFunc>,
    user_data: gpointer,
    error: *mut *mut GError,
) -> gpointer {
    let r = catch_unwind(AssertUnwindSafe(|| -> Result<*mut u8, LoadError> {
        // SAFETY: alloc_context returns a live, Active, zeroed allocation of
        // the documented layout, or NULL — which it does not abort on.
        let alloc = unsafe { alloc_context() };
        if alloc.is_null() {
            return Err(LoadError::new(ErrorKind::InsufficientMemory, c"not enough memory for the loader context"));
        }
        // SAFETY: `alloc` is a fresh Live allocation; ctx_ptr gives its sole
        // `&mut Context`, which we write once and then let go before returning
        // the raw pointer.
        unsafe {
            let c = &mut *ctx_ptr(alloc);
            c.size_func = size_func;
            c.prepared_func = prepared_func;
            c.updated_func = updated_func;
            c.user_data = user_data;
            c.acc = Accumulator::new(F::MAX_INPUT);
            c.failed = None;
        }
        Ok(alloc)
    }));
    match r {
        Ok(Ok(alloc)) => alloc.cast(),
        Ok(Err(e)) => {
            // SAFETY: `error` is gdk-pixbuf's GError return location.
            unsafe { set_error(error, &e) };
            ptr::null_mut()
        }
        Err(_) => {
            // SAFETY: `error` is gdk-pixbuf's GError return location.
            unsafe { set_error(error, &panic_error()) };
            ptr::null_mut()
        }
    }
}

pub unsafe extern "C" fn load_increment<F: Format>(
    context: gpointer,
    buf: *const u8,
    size: c_uint,
    error: *mut *mut GError,
) -> gboolean {
    if context.is_null() {
        // SAFETY: `error` is gdk-pixbuf's GError return location.
        unsafe { set_error(error, &LoadError::new(ErrorKind::Failed, c"no loader context")) };
        return FALSE;
    }
    let alloc = context.cast::<u8>();
    // SAFETY: `context` is the pointer begin_load returned. The state byte is
    // live unless the context was freed, and gdk-pixbuf never calls into a
    // freed context (it guards every write with priv->closed and nulls nothing
    // else); a nested call only ever sees Active or Stopping, both live.
    let state = unsafe { load_state(alloc) };
    if state != LoadState::Active {
        // A write from inside a stop_load callback: gdk-pixbuf's own
        // `g_return_val_if_fail (priv->closed == FALSE)` is not yet true
        // because it sets priv->closed after stop_load returns, so we refuse it
        // here instead. We set GDK_PIXBUF_ERROR_FAILED so gdk-pixbuf's failure
        // path does not warn that the module "didn't give a reason"; the context
        // is not mutated. The `error` slot is the app's fresh return location.
        // SAFETY: `error` is gdk-pixbuf's GError return location.
        unsafe { set_error(error, &LoadError::new(ErrorKind::Failed, c"loader is closing")) };
        return FALSE;
    }
    let chunk: &[u8] = if size == 0 || buf.is_null() {
        &[]
    } else {
        // SAFETY: the header contract: `buf` holds `size` readable bytes for
        // the duration of the call; we copy them out before returning.
        unsafe { std::slice::from_raw_parts(buf, size as usize) }
    };
    // SAFETY: state is Active, so this call holds the sole live reference to
    // the context. `stop_load` changes the state to Stopping *before* it
    // borrows, so no outer `&mut` can be live while we hold this one.
    let ctx = unsafe { &mut *ctx_ptr(alloc) };
    let r = catch_unwind(AssertUnwindSafe(|| {
        if let Some(e) = &ctx.failed {
            return Err(e.clone());
        }
        ctx.acc.push(chunk)
    }))
    .unwrap_or_else(|_| Err(panic_error()));
    match r {
        Ok(()) => TRUE,
        Err(e) => {
            // Record the failure before touching `error`, so `ctx` is not live
            // across `set_error` (which may call back into C's allocator).
            ctx.acc = Accumulator::new(0); // release the buffer now
            ctx.failed = Some(e.clone());
            // SAFETY: `ctx` is no longer used; `error` is gdk-pixbuf's GError
            // return location.
            unsafe { set_error(error, &e) };
            FALSE
        }
    }
}

pub unsafe extern "C" fn stop_load<F: Format>(context: gpointer, error: *mut *mut GError) -> gboolean {
    if context.is_null() {
        // SAFETY: `error` is gdk-pixbuf's GError return location.
        unsafe { set_error(error, &LoadError::new(ErrorKind::Failed, c"no loader context")) };
        return FALSE;
    }
    let alloc = context.cast::<u8>();
    // SAFETY: as in load_increment: `context` is a live pointer unless it was
    // freed, and gdk-pixbuf does not call into a freed context.
    if unsafe { load_state(alloc) } != LoadState::Active {
        // Nested stop_load (a handler called gdk_pixbuf_loader_close): the
        // outermost stop_load owns the context and will free it exactly once
        // after every callback returns. Report success and touch nothing.
        return TRUE;
    }
    // Claim ownership before any callback can run. A nested stop_load then sees
    // Stopping and returns TRUE without freeing; only this outermost call, after
    // `emit` has returned and no context reference exists, frees the context.
    //
    // SAFETY: the state byte is live (checked above); writing it changes no
    // live Rust reference, since nothing into `alloc` is borrowed yet.
    unsafe { store_state(alloc, LoadState::Stopping) };
    let taken = catch_unwind(AssertUnwindSafe(|| {
        // The `&mut Context` below lives only inside this block: every value
        // `emit` needs is copied out of it first, so no reference into the
        // context is alive across a C callback. That, plus the Stopping state,
        // is what lets a nested entry point return without touching the context
        // while this frame still owns it. `alloc` is a Copy raw pointer, so the
        // closure holds no reference of its own.
        let (img, size_func, prepared_func, updated_func, user_data) = {
            // SAFETY: the state is Stopping, so this is the sole live reference
            // to the context; it ends at the close of this block, before any
            // callback can run.
            let ctx = unsafe { &mut *ctx_ptr(alloc) };
            if let Some(e) = ctx.failed.take() {
                return Err(e);
            }
            let acc = std::mem::replace(&mut ctx.acc, Accumulator::new(0));
            let img = shim::finish::<F>(acc)?;
            (img, ctx.size_func, ctx.prepared_func, ctx.updated_func, ctx.user_data)
        };
        // SAFETY: the four values are copies of what begin_load stored;
        // `emit` holds no context reference across its callbacks.
        unsafe { emit(img, size_func, prepared_func, updated_func, user_data) }
    }))
    .unwrap_or_else(|_| Err(panic_error()));
    // SAFETY: no `&`/`&mut Context` is live now. Mark the allocation Dead so a
    // stray later read of the state byte cannot misread a freed context, then
    // free it exactly once.
    unsafe {
        store_state(alloc, LoadState::Dead);
        free_context(alloc);
    }
    match taken {
        Ok(()) => TRUE,
        Err(e) => {
            // SAFETY: `error` is gdk-pixbuf's GError return location. `e` is by
            // value and its message is `'static`, so nothing borrows `alloc`.
            unsafe { set_error(error, &e) };
            FALSE
        }
    }
}

/// `fill_vtable`: installs the incremental API only. Without `load`,
/// gdk_pixbuf_new_from_file feeds the file through the incremental path.
pub unsafe fn fill_vtable<F: Format>(module: *mut GdkPixbufModule) {
    if module.is_null() {
        return;
    }
    // SAFETY: gdk-pixbuf passes its own live, zero-initialized module struct;
    // the layout above matches gdk-pixbuf-io.h. Only our three slots are written.
    unsafe {
        (*module).begin_load = Some(begin_load::<F>);
        (*module).stop_load = Some(stop_load::<F>);
        (*module).load_increment = Some(load_increment::<F>);
    }
}

fn leak_strv(items: &[&'static CStr]) -> *mut *mut c_char {
    let v: Vec<*mut c_char> =
        items.iter().map(|s| s.as_ptr() as *mut c_char).chain(std::iter::once(ptr::null_mut())).collect();
    Box::leak(v.into_boxed_slice()).as_mut_ptr()
}

/// `fill_info`: points the format record at static strings. The two small
/// pointer arrays are leaked; gdk-pixbuf calls this once per module per
/// process (and not at all when it reads the format from loaders.cache).
pub unsafe fn fill_info<F: Format>(info: *mut GdkPixbufFormat) {
    if info.is_null() {
        return;
    }
    let _ = catch_unwind(|| {
        let sig: Vec<GdkPixbufModulePattern> = F::SIGNATURE
            .iter()
            .map(|p| GdkPixbufModulePattern {
                prefix: p.prefix.as_ptr() as *mut c_char,
                mask: p.mask.map_or(ptr::null_mut(), |m| m.as_ptr() as *mut c_char),
                relevance: p.relevance,
            })
            .chain(std::iter::once(GdkPixbufModulePattern {
                prefix: ptr::null_mut(),
                mask: ptr::null_mut(),
                relevance: 0,
            }))
            .collect();
        let sig = Box::leak(sig.into_boxed_slice()).as_mut_ptr();
        let mimes = leak_strv(F::MIME_TYPES);
        let exts = leak_strv(F::EXTENSIONS);
        // SAFETY: `info` is gdk-pixbuf's live GdkPixbufFormat. All strings are
        // 'static and gdk-pixbuf only reads module-filled fields (it never
        // frees or writes them), so casting away const is sound.
        unsafe {
            (*info).name = F::NAME.as_ptr() as *mut c_char;
            (*info).signature = sig;
            (*info).domain = F::DOMAIN.as_ptr() as *mut c_char;
            (*info).description = F::DESCRIPTION.as_ptr() as *mut c_char;
            (*info).mime_types = mimes;
            (*info).extensions = exts;
            (*info).flags = GDK_PIXBUF_FORMAT_THREADSAFE;
            (*info).license = F::LICENSE.as_ptr() as *mut c_char;
        }
    });
}

/// Export the two `fill_*` symbols gdk-pixbuf resolves in a module.
/// Use exactly once per cdylib.
#[macro_export]
macro_rules! export_loader {
    ($fmt:ty) => {
        /// # Safety
        /// Called by gdk-pixbuf with its own module struct.
        #[no_mangle]
        pub unsafe extern "C" fn fill_vtable(module: *mut $crate::ffi::GdkPixbufModule) {
            unsafe { $crate::ffi::fill_vtable::<$fmt>(module) }
        }
        /// # Safety
        /// Called by gdk-pixbuf with its own format struct.
        #[no_mangle]
        pub unsafe extern "C" fn fill_info(info: *mut $crate::ffi::GdkPixbufFormat) {
            unsafe { $crate::ffi::fill_info::<$fmt>(info) }
        }
    };
}
