#!/usr/bin/env python3
"""Syntax-aware source scan for the alloc-delta source lint (SPEC.md §6).

Derived from icns/tools/alloc_scan.py. GIF additionally handles conditional
contract expressions, bodyless trait contracts and test-only module registrations.
The module inventory keeps unregistered files in scope; only registrations that
are unreachable with cfg(test) off justify excluding a file.

This tool is a *falsification gate*, not an enforcement mechanism: it makes a
listing of allocating constructs in exec code, and it fails on a construct it
recognises. It cannot prove that no allocation occurs; only the production
call-graph audit (SPEC.md §6, not built) can do that.

It exists because Verus cannot see allocations. Two things must be true of the
production modules:

- every allocating construct lives inside the sealed `alloc_guard` (or in
  `#[cfg(test)]` code); and
- `.push(` appears in exec code only inside `alloc_guard`. `Seq::push` in a
  `spec fn`, an `ensures` clause, an `assert`, a `proof` block or any other
  ghost position is legal and must not be flagged.

The second rule needs to tell exec code from ghost code, so the scanner:

1. strips line comments (`//`) and block comments (`/* */`) *outside* string,
   char and raw-string literals (so `"https://x"` does not hide code after it);
2. blanks string/char literal bodies (so a `}` or `//` inside a literal cannot
   confuse the brace or comment tracking);
3. removes Verus non-exec regions inside `verus!`: `proof { ... }`,
   `assert(...)`, spec/proof functions, exec contracts and ghost bindings;
   ordinary Rust assertion macros remain executable and are scanned;
4. removes items explicitly gated by `#[cfg(test)]` (not by filename);
5. reports every surviving alloc construct, and every surviving `.push(`.

The scanner is not a Rust/Verus parser or a type checker. Its balanced-delimiter
and expression-continuation rules cover the syntax used here; unsupported syntax
may produce findings. Passing this lint does not replace the call-graph audit.

Usage:
    alloc_scan.py FILE...            scan the named files (0 = clean)
    alloc_scan.py --production-files SRCDIR   list non-test, non-guard files
    alloc_scan.py --self-test        run built-in positive/negative controls

Module inventory errors fail closed. Source inclusion, macro definitions,
module cfg_attr and inline-module path overrides need explicit inventory support
before this lint can exclude test-only files in a tree that uses those forms.
"""

import json
import os
import re
import sys
import tempfile

# Allocating / aborting constructs, the DIB/TGA pattern set extended with the
# alloc-delta items. `.push(` is handled separately: it is legal in ghost code.
ALLOC_PATTERN = re.compile(
    r"vec!\["
    r"|Vec::new"
    r"|Vec::with_capacity"
    r"|Vec::[a-zA-Z_]+"
    r"|String::"
    r"|\bString\b"
    r"|format!"
    r"|to_string\("
    r"|Box::"
    r"|Rc::"
    r"|Arc::"
    r"|from_iter"
    r"|iter::repeat"
    r"|repeat_n\("
    r"|\.take\("
    r"|\.into_boxed"
    r"|mem::forget"
    r"|Box::leak"
    r"|ManuallyDrop"
    r"|\.to_vec\("
    r"|\.to_owned\("
    r"|\.clone\("
    r"|\.\s*resize\w*\s*(?:\(|::<)"
    r"|\.reserve"
    r"|\.try_reserve"
    r"|\.extend"
    r"|\.collect"
    r"|\.\s*insert\w*\s*(?:\(|::<)"
    r"|\.\s*append\s*(?:\(|::<)"
    r"|\.\s*split_off\s*(?:\(|::<)"
    r"|\.\s*clone_from\s*(?:\(|::<)"
    r"|\.\s*shrink\w*\s*(?:\(|::<)"
    r"|\.\s*splice\s*(?:\(|::<)"
    r"|\.\s*into_vec\s*(?:\(|::<)"
    r"|\.\s*into_string\s*(?:\(|::<)"
    r"|\.\s*concat\s*(?:\(|::<)"
    r"|\.\s*join\s*(?:\(|::<)"
    r"|\.\s*repeat\s*(?:\(|::<)"
    r"|\bwith_capacity\s*\("
    r"|\b(?:HashMap|HashSet|BTreeMap|BTreeSet|VecDeque|BinaryHeap)\s*::"
    r"|handle_alloc_error"
    r"|process::abort"
)

# `.push(`, `.push_str(`, `.push_back(`, ... and turbofish/whitespace variants.
PUSH_PATTERN = re.compile(r"\.\s*push\w*\s*(?:\(|::<)")


def strip_literals_and_comments(text: str) -> str:
    """Replace comment and literal text with spaces, keeping every offset.

    Strings, chars and raw strings are blanked (not deleted) so byte offsets and
    the newline structure survive; comments are blanked the same way. A `//`
    inside a literal, and a `"` inside a comment, are both handled by tracking
    the lexical state.
    """
    out = list(text)
    n = len(text)
    i = 0
    while i < n:
        c = text[i]
        # Raw string: r"..." or r#"..."# (any number of #).
        if c == "r" and i + 1 < n and text[i + 1] in ('"', "#"):
            j = i + 1
            hashes = 0
            while j < n and text[j] == "#":
                hashes += 1
                j += 1
            if j < n and text[j] == '"':
                closer = '"' + "#" * hashes
                k = text.find(closer, j + 1)
                end = n if k < 0 else k + len(closer)
                for p in range(i, end):
                    if out[p] != "\n":
                        out[p] = " "
                i = end
                continue
        # Byte-string prefix b"..." / b'...'
        if c == "b" and i + 1 < n and text[i + 1] in ('"', "'"):
            i += 1
            c = text[i]
        if c == '"':
            j = i + 1
            while j < n:
                if text[j] == "\\":
                    j += 2
                    continue
                if text[j] == '"':
                    j += 1
                    break
                j += 1
            for p in range(i, min(j, n)):
                if out[p] != "\n":
                    out[p] = " "
            i = j
            continue
        if c == "'":
            # A char literal ('x', '\n', '\'') or a lifetime ('a, 'static, '_).
            j = i + 1
            if j < n and text[j] == "\\":
                j += 2
                if j < n and text[j] == "'":
                    j += 1
                    for p in range(i, j):
                        out[p] = " "
                    i = j
                    continue
            elif j + 1 < n and text[j + 1] == "'":
                for p in range(i, j + 2):
                    out[p] = " "
                i = j + 2
                continue
            # else: a lifetime, leave it alone.
            i = i + 1
            continue
        if c == "/" and i + 1 < n and text[i + 1] == "/":
            j = text.find("\n", i)
            j = n if j < 0 else j
            for p in range(i, j):
                out[p] = " "
            i = j
            continue
        if c == "/" and i + 1 < n and text[i + 1] == "*":
            depth = 1
            j = i + 2
            while j < n and depth:
                if text.startswith("/*", j):
                    depth += 1
                    j += 2
                elif text.startswith("*/", j):
                    depth -= 1
                    j += 2
                else:
                    j += 1
            for p in range(i, min(j, n)):
                if out[p] != "\n":
                    out[p] = " "
            i = j
            continue
        i += 1
    return "".join(out)


def mask_range(out: list, start: int, end: int) -> None:
    for p in range(start, min(end, len(out))):
        if out[p] != "\n":
            out[p] = " "


def find_matching(text: str, open_idx: int, open_ch: str, close_ch: str) -> int:
    """Index just past the `close_ch` matching `text[open_idx] == open_ch`."""
    depth = 0
    i = open_idx
    n = len(text)
    while i < n:
        if text[i] == open_ch:
            depth += 1
        elif text[i] == close_ch:
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    raise ValueError(f"unmatched {open_ch!r} at source offset {open_idx}")


SPEC_FN = re.compile(
    r"(?:\bpub\b\s*)?(?:\bopen\b\s*|\bclosed\b\s*)?\bspec\s+fn\b"
)
PROOF_FN = re.compile(r"\bproof\s+fn\b")
EXEC_FN = re.compile(r"(?<![\w:])\bfn\s+[A-Za-z_]\w*")
PROOF_BLOCK = re.compile(r"\bproof\b\s*\{")
ASSERT_CALL = re.compile(r"\bassert\s*\(")
GHOST_BIND = re.compile(r"\blet\s+(?:ghost|tracked)\b")
# A loop `invariant` clause is ghost code: it is a spec expression, erased in a
# normal build, so a `Seq::push` inside it allocates nothing. The clause runs
# from the keyword to the next `decreases` or the loop body's `{`.
INVARIANT = re.compile(r"\binvariant\b")
DECREASES = re.compile(r"\bdecreases\b")


CONTRACT = re.compile(r"\b(?:requires|ensures|recommends|decreases|returns|opens_invariants)\b")
CONTINUATION = re.compile(r"(?:else|as|is|has|matches)\b")


def find_fn_body(text: str, fn_start: int):
    """Return (body opening brace or None, end of body/declaration).

    After a parameter list, a semicolon ends a bodyless trait declaration,
    including its contracts. Inside a contract, a balanced brace expression
    continues when followed by an operator, a comma, `else`, a cast/pattern
    operator, or another block (the eventual function body). In particular an
    `if` arm followed by `else` is not an executable function body. Outside a
    contract the first brace is the body, even if its close precedes `;`.
    """
    n = len(text)
    paren = text.find("(", fn_start)
    if paren < 0:
        return None, None
    i = find_matching(text, paren, "(", ")")
    in_contract = False
    while i < n:
        c = text[i]
        if c == ";":
            return None, i + 1
        if CONTRACT.match(text, i):
            in_contract = True
        if c in "([":
            i = find_matching(text, i, c, {"(": ")", "[": "]"}[c])
            continue
        if c == "{":
            end = find_matching(text, i, "{", "}")
            t = end
            while t < n and text[t].isspace():
                t += 1
            continues = t < n and (
                text[t] in ",.([?:=!<>+-*/%&|^{"
                or CONTINUATION.match(text, t)
                or CONTRACT.match(text, t)
            )
            if not in_contract or not continues:
                return i, end
            i = end
            continue
        i += 1
    return None, None


def strip_verus_nonexec(text: str) -> str:
    """Blank ghost regions within one already-identified `verus!` block."""
    out = list(text)

    # `spec fn` / `proof fn` definitions and their bodies.
    for m in list(SPEC_FN.finditer(text)) + list(PROOF_FN.finditer(text)):
        brace, end = find_fn_body(text, m.end())
        if end is not None:
            mask_range(out, m.start(), end)

    # Exec function contracts, including trait declarations without a body.
    for m in EXEC_FN.finditer(text):
        brace, end = find_fn_body(text, m.end())
        if end is None:
            continue
        if brace is None:
            mask_range(out, m.start(), end)
        else:
            mask_range(out, m.start(), brace)

    # `proof { ... }` blocks.
    for m in PROOF_BLOCK.finditer(text):
        brace = text.index("{", m.start())
        end = find_matching(text, brace, "{", "}")
        mask_range(out, m.start(), end)

    # Verus `assert(..)` is ghost; Rust `assert!(..)` remains executable.
    for m in ASSERT_CALL.finditer(text):
        paren = text.index("(", m.start())
        end = find_matching(text, paren, "(", ")")
        mask_range(out, m.start(), end)

    # `let ghost x = ..;` / `let tracked x = ..;` bindings.
    for m in GHOST_BIND.finditer(text):
        semi = text.find(";", m.end())
        end = len(text) if semi < 0 else semi + 1
        mask_range(out, m.start(), end)

    # Loop `invariant` clauses: from the keyword to the next `decreases` or the
    # loop body's opening `{`. A clause may contain `{` inside a closure body
    # (`|j: int| ... { ... }`), so scan for the `decreases` keyword first and
    # fall back to the first `{` at brace depth zero.
    for m in INVARIANT.finditer(text):
        dec = DECREASES.search(text, m.end())
        brace = text.find("{", m.end())
        end = len(text)
        if dec is not None and (brace < 0 or dec.start() < brace):
            end = dec.start()
        elif brace >= 0:
            end = brace
        mask_range(out, m.start(), end)

    return "".join(out)


# Only an exact positive cfg(test) establishes exclusion. In particular neither
# a `_tests.rs` name, `#[test]`, `cfg(not(test))`, nor `cfg(any(test, feature=...))`
# makes a module test-only. Other configurations stay in scope conservatively.
ATTRIBUTES = re.compile(r"(?:#\s*\[(?:[^\[\]]|\[[^\[\]]*\])*\]\s*)+")
TEST_CFG = re.compile(r"#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]")
MODULE = re.compile(r"(?:\bpub(?:\s*\([^)]*\))?\s+)?\bmod\s+(\w+)\s*([;{])")
VERUS_BLOCK = re.compile(r"\bverus\s*!\s*\{")


def attribute_spans(attributes: str):
    """Yield only top-level attributes, not attribute-shaped macro arguments."""
    i = 0
    while i < len(attributes):
        if attributes[i].isspace():
            i += 1
            continue
        opening = re.match(r"#\s*\[", attributes[i:])
        if opening is None:
            raise ValueError("unsupported attribute syntax in module inventory")
        end = find_matching(attributes, i + opening.end() - 1, "[", "]")
        yield i, end
        i = end


def has_test_cfg(attributes: str) -> bool:
    return any(TEST_CFG.fullmatch(attributes[lo:hi])
               for lo, hi in attribute_spans(attributes))


def strip_test_items(text: str) -> str:
    """Blank explicitly cfg(test) fn/mod items, preserving all source offsets."""
    out = list(text)
    for attrs in ATTRIBUTES.finditer(text):
        if not has_test_cfg(attrs.group()):
            continue
        start = attrs.end()
        item = MODULE.match(text, start)
        if item:
            end = (item.end() if item[2] == ";" else
                   find_matching(text, item.end() - 1, "{", "}"))
        else:
            fn = re.match(r"(?:pub(?:\([^)]*\))?\s+)?(?:async\s+|unsafe\s+)?fn\s+\w+", text[start:])
            if not fn:
                continue  # unsupported item shape: keep scanning, do not suppress
            _, end = find_fn_body(text, start + fn.end())
        if end is not None:
            mask_range(out, attrs.start(), end)
    return "".join(out)


def strip_nonexec(text: str) -> str:
    """Apply Verus erasure only in verus!, and cfg(test) erasure everywhere."""
    text = strip_test_items(text)
    out = list(text)
    end = 0
    for m in VERUS_BLOCK.finditer(text):
        if m.start() < end:
            continue
        brace = m.end() - 1
        end = find_matching(text, brace, "{", "}")
        out[brace:end] = strip_verus_nonexec(text[brace:end])
    return "".join(out)


def module_edges(text: str, source: str) -> list:
    """External module registrations as (resolved path, test-only edge).

    Explicit paths come from the literal in the original attribute, never from
    a filename convention. Walk inline modules so their cfg(test) propagates to
    external children. A missing/unsupported path fails the inventory closed.
    """
    clean = strip_literals_and_comments(text)
    if re.search(r"\b(?:include|macro_rules)\s*!|\bmacro\s+\w+", clean):
        raise ValueError(f"{source}: source inclusion/macro definition needs module-inventory review")
    directory = os.path.dirname(source)
    stem = os.path.splitext(os.path.basename(source))[0]
    module_dir = directory if stem in ("lib", "main", "mod") else os.path.join(directory, stem)
    edges = []

    def walk(lo, hi, default_dir, path_dir, inherited_test=False):
        i = lo
        while i < hi:
            attrs = ATTRIBUTES.match(clean, i)
            item_start = attrs.end() if attrs else i
            item = MODULE.match(clean, item_start)
            if item is None:
                i = attrs.end() if attrs else i + 1
                continue
            if attrs and re.search(r"\bcfg_attr\b", attrs.group()):
                raise ValueError(f"{source}: cfg_attr on a module needs module-inventory review")
            test_only = inherited_test or bool(attrs and has_test_cfg(attrs.group()))
            name, kind = item.groups()
            if kind == "{":
                end = find_matching(clean, item.end() - 1, "{", "}")
                if attrs and re.search(r"\bpath\b", attrs.group()):
                    raise ValueError(f"{source}: path on inline module is unsupported")
                child_dir = os.path.join(default_dir, name)
                walk(item.end(), end - 1, child_dir, child_dir, test_only)
                i = end
                continue
            path_attrs = []
            if attrs:
                for attr_lo, attr_hi in attribute_spans(attrs.group()):
                    absolute = attrs.start() + attr_lo
                    attr = re.fullmatch(r"#\s*\[\s*path\s*=([^\]]*)\]", clean[absolute:attrs.start() + attr_hi])
                    if attr:
                        a, b = absolute + attr.start(1), absolute + attr.end(1)
                        path_attrs.append(text[a:b].strip())
            if path_attrs:
                if len(path_attrs) != 1:
                    raise ValueError(f"{source}: ambiguous module path attributes")
                # Ordinary quoted paths cover this tree. Reject unsupported Rust
                # escape/raw-literal forms rather than silently dropping a file.
                rel = json.loads(path_attrs[0])
                target = os.path.join(path_dir, rel)
            else:
                choices = [os.path.join(default_dir, name + ".rs"),
                           os.path.join(default_dir, name, "mod.rs")]
                present = [p for p in choices if os.path.isfile(p)]
                if len(present) != 1:
                    raise ValueError(f"{source}: cannot resolve module {name}: {choices}")
                target = present[0]
            target = os.path.realpath(target)
            if not os.path.isfile(target):
                raise ValueError(f"{source}: missing module {target}")
            edges.append((target, test_only))
            i = item.end()

    walk(0, len(clean), module_dir, directory)
    return edges


def production_files(srcdir: str) -> list:
    """All non-guard Rust files, excluding only test-registered module subtrees.

    Preserve the old scan's conservative coverage of unregistered files. Start
    at lib.rs and files with no incoming registration; orphan registration cycles
    also stay in scope. Follow every edge unless cfg(test) disables it. A second
    production registration of a test target therefore keeps that file scanned.
    """
    srcdir = os.path.realpath(srcdir)
    guard = os.path.join(srcdir, "alloc_guard") + os.sep
    files = {os.path.realpath(os.path.join(d, n))
             for d, _, names in os.walk(srcdir) for n in names
             if n.endswith(".rs") and not os.path.realpath(os.path.join(d, n)).startswith(guard)}
    edges = {}
    for path in files:
        with open(path, encoding="utf-8") as fh:
            edges[path] = module_edges(fh.read(), path)
        for target, _ in edges[path]:
            if target not in files and not target.startswith(guard):
                raise ValueError(f"{path}: module outside scan tree: {target}")
    targets = {target for es in edges.values() for target, _ in es}
    roots = (files - targets) | ({os.path.join(srcdir, "lib.rs")} & files)

    def reachable(starts, include_tests):
        seen = set()
        todo = list(starts)
        while todo:
            path = todo.pop()
            if path in seen or path not in files:
                continue
            seen.add(path)
            todo.extend(target for target, test in edges[path] if include_tests or not test)
        return seen

    roots |= files - reachable(roots, True)
    return sorted(reachable(roots, False))


# SPEC.md §6: `Reserved::push` and `Reserved::into_vec` are whitelisted by full
# path. The scanner cannot resolve types, so the audit passes `--allow-reserved`
# and the scanner blanks `.push(`/`.into_vec()` on a receiver named `res` only
# inside a function body that itself binds `res` from the guard: either
# `Ok(res)` / `Ok(mut res)` as a match arm over a `try_with_room(..)` call in
# the same body, or `let [mut] res = [match] ..try_with_room(..)` (ICNS binds
# `res` by `let mut res = match try_with_room::<Entry>(..) { Ok(r) => r, .. }`).
# A `res` parameter or any other `res` outside such a function is still flagged,
# and so is any receiver not named `res`. This is a textual falsification gate,
# not enforcement: inside a guard-using function a second, unrelated variable
# also named `res` would still be suppressed, and the audit separately asserts
# exactly one `try_with_room` call site and (from a run without the flag) exactly
# one `res.push(` and one `res.into_vec(` finding.
RESERVED_CALL = re.compile(r"\bres\s*\.\s*(?:push|into_vec)\s*(?:\(|::<)")
RESERVED_BIND = re.compile(
    r"\bOk\s*\(\s*(?:mut\s+)?res\s*\)"
    r"|\blet\s+(?:mut\s+)?res\s*(?::[^=;]*)?=\s*(?:match\s+)?[\w:]*try_with_room\b"
)


def reserved_fn_spans(body: str) -> list:
    """(Run on the comment-stripped text: strip_nonexec blanks fn headers.)
    Body spans of fns that call `try_with_room` and bind `res` from it."""
    spans = []
    for m in EXEC_FN.finditer(body):
        o, c = find_fn_body(body, m.end())
        if o is None:
            continue
        inner = body[o:c]
        if "try_with_room" in inner and RESERVED_BIND.search(inner):
            spans.append((o, c))
    return spans


def scan(text: str, allow_reserved: bool = False) -> list:
    """Return [(line, text, kind)] findings for one source text."""
    cleaned = strip_literals_and_comments(text)
    body = strip_nonexec(cleaned)
    if allow_reserved:
        chars = list(body)
        for lo, hi in reserved_fn_spans(cleaned):
            for m in RESERVED_CALL.finditer(body, lo, hi):
                for k in range(m.start(), m.end()):
                    if chars[k] != "\n":
                        chars[k] = " "
        body = "".join(chars)
    findings = []
    for m in ALLOC_PATTERN.finditer(body):
        line = body.count("\n", 0, m.start()) + 1
        findings.append((line, "alloc", m.group(0)))
    for m in PUSH_PATTERN.finditer(body):
        line = body.count("\n", 0, m.start()) + 1
        findings.append((line, "push", m.group(0)))
    return findings


def count_vec_macro(text: str) -> int:
    body = strip_nonexec(strip_literals_and_comments(text))
    return len(re.findall(r"\bvec!\[", body))


def self_test() -> int:
    """Built-in controls. 0 = the scanner behaved, 1 = it did not."""
    fails = 0

    def check(name, ok):
        nonlocal fails
        if not ok:
            print(f"self-test FAILED: {name}", file=sys.stderr)
            fails += 1

    bad_exec = """
verus! {
fn leaky(v: &mut Vec<u8>, n: usize)
    ensures final(v)@.len() == old(v)@.len() + n,
{
    let mut i: usize = 0;
    while i < n
        invariant 0 <= i <= n,
        decreases n - i,
    {
        v.push(0u8);
        i = i + 1;
    }
}
} // verus!
"""
    good_ghost = """
verus! {
pub open spec fn bytes4(v: int, a: u8) -> Seq<u8> {
    rgb555(v).push(a)
}

fn conv(v: u16, alpha: u8) -> (r: [u8; 4])
    ensures r@ == rgb555(v as int).push(alpha),
{
    proof {
        assert(Seq::<u8>::empty().push(0u8).len() == 1);
    }
    let rr = v as u8;
    proof {
        assert(rr == 0u8);
    }
    [0u8, 0, 0, alpha]
}
} // verus!
"""
    bad_vec = """
verus! {
fn alloc(n: usize) -> (v: Vec<u8>) {
    let v: Vec<u8> = vec![0u8; n];
    v
}
} // verus!
"""
    url_string = """
verus! {
fn f(v: &mut Vec<u8>) {
    let _u = "https://x"; // a URL is data, not a comment
    v.push(0u8);
}
} // verus!
"""
    # Negative control: an exec `.push(` must be flagged.
    check("exec .push flagged", len([f for f in scan(bad_exec) if f[1] == "push"]) == 1)
    # Positive control: ghost `Seq::push` in spec/ensures/proof must not be.
    check("ghost Seq::push not flagged", not [f for f in scan(good_ghost) if f[1] == "push"])
    check("ghost file otherwise clean", scan(good_ghost) == [])
    # Negative control: an exec `vec!` must be flagged.
    check("exec vec! flagged", len([f for f in scan(bad_vec) if f[2] == "vec!["]) == 1)
    check("vec! counted", count_vec_macro(bad_vec) == 1)
    # `//` inside a string must not hide the code after it on the same line.
    check("url string does not hide exec .push",
          len([f for f in scan(url_string) if f[1] == "push"]) == 1)
    # A `//` comment must not itself be scanned.
    commented = "verus! {\n// v.push(0u8);\nfn g() {}\n} // verus!\n"
    check("comment not scanned", scan(commented) == [])
    # Negative controls: each allocating construct, in exec code, must be flagged.
    ghost_prefix = good_ghost
    constructs = [
        "v.push(1)", "v . push (1)", "Vec::new()", "vec![1]", "v.reserve(1)",
        "v.try_reserve_exact(4)", "v.clone()", 'format!("x")',
        "alloc::alloc::handle_alloc_error(l)", "std::process::abort()",
        "it.collect::<Vec<u8>>()", "let _ = v.split_off(1)", "v.clone_from(src)",
        "v.extend_from_slice(s)", "v.extend(s)", "v.append(&mut w)",
        "v.insert(0, 1)", "v.resize(4, 0)", "v.resize_with(4, f)",
        "v.reserve_exact(4)", "v.shrink_to_fit()", "v.shrink_to(1)",
        "v.into_boxed_slice()", "s.to_vec()", "s.to_owned()", "s.to_string()",
        'String::from("x")', "Box::new(1)", "Rc::new(1)", "Arc::new(1)",
        "Vec::from(s)", "with_capacity(4)", "Vec::with_capacity(4)",
        "vec![0u8; 4]", 'st.push_str("a")', "d.push_back(1)", "v.insert_str(0, s)",
        "v.splice(0..1, w)", "b.into_vec()", "b.into_string()", "p.concat()",
        'p.join(",")', "v.repeat(2)", "HashMap::new()",
    ]
    for c in constructs:
        src = "verus! {\nfn bad(v: &mut Vec<u8>) { " + c + "; }\n} // verus!\n"
        check(f"flagged: {c}", bool(scan(src)))
    # Positive controls: non-allocating look-alikes must stay unflagged.
    for ok in ["let n = v.len(); let _ = v.get(n);", "let _ = v.split_at(1);",
               "v.copy_from_slice(s)", "v.clone_from_slice(s)", "v.truncate(1)",
               "v.swap(0, 1)", "let _ = v.iter().sum::<u8>();", "let _ = v.first();"]:
        src = "verus! {\nfn ok(v: &mut [u8], s: &[u8]) { " + ok + "; }\n} // verus!\n"
        check(f"unflagged: {ok}", scan(src) == [])
    # Proof-region Seq::push / Seq::insert are logical operations.
    proof_only = """
verus! {
fn f(v: &mut Vec<u8>)
    ensures final(v)@ == old(v)@.push(1).insert(0, 2),
{
    proof { let s = Seq::<u8>::empty().push(1); assert(s.push(2).len() == 2); }
}
} // verus!
"""
    check("proof-region Seq::push/insert unflagged", scan(proof_only) == [])
    # `--allow-reserved` scoping: `res` is suppressed only where the enclosing
    # function binds it from `try_with_room`.
    bad_res = "verus! {\nfn bad(res: &mut Vec<u8>) { res.push(1); }\n} // verus!\n"
    check("unrelated `res` Vec flagged under allow_reserved",
          len([f for f in scan(bad_res, allow_reserved=True) if f[1] == "push"]) == 1)
    bad_res2 = "verus! {\nfn bad(res: &mut Vec<u8>) { let _ = res.into_vec(); }\n} // verus!\n"
    check("unrelated `res.into_vec()` flagged under allow_reserved",
          bool(scan(bad_res2, allow_reserved=True)))
    bad_local = ("verus! {\nfn bad() { let mut res = Vec::new(); res.push(1); "
                 "let _ = try_with_room(); }\n} // verus!\n")
    check("local Vec `res` beside a stray try_with_room flagged",
          any(f[1] == "push" for f in scan(bad_local, allow_reserved=True)))
    # The guarded function with its one `res.push(entry)` replaced by `v.push(x)`.
    swapped = ("verus! {\nfn parse(n: usize, v: &mut Vec<u8>) {\n"
               "  let mut res = match sites::try_with_room::<E>(S, n, t) {\n"
               "    Ok(r) => r,\n    Err(o) => { return; }\n  };\n"
               "  v.push(1);\n  let out = res.into_vec();\n}\n} // verus!\n")
    check("different `v.push(x)` beside guard-bound `res` flagged",
          [f[1] for f in scan(swapped, allow_reserved=True)] == ["push"])
    good_let = swapped.replace("v.push(1);", "res.push(e);")
    check("`let mut res = match try_with_room` binding unflagged under allow_reserved",
          scan(good_let, allow_reserved=True) == [])
    good_res = ("verus! {\nfn good(n: usize) {\n  match sites::try_with_room::<E>(S, n, t) {\n"
                "    Ok(mut res) => { res.push(e); let v = res.into_vec(); }\n"
                "    Err(o) => {}\n  }\n}\n} // verus!\n")
    check("guard-bound `res` unflagged under allow_reserved",
          scan(good_res, allow_reserved=True) == [])
    check("guard-bound `res` flagged without allow_reserved",
          any(f[1] == "push" for f in scan(good_res)))
    # An `invariant` clause with `Seq::push` stays unflagged (ghost code).
    inv = ("verus! {\nfn f(n: usize) {\n  let mut i = 0;\n  while i < n\n"
           "    invariant s == Seq::<u8>::empty().push(1),\n    decreases n - i,\n"
           "  { i = i + 1; }\n}\n} // verus!\n")
    check("invariant-clause Seq::push unflagged", scan(inv) == [])
    # Regressions for all six formerly false-positive production sites: brace
    # expressions in contracts, bodyless trait contracts, and a proof fn with an
    # if/match in its requires. The executable bodies remain separately scanned.
    conditional = """
verus! {
trait Sink {
    fn put(&mut self, q: u32)
        ensures final(self).log() == old(self).log().push(q),
    ;
}
proof fn lemma(s: Seq<u8>, p: bool)
    requires p == if p { true } else { match p { true => true, _ => false } },
    ensures s.push(1).len() == s.len() + 1,
{ assert(s.push(1).len() > 0); }
fn conditional(v: &mut Vec<u8>, p: bool)
    requires p == if p { true } else { false },
    ensures s == if p { s.push(1) } else if p { s.push(2) } else { s.push(3) },
        n == if p { 1 } else { 2 },
{
    let ghost s2 = s.push(4);
    proof { assert(s2.push(5).len() > 0); }
    EXECUTABLE
}
}
"""
    check("conditional/bodyless/proof contracts are ghost",
          scan(conditional.replace("EXECUTABLE", "")) == [])
    # Every existing allocation control also runs directly beside those ghost
    # pushes, not just in an otherwise empty verus! function.
    for c in constructs:
        for mode in (False, True):
            findings = scan(conditional.replace("EXECUTABLE", c + ";"), mode)
            expected_line = conditional[:conditional.index("EXECUTABLE")].count("\n") + 1
            check(f"conditional contract preserves exec {c} (reserved={mode})",
                  bool(findings) and all(f[0] == expected_line for f in findings))
    for src in ('fn f() { assert!({ v.push(1); true }); }',
                'verus! { fn f() { assert_eq!(Vec::new(), v); } }',
                'fn f() { assert_ne!(vec![1], v); }'):
        check("executable Rust assertion arguments scanned", bool(scan(src)))
    for attribute in ("#[cfg(test)]", "#[cfg ( test )] #[allow(dead_code)]"):
        check("cfg(test) inline module excluded",
              scan(attribute + " mod arbitrary { fn f() { Vec::new(); } }") == [])
        check("cfg(test) function excluded",
              scan(attribute + " fn f() { v.push(1); }") == [])
        check("exec sibling of cfg(test) module retained",
              bool(scan(attribute + " mod tests { fn f() { Vec::new(); } }"
                        "fn production() { v.push(1); }")))
    for attribute in ("", "#[test]", "#[cfg(not(test))]",
                      '#[cfg(any(test, feature = "prod"))]',
                      '#[annotation(#[cfg(test)])]'):
        check(f"non-test-only module scanned: {attribute}",
              bool(scan(attribute + " mod tests { fn f() { v.push(1); } }")))

    # Real module registration controls, in isolated temporary trees. The name
    # of a source file never establishes its production/test classification.
    with tempfile.TemporaryDirectory(prefix="alloc-scan-modules-") as tmp:
        def put(name, source):
            path = os.path.join(tmp, name)
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "w", encoding="utf-8") as fh:
                fh.write(source)
            return os.path.realpath(path)

        test_path = put("arbitrary.rs", "mod child; fn f() { Vec::new(); }")
        child_path = put("arbitrary/child.rs", "fn f() { v.push(1); }")
        prod_path = put("looks_like_tests.rs", "fn f() { v.push(1); }")
        orphan_path = put("unregistered.rs", "fn f() { Box::new(1); }")
        registration = '#[path = "arbitrary.rs"] mod checks;\n'
        put("lib.rs", "#[cfg(test)] " + registration + "mod looks_like_tests;\n")
        live = production_files(tmp)
        check("cfg(test) external path and children excluded",
              test_path not in live and child_path not in live)
        check("production *_tests.rs and unregistered files retained",
              prod_path in live and orphan_path in live)
        for attribute in ("", "#[cfg(not(test))]",
                          '#[cfg(any(test, feature = "prod"))]',
                          '#[annotation(#[cfg(test)])]'):
            put("lib.rs", attribute + registration)
            live = production_files(tmp)
            check(f"external module retained without exact test-only cfg: {attribute}",
                  test_path in live and child_path in live)
        put("lib.rs", "#[cfg(test)] " + registration
            + '#[path = "arbitrary.rs"] mod also_production;')
        check("second production registration defeats test exclusion",
              test_path in production_files(tmp))
        put("lib.rs", '#[cfg(test)] mod outer { #[path = "../arbitrary.rs"] mod inner; }')
        os.makedirs(os.path.join(tmp, "outer"), exist_ok=True)
        check("inline test context propagates to external children",
              test_path not in production_files(tmp))
        for unsupported in ("mod missing;", 'include!("arbitrary.rs");',
                            '#[cfg_attr(feature = "prod", path = "arbitrary.rs")] mod checks;',
                            'macro_rules! register { () => { mod checks; } }'):
            put("lib.rs", unsupported)
            try:
                production_files(tmp)
            except ValueError:
                pass
            else:
                check(f"unsupported module discovery fails closed: {unsupported}", False)

    # Check the same complete module inventory that audit_alloc.sh scans, not a
    # stale list that silently omits newly registered production/proof modules.
    srcdir = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "gif_verus", "src")
    real_files = production_files(srcdir) if os.path.isdir(srcdir) else []
    for real in real_files:
        with open(real, encoding="utf-8") as fh:
            rtext = fh.read()
        name = os.path.basename(real)
        check(f"real {name} clean", scan(rtext) == [])
        check(f"real {name} clean under allow_reserved", scan(rtext, allow_reserved=True) == [])
    if fails == 0:
        print(f"self-test: OK ({len(constructs)} allocating constructs flagged, also beside conditional ghost contracts in both modes; "
              f"cfg(test) registration controls; {len(real_files)} real sources clean in both modes)")
    return 1 if fails else 0


def main(argv: list) -> int:
    if "--production-files" in argv:
        if len(argv) != 3 or argv[1] != "--production-files":
            print("usage: alloc_scan.py --production-files SRCDIR", file=sys.stderr)
            return 2
        for path in production_files(argv[2]):
            print(path)
        return 0
    if "--self-test" in argv:
        return self_test()
    if "--count-vec" in argv:
        files = [a for a in argv[1:] if not a.startswith("--")]
        total = 0
        for path in files:
            with open(path, "r", encoding="utf-8") as fh:
                total += count_vec_macro(fh.read())
        print(total)
        return 0
    allow_reserved = "--allow-reserved" in argv
    files = [a for a in argv[1:] if not a.startswith("--")]
    total = 0
    for path in files:
        with open(path, "r", encoding="utf-8") as fh:
            text = fh.read()
        findings = scan(text, allow_reserved=allow_reserved)
        if findings:
            print(f"{path}: allocating or aborting construct(s) in exec code:")
            for line, kind, what in findings:
                print(f"  {line}: {what}")
            total += len(findings)
    if total == 0 and files:
        print(f"alloc_scan: clean ({len(files)} file(s))")
    return 1 if total else 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv))
    except (OSError, ValueError) as exc:
        print(f"alloc_scan: {exc}", file=sys.stderr)
        sys.exit(2)
