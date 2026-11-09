#!/usr/bin/env python3
"""Run manifest mutations from a pristine source copy through the logged verifier.

Invoke only after the unmodified 102-vector crate has proved. Each case copies
mutation/base/icns_creusot and leaves its own verifier diagnostics in mutation/mN.
"""
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent
BASE = ROOT / "mutation/base/icns_creusot"
SOURCE = ROOT / "icns_creusot"


def replace_once(text, old, new):
    count = text.count(old)
    if count != 1:
        raise ValueError(f"expected one mutation target, found {count}: {old[:90]!r}")
    return text.replace(old, new, 1)


def mutate(n, text, fallback=False):
    if n == 1:
        return replace_once(text, "if b.len() > 67108864 {", "if b.len() >= 67108864 {")
    if n == 2:
        return replace_once(text, "super::be32_log(b, off + 4) > l - off {",
                            "super::be32_log(b, off + 4) > l - off - 8 {")
    if n == 3:
        old = ("} else if super::be32_log(b, off + 4) < 8 {\n"
               "                Err(Error::InvalidEntryLength)\n"
               "            } else if super::be32_log(b, off + 4) > l - off {\n"
               "                Err(Error::EntryOutOfBounds)\n"
               "            } else if acc.len() == 4096 {\n"
               "                Err(Error::TooManyEntries)")
        new = ("} else if acc.len() == 4096 {\n"
               "                Err(Error::TooManyEntries)\n"
               "            } else if super::be32_log(b, off + 4) < 8 {\n"
               "                Err(Error::InvalidEntryLength)\n"
               "            } else if super::be32_log(b, off + 4) > l - off {\n"
               "                Err(Error::EntryOutOfBounds)")
        return replace_once(text, old, new)
    if n == 4:
        return replace_once(text,
            "if is_legacy_m(t) { false }\n        else if is_some_m(nominal_model(t)) { true }",
            "if is_some_m(nominal_model(t)) { true }")
    if n == 5:
        if fallback:
            marker = "pub fn agree_entry_length_equals_remaining() {"
            return replace_once(text, marker, marker +
                "\n        proof_assert!(me_entry_length_equals_remaining_0().start > me_entry_length_equals_remaining_0().end);")
        return replace_once(text,
            "#[ensures(match result {\n    Ok(es) => parser_ok(bytes@, es@),\n    Err(_) => true,\n})]\npub fn parse",
            "pub fn parse")
    if n == 6:
        return replace_once(text, "(control as usize - 125, false)", "(control as usize - 124, false)")
    if n == 7:
        old = ("if count > n - produced {\n"
               "            proof_assert! { rmodel::ch_over(payload@, ctrl@, produced@, n@, out@); true };\n"
               "            return Err(Error::RleOverrun);\n"
               "        }\n"
               "        let needed = if literal { count } else { 1 };\n"
               "        if payload.len() - cur < needed {\n"
               "            proof_assert! { rmodel::ch_data(payload@, ctrl@, produced@, n@, out@); true };\n"
               "            return Err(Error::RleTruncated);\n"
               "        }")
        new = ("let needed = if literal { count } else { 1 };\n"
               "        if payload.len() - cur < needed {\n"
               "            proof_assert! { rmodel::ch_data(payload@, ctrl@, produced@, n@, out@); true };\n"
               "            return Err(Error::RleTruncated);\n"
               "        }\n"
               "        if count > n - produced {\n"
               "            proof_assert! { rmodel::ch_over(payload@, ctrl@, produced@, n@, out@); true };\n"
               "            return Err(Error::RleOverrun);\n"
               "        }")
        return replace_once(text, old, new)
    if n == 8:
        return replace_once(text, "    &p[4..]\n}", "    if p.len() >= 8 { &p[8..] } else { &p[4..] }\n}")
    if n == 9:
        old = ("let ci = match color_index {\n"
               "        Some(c) => c,\n"
               "        None => {\n"
               "            proof_assert!(mj_model_c(bytes@, entries@, None, mask_index) == Err(Error::MissingColor));\n"
               "            return Err(Error::MissingColor);\n"
               "        }\n"
               "    };\n"
               "    let mi = match mask_index {\n"
               "        Some(m) => m,\n"
               "        None => {\n"
               "            proof_assert!(mj_model_c(bytes@, entries@, Some(ci), None) == Err(Error::MissingMask));\n"
               "            return Err(Error::MissingMask);\n"
               "        }\n"
               "    };")
        new = ("let mi = match mask_index { Some(m) => m, None => return Err(Error::MissingMask) };\n"
               "    let ci = match color_index { Some(c) => c, None => return Err(Error::MissingColor) };")
        return replace_once(text, old, new)
    if n == 10:
        if fallback:
            return replace_once(text, "out[4 * i + 3] = alpha[i];", "out[4 * i + 3] = 0;")
        old = ("Ok((w, h, out)) => mj_model(bytes@, entries_view(entries@), color_index, mask_index) == Ok((w@, h@, out@)),\n"
               "    Err(er) => mj_model(bytes@, entries_view(entries@), color_index, mask_index) == Err(er),")
        new = ("Ok((w, h, out)) => match mj_model(bytes@, entries_view(entries@), color_index, mask_index) {\n"
               "        Ok((mw, mh, mo)) => mw == w@ && mh == h@ && mo.len() == out@.len()\n"
               "            && forall<i: Int> 0 <= i && i < out@.len() && i % 4 != 3 ==> mo[i] == out@[i],\n"
               "        Err(_) => false,\n"
               "    },\n"
               "    Err(er) => mj_model(bytes@, entries_view(entries@), color_index, mask_index) == Err(er),")
        if text.count(old) != 2:
            raise ValueError("expected combine postcondition plus dependent theorem")
        return text.replace(old, new, 1)
    raise ValueError(n)


def create_base():
    if BASE.exists():
        raise FileExistsError(f"mutation base already exists: {BASE}")
    (BASE / "src").mkdir(parents=True)
    for name in ("Cargo.toml", "Cargo.lock", "why3find.json"):
        shutil.copy2(SOURCE / name, BASE / name)
    shutil.copy2(SOURCE / "src/lib.rs", BASE / "src/lib.rs")
    if (SOURCE / ".why3find").exists():
        shutil.copytree(SOURCE / ".why3find", BASE / ".why3find")


def verify_case(n, fallback=False):
    directory = ROOT / f"mutation/m{n}"
    recheck = n == 1 and not fallback and (directory / "result.txt").exists()
    dest = directory / ("fallback/icns_creusot" if fallback else "icns_creusot")
    if dest.exists():
        raise FileExistsError(dest)
    shutil.copytree(BASE, dest)
    p = dest / "src/lib.rs"
    p.write_text(mutate(n, p.read_text(), fallback))
    label = f"mutation {n}" + (" fallback" if fallback else " recheck" if recheck else " direct")
    env = dict(os.environ, VERIFY_CRATE=str(dest.relative_to(ROOT)))
    result = subprocess.run([str(ROOT / "verify.sh"), label], cwd=ROOT,
                            env=env, text=True, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT)
    match = re.search(r"\[verify.sh\] run (\d+) exit=(\d+) .* log=(runs/\d+\.log)", result.stdout)
    if not match:
        raise RuntimeError("verifier wrapper did not report a run: " + result.stdout[-2000:])
    run, _rc, path = match.groups()
    logfile = ROOT / path
    shutil.copy2(logfile, directory / ("fallback.log" if fallback else "recheck.log" if recheck else "direct.log"))
    if result.returncode:
        shutil.copy2(logfile, directory / "failure.log")
    goal_names = re.findall(r"Goal Coma\.(vc_[^:]+): [^\n]+", logfile.read_text())
    summary = f"{label}: run {run}, exit={result.returncode}, log={path}, failed goals={goal_names}\n"
    with (directory / "result.txt").open("a") as output:
        output.write(summary)
    print(summary, flush=True)
    if result.returncode and not goal_names:
        raise RuntimeError(f"mutation {n} exited without a failed proof goal; see {logfile}")
    return result.returncode, goal_names


def main():
    if not BASE.exists():
        create_base()
    if (BASE / "src/lib.rs").read_bytes() != (SOURCE / "src/lib.rs").read_bytes():
        raise RuntimeError("mutation/base is stale; refresh it from the final proved lib.rs")
    for n in range(1, 11):
        rc, goals = verify_case(n)
        if rc == 0 and n in (5, 10):
            rc, goals = verify_case(n, fallback=True)
        if rc == 0:
            print(f"VACUITY: mutation {n} verified", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
