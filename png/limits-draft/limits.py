"""PNG-CORE-SPEC P1 cap values (draft; not frozen until Stage 2, ../REVIEW-BRIEF.md). The only place the six values
are written in Python. png_oracle.py loads this file from ../limits-frozen/ if that directory exists, else from here.
Values and the decisions behind them: LIMITS.md. Any change must keep caps_ok (../THEOREMS.md, Constants) and stay
inside the Stage-1 fixture envelope (LIMITS.md, "Permitted values"); otherwise it is a Stage-1 erratum."""

MAX_INPUT = 1 << 30                    # bytes of input (1 GiB; raised from 256 MiB 2026-11-17)
MAX_DIMENSION = (1 << 31) - 1          # pixels per side: the PNG wire limit; MAX_OUTPUT bounds the area
MAX_OUTPUT = 256 * 1024 * 1024         # bytes of one RGBA8 canvas, 4*W*H (2^26 pixels)
MAX_FRAMES = 65536                     # acTL num_frames
MAX_TOTAL_PIXELS = 1 << 30             # sum of w*h over the fcTL frames of one stream
MAX_ICC = 16 * 1024 * 1024             # bytes of a decompressed ICC profile
