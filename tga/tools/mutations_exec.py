#!/usr/bin/env python3
"""Executable-side mutations for the 13 model-only entries of THEOREMS.md's
proof-sensitivity protocol (Findings 2 and 7 of the Sol review).

M4e: the RLE anchors (6, 7, 15) follow the M4e split of `decode_rle` into
`rle_packet` / `rle_repeat_packet` / `rle_literal_packet` (4-space indent, the
`produced: &mut usize` counter, the `(Result, Ghost<..>)` return of
`rle_packet`); the mutations themselves are unchanged.

Each mutation edits only `src/tga.rs` in a disposable copy under
`mutation/exec-mN/tga_verus`, rebuilt from the pristine `mutation/base2/`
(a fresh copy of the current live `src/`). The frozen model, `model_vectors.rs`
and the vectors are untouched, so a failure is a *verifier* diagnostic on the
executable obligation the manifest names (T7/T9/T3/T1/T8 ...), not a corrupted
model.

Usage:
    tools/mutations_exec.py apply N
    tools/mutations_exec.py list
"""

import os
import shutil
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BASE = os.path.join(ROOT, "mutation", "base2")
DIR = os.path.join(ROOT, "mutation")

MUTATIONS = {
    # 1. |b|>MAX_INPUT -> >= in the executable parser.
    1: dict(
        what="the executable `parse_header` input cap changed from `>` to `>=`",
        edits=[("tga.rs", "    if b.len() > MAX_INPUT {", "    if b.len() >= MAX_INPUT {")],
        expect="decode / T7-T9 (L2 boundary reject)",
    ),
    # 2. image-type and map-flag checks swapped.
    2: dict(
        what="the executable `parse_header` checks the map flag before the image type",
        edits=[(
            "tga.rs",
            "    let image_type = b[2];\n    // R1: exactly the six supported types.\n    proof { type_ok_eq(image_type); }\n    let supported_type = image_type == 1 || image_type == 2 || image_type == 3 || image_type == 9\n        || image_type == 10 || image_type == 11;\n    if !supported_type {\n        proof {\n            reveal(crate::model::mh);\n        }\n        return Err(Error::UnsupportedImageType);\n    }\n    proof {\n        reveal(crate::model::mh);\n    }\n\n    let color_map_flag = b[1];\n    // R2: the map flag is 0 or 1.\n    if color_map_flag > 1 {\n        proof {\n            reveal(crate::model::mh);\n        }\n        return Err(Error::InvalidColorMapFlag);\n    }",
            "    let color_map_flag = b[1];\n    // R2: the map flag is 0 or 1.\n    if color_map_flag > 1 {\n        proof {\n            reveal(crate::model::mh);\n        }\n        return Err(Error::InvalidColorMapFlag);\n    }\n    proof {\n        reveal(crate::model::mh);\n    }\n\n    let image_type = b[2];\n    // R1: exactly the six supported types.\n    proof { type_ok_eq(image_type); }\n    let supported_type = image_type == 1 || image_type == 2 || image_type == 3 || image_type == 9\n        || image_type == 10 || image_type == 11;\n    if !supported_type {\n        proof {\n            reveal(crate::model::mh);\n        }\n        return Err(Error::UnsupportedImageType);\n    }",
        )],
        expect="T5/T9 (type-before-flag ordering)",
    ),
    # 3. dimension cap removed in the executable.
    3: dict(
        what="the executable dimension cap raised from MAX_DIMENSION to 65535",
        edits=[("tga.rs", "    if width < 1 || width > MAX_DIMENSION || height < 1 || height > MAX_DIMENSION {",
                "    if width < 1 || width > 65535 || height < 1 || height > 65535 {")],
        expect="T5/T8 (dimension cap)",
    ),
    # 4. map first ignored in the executable.
    4: dict(
        what="the executable `convert_index` ignores the map `first` (off = index*stride)",
        edits=[("tga.rs", "    let j = (index - h.map_first) as usize;", "    let j = index as usize;")],
        expect="T7 (palette-first-65535)",
    ),
    # 5. True-colour 16-bit attr 0: wrongly use bit 15 as alpha.
    5: dict(
        what="true-colour 16-bit attr 0 incorrectly uses bit 15 as alpha",
        edits=[(
            "tga.rs",
            "                conv555(v, 255)\n            } else if h.pixel_depth == 24 {",
            "                conv555(v, if h.pixel_depth == 16 && attr == 0 && v & 0x8000 == 0 { 0 } else { 255 })\n            } else if h.pixel_depth == 24 {",
        )],
        expect="T7 (raw-16-opaque-bit-off)",
    ),
    # 6. RLE count from (control&0x7f) instead of +1.
    6: dict(
        what="the executable RLE count changed to `(control & 0x7f)`",
        edits=[("tga.rs", "    let count = (control & 0x7f) as usize + 1;", "    let count = (control & 0x7f) as usize;")],
        expect="T3/T6/T7/T10 (rle-true-repeat)",
    ),
    # 7. Availability has priority over overrun; overrun rejection remains.
    7: dict(
        what="packet availability precedes the still-present overrun rejection",
        edits=[(
            "tga.rs",
            "    // Overrun: checked before reading any packet sample.\n    if count > n - *produced {",
            "    // Mutant: test packet availability before packet overrun.\n    let sb = h.sample_bytes();\n    let packet_missing = if control & 0x80 != 0 {\n        bytes.len() - *cursor < sb\n    } else {\n        (bytes.len() - *cursor) / sb < count\n    };\n    if packet_missing {\n        return (Err(Error::RleTruncated), Ghost(acc0));\n    }\n    // Overrun: still rejected if the encoded packet is available.\n    if count > n - *produced {",
        )],
        expect="T6/T9 (rle-repeat-overrun-no-sample)",
    ),
    # 9. vertical flip reversed in the executable.
    9: dict(
        what="the executable vertical flip reversed in `output_offset`",
        edits=[("tga.rs", "    let by = if flip_y { y } else { height as usize - 1 - y };",
                "    let by = if flip_y { height as usize - 1 - y } else { y };")],
        expect="T7/T8 (origin flips)",
    ),
    # 10. suffix accepted without a footer.
    10: dict(
        what="the executable accepts any nonempty suffix without a footer",
        edits=[("tga.rs", "                let ok = signature_tail(bytes);",
                "                let ok = true;")],
        expect="T7/T9 (trailing bytes)",
    ),
    # 12. bit replication replaced by rounded scaling in the executable.
    12: dict(
        what="the executable `expand5` uses rounded scaling instead of bit replication",
        edits=[("tga.rs", "    let v = (c << 3) | (c >> 2);", "    let v = ((c as u32 * 255 + 15) / 31) as u16;")],
        expect="conversion / T7 (raw-15-mixed)",
    ),
    # 18. 32-bit true-colour A byte used at attr 0 in the executable.
    18: dict(
        what="the executable uses the stored A byte for 32-bit true-colour at attr 0",
        edits=[("tga.rs", "                let alpha = if attr == 8 { s[3] } else { 255 };",
                "                let alpha = s[3];")],
        expect="conversion / T7 (raw-32-alpha-zero-ignores-byte)",
    ),
    # 19. greyscale-16 second byte used at attr 0 in the executable.
    19: dict(
        what="the executable uses the greyscale-16 second byte at attr 0",
        edits=[("tga.rs", "                let alpha = if attr == 8 { s[1] } else { 255 };",
                "                let alpha = s[1];")],
        expect="conversion / T7 (gray16-alpha-zero)",
    ),
    # 20. True-colour 16-bit attr 1: wrongly treat bit 15 clear as transparent.
    20: dict(
        what="true-colour 16-bit attr 1 incorrectly uses bit 15 as alpha",
        edits=[(
            "tga.rs",
            "                conv555(v, 255)\n            } else if h.pixel_depth == 24 {",
            "                conv555(v, if h.pixel_depth == 16 && attr == 1 && v & 0x8000 == 0 { 0 } else { 255 })\n            } else if h.pixel_depth == 24 {",
        )],
        expect="conversion / T7 (raw-16-alpha-zero)",
    ),
    # 15. whole-literal availability guard removed: an unchecked read.
    15: dict(
        what="the executable removes the whole-literal availability guard (unchecked read)",
        edits=[("tga.rs", "    if bytes.len() - *cursor < total {",
                "    if false && bytes.len() - *cursor < total {")],
        expect="T1/T3 (an out-of-bounds read obligation at the packet slice)",
    ),
    # 17. palette range guard removed: an unchecked index into the map.
    17: dict(
        what="the executable removes the palette range guard before indexing the map",
        edits=[(
            "tga.rs",
            "        let in_range = index_in_range(h, idx);\n        if !in_range {\n            return Err(Error::ColorIndexOutOfRange);\n        }\n        proof {\n            reveal(header_inv);\n            assert((*h).map_first as int <= idx as int);\n            assert((idx as int) < ((*h).map_first as int + (*h).map_length as int));\n            assert((*h).color_map_flag == 1u8);\n            reveal(crate::model::map_entry_depth_ok);\n        }\n        Ok(convert_index(h, map, idx))",
            "        proof {\n            reveal(header_inv);\n            reveal(crate::model::map_entry_depth_ok);\n        }\n        Ok(convert_index(h, map, idx))",
        )],
        expect="T1/T4 (an unchecked map index / subtraction underflow)",
    ),
}


def crate_dir(n):
    return os.path.join(DIR, "exec-m%d" % n, "tga_verus")


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
    dest = os.path.join(DIR, "exec-m%d" % n)
    if os.path.exists(dest):
        shutil.rmtree(dest)
    os.makedirs(os.path.join(dest, "tga_verus"))
    for name in ("Cargo.toml", "Cargo.lock", "src", "tests", ".gitignore"):
        src = os.path.join(BASE, "tga_verus", name)
        if os.path.isdir(src):
            shutil.copytree(src, os.path.join(dest, "tga_verus", name))
        elif os.path.exists(src):
            shutil.copy2(src, os.path.join(dest, "tga_verus", name))
    apply_edits(os.path.join(dest, "tga_verus", "src"), spec["edits"])
    print("executable mutation %d applied to mutation/exec-m%d/tga_verus" % (n, n))
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
