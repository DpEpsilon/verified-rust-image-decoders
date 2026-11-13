//! gdk-pixbuf loader module for BMP backed by the verified `dib_verus` core.
//!
//! * [`shim`]: safe, format-generic accumulation and pixel copy.
//! * [`ffi`]: the only module with `unsafe`; the gdk-pixbuf module ABI.
//! * [`bmp`]: the BMP `Format` (metadata + `dib_verus::try_decode_with_policy`, GdkCompat).

#![deny(unsafe_op_in_unsafe_fn)]

pub mod bmp;
pub mod ffi;
pub mod shim;

export_loader!(crate::bmp::Bmp);
