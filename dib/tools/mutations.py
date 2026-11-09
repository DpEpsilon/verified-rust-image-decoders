#!/usr/bin/env python3
"""Implementation-side mutations for THEOREMS.md's proof-sensitivity protocol.

Each mutation edits ONLY `src/dib.rs` in a disposable copy
under `mutation/mN/dib_verus`, rebuilt from the pristine `mutation/base/dib_verus`
(a fresh copy of the current live `src`).  The frozen model, `model_vectors.rs`,
the vectors and `spec-frozen/` are untouched, so a failure is a *verifier*
diagnostic on the executable obligation the manifest names, not a corrupted
model.

Usage:
    tools/mutations.py apply N
    tools/mutations.py list
"""

import os
import shutil
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BASE = os.path.join(ROOT, "mutation", "base", "dib_verus")
DIR = os.path.join(ROOT, "mutation")

# `file` names are relative to the crate `src/`.  `old`/`new` are exact strings.
MUTATIONS = {
    1: dict(
        what="the executable input cap `>` becomes `>=` in `parse_header`",
        edits=[("dib.rs", "    if len > MAX_INPUT {", "    if len >= MAX_INPUT {")],
        expect="decode=T2/T9/T10 (L2 boundary reject: input-exact-64m)",
    ),
    2: dict(
        what="swap dimensions and planes validation order in the executable header",
        edits=[("dib.rs", "    if planes != 1 {\n        proof {\n            reveal(crate::model::mh);\n        }\n        return Err(Error::InvalidPlanes);\n    }\n    if width_i <= 0 || signed_height == 0 || signed_height == i32::MIN {\n        proof {\n            reveal(crate::model::mh);\n            reveal(crate::model::s32_min);\n            assert(signed_height as int == 0 || signed_height as int == crate::model::s32_min()\n                || width_i <= 0);\n        }\n        return Err(Error::InvalidDimensions);\n    }",
                "    if width_i <= 0 || signed_height == 0 || signed_height == i32::MIN {\n        proof {\n            reveal(crate::model::mh);\n            reveal(crate::model::s32_min);\n            assert(signed_height as int == 0 || signed_height as int == crate::model::s32_min()\n                || width_i <= 0);\n        }\n        return Err(Error::InvalidDimensions);\n    }\n    if planes != 1 {\n        proof {\n            reveal(crate::model::mh);\n        }\n        return Err(Error::InvalidPlanes);\n    }")],
        expect="T2/T10 (planes-before-dimensions)",
    ),
    3: dict(
        what="parse OS/2 16-byte dimensions at CORE 16-bit offsets",
        edits=[("dib.rs", "    } else if size == 16 {\n        proof {\n            assert(bytes@.len() >= start + 16);\n        }\n        width_i = rd32s(bytes, start + 4);\n        signed_height = rd32s(bytes, start + 8);",
                "    } else if size == 16 {\n        proof {\n            assert(bytes@.len() >= start + 16);\n        }\n        width_i = rd16(bytes, start + 4) as i32;\n        signed_height = rd16(bytes, start + 6) as i32;")],
        expect="T2/T9 (header-16-BareDib)",
    ),
    4: dict(
        what="ignore biClrUsed for an indexed palette",
        edits=[("dib.rs", "        if used != 0 {\n            used\n        } else {\n            1u32 << bits\n        }",
                "        if false && used != 0 {\n            used\n        } else {\n            1u32 << bits\n        }")],
        expect="T2/T9 (palette-short-one-8)",
    ),
    5: dict(
        what="use a three-byte stride for INFO palette entries",
        edits=[("dib.rs", "    let pal_stride: usize = if core {\n        3\n    } else {\n        4\n    };",
                "    let pal_stride: usize = if core {\n        3\n    } else {\n        3\n    };")],
        expect="T2/T4/T9 (indexed-4-header-16)",
    ),
    6: dict(
        what="remove index guard before borrowed palette access",
        edits=[("dib.rs", "    if ix >= count {", "    if false && ix >= count {")],
        expect="T1/T4 (palette-short-invalid-index-4)",
    ),
    7: dict(
        what="permit overlapping RGB and alpha masks",
        edits=[("dib.rs", "    if (m0 & m1) != 0 || (m0 & m2) != 0 || (m0 & m3) != 0 || (m1 & m2) != 0 || (m1 & m3) != 0\n        || (m2 & m3) != 0 {",
                "    if false && ((m0 & m1) != 0 || (m0 & m2) != 0 || (m0 & m3) != 0 || (m1 & m2) != 0 || (m1 & m3) != 0\n        || (m2 & m3) != 0) {")],
        expect="T2/T9 (masks-overlap)",
    ),
    8: dict(
        what="replicate five-bit components instead of rounding",
        edits=[("dib.rs", "    val as u8\n}\n\n/// R6: R,G,B masks", "    if max == 31 { ((component << 3) | (component >> 2)) as u8 } else { val as u8 }\n}\n\n/// R6: R,G,B masks")],
        expect="T4/T9 (bi-rgb-16-630c)",
    ),
    9: dict(
        what="use BI_RGB32 alpha byte outside ICO",
        edits=[("dib.rs", "                _ => [rd(b, p + 2), rd(b, p + 1), rd(b, p), 255],",
                "                _ => [rd(b, p + 2), rd(b, p + 1), rd(b, p), rd(b, p + 3)],")],
        expect="T4/T9 (bi-rgb-32-11223300)",
    ),
    10: dict(
        what="ignore AND on an all-zero explicit ICO XOR alpha plane",
        edits=[("dib.rs", "        if !has_xor_alpha || !any_alpha {", "        if !has_xor_alpha {")],
        expect="T8/T9 (ico-alpha-zero-fallback-mask)",
    ),
    11: dict(
        what="reverse vertical row orientation",
        edits=[("dib.rs", "    let by = if h.stored_height < 0 {\n        y\n    } else {\n        h.height as usize - 1 - y\n    };",
                "    let by = if h.stored_height < 0 {\n        h.height as usize - 1 - y\n    } else {\n        y\n    };")],
        expect="T5/T9 (bottom-up-two-rows)",
    ),
    12: dict(
        what="advance each raw row by unpadded bytes",
        edits=[("dib.rs", "    let line = stride(h.width, h.bits);", "    let line = (h.width as usize * h.bits as usize + 7) / 8;")],
        expect="T5/T6/T9 (row-padding-24-two-pixels)",
    ),
    13: dict(
        what="the executable allows an RLE run to cross a row (bounds check removed)",
        edits=[("dib.rs", "        if *y >= h.height as usize || *x + quantity > h.width as usize {",
                "        if *y >= h.height as usize {")],
        expect="T7/T10 (rle8-run-over-row / rle8-absolute-row-overrun-precedes-short)",
    ),
    16: dict(
        what="accept the end of an RLE stream without an EOB command",
        edits=[("dib.rs", "        return (Err(Error::RleTruncated), Ghost(gw0));\n    }\n    let quantity = rd(b, *pos) as usize;",
                "        return (Ok(true), Ghost(gw0));\n    }\n    let quantity = rd(b, *pos) as usize;")],
        expect="T7/T9 (rle4-missing-end)",
    ),
    19: dict(
        what="the executable scans every remaining byte on each RLE command",
        edits=[("dib.rs", "    proof { *steps = Ghost(steps@ + 1); }\n    if b.len() - *pos < 2 {",
                "    let mut scan = *pos;\n    while scan < b.len()\n        invariant *pos <= scan <= b.len(), *pos <= b.len(),\n        decreases b.len() - scan,\n    {\n        let _byte = rd(b, scan);\n        scan += 1;\n        proof { *steps = Ghost(steps@ + 1); }\n    }\n    proof { *steps = Ghost(steps@ + 1); }\n    if b.len() - *pos < 2 {")],
        expect="T12 (linear work: a real remaining-suffix scan inside each command)",
    ),
    23: dict(
        what="the executable reverses the AND bit order inside each byte",
        edits=[("dib.rs", "            let bit = raw_bit & (0x80u8 >> (x % 8));",
                "            let bit = raw_bit & (1u8 << (x % 8));")],
        expect="T8/T9 (ico-mask-msb-and-padding, ico-mask-second-byte-msb)",
    ),
    24: dict(
        what="the executable reverses the RLE4 high/low nibble order",
        edits=[("dib.rs", "            ((code >> (if j % 2 == 0 { 4 } else { 0 })) & 0x0f) as u32",
                "            ((code >> (if j % 2 == 0 { 0 } else { 4 })) & 0x0f) as u32")],
        expect="T7/T9 (rle4-encoded, rle4-absolute-odd)",
    ),
    25: dict(
        what="the executable ignores the RLE delta offset (no advance)",
        edits=[("dib.rs", "        *x = *x + dx;\n        *y = *y + dy;",
                "        *x = *x;\n        *y = *y + dy;")],
        expect="T7/T9 (rle8-delta, rle8-zero-delta)",
    ),
    26: dict(
        what="reject EOB until the final RLE row has been filled",
        edits=[("dib.rs", "        return (Ok(true), Ghost(gw0));",
                "        if *y < h.height as usize && (*y + 1 < h.height as usize || *x < h.width as usize) { return (Err(Error::RleTruncated), Ghost(gw0)); }\n        return (Ok(true), Ghost(gw0));")],
        expect="T7/T9 (rle8-early-end)",
    ),
    27: dict(
        what="start BmpFile raw pixels at the palette end instead of bfOffBits",
        edits=[("dib.rs", "    let r = raw_loop(b, h, out, h.pixel_start, &mut gcur, steps);",
                "    let r = raw_loop(b, h, out, if h.mode == Mode::BmpFile { h.palette_end } else { h.pixel_start }, &mut gcur, steps);")],
        expect="T6/T9 (bmp-indexed-gap-4)",
    ),
    34: dict(
        what="skip the bfOffBits-versus-table-end check for high-depth BMP optimal tables",
        edits=[("dib.rs", "    if is_bmp && (file_offset as u64) < palette_end64 {",
                "    if is_bmp && bits <= 8 && (file_offset as u64) < palette_end64 {")],
        expect="T2/T9 (bmp-optimal-table-offset-one-byte-short)",
    ),
    29: dict(
        what="the executable uses 16-bit BI_RGB bit 15 as alpha",
        edits=[("dib.rs", "            Ok([channel(word, 0x7c00), channel(word, 0x03e0), channel(word, 0x001f), 255])",
                "            Ok([channel(word, 0x7c00), channel(word, 0x03e0), channel(word, 0x001f), if word & 0x8000 == 0 { 0 } else { 255 }])")],
        expect="T4/T9 (bi-rgb-16-4404)",
    ),
    14: dict(
        what="check absolute payload completeness before row bounds",
        edits=[("dib.rs", "    if *y >= h.height as usize || *x + q > h.width as usize {",
                "    if b.len() - *pos < (if h.bits == 8 { q } else { (q + 1) / 2 }) + (if h.bits == 8 { q } else { (q + 1) / 2 }) % 2 { return (Err(Error::RleTruncated), Ghost(gw0)); }\n    if *y >= h.height as usize || *x + q > h.width as usize {")],
        expect="T7/T10 (rle8-absolute-row-overrun-precedes-short)",
    ),
    15: dict(
        what="inspect first absolute palette index before whole packet availability",
        edits=[("dib.rs", "    if b.len() - *pos < padded {",
                "    if b.len() > *pos && ((if h.bits == 8 { rd(b, *pos) } else { rd(b, *pos) >> 4 }) as u32) >= h.count { return (Err(Error::PaletteIndexOutOfRange), Ghost(gw0)); }\n    if b.len() - *pos < padded {")],
        expect="T7/T10 (rle8-absolute-trunc-before-index)",
    ),
    17: dict(
        what="omit the full ICO mask availability check before mask reads",
        edits=[("dib.rs", "        if b.len() - pos < and_bytes {", "        if false && b.len() - pos < and_bytes {")],
        expect="T1/T8 (ico-truncated-and)",
    ),
    18: dict(
        what="compute N via wrapping u16 multiplication",
        edits=[("dib.rs", "    let n64 = width_i as u64 * height_v as u64;",
                "    let n64 = (width_i as u16).wrapping_mul(height_v as u16) as u64;")],
        expect="T1/T2/T3 (output-exact-64m-rle8)",
    ),
    20: dict(
        what="allocate a second full-size scratch RGBA buffer",
        edits=[("dib.rs", "    let out = vec![0u8; 4 * h.n];", "    let _scratch = vec![0u8; 4 * h.n];\n    let out = vec![0u8; 4 * h.n];")],
        expect="T3 (output-exact-64m-rle8; allocation-call count)",
    ),
    21: dict(
        what="accept arbitrary trailing bytes in bare DIB",
        edits=[("dib.rs", "            Mode::BareDib => {\n                if pos != b.len() {\n                    proof {",
                "            Mode::BareDib => {\n                if false && pos != b.len() {\n                    proof {")],
        expect="T9/T10 (rle8-early-eob-trailing)",
    ),
    22: dict(
        what="discard the ICO BI_RGB32 pixel alpha byte",
        edits=[("dib.rs", "                Mode::IcoEntryDib | Mode::IcoEntryDibSlack => {\n                    [rd(b, p + 2), rd(b, p + 1), rd(b, p), rd(b, p + 3)]\n                },",
                "                Mode::IcoEntryDib | Mode::IcoEntryDibSlack => {\n                    [rd(b, p + 2), rd(b, p + 1), rd(b, p), 255]\n                },")],
        expect="T4/T9 (ico-32-and-0)",
    ),
    28: dict(
        what="permit non-contiguous BITFIELDS masks",
        edits=[("dib.rs", "    (compact & compact.wrapping_add(1)) == 0", "    true")],
        expect="T2/T9 (masks-hole)",
    ),
    30: dict(
        what="reject ICO image size declared as X+A",
        edits=[("dib.rs", "        let valid = h.declared as u64 == image_len as u64 ||\n            (ico && h.declared as u64 == image_len as u64 + and_bytes as u64);",
                "        let valid = h.declared as u64 == image_len as u64;")],
        expect="T8/T9/T10 (ico-image-size-xor-and)",
    ),
    31: dict(
        what="apply AND as veto despite nonzero explicit XOR alpha",
        edits=[("dib.rs", "        if !has_xor_alpha || !any_alpha {", "        if true {")],
        expect="T8/T9 (ico-alpha-nonzero-ignores-and)",
    ),
    32: dict(
        what="accept an EOL after the final row",
        edits=[("dib.rs", "        if *y >= h.height as usize {\n            proof {\n                crate::model::lemma_mr_exit_eol_bounds", "        if false && *y >= h.height as usize {\n            proof {\n                crate::model::lemma_mr_exit_eol_bounds")],
        expect="T7/T9/T10 (rle8-eol-past-height)",
    ),
    33: dict(
        what="accept BITFIELDS masks outside pixel depth",
        edits=[("dib.rs", "    if !inside(m0, bits) || !inside(m1, bits) || !inside(m2, bits) || !inside(m3, bits) {",
                "    if false && (!inside(m0, bits) || !inside(m1, bits) || !inside(m2, bits) || !inside(m3, bits)) {")],
        expect="T2/T9 (masks-outside)",
    ),
    35: dict(
        what="treat BITFIELDS without an A mask as explicit ICO alpha",
        edits=[("dib.rs", "        let has_xor_alpha = h.bits == 32 && (h.compression == 0 || h.masks[3] != 0);",
                "        let has_xor_alpha = h.bits == 32;")],
        expect="T8/T9 (ico-fields-32-no-alpha-and-veto)",
    ),
    # ---- M1.1c: the delta's implementation-side mutants (36)-(43).  Each is
    # faithful to THEOREMS-DELTA.md's named mutant; the target vector is noted.
    36: dict(
        what="R19 mut (1): treat the new slack mode like BareDib at the final cursor check",
        edits=[("dib.rs", "        let exact = match h.mode {\n            Mode::BareDib | Mode::IcoEntryDib => true,\n            _ => false,\n        };",
                "        let exact = match h.mode {\n            Mode::BareDib | Mode::IcoEntryDib | Mode::IcoEntryDibSlack => true,\n            _ => false,\n        };")],
        expect="T9/T10 (slack-1-new / max-input-slack-IcoEntryDibSlack become TrailingBytes)",
    ),
    37: dict(
        what="R19 mut (2): apply the slack policy to the old ICO mode too",
        edits=[("dib.rs", "        let exact = match h.mode {\n            Mode::BareDib | Mode::IcoEntryDib => true,\n            _ => false,\n        };",
                "        let exact = match h.mode {\n            Mode::BareDib => true,\n            _ => false,\n        };")],
        expect="T9/T10 (slack-1-old / ico-* stop reporting TrailingBytes)",
    ),
    38: dict(
        what="R19 mut (3): skip the AND-mask availability check when a suffix exists (in either ICO mode)",
        edits=[("dib.rs", "        if b.len() - pos < and_bytes {", "        if b.len() - pos < and_bytes && h.mode == Mode::IcoEntryDib {")],
        expect="T1/T8/T9 (mask-short-3-new / ico-truncated-and: reads past the slice)",
    ),
    39: dict(
        what="R19 mut (4): accept any nonzero biSizeImage in the slack mode (no X / X+A rule)",
        edits=[("dib.rs", "        let valid = h.declared as u64 == image_len as u64 ||\n            (ico && h.declared as u64 == image_len as u64 + and_bytes as u64);",
                "        let valid = h.declared as u64 == image_len as u64\n            || (h.mode == Mode::IcoEntryDibSlack) || (ico && h.declared as u64 == image_len as u64 + and_bytes as u64);")],
        expect="T8/T10 (image-size-9-slack / image-size-includes-slack-new become Ok)",
    ),
    40: dict(
        what="R19 mut (5a): omit the doubled-height rule for the slack mode (use the raw stored height)",
        edits=[("dib.rs", "        Mode::IcoEntryDib | Mode::IcoEntryDibSlack => {\n            if signed_height < 0 || signed_height % 2 != 0 {\n                proof {\n                    reveal(crate::model::mh);\n                }\n                return Err(Error::InvalidDimensions);\n            }\n        }",
                "        Mode::IcoEntryDib => {\n            if signed_height < 0 || signed_height % 2 != 0 {\n                proof {\n                    reveal(crate::model::mh);\n                }\n                return Err(Error::InvalidDimensions);\n            }\n        }\n        Mode::IcoEntryDibSlack => {}")],
        expect="T2/T8/T9 (odd-height-new becomes Ok)",
    ),
    41: dict(
        what="R19 mut (5b): ignore the BI_RGB32 XOR alpha in the slack mode (always apply the AND fallback)",
        edits=[("dib.rs", "        let has_xor_alpha = h.bits == 32 && (h.compression == 0 || h.masks[3] != 0);\n        let any_alpha = alpha_present(h, out, steps);\n        if !has_xor_alpha || !any_alpha {",
                "        let has_xor_alpha = h.bits == 32 && (h.compression == 0 || h.masks[3] != 0)\n            && h.mode != Mode::IcoEntryDibSlack;\n        let any_alpha = alpha_present(h, out, steps);\n        if !has_xor_alpha || !any_alpha {")],
        expect="T8/T9 (alpha32-7f-mask80-new / alpha32-00-mask00-new bytes change)",
    ),
    42: dict(
        what="R19 mut (6): allow RLE8/RLE4 in the slack ICO mode",
        edits=[("dib.rs", "    if (signed_height < 0 && rle) || (ico && rle) {",
                "    if (signed_height < 0 && rle) || (ico && mode != Mode::IcoEntryDibSlack && rle) {")],
        expect="T2/T7 (rle8-forbidden-new / rle4-forbidden-new stop reporting UnsupportedOrientation)",
    ),
    43: dict(
        what="R19 mut (7): change the input cap test from `>` to `>=` (one byte off)",
        edits=[("dib.rs", "    if len > MAX_INPUT {", "    if len >= MAX_INPUT {")],
        expect="T1 (max-input-slack-IcoEntryDibSlack / input-exact-64m become FileTooLarge)",
    ),
    44: dict(
        what="R19 mut (8): rescan the entire remaining suffix on every slack-mode return",
        edits=[("dib.rs", "        let end = pos + and_bytes;\n        let exact = match h.mode {",
                "        let end = pos + and_bytes;\n        if h.mode == Mode::IcoEntryDibSlack {\n            let mut scan = end;\n            while scan < b.len()\n                invariant end <= scan <= b.len(),\n                decreases b.len() - scan,\n            {\n                let _byte = rd(b, scan);\n                scan += 1;\n                proof { *steps = Ghost(steps@ + 1); }\n            }\n        }\n        let exact = match h.mode {")],
        expect="T12 (only when the loop is charged; otherwise a source audit)",
    ),
}


def crate_dir(n):
    return os.path.join(DIR, "m%d" % n, "dib_verus")


def apply_edits(src_dir, edits):
    for fname, old, new in edits:
        path = os.path.join(src_dir, fname)
        text = open(path).read()
        if text.count(old) != 1:
            raise SystemExit(
                "anchor for %s matched %d times (expected 1); refusing to mutate: %r"
                % (fname, text.count(old), old[:60]))
        open(path, "w").write(text.replace(old, new, 1))


def cmd_apply(n, label=None):
    spec = MUTATIONS[n]
    dirname = label if label is not None else "m%d" % n
    dest = os.path.join(DIR, dirname)
    if os.path.exists(dest):
        shutil.rmtree(dest)
    os.makedirs(os.path.join(dest, "dib_verus"))
    for name in ("Cargo.toml", "Cargo.lock", "src", "tests"):
        src = os.path.join(BASE, name)
        dst = os.path.join(dest, "dib_verus", name)
        if os.path.isdir(src):
            shutil.copytree(src, dst)
        elif os.path.exists(src):
            shutil.copy2(src, dst)
    apply_edits(os.path.join(dest, "dib_verus", "src"), spec["edits"])
    print("mutation %d applied to mutation/%s/dib_verus" % (n, dirname))
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
        cmd_apply(int(sys.argv[2]), sys.argv[3] if len(sys.argv) > 3 else None)
        return 0
    print(__doc__)
    return 2


if __name__ == "__main__":
    sys.exit(main())
