//! A minimal crate that links vstd, used only to exercise the vstd-version
//! provenance check. The root does not call into vstd, so the walk stays small;
//! the point is that `vstd` is a linked dependency, which the tool must see and
//! compare against the allowlist's recorded `vstd_version`.
#![allow(unused_imports)]
extern crate vstd;
use vstd::prelude::*;

pub fn c3_vstd_root(n: usize) -> usize {
    n
}
