#!/usr/bin/env python3
"""Implementation-side mutations for THEOREMS.md's "Planned executable-only
mutations" table (M4b).

Each mutation edits ONLY `src/ico.rs` in a disposable copy under
`mutation/mN/ico_verus`, rebuilt from the pristine `mutation/base/ico_verus`
(a fresh copy of the current live crate's `src/`, `tests`, `Cargo.toml`,
`Cargo.lock`).  The frozen model `src/model.rs`, `src/model_vectors.rs`,
`src/witnesses.rs`, `src/lib.rs`, `spec-frozen/` and the vectors are untouched,
so a failure is a *verifier* diagnostic on the executable obligation the
manifest names (T1..T11), not a corrupted model.

Faithfulness: each mutation changes the executable code path the manifest names,
on the fixture/theorem the manifest names (`expect`).  `apply_edits` refuses to
mutate unless every anchor matches exactly once.

Usage:
    tools/mutations.py apply N
    tools/mutations.py list
"""

import os
import shutil
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BASE = os.path.join(ROOT, "mutation", "base", "ico_verus")
DIR = os.path.join(ROOT, "mutation")

MUTATIONS = {
    # ---- M1: input cap `>` -> `>=` (directory_guard) --------------------
    1: dict(
        what="directory_guard: the R2 input cap test `len > MAX_INPUT` becomes `len >= MAX_INPUT`",
        edits=[("ico.rs", "    if len > MAX_INPUT {", "    if len >= MAX_INPUT {")],
        expect="T2/T5 (L1 boundary: a slice of exactly MAX_INPUT bytes becomes FileTooLarge)",
    ),
    # ---- M2: read reserved before the length-6 guard --------------------
    2: dict(
        what="directory_guard: read the reserved word before the six-byte length guard",
        edits=[("ico.rs",
                "    // R3: all six header bytes must exist before any of the six-byte reads.\n"
                "    if len < DIRECTORY_HEADER_LEN {\n"
                "        return Err(Error::TruncatedDirectory);\n"
                "    }\n"
                "    let reserved = rd16(bytes, 0);",
                "    let reserved = rd16(bytes, 0);\n"
                "    // R3: all six header bytes must exist before any of the six-byte reads.\n"
                "    if len < DIRECTORY_HEADER_LEN {\n"
                "        return Err(Error::TruncatedDirectory);\n"
                "    }")],
        expect="T1/T2 (short-dir-0: the guarded read's precondition is no longer provable)",
    ),
    # ---- M3: swap the little-endian byte order (rd16) -------------------
    3: dict(
        what="rd16: build the word as `hi | (lo << 8)` (big-endian) instead of `lo | (hi << 8)`",
        edits=[("ico.rs", "    lo | (hi << 8)", "    hi | (lo << 8)")],
        expect="T2 (dib-basic-parse: the type/reserved/count/size/offset words all misread)",
    ),
    # ---- M4: accept count zero ------------------------------------------
    4: dict(
        what="directory_guard: the count-zero guard `count_word == 0` becomes `false && count_word == 0`",
        edits=[("ico.rs", "    if count_word == 0 {", "    if false && count_word == 0 {")],
        expect="T2 (empty-directory-1: an empty directory is accepted)",
    ),
    # ---- M5: check the table before the count cap -----------------------
    5: dict(
        what="directory_guard: move the whole-table guard before the count cap",
        edits=[("ico.rs",
                "    let count = count_word as usize;\n"
                "    if count > MAX_ENTRIES {\n"
                "        proof {\n"
                "            assert(crate::model::le16(bytes@, 4) > crate::model::max_entries());\n"
                "        }\n"
                "        return Err(Error::TooManyEntries);\n"
                "    }\n"
                "    proof {\n"
                "        assert(crate::model::le16(bytes@, 4) <= crate::model::max_entries());\n"
                "        assert(count as int == crate::model::le16(bytes@, 4));\n"
                "        crate::model::lemma_dir_end_bound(crate::model::le16(bytes@, 4));\n"
                "        assert(6 + 16 * (count as int) <= 16390);\n"
                "        assert(6 + 16 * (count as int) == DIRECTORY_HEADER_LEN as int + ENTRY_RECORD_LEN as int\n"
                "            * (count_word as int));\n"
                "    }\n"
                "    // R7/T7: `6 + 16*count <= 6 + 16*1,024 = 16,390`, computed in `u64` so the\n"
                "    // multiplication cannot wrap on any target.\n"
                "    let dir_end = DIRECTORY_HEADER_LEN as u64 + ENTRY_RECORD_LEN as u64 * count_word as u64;\n"
                "    if dir_end > bytes.len() as u64 {",
                "    let count = count_word as usize;\n"
                "    // R7/T7: `6 + 16*count <= 6 + 16*1,024 = 16,390`, computed in `u64` so the\n"
                "    // multiplication cannot wrap on any target.\n"
                "    let dir_end = DIRECTORY_HEADER_LEN as u64 + ENTRY_RECORD_LEN as u64 * count_word as u64;\n"
                "    if dir_end > bytes.len() as u64 {")],
        expect="T2/T6 (count-cap-1025: TooManyEntries vs TruncatedEntries ordering)",
    ),
    # ---- M6: 15-byte record stride ---------------------------------------
    6: dict(
        what="reconstruct: the R4 record cursor strides 15 bytes (`6 + 15*i`) instead of 16",
        edits=[("ico.rs", "    let p = DIRECTORY_HEADER_LEN + ENTRY_RECORD_LEN * i;",
                "    let p = DIRECTORY_HEADER_LEN + 15 * i;")],
        expect="T2 (two-dib-png: record 1 is read from the wrong offset)",
    ),
    # ---- M7: keep zero nominal dimensions as zero ------------------------
    7: dict(
        what="reconstruct: the R5 zero-means-256 rule is dropped (width = w0, height = h0)",
        edits=[("ico.rs",
                "    let width = if w0 == 0 {\n"
                "        256u16\n"
                "    } else {\n"
                "        w0 as u16\n"
                "    };",
                "    let width = w0 as u16;"),
               ("ico.rs",
                "    let height = if h0 == 0 {\n"
                "        256u16\n"
                "    } else {\n"
                "        h0 as u16\n"
                "    };",
                "    let height = h0 as u16;")],
        expect="T2/T3 (dimensions-0-0-type1: 256 becomes 0)",
    ),
    # ---- M8: interpret CUR words as ICO planes/depth ---------------------
    8: dict(
        what="reconstruct: the ICO/CUR word roles are swapped (a CUR record yields planes/bit_count)",
        edits=[("ico.rs",
                "    let (planes, bit_count, hotspot_x, hotspot_y) = if typ == TYPE_ICO {\n"
                "        (Some(lo), Some(hi), None, None)\n"
                "    } else {\n"
                "        (None, None, Some(lo), Some(hi))\n"
                "    };",
                "    let (planes, bit_count, hotspot_x, hotspot_y) = if typ == TYPE_ICO {\n"
                "        (None, None, Some(lo), Some(hi))\n"
                "    } else {\n"
                "        (Some(lo), Some(hi), None, None)\n"
                "    };")],
        expect="T2 (words-65535-0-type2: CUR hotspots become ICO planes/bit_count)",
    ),
    # ---- M9: abort parse on an invalid record ---------------------------
    9: dict(
        what="parse_tracked: abort with an error on the first invalid record instead of recording its status",
        edits=[("ico.rs",
                "        i += 1;\n    }\n    proof {",
                "        match e.status {\n"
                "            EntryStatus::Invalid(_) => {\n"
                "                return Err(Error::InvalidEntryRecord);\n"
                "            },\n"
                "            _ => {},\n"
                "        }\n"
                "        i += 1;\n    }\n    proof {")],
        expect="T2/T5 (valid-then-bad-reserved, bad-then-valid-oob: an invalid record aborts the list)",
    ),
    # ---- M10: check size zero after the range bounds ---------------------
    10: dict(
        what="record_status: the zero-size check is made conditional on not overlapping, and the overlap check on non-empty",
        edits=[("ico.rs", "    if len_word == 0 {", "    if len_word == 0 && offset_word >= dir_end {"),
               ("ico.rs", "    if offset_word < dir_end {", "    if offset_word < dir_end && len_word > 0 {")],
        expect="T2/T6 (zero-size-before-overlap: a zero-size overlapping record becomes a valid empty slice)",
    ),
    # ---- M11: compare offset with six, not the directory end -------------
    11: dict(
        what="record_status: the overlap test compares the offset against the six-byte header instead of dir_end",
        edits=[("ico.rs", "    if offset_word < dir_end {", "    if offset_word < DIRECTORY_HEADER_LEN as u32 {")],
        expect="T2/T3 (directory-overlap-21: an offset of 21 is no longer inside the directory)",
    ),
    # ---- M12: wrapping u32 add instead of subtraction --------------------
    12: dict(
        what="record_status: the range guard forms `(offset + size)` in u32 (truncating) instead of u64 subtraction",
        edits=[("ico.rs",
                "    let off = offset_word as u64;\n"
                "    let len = bytes.len() as u64;\n"
                "    if off > len || len_word as u64 > len - off {",
                "    let off = offset_word as u64;\n"
                "    let len = bytes.len() as u64;\n"
                "    let wide = ((offset_word as u64) + (len_word as u64)) as u32;\n"
                "    if wide as u64 > len {")],
        expect="T1/T2/T7 (wide-sum-wrap, size-4294967295: offset+size wraps in u32)",
    ),
    # ---- M13: classify a seven-byte prefix as PNG ------------------------
    13: dict(
        what="has_png_signature: the length guard `end - start < 8` becomes `< 7`",
        edits=[("ico.rs", "    if end - start < 8 {", "    if end - start < 7 {")],
        expect="T4 (signature-prefix-7: a seven-byte prefix is classified Png)",
    ),
    # ---- M14: require bytes after the eight-byte signature ---------------
    14: dict(
        what="has_png_signature: the length guard `end - start < 8` becomes `< 12` (an IHDR-bearing payload is required)",
        edits=[("ico.rs", "    if end - start < 8 {", "    if end - start < 12 {")],
        expect="T4/T5 (signature-prefix-8, png-8: an eight-byte signature-only payload becomes Dib)",
    ),
    # ---- M15: reject identical/nested/straddling ranges ------------------
    15: dict(
        what="parse_tracked: reject any two valid records whose payload ranges overlap",
        edits=[("ico.rs",
                "    proof {\n"
                "        assert(entries@ == Seq::new(hr.count as nat, |j: int| crate::model::me(bytes@, hr, j)));\n",
                "    // MUTATION M15: reject identical, nested or straddling bounded ranges.\n"
                "    let mut a = 0usize;\n"
                "    while a < entries.len()\n"
                "        invariant a <= entries.len(),\n"
                "        decreases entries.len() - a,\n"
                "    {\n"
                "        if a < entries.len() {\n"
                "            let pa = match entries[a].status {\n"
                "                EntryStatus::Valid { payload, .. } => payload,\n"
                "                _ => [0u32, 0u32],\n"
                "            };\n"
                "            let mut b2 = a + 1;\n"
                "            while b2 < entries.len()\n"
                "                invariant b2 <= entries.len(),\n"
                "                decreases entries.len() - b2,\n"
                "            {\n"
                "                match entries[b2].status {\n"
                "                    EntryStatus::Valid { payload: pb, .. } => {\n"
                "                        if pa[0] < pb[1] && pb[0] < pa[1] {\n"
                "                            return Err(Error::EntryOverlapsDirectory);\n"
                "                        }\n"
                "                    },\n"
                "                    _ => {},\n"
                "                }\n"
                "                b2 += 1;\n"
                "            }\n"
                "        }\n"
                "        a += 1;\n"
                "    }\n"
                "    proof {\n"
                "        assert(entries@ == Seq::new(hr.count as nat, |j: int| crate::model::me(bytes@, hr, j)));\n")],
        expect="T2 (two-identical-range, straddling-ranges: valid overlapping ranges are rejected)",
    ),
    # ---- M16: narrow the index to u8 before the index/count guard --------
    16: dict(
        what="decode_entry_impl: the index/count guard compares `index as u8` against `header.count as u8`",
        edits=[("ico.rs", "    if index >= header.count as u32 {",
                "    if index as u8 >= header.count as u8 {")],
        expect="T1/T5/T7 (index-narrow-256: the guard no longer bounds the u32 index)",
    ),
    # ---- M17: pass offset-to-EOF to DIB instead of the declared slice ----
    17: dict(
        what="decode_entry_impl: the DIB call gets `bytes[start..]` (offset to EOF) instead of `bytes[start..end]`",
        edits=[("ico.rs",
                "            match bytes.get(start..end) {\n"
                "                Some(slice) => {",
                "            match bytes.get(start..) {\n"
                "                Some(slice) => {")],
        expect="T5 (entry-size-truncates-dib: the trailing file bytes join the DIB slice)",
    ),
    # ---- M18: enforce directory/DIB dimension equality -------------------
    18: dict(
        what="decode_entry_impl: reject a DIB result whose dimensions differ from the directory record",
        edits=[("ico.rs",
                "                    match r0 {\n"
                "                        Ok(d) => Ok(d),\n"
                "                        Err(e) => Err(Error::Dib(e)),\n"
                "                    }",
                "                    match r0 {\n"
                "                        Ok(d) => {\n"
                "                            if d.width != selected.width as u32 || d.height\n"
                "                                != selected.height as u32 {\n"
                "                                Err(Error::InvalidEntryRecord)\n"
                "                            } else {\n"
                "                                Ok(d)\n"
                "                            }\n"
                "                        },\n"
                "                        Err(e) => Err(Error::Dib(e)),\n"
                "                    }")],
        expect="T5 (dib-mismatch-decode: a valid DIB whose dimensions differ from the record is rejected)",
    ),
    # ---- M19: decode a PNG signature as DIB -----------------------------
    19: dict(
        what="has_png_signature: always return false, so a PNG signature is never classified Png (it becomes Dib and is decoded)",
        edits=[("ico.rs", "    true\n}", "    false\n}")],
        expect="T5/T6 (two-dib-png-decode1: a PNG record is decoded as DIB instead of PngDelegation)",
    ),
    # ---- M20: rescan all previous records per record (charged) -----------
    20: dict(
        what="parse_tracked: rescan all previous records on each iteration, charging one step per scanned record",
        edits=[("ico.rs",
                "        let e = reconstruct(bytes, header.kind, header.dir_end, i, Ghost(hr), steps);",
                "        let mut k = 0usize;\n"
                "        while k < i\n"
                "            invariant k <= i, i <= header.count,\n"
                "            decreases i - k,\n"
                "        {\n"
                "            let _prev = entries[k];\n"
                "            proof { *steps = Ghost(steps@ + 1); }\n"
                "            k += 1;\n"
                "        }\n"
                "        let e = reconstruct(bytes, header.kind, header.dir_end, i, Ghost(hr), steps);")],
        expect="T9 (count-exact-1024: the charged step bound is no longer linear)",
    ),
    # ---- M21: allocate a second vector without ghost charge (AUDIT only) -
    21: dict(
        what="decode_entry_impl: allocate an extra uncharged scratch vector at entry",
        edits=[("ico.rs",
                "    // R8: the R2/R3 guards first, without allocating an entry list.\n"
                "    let header = match directory_guard(bytes) {",
                "    // MUTATION M21: an uncharged scratch allocation (AUDIT only).\n"
                "    let _scratch: Vec<u8> = vec![0u8; 4];\n"
                "    // R8: the R2/R3 guards first, without allocating an entry list.\n"
                "    let header = match directory_guard(bytes) {")],
        expect="T10 AUDIT only (dib-basic-parse: the ghost count cannot see this; tools/audit_alloc.sh FAILS)",
    ),
    # ---- M22: read eight signature bytes beyond a seven-byte payload -----
    22: dict(
        what="has_png_signature: drop the `end - start < 8` short-circuit, so the eight-byte comparison runs on short payloads",
        edits=[("ico.rs", "    if end - start < 8 {", "    if false {")],
        expect="T4 (png-prefix-followed-by-rest-7: the classifier reads past the declared range)",
    ),
    # ---- M23: compare the offset against only the current record end -----
    23: dict(
        what="reconstruct: pass the current record's end (`p + 16`) as the overlap boundary instead of the directory end",
        edits=[("ico.rs",
                "        let st = record_status(bytes, dir_end, offset, size, Ghost(hr), Ghost(i as int), steps);",
                "        let st = record_status(bytes, (p + 16) as u32, offset, size, Ghost(hr), Ghost(i as int), steps);")],
        expect="T2/T3 (table-boundary-record0-off22: only the current record bound is used)",
    ),
    # ---- M24: read size/offset as u16 ------------------------------------
    24: dict(
        what="reconstruct: read the size and offset fields as u16",
        edits=[("ico.rs", "    let size = rd32(bytes, p + 8);", "    let size = rd16(bytes, p + 8) as u32;"),
               ("ico.rs", "    let offset = rd32(bytes, p + 12);", "    let offset = rd16(bytes, p + 12) as u32;")],
        expect="T2/T7 (wide-size-10001, wide-offset-10016: the high 16 bits are lost)",
    ),
    # ---- M25: compare the index after narrowing to u16 -------------------
    25: dict(
        what="decode_entry_impl: compare the index against the count after narrowing both to u16",
        edits=[("ico.rs", "    if index >= header.count as u32 {",
                "    if index as u16 as u32 >= header.count as u32 {")],
        expect="T5/T7 (index-narrow-65536: a narrowed index defeats the IndexOutOfRange guard)",
    ),
    # ---- M26: index the record as `i & 255` ------------------------------
    26: dict(
        what="reconstruct: the record cursor resets after 256 records (`6 + 16*(i & 255)`)",
        edits=[("ico.rs", "    let p = DIRECTORY_HEADER_LEN + ENTRY_RECORD_LEN * i;",
                "    let p = DIRECTORY_HEADER_LEN + 16 * (i & 255);")],
        expect="T2/T5 (count-exact-1024: records past 256 reread the first 256)",
    ),
    # ---- M27: sort records by descending nominal size --------------------
    27: dict(
        what="parse_tracked: sort the record list by descending raw_size as it is built",
        edits=[("ico.rs",
                "        let e = reconstruct(bytes, header.kind, header.dir_end, i, Ghost(hr), steps);\n"
                "        entries.push(e);\n",
                "        let e = reconstruct(bytes, header.kind, header.dir_end, i, Ghost(hr), steps);\n"
                "        entries.push(e);\n"
                "        // MUTATION M27: sort by descending nominal size.\n"
                "        let mut a = 0usize;\n"
                "        let mut b = 1usize;\n"
                "        while b < entries.len()\n"
                "            invariant b <= entries.len(), a < b, a < entries.len(),\n"
                "            decreases entries.len() - b,\n"
                "        {\n"
                "            if entries[a].raw_size < entries[b].raw_size {\n"
                "                let tmp = entries[a];\n"
                "                entries.set(a, entries[b]);\n"
                "                entries.set(b, tmp);\n"
                "            }\n"
                "            a += 1;\n"
                "            b += 1;\n"
                "        }\n")],
        expect="T2 (directory-order-large-first, directory-order-small-first: the record order changes)",
    ),
    # ---- M28: trust the forged kind/range (skip the comparison) ----------
    28: dict(
        what="decode_entry_impl: accept the supplied record without comparing it to the wire reconstruction",
        edits=[("ico.rs", "    let record_matches = record_eq(&selected, &reconstructed);",
                "    let record_matches = true;")],
        expect="T1/T5 (forged-wrong-kind, forged-wrong-end, forged-empty: a forged record is trusted)",
    ),
    # ---- M29: return the wire record's defect before the mismatch check --
    29: dict(
        what="decode_entry_impl: return the reconstructed wire record's defect before the selected-record mismatch check",
        edits=[("ico.rs",
                "    let record_matches = record_eq(&selected, &reconstructed);\n"
                "    if !record_matches {",
                "    let record_matches = record_eq(&selected, &reconstructed);\n"
                "    // MUTATION M29: the wire defect precedes the selected-record mismatch.\n"
                "    match reconstructed.status {\n"
                "        EntryStatus::Invalid(d) => {\n"
                "            return Err(defect_to_error(d));\n"
                "        },\n"
                "        EntryStatus::Valid { .. } => {},\n"
                "    }\n"
                "    if !record_matches {")],
        expect="T5/T6 (wire-invalid-forged-valid, wire-invalid-forged-defect: the defect precedes InvalidEntryRecord)",
    ),
    # ---- M30: exact-slice mode instead of slack mode ---------------------
    30: dict(
        what="decode_entry_impl: call the DIB core in `Mode::IcoEntryDib` (exact slice) instead of `IcoEntryDibSlack`",
        edits=[("ico.rs",
                "                    let (r0, dsteps) = dib_verus::decode_with_steps(\n"
                "                        slice,\n"
                "                        Mode::IcoEntryDibSlack,\n"
                "                    );",
                "                    let (r0, dsteps) = dib_verus::decode_with_steps(\n"
                "                        slice,\n"
                "                        Mode::IcoEntryDib,\n"
                "                    );")],
        expect="T5 (dib-slack-64-decode: a payload with slack is rejected by exact mode)",
    ),
    # ---- M31: compare only width and status ------------------------------
    31: dict(
        what="record_eq: compare only width and status (drop height, colour count, offset, size, optional words)",
        edits=[("ico.rs",
                "    a.width == b.width && a.height == b.height && a.color_count == b.color_count && a.raw_offset\n"
                "        == b.raw_offset && a.raw_size == b.raw_size && opt_planes && opt_bb && opt_hx && opt_hy && st",
                "    let _ = (opt_planes, opt_bb, opt_hx, opt_hy);\n"
                "    a.width == b.width && st")],
        expect="T5/T6 (forged-wrong-height, forged-wrong-color-count, forged-wrong-planes, forged-wrong-bit-count,\n"
               "         forged-hotspot-x, forged-hotspot-y, forged-wrong-raw-size: a partially forged record is accepted)",
    ),
    # ---- M32: validate every supplied record -----------------------------
    32: dict(
        what="decode_entry_impl: validate every supplied record against its wire reconstruction instead of only the selected one",
        edits=[("ico.rs",
                "    // R8: only `entries.len()` and `entries[index]` may be inspected.\n"
                "    let selected = entries[index as usize];",
                "    // MUTATION M32: validate every supplied record, not only the selected one.\n"
                "    let mut jj = 0usize;\n"
                "    while jj < entries.len()\n"
                "        invariant entries@.len() == hr.count, jj <= entries.len(),\n"
                "        decreases entries.len() - jj,\n"
                "    {\n"
                "        let rec = reconstruct(bytes, header.kind, header.dir_end, jj, Ghost(hr), steps);\n"
                "        if !record_eq(&entries[jj], &rec) {\n"
                "            return Err(Error::InvalidEntryRecord);\n"
                "        }\n"
                "        jj += 1;\n"
                "    }\n"
                "    // R8: only `entries.len()` and `entries[index]` may be inspected.\n"
                "    let selected = entries[index as usize];")],
        expect="T5/T6 (forged-unselected-ignored: a forged unselected record changes the selected result)",
    ),
    # ---- M33: compare only the low byte of the header type ---------------
    33: dict(
        what="directory_guard: the type guard compares only the low byte (`kind as u8`) against ICO/CUR",
        edits=[("ico.rs", "    if kind != TYPE_ICO && kind != TYPE_CUR {",
                "    if (kind as u8) != TYPE_ICO as u8 && (kind as u8) != TYPE_CUR as u8 {")],
        expect="T2/T5/T6 (type-257, type-513: a high-byte-set type is accepted as ICO/CUR)",
    ),
    # ---- M17 rework (b): offset-to-EOF as the DIB slice end ---------------
    34: dict(
        what="decode_entry_impl: the DIB slice ends at EOF (`end = bytes.len()`) instead of the declared payload end",
        edits=[("ico.rs", "            let end = payload[1] as usize;", "            let end = bytes.len();")],
        expect="T5 (entry-size-truncates-dib: the DIB slice end is EOF, not the declared end)",
    ),
}


def apply_edits(src_dir, edits):
    for fname, old, new in edits:
        path = os.path.join(src_dir, fname)
        text = open(path).read()
        if text.count(old) != 1:
            raise SystemExit(
                "anchor for %s matched %d times (expected 1); refusing to mutate: %r"
                % (fname, text.count(old), old[:80]))
        open(path, "w").write(text.replace(old, new, 1))


def cmd_apply(n, label=None):
    spec = MUTATIONS[n]
    dirname = label if label is not None else "m%d" % n
    dest = os.path.join(DIR, dirname)
    if os.path.exists(dest):
        shutil.rmtree(dest)
    os.makedirs(os.path.join(dest, "ico_verus"))
    for name in ("Cargo.toml", "Cargo.lock", "src", "tests"):
        src = os.path.join(BASE, name)
        dst = os.path.join(dest, "ico_verus", name)
        if os.path.isdir(src):
            shutil.copytree(src, dst)
        elif os.path.exists(src):
            shutil.copy2(src, dst)
    apply_edits(os.path.join(dest, "ico_verus", "src"), spec["edits"])
    print("mutation %d applied to mutation/%s/ico_verus" % (n, dirname))
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
