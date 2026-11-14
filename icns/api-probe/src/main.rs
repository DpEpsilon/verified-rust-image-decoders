//! Compile-fail probe of `icns_verus`'s public `Parsed` surface.
//!
//! A bare `cargo build` must succeed: it exercises the *legal* uses of the
//! public API. Each feature below turns on one negative case, which must fail
//! to compile with the error code named in `README.md`. This is the regression
//! check for the E14 opaque handle: safe Rust outside the crate must not be able
//! to forge a `Parsed`, mutate the list a decode will use, or reach the
//! theorem-facing internals.
//!
//! The legal body proves the other half of the API: `try_parse` is callable,
//! the decoders are methods on the handle, and `entries()`/`bytes()` hand out
//! shared views.

use icns_verus::{try_parse, Error, Parsed, TryError};

fn main() {
    // A well-formed empty ICNS container: R2-R4 pass, the entry list is empty.
    let data: &[u8] = b"icns\0\0\0\x08";

    // `try_parse` is the only constructor; its result is the public handle type.
    let parsed: Parsed<'_> = match try_parse(data) {
        Ok(p) => p,
        Err(TryError::Format(e)) => panic!("unexpected format error: {}", e.name()),
        Err(TryError::OutOfMemory(_)) => panic!("unexpected OutOfMemory"),
    };

    // The three decoders are methods on the handle.
    let _ = parsed.try_decode_rgb(0);
    let _ = parsed.try_decode_mask(0);
    let _ = parsed.try_combine(None, None);

    // The shared views: an entry count and the input bytes.
    let _: &[icns_verus::Entry] = parsed.entries();
    let _: &[u8] = parsed.bytes();

    // The error type is public and spellable.
    let _ = |e: Error| e.name();

    // `Parsed` is `Send + Sync`; a caller can move it across threads.
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Parsed<'static>>();

    // ---- negative cases, one per feature ---------------------------------

    // `Parsed` has private fields and no other constructor.
    #[cfg(feature = "literal")]
    {
        let _ = Parsed { bytes: data, entries: vec![] };
    }

    // No `Default`.
    #[cfg(feature = "default_impl")]
    {
        let _: Parsed = Default::default();
    }

    // No `Clone`.
    #[cfg(feature = "clone")]
    {
        let _: Parsed = parsed.clone();
    }

    // No `Copy` (it owns a `Vec<Entry>`).
    #[cfg(feature = "copy")]
    {
        fn assert_copy<T: Copy>() {}
        assert_copy::<Parsed<'static>>();
    }

    // `entries()` is a shared view: no mutation of the list a decode will use.
    #[cfg(feature = "mut_entries")]
    {
        parsed.entries()[0].payload[1] = u32::MAX;
    }

    // `bytes()` is a shared view, and the handle is not `&mut`.
    #[cfg(feature = "mut_bytes")]
    {
        parsed.bytes()[0] = 0;
    }

    // The old `(bytes, entries)` decoders and the `_logged` internals are not
    // public.
    #[cfg(feature = "old_api")]
    {
        let _ = icns_verus::try_decode_rgb(data, &[], 0);
        let _ = icns_verus::try_decode_mask(data, &[], 0);
        let _ = icns_verus::try_combine(data, &[], None, None);
        let _ = icns_verus::try_parse_logged(data);
        let _ = icns_verus::try_decode_rgb_logged(data, &[], 0);
        let _ = icns_verus::try_decode_mask_logged(data, &[], 0);
        let _ = icns_verus::try_combine_logged(data, &[], None, None);
    }

    // The proof plumbing (`spec_bytes`/`spec_entries`/`inv`) is not callable
    // outside the crate.
    #[cfg(feature = "spec_api")]
    {
        let _ = parsed.spec_bytes();
        let _ = parsed.spec_entries();
        let _ = parsed.inv();
    }

    // The handle borrows its input: it cannot outlive, or be mutated through, a
    // separate owner of the same bytes.
    #[cfg(feature = "borrow_alias")]
    {
        let mut owned = data.to_vec();
        let alias = match try_parse(&owned) {
            Ok(p) => p,
            Err(_) => panic!("control parse failed"),
        };
        owned[0] = 0;
        let _ = alias.try_decode_rgb(0);
    }
}
