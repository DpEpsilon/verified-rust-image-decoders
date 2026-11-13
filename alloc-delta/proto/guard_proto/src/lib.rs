//! Prototype: the sealed allocation-log guard (SPEC.md §3) plus a toy decoder
//! that uses it for one output reservation (SPEC.md §4, DIB `Output`).

use vstd::prelude::*;

pub mod alloc_guard;

/// The toy decoder's theorem layer: `G`, `Prog`, `PreAlloc`, `Eq_f`, `Req`.
pub mod toy;
pub mod toy_entries;
