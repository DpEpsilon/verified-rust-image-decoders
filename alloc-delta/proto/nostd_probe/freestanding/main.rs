//! Freestanding link test on the host target: no std, no libc, no crt.
//! Links nostd_core (which pulls vstd[alloc]) into a `#![no_main]` binary with
//! its own `_start`, bump allocator, panic handler and mem* functions. If any
//! symbol from libc or std were required, the link would fail with an undefined
//! reference.
#![no_std]
#![no_main]
#![allow(clashing_extern_declarations, incompatible_msrv)]

extern crate alloc;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;

struct Bump { buf: UnsafeCell<[u8; 1 << 16]>, off: UnsafeCell<usize> }
unsafe impl Sync for Bump {}
unsafe impl GlobalAlloc for Bump {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let off = *self.off.get();
        let start = (off + l.align() - 1) & !(l.align() - 1);
        if start + l.size() > (1 << 16) { return core::ptr::null_mut(); }
        *self.off.get() = start + l.size();
        (self.buf.get() as *mut u8).add(start)
    }
    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
}
#[global_allocator]
static A: Bump = Bump { buf: UnsafeCell::new([0; 1 << 16]), off: UnsafeCell::new(0) };

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

#[no_mangle] pub extern "C" fn rust_eh_personality() {}
// Landing pads in the unwind-built rlibs reference this; with panic=abort and a
// non-returning panic handler it is never executed.
#[no_mangle] pub extern "C" fn _Unwind_Resume() -> ! { loop {} }

// Minimal mem* (no libc). Volatile-free simple loops; `no_builtins` keeps the
// compiler from turning them into calls to themselves.
#[no_mangle] pub unsafe extern "C" fn memcpy(d: *mut u8, s: *const u8, n: usize) -> *mut u8 {
    let mut i = 0; while i < n { *d.add(i) = *s.add(i); i += 1; } d }
#[no_mangle] pub unsafe extern "C" fn memmove(d: *mut u8, s: *const u8, n: usize) -> *mut u8 {
    if (d as usize) < (s as usize) { memcpy(d, s, n) } else { let mut i = n; while i > 0 { i -= 1; *d.add(i) = *s.add(i); } d } }
#[no_mangle] pub unsafe extern "C" fn memset(d: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0; while i < n { *d.add(i) = c as u8; i += 1; } d }
#[no_mangle] pub unsafe extern "C" fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    let mut i = 0; while i < n { let (x, y) = (*a.add(i), *b.add(i)); if x != y { return x as i32 - y as i32; } i += 1; } 0 }
#[no_mangle] #[allow(improper_ctypes_definitions)] pub unsafe extern "C" fn bcmp(a: *const u8, b: *const u8, n: usize) -> i32 { memcmp(a, b, n) }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Reach the verified crate's public items so they are linked in.
    let v = nostd_core::build_vec(16);
    let s = nostd_core::hex_bytes(&[1, 2, 3]);
    let d = nostd_core::describe(&nostd_core::Policy {
        trailing: nostd_core::TrailingOptions::FooterOnly,
        rle_overrun: nostd_core::RleOverrunOptions::Reject,
        profile: <nostd_core::Profile as core::default::Default>::default(),
    });
    let code = (v.len() + s.len() + d.len()) as i64;
    // exit(code) via raw syscall, no libc.
    unsafe {
        core::arch::asm!("syscall", in("rax") 60, in("rdi") code, options(noreturn));
    }
}
