//! Probe 2: what a `#![no_std]` core crate looks like for our patterns.
//!
//! This is a *copy* of the shapes in `tga/tga_verus/src/policy.rs` (the option
//! enums, the `Default` impls, the named-profile spec constructors) plus the
//! other constructs the four core crates use. The original is not touched.
//!
//! Every substitution the crate needs, with the reason:
//!
//! | original                                    | no_std form                        | why |
//! |---|---|---|
//! | `impl std::default::Default for Profile`     | `impl core::default::Default`      | `std` re-exports `core`'s trait; under `no_std` there is no `std` |
//! | `vec![..]`                                   | `alloc::vec![..]`                  | the `vec!` macro is defined in `alloc` (and re-exported by `std`) |
//! | `format!(..)` / `to_string()`                | `alloc::format!` / `alloc::string::ToString` | both are `alloc` items |
//! | `Box<dyn Error>` / `std::error::Error`       | `core::error::Error`               | `Error` moved to `core` in Rust 1.81 |
//! | `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` | unchanged                        | the derive macros resolve from `core`, no path needed |
//! | `#[cfg(test)] mod tests` with `std`          | `extern crate std;` inside the module | a `#[cfg(test)]` module may pull `std` in even in a `no_std` lib |

#![no_std]

extern crate alloc;

use vstd::prelude::*;

// The `alloc` re-exports the crate needs by name. `vec!` is a macro, so it is
// imported with `use` rather than reached by path.
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::String;

verus! {

// ---------------------------------------------------------------------------
// The option enums, copied in shape from tga/tga_verus/src/policy.rs. The
// derives resolve from `core` under no_std with no attribute change.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TrailingOptions {
    FooterOnly,
    Ignore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RleOverrunOptions {
    Reject,
    ClipPacket,
    ClipSamples,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Profile {
    Strict,
    Recommended,
    GdkCompat,
    ImageCrateCompat,
}

// The one textual change the `Default` impl needs: `std::default::Default` ->
// `core::default::Default`. The trait is the same trait; `std` merely
// re-exports it.
impl core::default::Default for Profile {
    fn default() -> Self {
        Profile::Strict
    }
}

// ---------------------------------------------------------------------------
// A struct with a `Vec` field, the shape of the decoder's model records.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub trailing: TrailingOptions,
    pub rle_overrun: RleOverrunOptions,
    pub profile: Profile,
}

pub open spec fn strict_policy() -> Policy {
    Policy {
        trailing: TrailingOptions::FooterOnly,
        rle_overrun: RleOverrunOptions::Reject,
        profile: Profile::Strict,
    }
}

// ---------------------------------------------------------------------------
// `alloc` container use: a `Vec`, a `String`, and the `vec!` macro.
// ---------------------------------------------------------------------------

/// Build a `Vec` with the `vec!` macro (alloc's, not std's) and return its
/// length. The proof obligation is the macro's own, exactly as under std.
pub fn build_vec(n: usize) -> (r: Vec<u8>)
    ensures
        r@.len() == n as nat,
{
    let v: Vec<u8> = vec![0u8; n];
    assert(v@.len() == n as nat);
    v
}

/// Build a `String` by pushing bytes, the shape of `icns::hex4` (`String::new`
/// then `push_str`), which is alloc's `String`. In a verified crate the `push`
/// calls carry vstd's `String::push` spec, so the same proof obligations arise
/// as under std.
pub fn hex_bytes(b: &[u8]) -> (s: String)
    ensures
        s@.len() == b@.len(),
{
    let mut s = String::new();
    let mut i: usize = 0;
    while i < b.len()
        invariant
            0 <= i <= b@.len(),
            s@.len() == i,
        decreases b@.len() - i,
    {
        s.push(char::from(b[i]));
        i = i + 1;
    }
    s
}

fn skeleton() { assert(1 == 0 + 1); }

} // verus!

/// `alloc::format!` outside the `verus!` block: ordinary Rust, not verified.
/// Inside `verus!` the call hits vstd's `fmt_req` precondition (run 4); that is
/// a property of Verus's `format!` support, not of `no_std`.
pub fn describe(p: &Policy) -> String {
    alloc::format!("{:?}/{:?}", p.trailing, p.rle_overrun)
}

// ---------------------------------------------------------------------------
// A test module inside a no_std library. `#[cfg(test)]` code is compiled as a
// separate crate that carries the test harness, which needs `std`; the module
// pulls it in with `extern crate std` even though the library itself is
// `#![no_std]`.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod policy_tests {
    extern crate std;

    use std::vec::Vec as StdVec;
    use super::*;

    #[test]
    fn default_profile_is_strict() {
        assert_eq!(Profile::default(), Profile::Strict);
    }

    #[test]
    fn vec_len_matches() {
        // Under cfg(test) std is available, so the std `vec!` works too.
        let v: StdVec<u8> = std::vec![1, 2, 3];
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn format_produces_bytes() {
        let p = Policy {
            trailing: TrailingOptions::FooterOnly,
            rle_overrun: RleOverrunOptions::Reject,
            profile: Profile::Strict,
        };
        let s = describe(&p);
        assert!(!s.is_empty());
    }

    // A test that reaches an alloc-only DefId via a std path.
    #[test]
    fn string_from_std() {
        let s: std::string::String = std::string::String::from("x");
        assert_eq!(s.len(), 1);
    }
}
