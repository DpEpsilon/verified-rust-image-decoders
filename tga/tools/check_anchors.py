#!/usr/bin/env python3
"""Check that every anchor in tools/mutations.py matches exactly once."""
import importlib.util, os, sys
spec = importlib.util.spec_from_file_location("m", os.path.join(os.path.dirname(os.path.abspath(__file__)), "mutations.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
bad = 0
for n in sorted(m.MUTATIONS):
    for fname, old, new in m.MUTATIONS[n]["edits"]:
        p = os.path.join(m.BASE, "tga_verus", "src", fname)
        t = open(p).read()
        c = t.count(old)
        if c != 1:
            bad += 1
            print("m%-2d %-10s anchor matches %d times" % (n, fname, c))
        else:
            newc = t.replace(old,new,1).count(new)
            print("m%-2d %-10s ok (new text present %d)" % (n, fname, newc))
print("bad:", bad)
sys.exit(1 if bad else 0)
