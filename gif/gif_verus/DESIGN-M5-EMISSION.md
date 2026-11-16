# M5 emission design

Written by the Sol design reviewer (2026-11-16) and saved to this path verbatim by the coordinator (the reviewer's instructions prohibit it from writing Markdown files). Not a proof claim: a design, not run through Verus or cargo.

**Keep backward emission, discard the collision story, and separate the pure LZW observation from the composed colour outcome.** The reported witness is wrong. Reverse writes are expressly allowed by G18, and the proved per-index compositor contract already permits an arbitrary in-range position. The missing integration theorem concerns a permutation of position/index pairs, not a collision between codes.

I recommend one concrete design: let the generic core finish its **pure LZW computation even after a sink colour rejection**, retain the earliest rejected raster position and its code's consumed-bit position in constant-sized state, and let the frame adapter return the colour error instead of the pure result. This bounded speculative computation makes the complete T-LZW observation unconditional while preserving G12.2's visible error precedence. It requires neither additional allocation nor a change to a frozen statement.

## 1. The witness does not describe either decoder

For `D = [140, 11]`, the bits beginning at bit zero are `0,0,1,1,0,0,0,1,1,1,0,1,…`.

With **m = 1**, the initial width is 2, Clear is 2, EOI is 3, and next is 4. The first code is literal **0**, consumed at bit 2; the second is **EOI 3**, consumed at bit 4. There is no KwKwK code. Source-derived calls of the integrated executable, assuming a two-colour Drawer, are exactly:

- `A = 2` or `A = 5`: `[(0, 0)]`, then `TooFewPixels` at bit 4.
- `A = 1`, Strict tail: `[(0, 0)]`, then `DataAfterEndOfInformation` at bit 4, because a whole payload byte follows the EOI-containing byte.
- `A = 1`, StopAtRaster: `[(0, 0)]`, then success at bit 2, E4 false.

The `m = 1, A > 1` cases are internal-helper experiments, not reachable Strict or RecommendedBeta frames: G9 rejects them before reading the sub-blocks. Strict rejects m = 1 altogether.

The nearby genuine M2 regression uses **m = 2**, as `gif/lzw_proof/LEDGER.md:63–65` actually says. Width is 3. The codes are Clear 4 at bits `[0,3)`, literal 1 at `[3,6)`, and KwKwK 6 at `[6,9)`. At the third code, q is **1**, not 3; the resolved string is `[1,1]`. Materialising entry 6 makes next 7, so width remains 3.

For `A = 2`, the integrated executable makes exactly `[(0,1), (1,1)]`: the KwKwK walk skips position 2 and puts position 1. Strict returns `TooManyPixels` at bit 9; StopAtRaster succeeds there with E4 true.

For `A = 5`, the integrated executable makes exactly `[(0,1), (2,1), (1,1)]`. It then reads EOI 5 at `[9,12)` and returns `TooFewPixels` at bit 12. The model's index sequence is `[1,1,1]`. There is no model value 0 at position 4, and no position 4 is emitted.

Setting `left = 3` changes canvas coordinates, not LZW q, which always starts at zero within the frame. It cannot turn the same payload into the alleged emission based at q = 3. The claims about a previous overlapping emission and an inserted step-8 call have no basis in this execution. Successive code intervals are disjoint because each completed non-crossing string advances q by its length.

I ran the frozen oracle's internal `lzw_decode` with `python3 -I`, bounded by `timeout`, and traced its code-read locals. It confirms the code sequences and forward emissions above. The reverse call sequences come directly from the Rust loop; I did not run a Rust binary.

Two checked-in consequences of the erroneous story must not be used:

- `code_log` in `gif/gif_verus/src/lzw.rs:1396–1419` fabricates an additional pair for each offered position; for a singleton string and singleton hint sequences it returns two pairs although the executable calls `put` once.
- A local ghost variable does not establish a Sink observation unless the trait contract connects each concrete `put` call to the sink's ghost log.

## 2. What is observable, and which M4 contract can be reused

G10.2 and G12.2 define **forward logical emission and error precedence**. G18 explicitly permits the physical prefix-chain walk to write indices last-first. These statements coexist because successful frame pixels do not depend on the order of writes to distinct cells, and an unsuccessful frame is never exposed.

For Reject colour handling, every non-transparent out-of-range palette index produces the same error name. Choosing a different invalid pixel within one code does not change even the R12 bit position: all that code's indices have the same consumed-code-bit position. Across codes, the first affected code matters. Tracking the minimum invalid q also provides the exact forward violation as a ghost witness.

The frozen `MC` result is `Err(e)` or `Ok(canvas, saved)`. It exposes no error canvas. G12.4's last-returned-frame guarantee must be read with G18's explicit no-copy/non-exposure implementation rule and the manifest's statement at line 186 that nothing is claimed about the canvas after an error. The implementation may have changed its private working bytes, including bytes beyond the logical first colour failure; it must fuse without returning another canvas. A previously returned borrowed view cannot remain usable across the next mutable frame call.

There is an important distinction inside M4. `gif/compose_proof/src/model.rs` also defines `draw_run` and `mc` as tuples that retain the **private working canvas through an error**. Reverse drawing need not equal those tuple canvas components on an error. Do not import the standalone `composite` postcondition as an unconditional integration contract. Use successful whole-canvas equality and exact error equality, with no error-canvas equality.

The individual `draw_index` contract is less restrictive than its README. Its requirements at `gif/compose_proof/src/compose.rs:175–179` only bound the incoming `st.q`; no requirement relates that value to a preceding call. Its postcondition increments that local q and relates the call to `draw_step` at the old q. Therefore a Drawer adapter can construct a temporary `DrawState { q }` for each supplied position, call the existing helper, and ignore the incremented temporary afterward. No change to that proved per-call contract or to the emit order is necessary. Reusing one persistent counter without resetting it would be incorrect.

The missing compose-side result is a permutation lemma: for distinct in-range raster positions, successful `draw_step`s commute because their retained target cells are distinct. Transparency and clipping are identity steps. Interlace injectivity supplies the address argument. Colour validity must still be checked **before clipping**, as both existing implementations do; an off-screen non-transparent invalid index remains an error.

## 3. Recommended core, Sink, and Drawer contracts

Keep the backward chain walk and make two internal result channels explicit:

`decode(...) -> DecodeReport { core: CoreObservation, colour: Option<ColourFault> }`

`CoreObservation` contains written count, pure error, consumed-bit position, and E4. `ColourFault` contains raster q and the position after its code. The existing frame-facing `Result<bool, Error>` adapter returns `ColorIndexOutOfRange` when `colour` is present; otherwise it returns the core error or core E4 success. On a visible colour error, its ghost returned position is the saved colour position, **not** the reader's later final position.

Unlike the current `if bad { return ... }`, a sink rejection does not stop the core. Complete the string, perform ordinary dictionary insertion when the pure computation requires it, and continue until the existing pure ML fold halts. The Sink remains private to the crate; these speculative calls are not an exposed callback API. Do not expose a frame when either result channel reports an error.

Use a sink specification with an immutable area A, an immutable validity predicate `ok(v)`, and a ghost call log H. For Drawer, `ok(v) ≡ transparent == Some(v) || v < C`. The exact `put(&mut self, q: u32, v: u8) -> bool` contract is:

- Requires the sink invariant and `q < A`.
- Ensures the immutable frame, table, screen, and A are unchanged.
- Ensures `H_after = H_before.push((q,v))`.
- Ensures the returned Boolean equals `ok(v)`, independently of previous failures and call order.
- Ensures the sink invariant and canvas length are preserved.
- For Drawer, ensures the canvas is exactly `draw_step`'s successful canvas, or the incoming canvas when `draw_step` rejects v.

Drawer must not turn every later Boolean false after one rejection: the Boolean describes this index. Any row cache must satisfy `row_start = row*w`, `row_y = IR(row,h)` when valid; its interval test must permit moves in either direction. The current interval-based cache already has that shape. The temporary-state adapter avoids needing a monotone cache proof.

Let `SP(s,q,A,k)` be the existing `sink_pairs`. At a pure iteration boundary, retain the M2 state relation and these additional invariants, taking H to mean the suffix appended during this decode:

`ML(m,D,A,ps) == L(m,D,A,ps,st)`

`O == st.o`, `|O| == st.q == q`

`|H| == q`

`∀i < |H|: let (p,v)=H[i]; p < q && v == O[p]`

`∀p < q: ∃!i < |H|: H[i].0 == p`

The last two clauses give exactly one call for each logical output position, with its model value. They do not assert call-order equality with a forward zip.

For the inner walk, snapshot the incoming log `Hpre`, resolve string s, let `k = |s| − j` for remaining chain length j, and maintain:

`H + SP(s,q,A,k) == Hpre + SP(s,q,A,0)`

Together with `chain(cur) == s.take(j)` on the readable current chain, this equation describes the calls already made and the calls still to make. `SP(...,k)` alone is the **remaining** log, not the performed log. Positions beyond A append nothing. At exit, H is exactly `Hpre + SP(s,q,A,0)`; logically append `s.take(min(|s|, A−q))` to O. On a crossing, finish with ML's corresponding halted observation; otherwise advance st and q as in M2.

For an exact physical trace statement, keep a ghost sequence B of the code emissions reached by the pure fold, including each code's base q, resolved s, and after-code bit position e. Require H to equal the concatenation of `SP(s,q,A,0)` over B. This is a ghost sequence, not an executable string buffer. Each completed block appends at the end; no splicing or collision hints occur.

Maintain the earliest colour fault in constant-sized executable state. On a false return for position p, replace the stored fault only if no fault exists or p is smaller. Save the current after-code bit position with it. Its invariant is the minimum invalid position in H, paired with that pair's code timestamp from B.

At every pure halt, establish all four T-LZW observations:

`raster(H) == ML(...).idx`

`core.error == ML(...).error`

`core.pos == ML(...).pos`

`core.e4 == ML(...).e4`

Here `raster(H)` selects the unique value at positions `0..|H|−1`; it neither sorts nor allocates at runtime. This is the Sink replacement for M2's `out.take(written)`. G18 already distinguishes physical last-first writes from this logical sequence order.

Then L-INTERLEAVE gives the public error: a rejected member of that prefix takes precedence over the pure outcome. The saved timestamp gives R12's position for the first colour violation. Otherwise the pure tuple applies unchanged. The ordinary palette-rejection branch is **not** a branch of the current pure `verdict`: its only direct colour branch is the checked literal ≥256. Prove palette precedence in the composition lemma, not by falsely citing a step-8 colour branch.

On public success every offered index is valid, so the permutation lemma gives whole-canvas equality with forward drawing and hence MC after disposal/save. Each opaque write uses the active table's exact RGB and alpha 255; transparency and Clip preserve the incoming pixel. Disposal 2 clears pixels, and disposal 3 restores previously valid pixels. This proves T13's pixel clause. Existing `lemma_draw_run_pixels` is not sufficient unchanged: its hypotheses exclude off-screen Clip rectangles and require an in-range table address even for an out-of-range transparent index. Add the corresponding generalized per-put preservation lemma with separate transparent, clipped, and actual-write branches.

### Work and storage

Finishing the pure computation after colour rejection does not exceed the same pure LZW work bound required for an accepting sink. Full non-crossing strings together emit at most A indices. A final crossing string has length at most q+1 by L-CHAIN, so total chain iterations are at most `2A−1`. Calls to `put` number at most A. Code iterations are at most `4|D|`, since each consumes at least two bits; reader-fill iterations number at most `|D|`. These dominant loops therefore cost at most `5|D| + 2A` before the existing parser, disposal, and save charges. The proposed additional fault tracking is straight-line work and costs no loop iterations under G18's definition. The final proof must sum the actual helper loops, but this design introduces no new loop or per-frame fixed initialization.

The table keeps all three frozen regions unchanged; only scalar fault/report state and erased ghost sequences are added. C_FIXED remains 41,984, charged once. Forward re-walking is not the recommendation: recovering all characters by separate root-to-character traversals costs about `len*(len−1)/2` link steps per string. The fixed dictionary cap makes this asymptotically linear with a large constant, but does not establish the frozen C_CALL=32/C_WORK=64 bounds, and C_FIXED cannot pay repeated per-string work. T12 is the work obligation; T13 supplies the canvas/pixel and resource clauses.

## 4. Frozen statements and proof work

No mandatory erratum follows from this design. Read G12.4 observationally as G18 and the manifest explicitly direct; do not silently strengthen it to physical rollback. Likewise T-LZW's index order is the logical position order already used by M2's output-buffer theorem, not a requirement to invoke the private Sink in increasing position order. If the owner instead requires physical rollback or public call-order equality, those are stronger contracts and conflict with the present G18/M2 design; neither is required by the combined frozen text.

Proceed in this order:

1. Retire the false collision explanation and `code_log` from the future integration proof; keep `sink_pairs` and its valid lemmas.
2. Give Sink the log/validity contract and prove Drawer's per-call relation to `draw_step`, including arbitrary q, Clip, and transparency before lookup.
3. Repair the existing reader interface before proving decode: `BitReader::read` requires `fetched < data_len`, although a valid read can use buffered bits after fetching the final byte. For m=2, D above, A=5, the EOI read has `fetched == data_len` and seven bits remain. Permit equality and establish strict inequality only when the fill loop actually needs another byte.
4. Adapt M2's pure loop and table agreement, using the performed-plus-remaining log invariant and separate scalar colour-fault state; retain dictionary evolution after a palette failure.
5. Prove the four pure observations, the timestamped minimum-fault statement, and L-INTERLEAVE's public-result projection.
6. Add successful permutation/whole-canvas and generalized pixel-preservation lemmas; make the frame-error invariant require fusion and non-exposure rather than equality with M4's private error canvas.
7. Connect parser-established m admissibility and fresh-reader facts, then prove the actual step sum and runtime regressions.

The plain m guard has a straightforward treatment: the integrated helper's precondition includes the policy-admissible m established by G9, making its defensive `m < 1 || m > 11` branch unreachable on integration calls. A total helper theorem outside that domain must use ML's outer `m_ok` branch and must receive enough policy information to distinguish Strict from RecommendedBeta; `LzwOptions` alone does not encode that range. Do not claim equality for arbitrary unchecked m from the current signature.

I inspected the requested proof/spec interfaces and checked the small payload with the frozen Python oracle. I did not run Verus or cargo, prove the proposed permutation/log lemmas, verify the entire iterator fusion implementation, or complete the numerical T12 summation. The recommendation is a design, not a claim that M5 already verifies.
