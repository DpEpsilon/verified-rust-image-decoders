import re
OLD="018ed17a077916108c5d2aead35f4d493207a633d0c69cc806a8627e08cf22e6"
NEW="26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7"
core = {
 "dib": dict(run="556", n="1764", prev="1,765 (run 552)", extra="The live source now carries two `#[verifier::rlimit(20)]` attributes, on `parse_header` and `dib_finish` (RUNLOG runs 555-556); statements in this file that no solver-budget attribute was added describe run 552 only. `alloc_guard/sites.rs` is unchanged (SHA-256 `33465c740e10162d0d6634afd0a6033e26f3d0f523b40e375f4b1d46bcdf000a`). DIB has no other use of A-STD-2: its guard makes one byte reservation and has no element list.", astd2="DIB has no other use of A-STD-2"),
 "tga": dict(run="231", n="974", prev="975 (run 230)", extra="The live TGA source has no `rlimit` attribute. `alloc_guard/sites.rs` is unchanged (SHA-256 `58e1fb285124557080dde6a6bc01c0d6e839cd5402b804b08b240de43750c885`). TGA has no other use of A-STD-2: its guard makes one byte reservation and has no element list. `tests/fault_injection.rs` still labels its no-`realloc` assertions A-STD-2; they now check A-STD-4.", astd2=""),
 "icns": dict(run="51", n="412", prev="413 (run 50)", extra="`alloc_guard/sites.rs` is unchanged (SHA-256 `adcc09bed170a230216248e1c3898dcc1b32af392b45032352ed717044c88c41`). A-STD-2 is still used, by `try_with_room`/`Reserved` in `sites.rs` for the entry list (the `push` of each entry below `room`); it is no longer used by `try_zeroed`, which serves the three channel buffers, the RGB output and the combined output. `tests/fault_injection.rs` labels its no-`realloc` assertions A-STD-2; for the `try_zeroed` buffers they now check A-STD-4.", astd2=""),
 "ico": dict(run="175", n="346", prev="347 (run 172)", extra="`alloc_guard/sites.rs` is unchanged (SHA-256 `5c7b24a05052823eb33fdc22576dc97a84faa6aac2e17c742790b239e0767bc1`). A-STD-2 is still used, by `try_with_room`/`Reserved` in `sites.rs` for the entry list; it is no longer used by `try_zeroed`, which `dib_verus` calls for the pixel buffer (ICO delegates DIB entries and has no `try_zeroed` call of its own).", astd2=""),
}
def block(c,h):
    d=core[c]
    return (f"{h} ERRATA E6 update (2026-11-13): `try_zeroed` fills with `resize`\n\n"
f"`alloc-delta/ERRATA.md` E6 changed the guard body in all four cores: `try_zeroed` now calls `try_reserve_exact(n)` and, on `Ok`, `v.resize(n, 0u8)`, instead of a `push` loop. `alloc_guard/mod.rs` is byte-identical in the four cores with SHA-256 `{NEW}`; the pin in `tools/audit_alloc.*` was updated. The trusted list gains A-STD-4: `Vec::<u8>::resize(n, 0)` on a `Vec` with `len == 0` and `capacity() >= n` does not allocate or reallocate. It is observed behaviour of std 1.99.1 (pinned source, counting-allocator and fault-injection tests: one allocation, zero reallocations), not a documented guarantee; `Vec`'s docs say bulk insertion methods \"may reallocate, even when not necessary\". A reallocation inside `resize` would be infallible and would abort, so it cannot be detected after the fact. The reason is cost: for a 64 MiB buffer the `push` loop takes 47.8 ms and `resize(n, 0)` 6.6 ms (`bench/fillbench/RESULTS.md`).\n\n"
f"Latest verifier result: run {d['run']}, **{d['n']} verified, 0 errors** (before E6: {d['prev']}). {d['extra']}\n\n"
f"Statements elsewhere in this file that describe the guard as a `push` loop, that use A-STD-2 for `try_zeroed`, that quote the old `mod.rs` hash `{OLD[:8]}...` or that give the pre-E6 verified count describe the cores before E6 and are superseded by this section. Test and lint results recorded in this file predate E6 unless dated 2026-11-13; a post-E6 rerun of them is not recorded here.\n")

def append(path, text, top=False, after_first_para=False):
    s=open(path).read()
    if top:
        s=text+"\n"+s
    elif after_first_para:
        lines=s.split("\n"); i=2  # after title+blank; find end of first paragraph after title
        j=i
        while j<len(lines) and lines[j].strip()!="": j+=1
        lines[j:j]=["",text.rstrip("\n")]
        s="\n".join(lines)
    else:
        s=s.rstrip("\n")+"\n\n"+text
    open(path,"w").write(s)

def note_after_para(path, needle, note, count=None):
    s=open(path).read().split("\n"); out=[]; i=0; n=0
    marks=[k for k,l in enumerate(s) if needle in l]
    # process from the back
    for k in reversed(marks):
        j=k
        while j<len(s) and s[j].strip()!="": j+=1
        s[j:j]=["", note]
        n+=1
    open(path,"w").write("\n".join(s)); return n

SUP=f"> Superseded by ERRATA E6 (2026-11-13): `alloc_guard/mod.rs` now has SHA-256 `{NEW}` (fill by `resize`; see the E6 section of this file)."
for c in core:
    append(f"{c}/NO_STD.md", block(c,"##").replace("## ERRATA","## ERRATA",1), after_first_para=True)
    sec = block(c,"##")
    if c=="tga": append(f"{c}/NOTES.md", sec, top=True)
    else: append(f"{c}/NOTES.md", sec)
    # ledger
    led=open(f"{c}/LEDGER.md").read().rstrip("\n")
    d=core[c]
    led+=f"\n\nE6 (2026-11-13, ERRATA E6, `try_zeroed` fills with `resize(n, 0)`, trusted item A-STD-4): run {d['run']}, {d['n']} verified, 0 errors (previous total {d['prev']}). Agent, tokens and tool counts are not recorded here.\n"
    open(f"{c}/LEDGER.md","w").write(led)
# inline superseded notes at old-hash mentions (non-frozen prose)
for path in ["tga/NOTES.md","tga/NO_STD.md","icns/NOTES.md","ico/NOTES.md"]:
    n=note_after_para(path, OLD, SUP); print(path,"hash notes:",n)
n=note_after_para("dib/NOTES.md","exact push-fill body","> Superseded by ERRATA E6 (2026-11-13): the guard fill is now `resize(n, 0u8)`, not a `push` loop; see the E6 section at the end of this file."); print("dib push-fill",n)
n=note_after_para("dib/NO_STD.md",OLD,SUP); print("dib NO_STD",n)
n=note_after_para("icns/LEDGER.md","`mod.rs` unchanged at `018ed17a","> Superseded by ERRATA E6 (2026-11-13): `mod.rs` is now `26e4c832...`."); print(n)
