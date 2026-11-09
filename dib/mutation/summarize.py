#!/usr/bin/env python3
"""Summarize each mutation/mN/result.txt + failure.log into a kill record."""
import os, re
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DIR = os.path.join(ROOT, "mutation")
for d in sorted(os.listdir(DIR), key=lambda x: (len(x), x)):
    if not re.fullmatch(r"m\d+", d):
        continue
    rt = os.path.join(DIR, d, "result.txt")
    t = open(rt).read() if os.path.exists(rt) else ""
    crate = re.search(r"crate result: (.*)", t)
    crate = crate.group(1).strip() if crate else "<none>"
    m = re.search(r"(\d+) verified, (\d+) errors", crate)
    if m and m.group(2) == "0":
        kind = "SURVIVED"
    elif m:
        kind = "VERIFIER-KILL"
    else:
        kind = "OTHER"
    # first real diagnostic from failure.log
    fl = os.path.join(DIR, d, "failure.log")
    first = "-"; loc = "-"
    if os.path.exists(fl):
        lines = open(fl).read().split("\n")
        for i, ln in enumerate(lines):
            s = ln.strip()
            if s.startswith("error:") and "could not compile" not in s and "aborting" not in s:
                first = s[len("error:"):].strip()
                for j in range(i+1, min(i+4, len(lines))):
                    mm = re.search(r"(-->|at) (\S+:\d+):", lines[j])
                    if mm:
                        loc = mm.group(2); break
                break
    print("%-5s %-13s %-42s %-40s %s" % (d, kind, crate, loc, first))
