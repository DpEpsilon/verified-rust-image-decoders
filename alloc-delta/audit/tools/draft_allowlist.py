#!/usr/bin/env python3
"""Draft an alloc-audit/2 allowlist from a diagnostic audit report.

THIS IS A DRAFTING AID, NOT A REVIEW. It writes `reviewer = ""`, so the tool
refuses the result (`allowlist-reviewer`) until a named human signs it.

Inputs:
  report     a diagnostic report produced with ALLOC_AUDIT_EDGES=1
  decisions  the hand-written per-core decisions (alloc entries, root
             exclusions, generic exports, guard call sites, dependency
             contracts): alloc-delta/audit/allowlist/decisions/<core>.toml

What it adds mechanically, and by which rule (anything that matches no rule is
printed and NOT admitted, so the audit keeps failing on it):
  * [meta] from the report header (source hash, profile, features, vstd crate
    hash and locked checksum, verus_builtin crate hash, crate name).
  * proc-macro entries for the crates the tool reported, bound by artifact hash.
  * root-exclude for every exported `<crate::T as core::fmt::Debug>::fmt`.
  * intrinsic leaves on a fixed list of non-allocating intrinsics.
  * the ffi probe `alloc::alloc::__rust_no_alloc_shim_is_unstable_v2`.
  * diverging (panic-family) no-mir leaves, one entry per caller, with a
    citation chosen by the caller's kind (ub_checks precondition, crate slice
    indexing, ...), listing the crate functions that reach that caller.
  * panic edges (Assert terminators), one entry per (function, panic kind),
    citing Safe'/T1 for crate functions and the vstd precondition for vstd.
"""
import re
import sys
import tomllib
from collections import defaultdict

SAFE_INTRINSICS = {
    "core::intrinsics::cold_path": "codegen hint",
    "core::intrinsics::ctpop": "population count",
    "core::intrinsics::ctlz": "leading-zero count",
    "core::intrinsics::cttz": "trailing-zero count",
    "core::intrinsics::write_bytes": "memset",
    "core::intrinsics::compare_bytes": "memcmp",
    "core::intrinsics::raw_eq": "bytewise equality of two values",
    "core::intrinsics::copy_nonoverlapping": "memcpy",
    "core::intrinsics::copy": "memmove",
}

CORE_THEOREMS = {
    "dib_verus": "dib/THEOREMS.md T1 restated as Safe' on try_* (alloc-delta THEOREMS-ALLOC-DELTA.md §1)",
    "tga_verus": "tga/THEOREMS.md T1 restated as Safe' on try_* (alloc-delta THEOREMS-ALLOC-DELTA.md §2)",
    "icns_verus": "icns/THEOREMS.md T1 restated as Safe' on try_* (alloc-delta THEOREMS-ALLOC-DELTA.md §3)",
    "ico_verus": "ico/THEOREMS.md T1 restated as Safe' on try_* (alloc-delta THEOREMS-ALLOC-DELTA.md §4)",
}


def q(s):
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


def parse_report(path):
    header, sections, cur = {}, defaultdict(list), None
    with open(path) as f:
        for line in f:
            line = line.rstrip("\n")
            m = re.match(r"^-- (.*) \(\d+\) --$", line)
            if m:
                cur = m.group(1)
                continue
            if cur is None:
                if ": " in line:
                    k, v = line.split(": ", 1)
                    header.setdefault(k, v)
            elif line.startswith("  "):
                sections[cur].append(line[2:])
    return header, sections


def strip_args(label):
    """Drop the trailing generic-argument group and any [shim] tag."""
    label = re.sub(r" \[[^\]]*\]$", "", label)
    if label.endswith(">"):
        depth = 0
        for i in range(len(label) - 1, -1, -1):
            if label[i] == ">":
                depth += 1
            elif label[i] == "<":
                depth -= 1
                if depth == 0:
                    if i > 0 and (label[i - 1].isalnum() or label[i - 1] in "_]}"):
                        return label[:i]
                    break
    return label


def main():
    report, decisions_path, out_path = sys.argv[1:4]
    header, sec = parse_report(report)
    crate = header["crate"]
    theorem = CORE_THEOREMS.get(crate, "Safe'")
    with open(decisions_path, "rb") as f:
        decisions = tomllib.load(f)
    dec_text = open(decisions_path).read()

    # Graph: callers of each node, for ancestor listings.
    callers = defaultdict(set)
    for e in sec.get("edges", []):
        m = re.match(r"^(.*) -> (.*) \[([a-z-]+)\] @ .*$", e)
        if m:
            callers[m.group(2)].add(m.group(1))

    def local_ancestors(node, limit=12):
        seen, out, stack = {node}, set(), [node]
        while stack:
            n = stack.pop()
            for c in callers.get(n, ()):
                if c in seen:
                    continue
                seen.add(c)
                if c.startswith("crate::") or c.startswith("<crate::"):
                    out.add(strip_args(c))
                else:
                    stack.append(c)
        out = sorted(out)
        return out[:limit] + (["..."] if len(out) > limit else [])

    vstd = header.get("vstd", "")
    def field(name):
        m = re.search(name + r'=Some\("([^"]*)"\)', vstd)
        return m.group(1) if m else ""

    lines = []
    w = lines.append
    w("# alloc-audit/2 allowlist, DRAFT. Generated by alloc-delta/audit/tools/draft_allowlist.py")
    w(f"# from {report.split('/')[-1]} plus decisions/{decisions_path.split('/')[-1]}.")
    w("# reviewer is empty: the tool refuses this file (allowlist-reviewer) until a named human")
    w("# reviews every entry below and signs it. Regenerating it discards that review.")
    w("")
    w("[meta]")
    w('format = "alloc-audit/2"')
    w(f"crate_name = {q(crate)}")
    w(f'rustc_commit = {q(header["rustc-commit"].split(" (full ")[1].rstrip(")"))}')
    w(f'target = {q(header["target"])}')
    w(f'profile = {q(header["profile"])}')
    feats = re.findall(r'"([^"]*)"', header.get("features", "[]"))
    w("features = [" + ", ".join(q(x) for x in feats) + "]")
    w(f'source_hash = {q(header["source_hash"])}')
    if "linked=true" in vstd:
        w(f'vstd_version = {q(field("version"))}')
        w(f'vstd_svh = {q(field("svh"))}')
        w(f'vstd_source_checksum = {q(field("locked_checksum"))}')
        vb = [l for l in sec.get("linked artifacts and command line", []) if l.startswith("crate verus_builtin ")]
        if vb:
            w(f'verus_builtin_svh = {q(re.search(r"svh=(\S+)", vb[0]).group(1))}')
    else:
        w('vstd_version = "none"')
    w('reviewer = ""')
    w('drafted_by = "draft_allowlist.py (Claude agent), 2026-11-13; unsigned"')
    roots = decisions.get("required_roots", [])
    w("required_roots = [" + ", ".join(q(r) for r in roots) + "]")
    w("")

    # Hand-written decisions verbatim (everything after the required_roots line).
    body = dec_text.split("# --- entries ---", 1)
    w("# ===== hand-written decisions (allowlist/decisions) =====")
    w(body[1].strip() if len(body) == 2 else "")
    w("")
    w("# ===== mechanical entries (draft_allowlist.py rules) =====")

    for l in sec.get("build inputs (compiler-resolved)", []):
        m = re.match(r"^proc-macro (\S+) svh=\S+ (\S+) sha256=(\S+)", l)
        if m:
            w("[[entry]]")
            w('kind = "proc-macro"')
            w(f"item = {q(m.group(1))}")
            w(f"sha256 = {q(m.group(3))}")
            w(f'reason = "Verus macro crate of the pinned Verus release, bound by name and by the hash of the artifact this build loaded ({m.group(2).split("/")[-1]})"')
            w("")

    for r in header.get("roots", "").split(", ") + header.get("roots-excluded", "").split(", "):
        if re.match(r"^<crate::.* as core::fmt::Debug>::fmt$", r):
            w("[[entry]]")
            w('kind = "root-exclude"')
            w(f"item = {q(r)}")
            w('citation = "derive(Debug): formats through the caller-supplied core::fmt::Formatter (a dyn fmt::Write); not reachable from any try_* root; the caller, not this crate, owns any allocation behind the writer"')
            w("")

    unhandled = []
    seen = set()
    for l in sec.get("leaves", []):
        m = re.match(r"^(ok|!!) \[([a-z-]+)(,diverging)?\] (.*) <- (.*)$", l)
        if not m:
            continue
        kind, div, item, caller = m.group(2), bool(m.group(3)), m.group(4), m.group(5)
        base = strip_args(item)
        key = (kind, base, caller if div else "")
        if key in seen:
            continue
        seen.add(key)
        if kind == "intrinsic" and base in SAFE_INTRINSICS and not div:
            w("[[entry]]")
            w('kind = "intrinsic"')
            w(f"item = {q(base)}")
            w('contract = "none: compiler intrinsic, lowered to inline code or a libc mem* call; allocates nothing"')
            w(f'citation = "core::intrinsics::{base.split("::")[-1]} ({SAFE_INTRINSICS[base]}), core source at rustc 58cf8f9c3"')
            w("")
        elif kind == "ffi" and base == "alloc::alloc::__rust_no_alloc_shim_is_unstable_v2":
            w("[[entry]]")
            w('kind = "ffi"')
            w(f"item = {q(base)}")
            w('contract = "none: a volatile read of the allocator-shim stability marker symbol; allocates nothing"')
            w('citation = "alloc/src/alloc.rs at rustc 58cf8f9c3 (Global::alloc_impl_runtime / alloc::alloc::alloc)"')
            w("")
        elif kind == "no-mir" and div:
            cb = strip_args(caller)
            anc = local_ancestors(caller)
            via = "; reached from crate functions: " + ", ".join(anc) if anc else ""
            if cb.endswith("::precondition_check") or cb == "core::ub_checks::maybe_is_nonoverlapping::runtime" or cb.startswith("core::ptr::const_ptr::<impl *const T>::is_aligned_to"):
                cit = ("ub_checks precondition: this call sits behind core::intrinsics::ub_checks(), which is false in a crate compiled "
                       "with debug-assertions=off (the recorded profile), so the branch is dead in this artifact")
                if cb.startswith("core::ptr::const_ptr"):
                    cit = ("is_aligned_to panics only for an alignment that is not a power of two; its callers here are ub_checks "
                           "precondition checks (dead with debug-assertions=off) passing align_of::<T>()")
            elif cb.startswith("<core::ops::range::Range") and "SliceIndex" in cb:
                cit = f"slice range indexing in verified crate code: {theorem} discharges every range bound, so slice_index_fail cannot be reached"
            elif cb.startswith("core::option::Option::<T>::unwrap"):
                cit = f"Option::unwrap in verified code: {theorem}; the verifier proves the option is Some at each unwrap"
            else:
                unhandled.append(l)
                continue
            w("[[entry]]")
            w('kind = "no-mir"')
            w(f"item = {q(base)}")
            w(f"allow = {q(cb)}")
            w('contract = "diverges (panic, core); allocates nothing in core. Under a std consumer the panic hook may allocate a payload: this entry is admitted only because the edge is unreachable (citation)"')
            w(f"citation = {q(cit + via)}")
            w("")
        else:
            unhandled.append(l)

    pseen = set()
    for l in sec.get("panic edges (per function)", []):
        m = re.match(r"^(ok|!!) (\S+) in (.*?)( at .*)?$", l)
        if not m:
            continue
        name, fn = m.group(2), m.group(3)
        fb = strip_args(fn)
        if (name, fb) in pseen:
            continue
        pseen.add((name, fb))
        if fb.startswith("crate::") or fb.startswith("<crate::"):
            cit = f"{theorem}: {fb} is verified exec code; Verus discharges the {name} obligation (bounds/division/overflow) at every site, so the compiler-inserted check cannot fire"
        elif re.match(r"^<(usize|core::ops::range::Range[A-Za-z]*<usize>) as core::slice::index::SliceIndex<\[T\]>>::index(_mut)?$", fb):
            anc = local_ancestors(fn)
            cit = f"slice/array indexing in verified crate code: {theorem}; Verus discharges every index bound, so this check cannot fire; reached from crate functions: " + ", ".join(anc)
        elif "vstd::pervasive::VecAdditionalExecFns" in fb and fb.endswith("::set"):
            cit = "vstd spec of Vec::set requires i < old(self).len(); every caller is verified against that precondition"
        else:
            unhandled.append(l)
            continue
        w("[[entry]]")
        w('kind = "panic"')
        w(f"item = {q(name)}")
        w(f"allow = {q(fb)}")
        w('contract = "diverges (panic); allocates nothing in core; admitted only as unreachable"')
        w(f"citation = {q(cit)}")
        w("")

    with open(out_path, "w") as f:
        f.write("\n".join(lines) + "\n")
    if unhandled:
        print(f"draft_allowlist: {len(unhandled)} item(s) matched no rule and were NOT admitted:", file=sys.stderr)
        for u in unhandled:
            print("   ", u, file=sys.stderr)


if __name__ == "__main__":
    main()
