#!/usr/bin/env python3
"""Rewrite mutation/mN/result.txt from mutation/mN/failure.log.

The verifier prints two `verification results` lines per run: one for `vstd`
(the support crate, 2159 verified, 0 errors) and one for the crate under test.
Only the second is evidence, and when the crate fails to compile there is none
at all -- `cargo verus` then exits non-zero without a summary.  This script
records both facts honestly, plus the first few `error:` diagnostics with the
source locations they point at.
"""
import glob
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def summarise(d):
    log = os.path.join(d, "failure.log")
    if not os.path.exists(log):
        return None
    text = open(log).read()
    idx = text.find("Checking tga_verus")
    crate = text[idx:] if idx >= 0 else text
    sums = re.findall(r"verification results:: (\d+) verified, (\d+) errors", crate)
    # Pair each `error:` diagnostic with the first source location after it, so
    # the annotation belongs to the error it follows.
    diags = []
    for m in re.finditer(r"^error(?:\[[^\]]+\])?: (.*)$", crate, re.M):
        what = m.group(1)
        loc = ""
        lm = re.search(r"^\s+--> (src/[^\s:]+:\d+:\d+)$", crate[m.end():], re.M)
        if lm:
            loc = " at " + lm.group(1)
        diags.append(what + loc)
    lines = []
    lines.append("mutation %s" % os.path.basename(d)[1:])
    if sums:
        lines.append("crate result: %s verified, %s errors" % sums[-1])
    else:
        lines.append("crate result: no verification summary "
                     "(the crate failed to compile, so no obligation was checked)")
    lines.append("error diagnostics: %d" % len(diags))
    for e in diags[:14]:
        lines.append("  %s" % e)
    lines.append("log: %s" % os.path.relpath(log, ROOT))
    return "\n".join(lines) + "\n"


def main():
    out = []
    for d in sorted(glob.glob(os.path.join(ROOT, "mutation", "m[0-9]*"))):
        s = summarise(d)
        if s is None:
            continue
        open(os.path.join(d, "result.txt"), "w").write(s)
        out.append(s)
    sys.stdout.write("\n".join(out))


if __name__ == "__main__":
    main()
