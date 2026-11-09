#!/usr/bin/env python3
"""Apply / revert one of the ten mutations of THEOREMS.md's protocol.

Each mutation edits an exact text region of `src/model.rs` or `src/icns.rs` in a
disposable copy of the crate under `mutation/mN/`.  The edits are textual and
assert on the presence of their anchor, so a stale anchor fails loudly instead of
silently mutating nothing.

Usage:
    tools/mutations.py apply N     # copy mutation/base to mutation/mN, mutate
    tools/mutations.py list        # the ten mutations and what each targets
"""

import os
import shutil
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BASE = os.path.join(ROOT, "mutation", "base")
DIR = os.path.join(ROOT, "mutation")

# Each entry: (what, [(file, old, new), ...], expected-failing obligation).
MUTATIONS = {
    1: dict(
        what="MP's size check changed from |b|>67,108,864 to |b|>=67,108,864 "
             "(L2 exact-limit BadMagic, T7 and the size boundary break)",
        edits=[(
            "model.rs",
            "pub open spec fn mp(b: Seq<u8>) -> Result<Seq<MpEntry>, MpError> {\n"
            "    if b.len() > 67108864 {",
            "pub open spec fn mp(b: Seq<u8>) -> Result<Seq<MpEntry>, MpError> {\n"
            "    if b.len() >= 67108864 {",
        )],
        expect="lemma_l2_exact_limit_bad_magic / L2 instantiation",
    ),
    2: dict(
        what="the entry bound check changed from q>L-off to q>L-off-8 "
             "(an entry that exactly fills the file is rejected)",
        edits=[(
            "model.rs",
            "        if q < 8 {\n"
            "            Err(MpError::InvalidEntryLength)\n"
            "        } else if q > L - off {\n"
            "            Err(MpError::EntryOutOfBounds)\n"
            "        } else if acc.len() >= 4096 {",
            "        if q < 8 {\n"
            "            Err(MpError::InvalidEntryLength)\n"
            "        } else if q > L - off - 8 {\n"
            "            Err(MpError::EntryOutOfBounds)\n"
            "        } else if acc.len() >= 4096 {",
        )],
        expect="entry-length-equals-remaining model-vector obligation / T7",
    ),
    3: dict(
        what="TooManyEntries moved ahead of the 4,097th entry's length check "
             "(a malformed 4,097th header must lose to the count limit)",
        edits=[
            # `scan` (the model MP itself) and `walk` (its count-indexed form)
            # must move together, so the wrong order is a property of the model
            # rather than an inconsistency between the two loops.  The L6
            # statements are untouched; it is their proofs that must fail.
            (
                "model.rs",
                "        if q < 8 {\n"
                "            Err(MpError::InvalidEntryLength)\n"
                "        } else if q > L - off {\n"
                "            Err(MpError::EntryOutOfBounds)\n"
                "        } else if acc.len() >= 4096 {\n"
                "            Err(MpError::TooManyEntries)\n"
                "        } else {",
                "        if acc.len() >= 4096 {\n"
                "            Err(MpError::TooManyEntries)\n"
                "        } else if q < 8 {\n"
                "            Err(MpError::InvalidEntryLength)\n"
                "        } else if q > L - off {\n"
                "            Err(MpError::EntryOutOfBounds)\n"
                "        } else {",
            ),
            (
                "model.rs",
                "        if q < 8 {\n"
                "            Err(MpError::InvalidEntryLength)\n"
                "        } else if q > L - off {\n"
                "            Err(MpError::EntryOutOfBounds)\n"
                "        } else if j >= 4096 {\n"
                "            Err(MpError::TooManyEntries)\n"
                "        } else {",
                "        if j >= 4096 {\n"
                "            Err(MpError::TooManyEntries)\n"
                "        } else if q < 8 {\n"
                "            Err(MpError::InvalidEntryLength)\n"
                "        } else if q > L - off {\n"
                "            Err(MpError::EntryOutOfBounds)\n"
                "        } else {",
            ),
        ],
        expect="lemma_l6_vectors (q=7 invalid-4097th-entry-before-limit)",
    ),
    4: dict(
        what="the legacy(c)=None guard deleted from signature classification "
             "(an R8 mask whose payload starts with a PNG signature becomes opaque)",
        edits=[(
            "model.rs",
            "    if legacy_side(code).is_some() {\n"
            "        (false, None)\n"
            "    } else if nominal_side(code).is_some() {",
            "    if nominal_side(code).is_some() {",
        )],
        expect="png-signature-legacy-mask-stays-mask / T7",
    ),
    5: dict(
        what="the range conjunct deleted from entry_inv, i.e. the parse-success "
             "range/provenance postcondition weakened (T3-T6 call sites and T13 "
             "must no longer be derivable)",
        edits=[(
            "model.rs",
            "pub open spec fn entry_inv(b: Seq<u8>, e: Entry) -> bool {\n"
            "    &&& 8 <= e.payload[0] <= e.payload[1] <= b.len()\n"
            "    &&& is_hex8(e.type_hex@)",
            "pub open spec fn entry_inv(b: Seq<u8>, e: Entry) -> bool {\n"
            "    &&& is_hex8(e.type_hex@)",
        )],
        expect="decode_rgb/decode_mask payload_range precondition (T3-T6 derivation)",
    ),
    6: dict(
        what="a repeat run count changed from control-125 to control-124 "
             "(MR's repeat arithmetic no longer matches R12)",
        edits=[(
            "model.rs",
            "pub open spec fn rle_count(control: u8) -> int {\n"
            "    if control < 128 {\n"
            "        control as int + 1\n"
            "    } else {\n"
            "        control as int - 125\n"
            "    }\n"
            "}",
            "pub open spec fn rle_count(control: u8) -> int {\n"
            "    if control < 128 {\n"
            "        control as int + 1\n"
            "    } else {\n"
            "        control as int - 124\n"
            "    }\n"
            "}",
        )],
        expect="rle_decode postcondition (T9) / rgb-is32, repeat-overrun-before-missing-value",
    ),
    7: dict(
        what="MR's overrun and payload-availability checks swapped, so a missing "
             "data byte is reported before the overrun",
        edits=[(
            "model.rs",
            "        let control = s[cur];\n"
            "        if rle_count(control) > n - p {\n"
            "            Err(Error::RleOverrun)\n"
            "        } else if cur + 1 + rle_needed(control) > s.len() {\n"
            "            Err(Error::RleTruncated)\n"
            "        } else {",
            "        let control = s[cur];\n"
            "        if cur + 1 + rle_needed(control) > s.len() {\n"
            "            Err(Error::RleTruncated)\n"
            "        } else if rle_count(control) > n - p {\n"
            "            Err(Error::RleOverrun)\n"
            "        } else {",
        )],
        expect="rle_decode postcondition (T9) / repeat-overrun-before-missing-value",
    ),
    8: dict(
        what="R11's it32 prefix removal strips eight zero bytes rather than "
             "exactly four",
        edits=[(
            "model.rs",
            "pub open spec fn it32_prefix(p: Seq<u8>) -> Seq<u8> {\n"
            "    if p.len() >= 4 && p[0] == 0 && p[1] == 0 && p[2] == 0 && p[3] == 0 {\n"
            "        p.subrange(4, p.len() as int)\n"
            "    } else {\n"
            "        p\n"
            "    }\n"
            "}",
            "pub open spec fn it32_prefix(p: Seq<u8>) -> Seq<u8> {\n"
            "    if p.len() >= 8 && p[0] == 0 && p[1] == 0 && p[2] == 0 && p[3] == 0\n"
            "        && p[4] == 0 && p[5] == 0 && p[6] == 0 && p[7] == 0 {\n"
            "        p.subrange(8, p.len() as int)\n"
            "    } else {\n"
            "        p\n"
            "    }\n"
            "}",
        )],
        expect="decode_rgb postcondition (T10) / it32-eight-zero-prefix-then-literals",
    ),
    9: dict(
        what="MJ's absent-index priority reversed: the mask index is tested "
             "before the colour index",
        edits=[(
            "model.rs",
            "    match ci {\n"
            "        None => Err(Error::MissingColor),\n"
            "        Some(i) => match mi {\n"
            "            None => Err(Error::MissingMask),\n"
            "            Some(j) => match mc(b, E, i) {",
            "    match mi {\n"
            "        None => Err(Error::MissingMask),\n"
            "        Some(j) => match ci {\n"
            "            None => Err(Error::MissingColor),\n"
            "            Some(i) => match mc(b, E, i) {",
        )],
        expect="combine postcondition (T12) / mask-without-colour",
    ),
    10: dict(
        what="the RGBA alpha-byte equality removed from combine's modular loop "
             "postcondition (the interleave equality is no longer derivable)",
        edits=[(
            "icns.rs",
            "            forall|k: int| 0 <= k < i ==> #[trigger] out@[4 * k + 2] == rgb.data@[3 * k + 2],\n"
            "            forall|k: int| 0 <= k < i ==> #[trigger] out@[4 * k + 3] == mask.data@[k],\n"
            "        decreases n - i,",
            "            forall|k: int| 0 <= k < i ==> #[trigger] out@[4 * k + 2] == rgb.data@[3 * k + 2],\n"
            "        decreases n - i,",
        )],
        expect="combine postcondition (T12) / combine-nonuniform-alpha",
    ),
}

# Mutation 5's mandated fallback: if the verifier re-derives the range bound by
# unfolding, assert the false `start > end` claim at the
# entry-length-equals-remaining vector instead.
M5_FALLBACK = dict(
    what="fallback for mutation 5: assert the false claim start > end for the "
         "parsed entry of entry-length-equals-remaining",
    edits=[(
        "model_vectors.rs",
        "pub proof fn agree_entry_length_equals_remaining()\n"
        "    ensures\n"
        "        mp(b_entry_length_equals_remaining()) == Ok(e_entry_length_equals_remaining()),\n"
        "{\n"
        "    assert(mp(b_entry_length_equals_remaining()) == Ok(e_entry_length_equals_remaining())) by (compute);\n"
        "}",
        "pub proof fn agree_entry_length_equals_remaining()\n"
        "    ensures\n"
        "        mp(b_entry_length_equals_remaining()) == Ok(e_entry_length_equals_remaining()),\n"
        "{\n"
        "    assert(mp(b_entry_length_equals_remaining()) == Ok(e_entry_length_equals_remaining())) by (compute);\n"
        "    // mutation-5 fallback: the parsed entry's payload range would be inverted.\n"
        "    let ms = e_entry_length_equals_remaining();\n"
        "    assert(ms[0].payload.0 > ms[0].payload.1);\n"
        "}",
    )],
    expect="agree_entry_length_equals_remaining (false start > end claim)",
)


def crate_dir(n):
    return os.path.join(DIR, "m%d" % n, "icns_verus")


def apply_edits(src_dir, edits):
    for fname, old, new in edits:
        path = os.path.join(src_dir, fname)
        text = open(path).read()
        if text.count(old) != 1:
            raise SystemExit(
                "anchor for %s matched %d times (expected 1); refusing to mutate"
                % (fname, text.count(old)))
        open(path, "w").write(text.replace(old, new, 1))


def cmd_apply(n, fallback=False):
    spec = MUTATIONS[n]
    if fallback:
        spec = M5_FALLBACK
    dest = os.path.join(DIR, "m%d" % n)
    if os.path.exists(dest):
        shutil.rmtree(dest)
    os.makedirs(os.path.join(dest, "icns_verus"))
    for name in ("Cargo.toml", "Cargo.lock", "src", "tests", ".gitignore"):
        src = os.path.join(BASE, name)
        if os.path.isdir(src):
            shutil.copytree(src, os.path.join(dest, "icns_verus", name))
        else:
            shutil.copy2(src, os.path.join(dest, "icns_verus", name))
    apply_edits(os.path.join(dest, "icns_verus", "src"), spec["edits"])
    print("mutation %d applied to mutation/m%d/icns_verus" % (n, n))
    print("  what: %s" % spec["what"])
    print("  expected failing obligation: %s" % spec["expect"])


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return 2
    cmd = sys.argv[1]
    if cmd == "list":
        for n in sorted(MUTATIONS):
            m = MUTATIONS[n]
            print("%2d  %s" % (n, m["what"]))
            print("    expected: %s" % m["expect"])
        return 0
    if cmd == "apply":
        n = int(sys.argv[2])
        fallback = len(sys.argv) > 3 and sys.argv[3] == "fallback"
        cmd_apply(n, fallback)
        return 0
    print(__doc__)
    return 2


if __name__ == "__main__":
    sys.exit(main())