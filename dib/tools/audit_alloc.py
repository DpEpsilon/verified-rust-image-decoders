#!/usr/bin/env python3
"""Conservative source FALSIFICATION lint, not the typed call-graph audit.

It can only find allocating constructs by name (a method-name/path pattern list
over text with spec/proof regions erased). A clean run does not prove absence of
allocation: an allocating method missing from BAD, or reached through a callee,
is not seen. It exists to catch regressions, not to certify the code.

The eraser recognizes this tree's Verus formatting. Contract-bearing functions
must put their body opener on a line at the function's indentation. Unknown
layouts fail closed. New Rust/Verus syntax requires reviewing this lexer too.
"""
import hashlib
from pathlib import Path
import re
import sys

PINS = {
    "mod.rs": "26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7",
    "sites.rs": "33465c740e10162d0d6634afd0a6033e26f3d0f523b40e375f4b1d46bcdf000a",
}


def blank(text):
    return re.sub(r"[^\n]", " ", text)


def code_only(text):
    """Blank comments and literal contents while preserving source positions."""
    out = list(text)
    i = 0
    while i < len(text):
        start = i
        if text.startswith("//", i):
            i = text.find("\n", i)
            if i < 0: i = len(text)
        elif text.startswith("/*", i):
            i += 2
            depth = 1
            while depth and i < len(text):
                if text.startswith("/*", i): depth += 1; i += 2
                elif text.startswith("*/", i): depth -= 1; i += 2
                else: i += 1
            if depth: raise ValueError("unterminated block comment")
        elif (m := re.match(r'(?:br|r)(#*)"', text[i:])):
            end = '"' + m[1]
            j = text.find(end, i + len(m[0]))
            if j < 0: raise ValueError("unterminated raw string")
            i = j + len(end)
        elif text[i] == '"':
            i += 1
            while i < len(text) and text[i] != '"':
                i += 2 if text[i] == "\\" else 1
            if i >= len(text): raise ValueError("unterminated string")
            i += 1
        elif (m := re.match(r"'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^'\\\n])'", text[i:])):
            i += len(m[0])
        else:
            i += 1
            continue
        out[start:i] = blank(text[start:i])
    return "".join(out)


def matching(s, start, left="{", right="}"):
    depth = 0
    for i in range(start, len(s)):
        if s[i] == left: depth += 1
        if s[i] == right:
            depth -= 1
            if depth == 0: return i + 1
    raise ValueError(f"unclosed {left} at {start}")


def erase_ghost(text):
    s = code_only(text)
    out = list(s)
    # Find function boundaries before blanking nested proof regions. Spec/proof
    # function bodies disappear; exec bodies remain, but contracts disappear.
    for m in re.finditer(r"\bfn\s+[A-Za-z_]\w*", s):
        line = s.rfind("\n", 0, m.start()) + 1
        prefix = s[line:m.start()]
        indent = len(prefix) - len(prefix.lstrip())
        op = s.find("(", m.end())
        if op < 0: raise ValueError("function has no parameter list")
        after = matching(s, op, "(", ")")
        body = s.find("{", after)
        semicolon = s.find(";", after)
        if semicolon >= 0 and (body < 0 or semicolon < body):
            # A declaration without a body; retain it for the ordinary scan.
            continue
        if body < 0: raise ValueError("function has no body")
        header = s[after:body]
        if re.search(r"\b(?:requires|ensures|recommends|decreases|returns|opens_invariants)\b", header):
            while True:
                bol = s.rfind("\n", 0, body) + 1
                lead = s[bol:body]
                if not lead.strip() and len(lead) <= indent: break
                body = s.find("{", matching(s, body))
                if body < 0: raise ValueError("unsupported contract/body layout")
        end = matching(s, body)
        if re.search(r"\b(?:spec|proof)\s+$", prefix):
            out[line:end] = blank(s[line:end])
        else:
            out[after:body] = blank(s[after:body])
    s = "".join(out)
    # Proof blocks, including those nested within an executable function.
    for m in re.finditer(r"\bproof\s*\{", s):
        begin = s.index("{", m.start(), m.end())
        end = matching(s, begin)
        out[m.start():end] = blank(s[m.start():end])
    s = "".join(out)
    for m in re.finditer(r"\blet\s+(?:ghost|tracked)\b", s):
        end = s.find(";", m.end())
        if end < 0: raise ValueError("unterminated ghost declaration")
        out[m.start():end + 1] = blank(s[m.start():end + 1])
    return "".join(out)


BAD = re.compile(
    r"\b(?:Vec|String|Box|Rc|Arc|HashMap|HashSet|BTreeMap|BTreeSet|VecDeque|BinaryHeap)\s*::"
    r"|\b(?:vec|format)\s*!"
    r"|\.\s*(?:push\w*|insert\w*|append|split_off|clone|clone_from|to_vec|to_owned|to_string|collect|"
    r"resize\w*|reserve\w*|try_reserve\w*|extend\w*|shrink\w*|splice|into_boxed\w*|into_vec|into_string|"
    r"concat|join|repeat)\s*(?:\(|::<)"
    r"|\bwith_capacity\s*\("
    r"|\b(?:handle_alloc_error|abort|alloc|alloc_zeroed|realloc|from_iter)\s*\("
    r"|\b(?:GlobalAlloc|Allocator)\b"
)


def violations(text):
    s = erase_ghost(text)
    return [(s.count("\n", 0, m.start()) + 1, m[0]) for m in BAD.finditer(s)]


def self_test():
    ghost = '''
open spec fn relation(s: Seq<int>) -> Seq<int> { s.push(1) }
proof fn lemma(s: Seq<int>)
    ensures s.push(1).len() > 0,
{ assert(s.push(1).len() > 0); }
fn run(v: &mut Vec<u8>)
    ensures final(v)@ == old(v)@.push(1),
{
    proof { let s = seq![1]; let t = s.push(2); }
    let ghost t = seq![1].push(2);
    /* nested /* .push(1) */ format!("x") */
    let message = r#"// .push(2)"#;
}
'''
    assert not violations(ghost), violations(ghost)
    for construct in ["v.push(1)", "v . push (1)", "Vec::new()", "vec![1]",
                      "v.reserve(1)", "v.try_reserve_exact(4)", "v.clone()",
                      "format!(\"x\")", "alloc::alloc::handle_alloc_error(l)",
                      "std::process::abort()", "v.collect::<Vec<_>>()",
                      "let _ = v.split_off(1)", "v.clone_from(src)", "v.extend_from_slice(s)",
                      "v.extend(s)", "v.append(&mut w)", "v.insert(0, 1)", "v.resize(4, 0)",
                      "v.resize_with(4, f)", "v.reserve_exact(4)", "v.shrink_to_fit()",
                      "v.into_boxed_slice()", "s.to_vec()", "s.to_owned()", "s.to_string()",
                      "st.clone()", "it.collect::<Vec<u8>>()", "String::from(\"x\")",
                      "Box::new(1)", "Rc::new(1)", "Arc::new(1)", "Vec::from(s)",
                      "with_capacity(4)", "Vec::with_capacity(4)", "vec![0u8; 4]",
                      "x.to_string()", "st.push_str(\"a\")", "format!(\"{}\", 1)"]:
        assert violations(ghost + "\nfn bad() { " + construct + "; }"), construct
    # Positive controls: non-allocating look-alikes must stay unflagged.
    for ok in ["let n = v.len(); let _ = v.get(n);", "let _ = v.split_at(1);", "v.copy_from_slice(s)",
               "v.clone_from_slice(s)", "v.truncate(1)", "v.swap(0, 1)", "let _ = v.iter().sum::<u8>();"]:
        assert not violations("fn ok(v: &mut [u8], s: &[u8]) { " + ok + "; }"), ok
    # Proof-region stripping: Seq::push / Seq::insert inside proofs, specs and
    # ensures clauses are logical operations and must remain unflagged.
    proof_only = '''
fn f(v: &mut Vec<u8>)
    ensures final(v)@ == old(v)@.push(1).insert(0, 2),
{
    proof { let s = Seq::<u8>::empty().push(1); assert(s.push(2).len() == 2); }
}
'''
    assert not violations(proof_only), violations(proof_only)
    print("allocation lint controls: ghost push ignored; executable allocation/push/split_off/clone_from/abort rejected; non-allocating look-alikes accepted")


def main():
    self_test()
    root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path(__file__).resolve().parents[1] / "dib_verus/src"
    if root.is_file(): root = root.parent
    failed = False
    for name, expected in PINS.items():
        p = root / "alloc_guard" / name
        actual = hashlib.sha256(p.read_bytes()).hexdigest()
        print(f"guard {name}: {actual}")
        if actual != expected:
            print(f"FAIL: guard hash mismatch: {p}"); failed = True
    # Explicit production module inventory: additions require review rather than
    # silently acquiring an exemption. Only the cfg(test) policy harness is out.
    expected_modules = {"lib.rs", "dib.rs", "model.rs", "model_vectors.rs", "policy_tests.rs", "alloc_guard/mod.rs", "alloc_guard/sites.rs"}
    actual_modules = {str(p.relative_to(root)) for p in root.rglob("*.rs")}
    if actual_modules != expected_modules:
        print("FAIL: source module inventory changed", actual_modules ^ expected_modules); failed = True
    lib = code_only((root / "lib.rs").read_text())
    if not re.search(r"#\[cfg\(test\)\]\s*mod policy_tests\s*;", lib):
        print("FAIL: policy_tests exclusion is not cfg(test)"); failed = True
    calls = 0
    for name in sorted(expected_modules - {"policy_tests.rs", "alloc_guard/mod.rs", "alloc_guard/sites.rs"}):
        p = root / name
        text = p.read_text()
        for line, token in violations(text):
            print(f"FAIL {p}:{line}: {token}"); failed = True
        calls += len(re.findall(r"\btry_zeroed\s*\(", erase_ghost(text)))
    print(f"executable try_zeroed call sites: {calls} (expected 1)")
    failed |= calls != 1
    print("FAIL" if failed else "OK: source lint only; not a transitive allocation audit")
    return int(failed)


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, AssertionError, OSError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
