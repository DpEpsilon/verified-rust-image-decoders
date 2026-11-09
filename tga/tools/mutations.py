#!/usr/bin/env python3
"""Apply / revert one of the twenty mutations of THEOREMS.md's protocol.

Each mutation edits an exact text region of `src/model.rs` or `src/tga.rs` in a
disposable copy of the crate under `mutation/mN/`.  The edits are textual and
assert on the presence of their anchor, so a stale anchor fails loudly instead
of silently mutating nothing.  Everything lives under `mutation/`, which is
inside this project, so `./verify.sh` counts every run.

Where THEOREMS.md names a fixture and a theorem that must both fail, the edit is
made in `src/model.rs` (the specification authority): changing the model alone
breaks the fixture-agreement obligation in `model_vectors.rs` *and* the
executable's `decode = MP` postcondition, because the executable is still the
original decoder.  Where THEOREMS.md names a theorem that can only fail on the
executable side (an unguarded read, the loop's own produced count, the `N`
arithmetic), the edit is made in `src/tga.rs`.

Usage:
    tools/mutations.py apply N     # copy mutation/base to mutation/mN, mutate
    tools/mutations.py list        # the twenty mutations and what each targets
"""

import os
import shutil
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BASE = os.path.join(ROOT, "mutation", "base")
DIR = os.path.join(ROOT, "mutation")

MUTATIONS = {
    1: dict(
        what="R1's input cap changed from |b|>67,108,864 to |b|>=67,108,864 in the model (mh)",
        edits=[(
            "model.rs",
            "pub open spec fn mh(b: Seq<u8>) -> Result<MHModel, Error> {\n    if b.len() > max_input() {",
            "pub open spec fn mh(b: Seq<u8>) -> Result<MHModel, Error> {\n    if b.len() >= max_input() {",
        )],
        expect="lemma_l2_input_at_64m / L2 and T7/T9",
    ),
    2: dict(
        what="the image-type and map-flag checks swapped in the model (mh)",
        edits=[(
            "model.rs",
            "    } else if !type_ok(b[2]) {\n        Err(Error::UnsupportedImageType)\n    } else if b[1] > 1u8 {\n        Err(Error::InvalidColorMapFlag)",
            "    } else if b[1] > 1u8 {\n        Err(Error::InvalidColorMapFlag)\n    } else if !type_ok(b[2]) {\n        Err(Error::UnsupportedImageType)",
        )],
        expect="type-before-flag-and-dimensions and T5/T9",
    ),
    3: dict(
        what="the R2 dimension cap removed from the model (max_dimension -> 65535)",
        edits=[(
            "model.rs",
            "pub open spec fn max_dimension() -> int {\n    8192\n}",
            "pub open spec fn max_dimension() -> int {\n    65535\n}",
        )],
        expect="dimensions-8193-1 and T5/T8",
    ),
    4: dict(
        what="the colour-map first index ignored in the model's conv_map (off = index*stride)",
        edits=[(
            "model.rs",
            "    let off = (index - h.map_first) * stride;",
            "    let off = index * stride;",
        )],
        expect="palette-first-65535 and T7",
    ),
    5: dict(
        what="bit 15 used as alpha even when the 16-bit true-colour attribute count is zero",
        edits=[(
            "model.rs",
            "    if h.kind == 1 && (h.depth == 15 || h.depth == 16) {\n"
            "        // True-colour 15/16: five-bit channels, alpha 255 for counts 0 and 1.\n"
            "        bytes4_15(le16s(s, 0), 255)",
            "    if h.kind == 1 && (h.depth == 15 || h.depth == 16) {\n"
            "        // True-colour 15/16: five-bit channels, alpha 255 for counts 0 and 1.\n"
            "        bytes4_15(le16s(s, 0), if h.depth == 16 && h.attr == 0 && bit15(le16s(s, 0)) {\n"
            "            255u8\n"
            "        } else if h.depth == 16 && h.attr == 0 {\n"
            "            0u8\n"
            "        } else {\n"
            "            255u8\n"
            "        })",
        )],
        expect="raw-16-opaque-bit-off and T7 (and the conversion lemma)",
    ),
    6: dict(
        what="R11's repeat count changed from (control&0x7f)+1 to control&0x7f in the model",
        edits=[(
            "model.rs",
            "pub open spec fn rle_count(control: int) -> int {\n"
            "    if control >= 128 {\n"
            "        control - 127\n"
            "    } else {\n"
            "        control + 1\n"
            "    }\n"
            "}",
            "pub open spec fn rle_count(control: int) -> int {\n"
            "    if control >= 128 {\n"
            "        control - 128\n"
            "    } else {\n"
            "        control - 0\n"
            "    }\n"
            "}",
        )],
        expect="rle-true-repeat and T3/T6/T7/T10",
    ),
    7: dict(
        what="the model's packet availability and overrun checks swapped (mr_at, mr_cur and mr_pix)",
        edits=[
            (
                "model.rs",
                "        let control = s[cur] as int;\n"
                "        let count = rle_count(control);\n"
                "        if count > h.n - prod {\n"
                "            Err((Error::RleOverrun, cur + 1, prod, out))\n"
                "        } else {\n"
                "            let start = cur + 1;\n"
                "            if rle_repeat(control) {\n"
                "                let pe = start + h.sample_bytes;\n"
                "                if s.len() < pe {\n"
                "                    Err((Error::RleTruncated, start, prod, out))",
                "        let control = s[cur] as int;\n"
                "        let count = rle_count(control);\n"
                "        if count > h.n - prod {\n"
                "            let start = cur + 1;\n"
                "            if rle_repeat(control) {\n"
                "                let pe = start + h.sample_bytes;\n"
                "                if s.len() < pe {\n"
                "                    Err((Error::RleTruncated, start, prod, out))\n"
                "                } else if cur + 1 + count * h.sample_bytes > s.len() {\n"
                "                    Err((Error::RleTruncated, start, prod, out))\n"
                "                } else {\n"
                "                    Err((Error::RleOverrun, cur + 1, prod, out))\n"
                "                }\n"
                "            } else {\n"
                "                let total = count * h.sample_bytes;\n"
                "                let pe = start + total;\n"
                "                if s.len() < pe {\n"
                "                    Err((Error::RleTruncated, start, prod, out))\n"
                "                } else {\n"
                "                    Err((Error::RleOverrun, cur + 1, prod, out))\n"
                "                }\n"
                "            }\n"
                "        } else {\n"
                "            let start = cur + 1;\n"
                "            if rle_repeat(control) {\n"
                "                let pe = start + h.sample_bytes;\n"
                "                if s.len() < pe {\n"
                "                    Err((Error::RleTruncated, start, prod, out))",
            ),
            (
                "model.rs",
                "        let control = s[cur] as int;\n"
                "        let count = rle_count(control);\n"
                "        if count > h.n - prod {\n"
                "            Err(Error::RleOverrun)\n"
                "        } else {\n"
                "            let start = cur + 1;\n"
                "            if rle_repeat(control) {\n"
                "                let pe = start + h.sample_bytes;\n"
                "                if s.len() < pe {\n"
                "                    Err(Error::RleTruncated)",
                "        let control = s[cur] as int;\n"
                "        let count = rle_count(control);\n"
                "        if count > h.n - prod {\n"
                "            let start = cur + 1;\n"
                "            if rle_repeat(control) {\n"
                "                let pe = start + h.sample_bytes;\n"
                "                if s.len() < pe {\n"
                "                    Err(Error::RleTruncated)\n"
                "                } else if cur + 1 + count * h.sample_bytes > s.len() {\n"
                "                    Err(Error::RleTruncated)\n"
                "                } else {\n"
                "                    Err(Error::RleOverrun)\n"
                "                }\n"
                "            } else {\n"
                "                let total = count * h.sample_bytes;\n"
                "                let pe = start + total;\n"
                "                if s.len() < pe {\n"
                "                    Err(Error::RleTruncated)\n"
                "                } else {\n"
                "                    Err(Error::RleOverrun)\n"
                "                }\n"
                "            }\n"
                "        } else {\n"
                "            let start = cur + 1;\n"
                "            if rle_repeat(control) {\n"
                "                let pe = start + h.sample_bytes;\n"
                "                if s.len() < pe {\n"
                "                    Err(Error::RleTruncated)",
            ),
        ],
        expect="rle-repeat-overrun-no-sample and T6/T9",
    ),
    8: dict(
        what="the RLE produced count reset at each row in the executable's decode_rle",
        edits=[(
            "tga.rs",
            "                    },\n"
            "                };\n"
            "                let off = output_offset(width, height, fx, fy, produced);",
            "                    },\n"
            "                };\n"
            "                produced = 0;\n"
            "                let off = output_offset(width, height, fx, fy, produced);",
        )],
        expect="rle-crosses-row and T6/T7",
    ),
    9: dict(
        what="the vertical flip reversed in the model (oyr)",
        edits=[(
            "model.rs",
            "pub open spec fn oyr(h: int, fy: bool, y: int) -> int {\n"
            "    if fy {\n"
            "        y\n"
            "    } else {\n"
            "        h - 1 - y\n"
            "    }\n"
            "}",
            "pub open spec fn oyr(h: int, fy: bool, y: int) -> int {\n"
            "    if fy {\n"
            "        h - 1 - y\n"
            "    } else {\n"
            "        y\n"
            "    }\n"
            "}",
        )],
        expect="origin-bottom-left and T7/T8",
    ),
    10: dict(
        what="the R12 suffix rule weakened in the model (any nonempty suffix accepted)",
        edits=[(
            "model.rs",
            "pub open spec fn mp_suffix_ok(b: Seq<u8>, cursor: int) -> bool {\n"
            "    let suffix = b.subrange(cursor, b.len() as int);\n"
            "    suffix.len() == 0 || footer(suffix)\n"
            "}",
            "pub open spec fn mp_suffix_ok(b: Seq<u8>, cursor: int) -> bool {\n"
            "    let suffix = b.subrange(cursor, b.len() as int);\n"
            "    suffix.len() == 0 || suffix.len() >= 1\n"
            "}",
        )],
        expect="trailing-one-byte and T7/T9",
    ),
    11: dict(
        what="a footer offset dereferenced instead of treated as opaque in the executable",
        edits=[(
            "tga.rs",
            "                if !ok {",
            "                { let ext_off = le16(bytes, bytes.len() - 26) as usize; let probe = bytes[ext_off]; if probe == 1u8 { return Err(Error::TrailingBytes); } }\n"
            "                if !ok {",
        )],
        expect="footer-opaque-offsets and T1 (an unguarded read)/T7",
    ),
    12: dict(
        what="R8's bit replication (c<<3)|(c>>2) replaced by rounded scaling (c*255+15)/31 in the model",
        edits=[(
            "model.rs",
            "pub open spec fn rep5(c: int) -> int {\n    c * 8 + c / 4\n}",
            "pub open spec fn rep5(c: int) -> int {\n    (c * 255 + 15) / 31\n}",
        )],
        expect="raw-15-mixed-0c63, raw-16-mixed-0444, palette-15-mixed-0c63 and the conversion lemma/T7",
    ),
    13: dict(
        what="the map started at byte 18 rather than 18+ID_length in the executable's decode",
        edits=[(
            "tga.rs",
            "    let map_start = cursor;",
            "    let map_start: usize = 18;",
        )],
        expect="indexed-map-after-id-3 / indexed-map-after-id-255 and T4/T7",
    ),
    14: dict(
        what="a fixed three-byte entry stride in the executable's convert_index",
        edits=[(
            "tga.rs",
            "    let stride = h.map_bytes_per_entry();\n"
            "    proof {\n"
            "        reveal(header_inv);\n"
            "        reveal(crate::model::stride_of);",
            "    let stride = 3usize;\n"
            "    proof {\n"
            "        reveal(header_inv);\n"
            "        reveal(crate::model::stride_of);",
        )],
        expect="palette-16-index-6-type-9 / palette-32-index-6-type-9 and T4/T7",
    ),
    15: dict(
        what="the whole-literal availability guard removed before reading a literal packet",
        edits=[(
            "tga.rs",
            "            let total = count * sb;\n"
            "            if bytes.len() - *cursor < total {",
            "            let total = count * sb;\n"
            "            if false && bytes.len() - *cursor < total {",
        )],
        expect="rle-literal-missing-sample and T3/T1 (an out-of-bounds read)",
    ),
    16: dict(
        what="N = width*height computed without widening in the executable's decode",
        edits=[
            (
                "tga.rs",
                "    let n = width as usize * height as usize;",
                "    let n = ((width as u16) * (height as u16)) as usize;",
            ),
        ],
        expect="T1/T2/T8 (valid headers with N>65,535 overflow u16)",
    ),
    17: dict(
        what="the palette range guard removed before indexing in the executable (index_in_range)",
        edits=[(
            "tga.rs",
            "    index >= h.map_first && index < h.map_first + h.map_length",
            "    index == index",
        )],
        expect="index-below-first and T4/T1 (an out-of-bounds read)",
    ),
    18: dict(
        what="the stored A byte used even when the 32-bit true-colour attribute count is zero",
        edits=[(
            "model.rs",
            "    } else if h.kind == 1 && h.depth == 32 {\n"
            "        seq![s[2], s[1], s[0], (if h.attr == 8 { s[3] } else { 255u8 })]",
            "    } else if h.kind == 1 && h.depth == 32 {\n"
            "        seq![s[2], s[1], s[0], s[3]]",
        )],
        expect="raw-32-alpha-zero-ignores-byte, the conversion lemma and T7",
    ),
    19: dict(
        what="the stored second byte of 16-bit greyscale used when the attribute count is zero",
        edits=[(
            "model.rs",
            "    } else if h.kind == 2 && h.depth == 16 {\n"
            "        seq![s[0], s[0], s[0], (if h.attr == 8 { s[1] } else { 255u8 })]",
            "    } else if h.kind == 2 && h.depth == 16 {\n"
            "        seq![s[0], s[0], s[0], s[1]]",
        )],
        expect="gray16-alpha-zero, the conversion lemma and T7",
    ),
    20: dict(
        what="bit 15 clear treated as transparent for 16-bit true-colour attribute count one",
        edits=[(
            "model.rs",
            "    if h.kind == 1 && (h.depth == 15 || h.depth == 16) {\n"
            "        // True-colour 15/16: five-bit channels, alpha 255 for counts 0 and 1.\n"
            "        bytes4_15(le16s(s, 0), 255)",
            "    if h.kind == 1 && (h.depth == 15 || h.depth == 16) {\n"
            "        // True-colour 15/16: five-bit channels, alpha 255 for counts 0 and 1.\n"
            "        bytes4_15(le16s(s, 0), if h.attr == 1 && !bit15(le16s(s, 0)) {\n"
            "            0u8\n"
            "        } else {\n"
            "            255u8\n"
            "        })",
        )],
        expect="raw-16-alpha-zero, the conversion lemma and T7",
    ),
}


def crate_dir(n):
    return os.path.join(DIR, "m%d" % n, "tga_verus")


def apply_edits(src_dir, edits):
    for fname, old, new in edits:
        path = os.path.join(src_dir, fname)
        text = open(path).read()
        if text.count(old) != 1:
            raise SystemExit(
                "anchor for %s matched %d times (expected 1); refusing to mutate"
                % (fname, text.count(old)))
        open(path, "w").write(text.replace(old, new, 1))


def cmd_apply(n):
    spec = MUTATIONS[n]
    dest = os.path.join(DIR, "m%d" % n)
    if os.path.exists(dest):
        shutil.rmtree(dest)
    os.makedirs(os.path.join(dest, "tga_verus"))
    for name in ("Cargo.toml", "Cargo.lock", "src", "tests", ".gitignore"):
        src = os.path.join(BASE, "tga_verus", name)
        if os.path.isdir(src):
            shutil.copytree(src, os.path.join(dest, "tga_verus", name))
        else:
            shutil.copy2(src, os.path.join(dest, "tga_verus", name))
    apply_edits(os.path.join(dest, "tga_verus", "src"), spec["edits"])
    print("mutation %d applied to mutation/m%d/tga_verus" % (n, n))
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
        cmd_apply(int(sys.argv[2]))
        return 0
    print(__doc__)
    return 2


if __name__ == "__main__":
    sys.exit(main())
