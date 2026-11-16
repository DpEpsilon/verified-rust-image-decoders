#!/usr/bin/env python3
"""Build the compositor test cases from the reference oracles.

For every animation, disposal and interlace vector in `spec-draft/VECTORS.json`
this runs `gif_oracle.py` (the Strict reference decoder) and records, per frame,
everything the verified compositor needs: the screen size, the rectangle, the
interlace flag, the active colour table, the indices of the frame in stream order,
the previous frame's disposal and rectangle, and the canvas the oracle produced.

The output is `tests/expected/compositor.json`, which `tests/compositor.rs` reads
and replays through the verified `composite`/`draw_index`/`interlace_row`. The
oracle is evidence for the cases, not part of any proof.
"""

import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
GIF_DIR = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, os.path.join(GIF_DIR, "spec-draft"))

import gif_oracle  # noqa: E402

# The M4 witness gate's record names (THEOREMS.md, "M4 (T4, T5, T7, T8)").
NAMES = [
    "interlace-h1", "interlace-h2", "interlace-h3", "interlace-h4", "interlace-h5",
    "interlace-h6", "interlace-h7", "interlace-h8", "interlace-h9", "interlace-h16",
    "interlace-h17", "interlace-inset", "frame-inset", "frame-touches-bottom-right",
    "frame-touches-right-edge-wire-max", "local-table-overrides-global",
    "index-out-of-range", "index-out-of-range-but-transparent", "gce-transparent-2",
    "lzw-index-error-before-code-error", "anim-dispose-none-then-inset",
    "anim-dispose-background-then-inset", "anim-dispose-previous-first-frame",
    "anim-dispose-previous-middle", "anim-dispose-previous-twice",
    "anim-dispose-background-transparent-pixels", "anim-last-frame-disposal-not-applied",
    "anim-gce-scope-one-frame", "anim-local-table-scope-one-frame",
    "gce-transparent-flag-clear-index-ignored",
]


def expand(rec):
    """G17's input expansion: `pad_to`, `repeat_hex`, `repeat`, `suffix_hex`."""
    b = bytes.fromhex(rec["input_hex"])
    if "repeat_hex" in rec:
        b += bytes.fromhex(rec["repeat_hex"]) * rec.get("repeat", 0)
    if "suffix_hex" in rec:
        b += bytes.fromhex(rec["suffix_hex"])
    if "pad_to" in rec:
        b += bytes(rec["pad_to"] - len(b))
    return b


def indices_of(m, data, area):
    """The frame's emitted index sequence, in stream order (G12.2's order)."""
    out = []
    gif_oracle.lzw_decode(m, data, area, lambda q, v: out.append(v))
    return out


class Recorder:
    """Walks the stream like `gif_oracle.run`, recording each frame's inputs."""

    def __init__(self, b):
        self.b = b
        self.cases = []

    def run(self):
        b = self.b
        scr, p = gif_oracle.parse_header(b)
        W, H = scr["width"], scr["height"]
        canvas = bytearray(4 * W * H)
        st = {"gce": None, "frames_seen": 0, "total_pixels": 0, "loop_count": None}
        prev = None
        frames = []
        while True:
            if p >= len(b):
                raise gif_oracle.GifError("MissingTrailer")
            c = b[p]
            if c == 0x3B:
                break
            if c == 0x21:
                p = gif_oracle.parse_extension(b, p, st)
                continue
            if c != 0x2C:
                raise gif_oracle.GifError("UnknownBlock")
            fh, p = gif_oracle.parse_image_descriptor(b, p, scr, st)
            m, data, p = gif_oracle.read_image_data(b, p)
            gce, st["gce"] = st["gce"], None
            st["frames_seen"] += 1
            st["total_pixels"] += fh["rect"][2] * fh["rect"][3]
            left, top, w, h = fh["rect"]
            transp = (gce or {}).get("transparent")
            idx = indices_of(m, data, w * h)
            work = bytearray(canvas)
            gif_oracle.dispose(work, scr, prev)
            disp = gce["disposal"] if gce else 0
            saved = [bytes(work[4 * (y * W + left):4 * (y * W + left + w)]) for y in range(top, top + h)] \
                if disp == 3 else None
            errors = None
            try:
                gif_oracle.render_frame(work, scr, fh, gce, m, data)
            except gif_oracle.GifError as e:
                errors = e.name
            canvas[:] = work
            prev = {"disposal": disp, "rect": fh["rect"], "saved": saved}
            self.cases.append({
                "screen": [W, H],
                "rect": list(fh["rect"]),
                "interlaced": fh["interlaced"],
                "table_hex": fh["table"].hex(),
                "indices_hex": bytes(idx).hex(),
                "prev_disposal": None if not frames else frames[-1]["disposal"],
                "prev_rect": None if not frames else frames[-1]["rect"],
                "disposal": disp,
                "transparent": transp,
                "error": errors,
                "rgba_hex": bytes(canvas).hex(),
            })
            frames.append({"disposal": disp, "rect": fh["rect"]})


def main():
    vectors = json.load(open(os.path.join(GIF_DIR, "spec-draft", "VECTORS.json")))
    by_name = {}
    for rec in vectors:
        by_name[rec["name"]] = rec
    cases = []
    for name in NAMES:
        rec = by_name.get(name)
        if rec is None:
            print("missing vector", name)
            continue
        b = expand(rec)
        try:
            r = Recorder(b)
            r.run()
        except gif_oracle.GifError as e:
            cases.append({"name": name, "error": e.name, "frames": []})
            continue
        cases.append({"name": name, "error": None, "frames": r.cases})
    out = os.path.join(HERE, "..", "tests", "expected", "compositor.json")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    json.dump(cases, open(out, "w"), indent=1)
    n = sum(len(c["frames"]) for c in cases)
    print(f"wrote {out}: {len(cases)} records, {n} frames")


if __name__ == "__main__":
    main()
