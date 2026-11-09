#!/bin/bash
# inject.sh <generated.rs> — replace the `model_vectors` module in the crate with the file's body.
set -e
gen="$1"
crate="${2:-icns_creusot}"
python3 - "$gen" "$crate" <<'PY'
import sys
gen, crate = sys.argv[1], sys.argv[2]
p = crate + "/src/lib.rs"
s = open(p).read()
key = "// ===== M4c model_vectors =====\n"
if key in s:
    s = s[:s.index(key)].rstrip() + "\n"
mv = open(gen).read()
j = mv.index("use creusot_std::prelude::*;")
body = mv[j:]
s += key + "#[cfg(creusot)]\npub mod model_vectors {\n"
for line in body.split("\n"):
    s += ("    " + line if line.strip() else line) + "\n"
s += "}\n"
open(p, "w").write(s)
print("injected %d lines" % body.count("\n"))
PY
