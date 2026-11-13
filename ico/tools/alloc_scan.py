#!/usr/bin/env python3
"""Syntax-aware source scan for the alloc-delta source lint (SPEC.md §6).

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
3. removes the text of non-exec regions: `proof { ... }` blocks and block-form
   `assert!(...)`/`assert(...)`, `spec fn`/`open spec fn`/`closed spec fn`
   bodies and their `requires`/`ensures` clauses, `proof fn` definitions, and
   `let ... = Ghost(..)`/`ghost` bindings;
4. reports every surviving alloc construct, and every surviving `.push(`.

Usage:
    alloc_scan.py FILE...            scan the named files (0 = clean)
    alloc_scan.py --self-test        run built-in positive/negative controls
"""

import os
import re
import sys

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
    return n


SPEC_FN = re.compile(
    r"(?:\bpub\b\s*)?(?:\bopen\b\s*|\bclosed\b\s*)?\bspec\s+fn\b"
)
PROOF_FN = re.compile(r"\bproof\s+fn\b")
EXEC_FN = re.compile(r"(?<![\w:])\bfn\s+[A-Za-z_]\w*")
PROOF_BLOCK = re.compile(r"\bproof\b\s*\{")
ASSERT_CALL = re.compile(r"\bassert(?:_eq|_ne|_ok|_err)?!?\s*\(")
GHOST_BIND = re.compile(r"\blet\s+(?:ghost|tracked)\b")


def find_fn_body(text: str, fn_start: int):
    """Return (open, close) of the body braces of the `fn` at `fn_start`.

    Scans past the parameter list and any `requires`/`ensures` clauses. A brace
    block is a clause expression (not the body) when the significant character
    after its closing `}` is punctuation; the body's `}` is followed by an item
    start, an enclosing `}`, or end of input. Comments and literals are already
    blanked, so `///` does not appear as punctuation here.
    """
    n = len(text)
    paren = text.find("(", fn_start)
    if paren < 0:
        return None, None
    i = find_matching(text, paren, "(", ")")
    while i < n:
        c = text[i]
        if c == ";":
            return None, None  # bodyless declaration
        if c == "(":
            i = find_matching(text, i, "(", ")")
            continue
        if c == "[":
            i = find_matching(text, i, "[", "]")
            continue
        if c == "{":
            end = find_matching(text, i, "{", "}")
            t = end
            while t < n and text[t] in " \t\r\n":
                t += 1
            if t >= n or text[t] == "}" or text[t] == "#" or text[t].isalpha() or text[t] == "_":
                return i, end
            i = end
            continue
        i += 1
    return None, None


def strip_nonexec(text: str) -> str:
    """Blank Verus ghost/proof/spec regions so only exec bodies remain."""
    out = list(text)

    # `spec fn` / `proof fn` definitions and their bodies.
    for m in list(SPEC_FN.finditer(text)) + list(PROOF_FN.finditer(text)):
        brace, _ = find_fn_body(text, m.end())
        if brace is None:
            semi = text.find(";", m.end())
            mask_range(out, m.start(), semi + 1 if semi >= 0 else len(text))
        else:
            end = find_matching(text, brace, "{", "}")
            mask_range(out, m.start(), end)

    # Exec functions' signatures and clauses: keep only the body braces.
    for m in EXEC_FN.finditer(text):
        brace, _ = find_fn_body(text, m.end())
        if brace is None:
            continue
        end = find_matching(text, brace, "{", "}")
        mask_range(out, m.start(), brace + 1)  # through the opening `{` is kept open
        mask_range(out, end - 1, end)          # blank the closing `}` too

    # `proof { ... }` blocks.
    for m in PROOF_BLOCK.finditer(text):
        brace = text.index("{", m.start())
        end = find_matching(text, brace, "{", "}")
        mask_range(out, m.start(), end)

    # `assert!(..)` / `assert_eq!(..)` and friends.
    for m in ASSERT_CALL.finditer(text):
        paren = text.index("(", m.start())
        end = find_matching(text, paren, "(", ")")
        mask_range(out, m.start(), end)

    # `let ghost x = ..;` / `let tracked x = ..;` bindings.
    for m in GHOST_BIND.finditer(text):
        semi = text.find(";", m.end())
        end = len(text) if semi < 0 else semi + 1
        mask_range(out, m.start(), end)

    return "".join(out)


# SPEC.md §6: `Reserved::push` and `Reserved::into_vec` are whitelisted by full
# path. The scanner cannot resolve types, so the audit passes `--allow-reserved`
# and the scanner blanks `.push(`/`.into_vec()` on a receiver named `res` only
# inside a function body that itself binds `res` from the guard: either
# `Ok(res)` / `Ok(mut res)` as a match arm over a `try_with_room(..)` call in
# the same body, or `let [mut] res = ..try_with_room(..)`. A `res` parameter or
# any other `res` outside such a function is still flagged. This is a textual
# falsification gate, not enforcement: inside a guard-using function a second,
# unrelated variable also named `res` would still be suppressed, and the audit
# separately asserts exactly one `try_with_room` call site.
RESERVED_CALL = re.compile(r"\bres\s*\.\s*(?:push|into_vec)\s*(?:\(|::<)")
RESERVED_BIND = re.compile(
    r"\bOk\s*\(\s*(?:mut\s+)?res\s*\)"
    r"|\blet\s+(?:mut\s+)?res\s*(?::[^=;]*)?=\s*[\w:]*try_with_room\b"
)


def reserved_fn_spans(body: str) -> list:
    """(Run on the comment-stripped text: strip_nonexec blanks fn headers.)
    Body spans of fns that call `try_with_room` and bind `res` from it."""
    spans = []
    for m in EXEC_FN.finditer(body):
        o, c = find_fn_body(body, m.end())
        if o is None:
            continue
        inner = body[o:c + 1]
        if "try_with_room" in inner and RESERVED_BIND.search(inner):
            spans.append((o, c + 1))
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
    good_res = ("verus! {\nfn good(n: usize) {\n  match sites::try_with_room::<E>(S, n, t) {\n"
                "    Ok(mut res) => { res.push(e); let v = res.into_vec(); }\n"
                "    Err(o) => {}\n  }\n}\n} // verus!\n")
    check("guard-bound `res` unflagged under allow_reserved",
          scan(good_res, allow_reserved=True) == [])
    check("guard-bound `res` flagged without allow_reserved",
          any(f[1] == "push" for f in scan(good_res)))
    # Positive control on the real guard-using function (skipped if not found).
    real = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                        "..", "ico_verus", "src", "ico.rs")
    if os.path.exists(real):
        rtext = open(real).read()
        check("real ico.rs clean under allow_reserved", scan(rtext, allow_reserved=True) == [])
        check("real ico.rs `res.push` flagged without allow_reserved",
              any(f[1] == "push" for f in scan(rtext)))
    if fails == 0:
        print(f"self-test: OK ({len(constructs)} allocating constructs flagged; look-alikes, ghost Seq::push/insert and comments not)")
    return 1 if fails else 0


def main(argv: list) -> int:
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
    sys.exit(main(sys.argv))
