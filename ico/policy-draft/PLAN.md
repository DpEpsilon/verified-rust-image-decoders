# ICO policy layer: proof-phase plan (draft)

Conclusion first: the plan has two stages that match QUESTIONS Q1. **Stage A** (container-only) touches only `ico_verus`: typed profiles, policy-indexed `MC`/`ME`/`MD`, the Strict bridge, `try_parse_with_policy`/`try_decode_entry_with_policy`, and the 9 container fields; every DIB entry still decodes under DIB Strict slack mode. Estimate 120–220 new obligations over the shipped 346, 1–2 dispatches. **Stage B** (DIB pass-through) is mainly DIB work: a revised, separately versioned DIB policy spec that admits the ICO slack mode, the 6 ICO-only DIB fields, and a public **logged** DIB policy entry point that `import_dib` can consume. Estimate 250–450 DIB obligations plus 40–80 ICO obligations, 3–6 dispatches, with the highest stall risk of any stage in this repository (DIB P2a and alloc-delta ad3 stalled on per-function solver limits in the same code). The only measured corpus gain (up to 10 of 433 files, `image_size=Ignore`) needs Stage B.

The structure follows `dib/BRIEF-POLICY.md` (P1, then option groups) and TGA's obligations E, R, W, A (`tga/policy/THEOREMS-POLICY-DELTA.md`).

## 1. What changes in the core

**Stage A (ICO only).**
- New `ico_verus/src/policy.rs` and `policy_model.rs`: field enums for `dir_reserved`, `dir_type`, `entry_cap`, `entry_reserved`, `entry_range`, `entry_overlap`, `dimension_match`, `dir_field_limit`, `unselected_defects`; a `#[non_exhaustive]` `IcoProfile` (`Strict` first; others per Q3); crate-private constructors and a `#[cfg(test)]`-only custom-record path, as in DIB/TGA.
- Policy-indexed models: `H(b,P)` (reserved/type/cap), `ME(b,H,i,P)` (per-record defects: reserved, overlap, range), `MC(b,P)`, `MD(b,E,i,P)`. `MD` stays **total** for every typed `(b,E,i)` (R8/R11): the forged-record check compares `E[i]` with `ME(b,H,i,P)` at the **same** `P`, so a record parsed under one profile and decoded under another is `InvalidEntryRecord` whenever the reconstructions differ. That is the simple, sound choice; say so in the spec.
- `entry_range=ToEof` changes the slice handed to DIB from `[offset, offset+size)` to `[offset, |b|)`; the DIB call, its `decode_mp_eq` postcondition and `import_dib` are unchanged because the mode is still `IcoEntryDibSlack`. `dimension_match` and `ico_compression`-style narrowing in Stage A are checks on already-parsed fields plus, for `GdkExact`, the DIB header width/height, which `MD` must read from the slice with guarded reads before the DIB call (a new read, bounded by 16 header bytes).
- New public API: `try_parse_with_policy(bytes, profile)` and `try_decode_entry_with_policy(bytes, entries, index, profile) -> (Result<Decoded, TryError>, bool)`. The shipped `try_parse`/`try_decode_entry`/`_with_steps` keep signatures, contracts and bodies.

**Stage B (DIB, then ICO).**
- DIB: a policy spec revision (new version, new frozen hashes; the 1.0 policy spec forbids ICO policies, D1/D7/D10/D14) that (i) extends `BmpMode` or adds `IcoPolicyMode::Slack`, (ii) adds `ico_height`, `alpha32`, `and_mask`, `ico_rle`, `ico_compression`, `mono_palette` to `Policy` with Strict values equal to DIB 1.1 R3/R4/R8/R12, (iii) defines which of the 22 existing fields have an ICO site (OPTIONS §C: 8 do) and fixes the rest to Strict in ICO mode, (iv) extends the `recovered` closed list with the ICO events (QUESTIONS Q10).
- DIB executable: make the policy path's logged function public in the shape `import_dib` needs: `try_decode_with_policy_logged(bytes, mode, profile) -> (Result<Decoded, TryError>, bool, Ghost<nat> steps, Tracked<AllocLog>)` with `g_logged_policy`, `log_tied`, the step bound `16·(|b|+N)` and `decode_policy_mp_eq`. Today `try_decode_policy_tracked` (`dib.rs:10590`) is `pub(crate)` and returns no steps.
- ICO: `import_dib_policy` (a copy of `import_dib`, `alloc_guard/sites.rs:1310`, over the policy log predicates); `MD(b,E,i,P)` delegates to `MP_policy(slice, IcoSlack, dib_record(P))`; the ICO profile → DIB record map is a fixed, exhaustive function (Q1(b)).

## 2. Strict bridge

Define `H_1_0 := H(·,Strict)`, `ME_1_0 := ME(·,Strict)`, `MC_1_0 := MC(·,Strict)`, `MD_1_0 := MD(·,Strict)` definitionally, so T2–T6 transfer by unfolding. Stage B additionally needs DIB's own bridge in ICO mode, `MP_policy(s, IcoSlack, Strict) = MP_1_1(s, IcoEntryDibSlack)`, which the current DIB bridge does not cover (it is stated for BMP modes). Prove an executable `t5_strict_is_1_0(bytes, entries, index)`: the Strict policy call and the shipped call agree on every outcome, `recovered=false`, OOM excepted as in TGA's `t7_strict_is_1_0`.

## 3. Theorems (P1/P2 structure)

| Claim | Policy change |
|---|---|
| T1 safety | Universal over every retained ICO record (and, in Stage B, every DIB record reachable through the map). New reads: `GdkExact` reads 12 DIB header bytes; `ToEof` passes a longer, still checked slice. |
| T2/T3 exact parse, record validity | `parse_with_policy = MC(·,P)` per named profile; under `entry_overlap=Ignore` the valid-range invariant weakens from `dir_end ≤ start` to `6 ≤ start` (state it; T3's bound changes). `entry_cap=Max65535` changes T3/T7's `count ≤ 1024`. |
| T4 signature | Unchanged (no field touches classification). |
| T5/T6 exact total decode, precedence | Per named profile, with the selected-record comparison at the same `P` (§1). Stage B: the DIB call is the policy logged entry and the slice model is `MP_policy`. |
| T7 bounds | `dir_end ≤ 16,390` becomes `≤ 1,048,566` under `Max65535`; all other bounds unchanged. |
| T8 termination, T9 work | ICO's own `steps(parse) ≤ 16·(|b|+1)` holds for every record (the record loop is still one pass); the combined bound uses DIB's policy step bound in Stage B. |
| T10 allocation | One entry-list reservation in `parse` (its element bound follows `entry_cap`), none in the decode wrapper; DIB's reservations imported unchanged in Stage A and through `import_dib_policy` in Stage B. |
| **R** | `MD(b,E,i,Strict)=Ok(d) ⇒ MD(b,E',i,Recommended)=Ok(d)` with `recovered=false`, where `E'` is the Recommended parse (equal to the Strict parse when Recommended's ICO fields are Strict). With Recommended = Strict at the ICO level, R reduces to DIB's R in ICO mode: `image_size=Ignore` is inert when `biSizeImage ∈ {0,X,X+A}`. |
| **E** | `recovered` iff an event of the closed list (DIB's list in ICO mode, plus `ToEof` reading past the declared end and an `and_mask` relaxation firing). |

## 4. Fallible allocation

Keep every guarantee of the shipped `try_*` crate: `try_decode_entry_with_policy` makes no reservation of its own; `OutOfMemory` carries `AllocSite::Dib(site)` and the element count of DIB's failed request, exactly as `import_dib` reports it today; non-OOM results equal the model; OOM implies `recovered=false`; the logged variant returns the composed log for a caller. Stage A changes no reservation (the parse list bound changes only with `entry_cap`). Stage B's risk is DIB-side: the policy path's G/Prog/PreAlloc predicates exist (`g_policy`-style in DIB `alloc-wip-*`, landed in ad4) but are not exported in the `g_logged`/`log_tied` form that ICO's `import_dib` requires; exporting them is part of Stage B's DIB milestone, and `tga`'s `try_decode_policy_tracked` (`tga.rs:9207`) is the model to copy.

## 5. Vectors and oracle

- `policy/policy_oracle.py`: extends the frozen ICO checker (`spec-frozen/check_vectors.py`) with policy records; for DIB entries it calls the DIB policy oracle (`dib/policy/policy_oracle.py`) in Stage B, which itself needs an ICO-mode extension. Strict must reproduce all 217 frozen ICO vectors on every run.
- `policy/gen_policy_vectors.py`: every case of `ico_cases` in `/home/ws/projects/leniency/policy-probe/probe_cases.py`, one positive/negative witness per field value, and coupled cases: `entry_range=ToEof` × `and_mask`, `alpha32` × `and_mask=OptionalAtEnd` (all-zero alpha with no mask), `entry_reserved` × `entry_range`, `dimension_match` × `ico_height`.
- `policy/validate.py`: GdkCompat vs gdk-pixbuf 2.42.12 (forced type), ImageCrateCompat vs `image` 0.25.10, on vectors and the 433-file corpus. Because both libraries select internally, compare on the library's chosen entry (reconstruct gdk's score order and `image`'s best-entry rule, OPTIONS §D) or on single-entry files only. Count both-reject as agreement only in a separate column, as TGA's REPORT does.
- `FREEZE.txt` over spec, delta, oracle, generator and vectors; Stage B also re-freezes the DIB policy files under a new version.

## 6. Staging and estimate

| Milestone | Content | Estimate |
|---|---|---|
| A1 | Profiles (`Strict` only, per Q3), policy models with Strict as definitional instance, Strict bridge, `try_*_with_policy`, E constant false. | 50–90 obligations; 1 dispatch. |
| A2 | The 9 container fields for GdkCompat/ImageCrateCompat (owner may cut `entry_cap` and `dir_field_limit`). | 70–130 obligations; 1 dispatch. |
| B1 (DIB) | DIB policy spec revision; ICO slack mode in `decode_with_policy`; DIB ICO-mode Strict bridge; public logged policy entry with steps and log predicates; `image_size=Ignore` in ICO mode; Recommended. | 120–200 DIB obligations; 1–2 dispatches. |
| B2 (DIB) | The 6 ICO-only fields; the 32-bit alpha and AND-mask fields change the R12 pass and its work accounting. | 130–250 DIB obligations; 2–3 dispatches. |
| B3 (ICO) | `import_dib_policy`, `MD` over `MP_policy`, R for Recommended, profile map, conformance. | 40–80 obligations; 1 dispatch. |

References: DIB P1 added 37 obligations; DIB's full 22-field policy plus fallible allocation reached 1,765 (from 1,557) only after stalls in P2a and ad1–ad3; TGA's 13 fields added 305 (642→947) over 7 dispatches. Hazards: an ICO profile silently selecting a DIB record with BMP-only fields active in ICO mode; a forged-record check that compares against the Strict reconstruction under a non-Strict profile; `ToEof` slices interacting with `IcoEntryDibSlack`'s "ignore after AND" rule (a ToEof slice always has surplus bytes, so slack mode is mandatory there); and `alpha32`/`and_mask` changes to the R12 whole-image alpha scan breaking DIB's `16·(|b|+N)` bound if the scan is repeated.
