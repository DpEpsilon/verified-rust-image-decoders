#!/usr/bin/env python3
"""Render the markdown mutation table of PROOF-REPORT-M4b.md from the
mutation/mN/result.txt files produced by tools/run_mutations.sh.

Prints one row per mutation: the mutation, the failing obligations named in the
kept verifier log, the crate result and the log path.
"""
import importlib.util
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def load_mutations():
    spec = importlib.util.spec_from_file_location(
        "m", os.path.join(ROOT, "tools", "mutations.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m.MUTATIONS


def crate_summary(text):
    idx = text.find("Checking tga_verus")
    crate = text[idx:] if idx >= 0 else text
    sums = re.findall(r"verification results:: (\d+) verified, (\d+) errors", crate)
    if sums:
        return "%s verified, %s errors" % sums[-1]
    return None


def failing_obligations(text):
    """Names of the proof items the log blames, in first-seen order."""
    out = []
    # generated vector obligations
    for m in re.finditer(r"pub proof fn agree_(\w+)", text):
        n = "agree_" + m.group(1)
        if n not in out:
            out.append(n)
    # named lemmas / functions with a `error:` diagnostic pointing at their line
    for m in re.finditer(r"^\s+--> (src/[^\s:]+):(\d+):\d+$", text, re.M):
        out.append(m.group(1) + ":" + m.group(2))
    # deduplicate while keeping order, cap
    seen = []
    for o in out:
        if o not in seen:
            seen.append(o)
    return seen


def main():
    M = load_mutations()
    for n in sorted(M):
        d = os.path.join(ROOT, "mutation", "m%d" % n)
        log = os.path.join(d, "failure.log")
        if not os.path.exists(log):
            continue
        text = open(log).read()
        s = crate_summary(text)
        nerr = len(re.findall(r"^error(?:\[[^\]]+\])?: ", text, re.M))
        res = "%s errors" % (s.split(", ")[1].split()[0] if s else "no summary")
        print("| %d | %s | %s / %d diagnostics | %s | `mutation/m%d/failure.log` |"
              % (n, M[n]["what"].split("(")[0].strip(), res, nerr, s or
                 "compile aborted", n))


if __name__ == "__main__":
    sys.exit(main())
