//! Prototype of ICO's side of the guard: a second crate, a second sealed log
//! type (`guard_proto_dep::alloc_guard::AllocLog`), and `import_dib` consuming
//! DIB's (`guard_proto`) log across the crate boundary (SPEC.md §3.1, gate F3c).

use vstd::prelude::*;

pub mod alloc_guard;
pub mod ico;
