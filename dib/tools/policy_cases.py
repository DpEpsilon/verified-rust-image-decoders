#!/usr/bin/env python3
"""Stream test-only policy cases and independent frozen-oracle expectations.

The frozen CUSTOM-VECTORS.json remains unchanged. This covers every option
value, selected coupled groups, all byte prefixes, and both full compatibility
assignments without adding those assignments to release profile selection.
"""
import importlib.util
import itertools
import json
import struct
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location("dib_policy_oracle", ROOT / "policy/policy_oracle.py")
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)
FIELDS = ("file_size", "image_size", "planes", "reserved", "offset", "high_count", "headers", "masks")
PIXEL_FIELDS = ("unsupported", "rle_overrun", "palette_index", "rle_eol", "rle_eob", "short_pixels")
LATER_FIELDS = ("rle_delta", "rle_fill", "scaling", "bitfield16_alpha", "incomplete_metadata", "header_bytes")
IMPLEMENTED_FIELDS = FIELDS + ("bitfield_offset", "short_indexed_gap") + PIXEL_FIELDS + LATER_FIELDS
NEW_FIELDS = IMPLEMENTED_FIELDS


def emit(name, data, mode, policy, group):
    try:
        w, h, rgba, recovered = oracle._decode(data, mode, policy)
        expected = dict(width=w, height=h, rgba_hex=rgba.hex(), recovered=recovered)
    except oracle.Invalid as error:
        expected = dict(error=str(error))
    print(json.dumps(dict(name=name, input_hex=data.hex(), mode=mode,
                          policy=policy, expected=expected, group=group)), flush=True)


def image(*, mode="BmpFile", size=40, bits=24, compression=0, planes=1,
          reserved=0, used=0, offset=None, masks=(0x7c00, 0x3e0, 0x1f, 0),
          payload=b"\x11\x22\x33\x80", declared=0, width=1, height=1,
          table=b"", file_size=0):
    header = bytearray(size)
    struct.pack_into("<I", header, 0, size)
    if size == 12:
        struct.pack_into("<HHHH", header, 4, width, height, planes, bits)
    else:
        struct.pack_into("<iiHH", header, 4, width, height, planes, bits)
        if size >= 40:
            struct.pack_into("<II", header, 16, compression, declared)
            struct.pack_into("<I", header, 32, used)
    external = b""
    if compression == 3:
        if size in (40, 64):
            external = struct.pack("<III", *masks[:3])
        else:
            for i in range(4 if size >= 56 else 3):
                struct.pack_into("<I", header, 40 + 4 * i, masks[i])
    dib = header + external + table + payload
    if mode == "BareDib":
        return bytes(dib)
    if offset is None:
        offset = 14 + len(header) + len(external) + len(table)
    return b"BM" + struct.pack("<IHHI", file_size, reserved, 0, offset) + dib


def fixtures():
    rows = []

    def add(name, mode="BmpFile", **kw):
        rows.append((name, image(mode=mode, **kw), mode))

    add("ordinary-rgb")
    add("planes-reserved-size", planes=0, reserved=1, file_size=999, declared=5)
    add("rgb-clamp-exact-size", offset=0, declared=4)
    add("rgb-offset-inside-header", offset=18)
    add("rgb-offset-past-eof", offset=0xffffffff)
    add("high-count32", bits=32, used=1, table=b"\x00" * 4)
    add("high-count-max", bits=32, used=0xffffffff)
    add("high-count-bare", mode="BareDib", bits=32, used=1)
    add("header16", size=16)
    add("header64", size=64)
    add("default16", bits=16, compression=3, masks=(0, 0x3e0, 0x1f, 0))
    add("default32-alpha", bits=32, compression=3, masks=(0, 0xff00, 0xff, 0))
    add("default16-bare", mode="BareDib", size=56, bits=16, compression=3,
        masks=(0, 0x3e0, 0x1f, 0))
    add("holey16", bits=16, compression=3, masks=(0x55, 0x3e0, 0x1f, 0))
    add("holey32-alpha", size=56, bits=32, compression=3,
        masks=(0xff0000, 0xff00, 0xff, 0xa5000000))
    add("overlap16", bits=16, compression=3, masks=(0x7c00, 0x7c00, 0x1f, 0))
    add("above-depth16", bits=16, compression=3, masks=(0x10000, 0x3e0, 0x1f, 0))
    add("bitfields-offset-zero", bits=16, compression=3, offset=0)
    add("bitfields-offset-eof", bits=16, compression=3, offset=0xffffffff)
    add("indexed-blank", bits=8, used=1, offset=0, table=b"\x33\x22\x11\x00",
        payload=b"\x00" * 4)
    add("rle-blank", bits=8, compression=1, used=1, offset=0,
        table=b"\x33\x22\x11\x00", payload=b"\x00\x01")
    add("rle-indexed", bits=8, compression=1, used=1,
        table=b"\x33\x22\x11\x00", payload=b"\x01\x00\x00\x01")
    add("planes-before-dimensions", planes=0, width=0)
    add("high-count-before-output-cap", bits=32, used=1, width=8192, height=8192)
    add("bitfields-offset-gap", bits=16, compression=3, offset=70,
        payload=b"\x11\x22\x33\x80")
    add("bitfields-offset-beyond", bits=16, compression=3, offset=0xffffffff)
    add("bitfields-offset-bare", mode="BareDib", bits=16, compression=3)
    add("indexed-short-gap", bits=8, used=1, offset=70,
        table=b"\x33\x22\x11\x00", payload=b"\x00" * 4)
    add("rle-short-gap", bits=8, compression=1, used=1, offset=70,
        table=b"\x33\x22\x11\x00", payload=b"\x00\x01")
    # Rgb2 uses high-first samples; compressed two-bit input must reach R4's
    # compression checks rather than fail the depth gate.
    table2 = bytes.fromhex("332211006655440099887700ccbbaa00")
    add("rgb2-four-samples", bits=2, width=4, table=table2, payload=b"\x1b\x00\x00\x00")
    add("rgb2-bare", mode="BareDib", bits=2, width=4, table=table2, payload=b"\x1b\x00\x00\x00")
    add("rgb2-header16", size=16, bits=2, table=table2)
    add("rgb2-header12", size=12, bits=2)
    for compression in (1, 2, 3, 4):
        add(f"rgb2-compression-{compression}", bits=2, compression=compression, table=table2)
    add("rgb2-count-too-large", bits=2, used=5)
    add("rgb2-short-count", bits=2, used=1, table=table2[:4], payload=b"\x40\x00\x00\x00")
    for bits, comp in ((8, 1), (4, 2)):
        for name, data in {
            "encoded-clip": b"\x04\x00\x00\x01",
            "encoded-discard-invalid": b"\x04\x0f\x00\x01",
            "absolute-clip": b"\x00\x04\x00\x00\xff\xff\x00\x01",
            "eol-eof": b"\x00\x00",
            "eol-eob": b"\x00\x00\x00\x01",
            "one-byte": b"\x01",
            "empty": b"",
            "short-delta": b"\x00\x02\x01",
            "short-absolute": b"\x00\x06\x01",
            "black-index": b"\x01\xff\x00\x01",
            "delta-at-width": b"\x00\x02\x02\x00\x00\x01",
        }.items():
            add(f"rle{bits}-{name}", bits=bits, compression=comp, used=1,
                width=2, table=table2[:4], payload=data)
    for bits in (8, 16, 24, 32):
        for mode in ("BmpFile", "BareDib"):
            for size in (0, 2, 4, 6, 8, 9):
                add(f"short-{mode}-{bits}-{size}", mode=mode, bits=bits, height=2,
                    used=1 if bits == 8 else 0, table=table2[:4] if bits == 8 else b"",
                    payload=b"\x00" * size)
    for bits, comp in ((8, 1), (4, 2)):
        for name, data in {
            "delta-at-height": b"\x00\x02\x00\x02\x00\x01",
            "delta-past-both": b"\x00\x02\xff\xff\x00\x01",
            "delta-terminal-paint": b"\x00\x02\x01\x02\x01\x00\x00\x01",
            "delta-short-at-height": b"\x00\x02\x01\x02\x00\x02",
            "fill-all": b"\x02\x00\x00\x00\x02\x00\x00\x01",
            "fill-some": b"\x01\x00\x00\x01",
            "fill-none": b"\x00\x01",
        }.items():
            add(f"rle{bits}-{name}", bits=bits, compression=comp, used=1,
                width=2, height=2, table=table2[:4], payload=data)
    for mask in (0x155, 0x3ff, 0xffff, 0x7fffffff, 0xffffffff):
        add(f"scaled-mask-{mask:x}", size=56, bits=32, compression=3,
            masks=(mask, 0xff00, 0xff, 0xff000000), payload=b"\x95\xa6\xb7\xc8")
    add("alpha16-zero", size=56, bits=16, compression=3,
        masks=(0x7c00, 0x3e0, 0x1f, 0x8000), payload=b"\x00\x00\x00\x00")
    add("alpha16-one", size=56, bits=16, compression=3,
        masks=(0x7c00, 0x3e0, 0x1f, 0x8000), payload=b"\x00\x80\x00\x00")
    for mode in ("BmpFile", "BareDib"):
        for size in (52, 56, 64, 108, 124):
            for compression in (0, 3):
                complete = image(mode=mode, size=size, bits=16, compression=compression,
                                 offset=18, payload=b"")
                start = 14 if mode == "BmpFile" else 0
                for available in (40, 51, 52, 55, 56):
                    if available < size:
                        rows.append((f"parsed-{mode}-{size}-{compression}-{available}",
                                     complete[:start + available], mode))
        for bits, used in ((8, 1), (24, 1), (32, 0)):
            data = image(mode=mode, size=124, bits=bits, used=used, offset=18, payload=b"")
            rows.append((f"parsed-table-{mode}-{bits}", data[:(54 if mode == "BmpFile" else 40)], mode))
    return rows


def main():
    custom = json.loads((ROOT / "policy/CUSTOM-VECTORS.json").read_text())
    for row in custom:
        p = row["policy"]
        if all(p[k] == v for k, v in oracle.STRICT.items() if k not in IMPLEMENTED_FIELDS):
            oracle.check_vector(row, p)
            emit(row["name"], bytes.fromhex(row["input_hex"]), row["mode"], p, "custom-frozen")

    seeds = fixtures()
    for value in oracle.OPTIONS["unsupported"]:
        p = dict(oracle.STRICT, unsupported=value)
        for name, data, mode in seeds:
            emit(f"unsupported-{value}-{name}", data, mode, p, "rgb2")
    for values in itertools.product(*(oracle.OPTIONS[k] for k in PIXEL_FIELDS)):
        p = dict(oracle.STRICT, **dict(zip(PIXEL_FIELDS, values)))
        for name, data, mode in seeds:
            emit(f"pixel-coupled-{name}", data, mode, p, "pixel-combinations")
    for bit_offset, offset, gap in itertools.product(
            oracle.OPTIONS["bitfield_offset"], oracle.OPTIONS["offset"],
            oracle.OPTIONS["short_indexed_gap"]):
        p = dict(oracle.STRICT, bitfield_offset=bit_offset, offset=offset,
                 short_indexed_gap=gap)
        for name, data, mode in seeds:
            emit(f"coupled-{bit_offset}-{offset}-{gap}-{name}", data, mode, p,
                 "coupled-new")
    for index, values in enumerate(itertools.product(*(oracle.OPTIONS[k] for k in FIELDS))):
        p = dict(oracle.STRICT, **dict(zip(FIELDS, values)))
        for name, data, mode in seeds:
            emit(f"combination-{index}-{name}", data, mode, p, "all-combinations")

    for group, fields in (
        ("rle-coupled", ("rle_delta", "rle_fill", "rle_eol", "rle_eob", "rle_overrun", "palette_index")),
        ("scaling-coupled", ("scaling", "masks", "bitfield16_alpha")),
        ("metadata-coupled", ("header_bytes", "incomplete_metadata", "high_count", "offset", "short_pixels", "bitfield_offset")),
    ):
        for values in itertools.product(*(oracle.OPTIONS[k] for k in fields)):
            p = dict(oracle.STRICT, **dict(zip(fields, values)))
            for name, data, mode in seeds:
                emit(name, data, mode, p, group)
    for profile in ("GdkCompat-2.42.12", "ImageCrateCompat-0.25.10"):
        p = oracle.PROFILES[profile]
        for name, data, mode in seeds:
            emit(name, data, mode, p, f"profile-fixtures/{profile}")
            for end in range(len(data)):
                emit(f"{name}-prefix-{end}", data[:end], mode, p, f"profile-prefixes/{profile}")

    # Each byte boundary exercises the early error order and guarded readers.
    for field in NEW_FIELDS:
        for value in oracle.OPTIONS[field]:
            p = dict(oracle.STRICT, **{field: value})
            for name, data, mode in seeds:
                for end in range(len(data)):
                    emit(f"{field}-{value}-{name}-prefix-{end}", data[:end], mode, p, "prefixes")

    corpus = json.loads((ROOT / "dib_verus/tests/corpus_expected.json").read_text())["files"]
    bmp = [row for row in corpus if row["mode"] == "BmpFile"]
    assert len(bmp) == 71
    for profile in ("GdkCompat-2.42.12", "ImageCrateCompat-0.25.10"):
        for row in bmp:
            emit(row["file"], (ROOT / "corpus" / row["file"]).read_bytes(), "BmpFile",
                 oracle.PROFILES[profile], f"profile-corpus/{profile}")
    for field in NEW_FIELDS:
        for value in oracle.OPTIONS[field]:
            p = dict(oracle.STRICT, **{field: value})
            for row in bmp:
                name = row["file"]
                emit(name, (ROOT / "corpus" / name).read_bytes(), "BmpFile", p,
                     f"corpus/{field}={value}")


if __name__ == "__main__":
    main()
