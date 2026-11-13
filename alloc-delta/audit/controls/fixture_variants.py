#!/usr/bin/env python3
"""Write the control fixtures that are single-field edits of a base fixture.

Usage: fixture_variants.py <fixtures dir>
Each variant is named after the control that uses it. Edits are line-based
on [meta] keys, so each variant differs from its base in exactly the stated
way (diff the two files to check)."""
import hashlib, os, re, sys

d = sys.argv[1]
NAMED = "A. Fixture-Reviewer (control fixture, not a review)"

def load(n):
    return open(os.path.join(d, n + ".toml")).read()

def meta_set(t, key, val):
    pat = re.compile(rf'^{key} = .*$', re.M)
    line = f'{key} = {val}'
    if pat.search(t):
        return pat.sub(line, t, count=1)
    return t.replace("[meta]\n", f"[meta]\n{line}\n", 1)

def meta_del(t, key):
    return re.sub(rf'^{key} = .*\n', '', t, count=1, flags=re.M)

def meta_get(t, key):
    m = re.search(rf'^{key} = "(.*)"$', t, re.M)
    return m.group(1) if m else None

def write(n, t, why):
    t = f"# CONTROL VARIANT {n}: {why}\n" + t
    open(os.path.join(d, n + ".toml"), "w").write(t)

def reviewer(t, r):
    return meta_set(t, "reviewer", f'"{r}"')

c1 = reviewer(load("c1_clean"), "prototype-fixture")
open(os.path.join(d, "c1_clean.toml"), "w").write(c1)
full = meta_get(c1, "rustc_commit")
write("c1_named_reviewer", reviewer(c1, NAMED), "named reviewer; passes without --prototype")
write("c1_bad_hash", meta_set(c1, "source_hash", '"' + "0" * 64 + '"'), "wrong source_hash")
write("c1_bad_reviewer", reviewer(c1, ""), "empty reviewer")
write("c1_reviewer_missing", meta_del(c1, "reviewer"), "reviewer key absent")
write("c1_commit_mismatch", meta_set(c1, "rustc_commit", '"deadbeef"'), "rustc_commit deadbeef")
write("c1_commit_missing", meta_del(c1, "rustc_commit"), "rustc_commit key absent")
write("c1_commit_short", meta_set(c1, "rustc_commit", '"58cf8f"'), "6-hex commit prefix")
write("c1_commit_toolong", meta_set(c1, "rustc_commit", f'"{full}ab"'), "full hash plus two hex digits")
write("c1_profile_mismatch", meta_set(c1, "profile", '"opt-level=3,debug-assertions=off,share-generics=off"'), "records opt-level=3; runs at 2")
write("c1_profile_missing", meta_del(c1, "profile"), "profile key absent")
write("c1_share_generics", meta_set(c1, "profile", '"opt-level=2,debug-assertions=off,share-generics=on"'), "profile matches a -Zshare-generics=y run")
write("c1_source_hash_missing", meta_del(c1, "source_hash"), "source_hash key absent")
write("c1_target_mismatch", meta_set(c1, "target", '"i686-unknown-linux-gnu"'), "target i686")
write("c1_target_missing", meta_del(c1, "target"), "target key absent")
write("c1_vstd_undeclared", meta_del(c1, "vstd_version"), "no vstd = none on a non-vstd crate")
write("x3_wildcard_build_input", reviewer(c1, NAMED) + '\n[[entry]]\nkind = "include"\nitem = "*"\nsha256 = "00"\nreason = "control: a wildcard build-input declaration"\n', "include item = *")
write("x22_unused_entry", reviewer(c1, NAMED) + '\n[[entry]]\nkind = "alloc"\nitem = "alloc::vec::Vec::<T, A>::push"\nallow = "crate::Holder::len"\ncontract = "control"\ncitation = "control"\n', "an alloc entry whose caller never reaches the item")

c3 = reviewer(load("c3_vstd_ok"), "prototype-fixture")
open(os.path.join(d, "c3_vstd_ok.toml"), "w").write(c3)
write("c3_vstd_change", meta_set(c3, "vstd_version", '"0.0.0-1999-01-01-0000"'), "different vstd version")
write("c3_vstd_missing", meta_del(c3, "vstd_version"), "vstd_version absent")
write("c3_vstd_undet", c3, "same as c3_vstd_ok; the run removes every source of the linked version")
# x7: production binding. The recorded identity is the registry vstd that the
# real cores link (copied from TGA's allowlist); the control links the
# prebuilt /opt/verus/libvstd.rlib, whose version string is the same.
tga = open(os.path.join(d, "..", "..", "allowlist", "x86_64-unknown-linux-gnu", "release", "tga_verus.toml")).read()
x7 = reviewer(c3, NAMED)
for k in ("vstd_svh", "vstd_source_checksum", "verus_builtin_svh"):
    x7 = meta_set(x7, k, f'"{meta_get(tga, k)}"')
write("x7_vstd_artifact", x7, "records the registry vstd's crate hash and locked checksum; the run links a different vstd rlib with the same version string")

for n in ("c2_two_guards", "n11_shared_instance", "n12_suffix_lookalike", "n13_root_lookalike", "n14_two_guards_plus_third"):
    t = reviewer(load(n), "prototype-fixture")
    open(os.path.join(d, n + ".toml"), "w").write(t)
x1 = reviewer(load("x1_extern_namesake"), NAMED)
open(os.path.join(d, "x1_extern_namesake.toml"), "w").write(x1)
write("x1a_ambiguous", x1.replace('allow = "crate::guard::try_zeroed"', 'allow = "guard::try_zeroed"'), "allow written without crate::, naming both the local and the external try_zeroed")

for n in ("x14_inlined_helper", "x23_star_alloc", "x12_helperdep", "x12_foreign_crate", "x2_include_after_url"):
    t = reviewer(load(n), NAMED)
    open(os.path.join(d, n + ".toml"), "w").write(t)
x24 = reviewer(load("x24_env"), NAMED)
open(os.path.join(d, "x24_env.toml"), "w").write(x24)
write("x24_env_undeclared", re.sub(r'\[\[entry\]\]\nkind = "env"\nitem = "ALLOC_AUDIT_X24"\nreason = "control fixture"\n', '', x24), "env entry removed")
x12 = reviewer(load("x12_foreign_crate"), NAMED)
dep = '\n[[entry]]\nkind = "crate"\nitem = "helperdep"\nreason = "control"\n\n[[entry]]\nkind = "dependency"\nitem = "helperdep"\nreport = "$RUN/x12_helperdep.txt"\nreason = "control: helperdep audited in the same run"\n'
write("x12b_admitted", x12 + dep, "helperdep admitted by a crate entry with its dependency contract")
x12c = reviewer(load("x12c_two_helpers"), NAMED)
write("x12c_two_helpers", x12c + dep, "crate entry helperdep while two crates named helperdep are linked")
x2 = reviewer(load("x2_include_after_url"), NAMED)
h = hashlib.sha256(open(os.path.join(d, "..", "testcases", "x", "p.bin"), "rb").read()).hexdigest()
write("x2_declared", x2 + f'\n[[entry]]\nkind = "include"\nitem = "x/p.bin"\nsha256 = "{h}"\nreason = "control: the payload, declared with its hash"\n', "the include declared with its hash")
print("fixture variants written")
