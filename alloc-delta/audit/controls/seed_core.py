#!/usr/bin/env python3
"""Make a seeded copy of a real core for the seeded-violation controls.

Usage: seed_core.py <core> <seed> <dest-dir>

Copies <repo>/<core>/<core>_verus (without target/) to <dest-dir>/<core>_verus
and, unless seed is `none`, inserts one call `let _seed = seeded_alloc(n);`
on the production reservation path, immediately before the core's guard call,
plus a plain (non-Verus) function `seeded_alloc` at the end of the module that
allocates outside the guard with the seeded construct. The copy's
alloc_guard/ files are untouched, so its guard hashes still match.
"""
import os
import shutil
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))

SEEDS = {
    "vec_zeroed": "let v = alloc::vec![0u8; n]; core::hint::black_box(v.len())",
    "with_capacity": "let v: alloc::vec::Vec<u8> = alloc::vec::Vec::with_capacity(n); core::hint::black_box(v.capacity())",
    "box_new": "let b = alloc::boxed::Box::new(n); core::hint::black_box(*b)",
    "push": "let mut v: alloc::vec::Vec<u8> = alloc::vec::Vec::new(); v.push(n as u8); core::hint::black_box(v.len())",
    "string_from": "let s = alloc::string::String::from(\"seed\"); core::hint::black_box(s.len() + n)",
    "format": "let s = alloc::format!(\"{}\", n); core::hint::black_box(s.len())",
    "to_vec": "let a = [1u8, 2, 3]; let v = a[..].to_vec(); core::hint::black_box(v.len() + n)",
    "collect": "let v: alloc::vec::Vec<usize> = (0..n % 7).map(|i| i * 2).collect(); core::hint::black_box(v.len())",
}

# (file, anchor line prefix) of each core's production guard call.
ANCHORS = {
    "tga": ("tga.rs", "    match crate::alloc_guard::try_zeroed(AllocSite::Output, 4 * n, Tracked(log)) {"),
    "dib": ("dib.rs", "    match crate::alloc_guard::try_zeroed(crate::alloc_guard::AllocSite::Output, 4 * n,"),
    "icns": ("icns.rs", "    let mut out = match crate::alloc_guard::try_zeroed(AllocSite::RgbOut, 3 * n, Tracked(log)) {"),
}


def main():
    core, seed, dest = sys.argv[1:4]
    src = os.path.join(REPO, core, f"{core}_verus")
    dst = os.path.join(dest, f"{core}_verus")
    if os.path.exists(dst):
        shutil.rmtree(dst)
    shutil.copytree(src, dst, ignore=shutil.ignore_patterns("target"))
    if seed == "none":
        return
    fname, anchor = ANCHORS[core]
    if os.environ.get("ALLOC_AUDIT_SEED_SELFTEST_BAD_ANCHOR") == "1":
        # Self-test of the controls runner: simulate an anchor that no longer
        # exists in the core, so generation must fail and be counted.
        anchor = anchor + "  /* anchor that does not exist */"
    p = os.path.join(dst, "src", fname)
    text = open(p).read()
    if text.count(anchor) != 1:
        sys.exit(f"seed_core: anchor occurs {text.count(anchor)} times in {fname}")
    text = text.replace(anchor, "    let _seed = seeded_alloc(n);\n" + anchor)
    text += (
        "\n// SEEDED VIOLATION (alloc-delta/audit control): allocates outside the guard.\n"
        f"fn seeded_alloc(n: usize) -> usize {{ {SEEDS[seed]} }}\n"
    )
    open(p, "w").write(text)


if __name__ == "__main__":
    main()
