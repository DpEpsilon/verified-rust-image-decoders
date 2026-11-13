//! Negative control N3: `format!` allocates a `String`.
//! Expected: allocation reached through `alloc::fmt::format` /
//! `String` growth, from `n3_format`.

pub fn n3_format(x: u32) -> String {
    format!("value={x}")
}
