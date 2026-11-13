p="README.md"; s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,a[:50]; s=s.replace(a,b)
rep("Pure functions, bytes in and pixels (or byte ranges) out: no I/O, no `unsafe`, no dependencies except `vstd`. Written largely",
"Pure functions, bytes in and pixels (or byte ranges) out: no I/O, no `unsafe` in the cores, no dependencies except `vstd` (ICO also depends on the DIB core). Each core is `#![no_std]` and uses `alloc`; its public entry points are the fallible `try_*` functions, which return `TryError { Format(Error), OutOfMemory(..) }` when a buffer reservation fails instead of aborting. Written largely")
old_tbl=s[s.index("| Format | Dir"):s.index("`leniency/`:")]
new_tbl="""| Format | Dir | Public entry points | Verus result (all 0 errors) | Vectors | Mutations that break the proof (campaigns predate the allocation change) |
|---|---|---|---|---|---|
| ICNS (container + legacy RLE/mask) | `icns/` | `try_parse`, `try_decode_rgb`, `try_decode_mask`, `try_combine` | 412 verified | 111 | 10/10 |
| TGA | `tga/` | `try_decode`, `try_decode_with_policy`, `try_decode_with_steps_logged` | 974 verified | 169 | 15/15 (executable-side) |
| DIB / BMP / ICO-entry bitmap | `dib/` | `try_decode`, `try_decode_with_policy`, `try_decode_with_steps`, `try_decode_with_steps_logged` | 1764 verified (spec 1.1; 1.0 is `dib/spec-frozen`) | 302 | 43/44 verifier kills + 1 audit kill |
| ICO / CUR container (over the DIB core) | `ico/` | `try_parse`, `try_parse_with_steps`, `try_decode_entry`, `try_decode_entry_with_steps` | 346 verified | 217 | 32 verifier kills + 1 audit kill of 33, executable-side (m17 reworked as m34) |

The infallible names (`parse`, `decode`, `decode_with_steps`, `decode_entry`, ...) were removed in the allocation change (`alloc-delta/`, frozen spec `alloc-delta/SPEC.md`, errata `alloc-delta/ERRATA.md`); older notes and proof reports that name them describe the pre-change cores. The counts above are the latest `RUNLOG.tsv` runs (DIB 556, TGA 231, ICNS 51, ICO 175, all on 2026-11-13).

"""
s=s.replace(old_tbl,new_tbl)
rep("For **all** input slices (`decode` has no precondition)","For **all** input slices (the `try_*` entry points have no precondition on the bytes)")
rep("- bounded allocation: one output buffer of exactly the size the header implies, within the 64 MiB cap, allocated only after header checks;",
"- bounded allocation, as a verified ghost log of reservations: one output buffer of exactly the size the header implies (ICNS and ICO also reserve an entry list), within the 64 MiB cap, reserved only after header checks; a failed reservation returns `OutOfMemory` with no partial output;")
rep("- Allocation failure/OOM, stack use, physical memory; the single-allocation rule is a *ghost* count, so an extra `vec!` is not caught by the verifier: `dib/tools/audit_alloc.sh` is a source audit (mutation 20 is an audit kill, not a proof kill). TGA allocates the output before the ID/map availability check (bounded, allowed by the spec).",
"""- Stack use and physical memory. The reservation log is *ghost*: the verifier does not see allocations, so an extra `vec!` outside the guard is caught only by the source lint `*/tools/audit_alloc.sh` (DIB mutation 20 is an audit kill, not a proof kill). The lint matches method names in text; the production call-graph audit that `alloc-delta/SPEC.md` section 6 requires does not exist.
- The guard `alloc_guard/mod.rs` (byte-identical in all four cores, SHA-256 `26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7`) is hash-pinned text, not a verified body. It rests on trusted std behaviour, A-STD-1 to A-STD-4: after `try_reserve_exact(n)` returns `Ok` the capacity is at least `n` (A-STD-1), `push` does not reallocate below capacity (A-STD-2, used by ICNS and ICO for entry lists), `Err` means the allocator failed (A-STD-3), and `resize(n, 0)` on an empty `Vec<u8>` with capacity at least `n` does not reallocate (A-STD-4). A-STD-4 is observed behaviour of std 1.99.1, not a documented guarantee (ERRATA E6); it replaced a documented-only `push` loop because the loop cost 47.8 ms against 6.6 ms for a 64 MiB buffer. A reallocation inside `resize` would be infallible and abort, and cannot be detected after the fact.
- TGA allocates the output before the ID/map availability check (bounded, allowed by the spec).""")
rep("- Soundness of Verus, vstd, Z3 and rustc; the SMT solver budgets were not raised (one `rlimit(80)` on one TGA lemma).",
"- Soundness of Verus, vstd, Z3 and rustc. Solver budgets: the live DIB source has two `#[verifier::rlimit(20)]` attributes (`parse_header`, `dib_finish`, added at run 556); the live TGA, ICNS and ICO sources have none.")
rep("cd icns/icns_verus && cargo test && cargo verus verify     # same for tga/tga_verus, dib/dib_verus","cd icns/icns_verus && cargo test && cargo verus verify     # same for tga/tga_verus, dib/dib_verus, ico/ico_verus")
rep("`./verify.sh \"<note>\"` in each project root","`./verify.sh \"<note>\"` in each project root (`icns/`, `tga/`, `dib/`, `ico/`)")
open(p,"w").write(s)
