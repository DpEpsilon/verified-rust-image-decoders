# Sol review of the TGA Verus core (saved by the coordinator from Sol's reply, 2026-11-08)

Verdict: public `decode` has an unconditional byte-exact `MP` postcondition; Sol reproduced 578 verified / 0 errors and 169/169 in release mode and found no concrete decoder misbehaviour against R1-R14. But the proof package does not establish every claim in its frozen manifest.

Blocking
1. T6 weaker than manifest. tga.rs:2742-2812,3727-3769: decode_rle starts produced=0 and contracts compare to mr(..,c0g,0); t6_rle_exact has no produced0 parameter, so no helper theorem covers nonzero starting produced counts. tga.rs:381-398,521-562: on Err, rle_result_eq compares only the error tag, not final cursor/produced; rle_pixels_eq says nothing about partial output (a wrong error-path cursor would still satisfy it). The witness also requires bytes.len()<=MAX_INPUT (:3742). T7 (public decode = MP) is sound; do not describe the helper predicates as exact equality to MR including ghost error state or arbitrary (cursor0,produced0).
2. Mutation campaign: 13 of 20 mutations (m1-m7, m9, m10, m12, m18-m20) alter src/model.rs and leave tga.rs identical (e.g. m4 changes conv_map's model offset; m10 replaces the model's footer rule). They show the vectors reject a corrupted model, not that the concrete T7 proof detects implementation faults (THEOREMS.md:58-83). Rerun with changes confined to executable behaviour, keeping frozen model and vectors. Logs show real proof failures (not syntax errors).
3. T8 per-cell witness: tga.rs:3820-3843 assumes mp_ok_shape (model.rs:3868-3882, an existential); no proved lemma MP(b)=Ok(w,h,out) => mp_ok_shape(b,w,h,out), and t8_output_cells is not called from a public-result proof. Output equality itself is established by T7 (independent MP assembly + lemma_quad_ok_to_orient); gap is in the separately advertised T8.

Should-fix
4. Out-of-domain MR returns an ordinary API error instead of the manifest's OutsideDomain marker (model.rs:1403-1416,1468-1482,1524-1531), e.g. mr(s,h,map,-1,0) = Err(ColorIndexOutOfRange,..).
5. No 32-bit verification run (only x86_64 installed); hand-traced: 4*N<=67,108,864, map<=262,140, input<=67,108,864 fit 32-bit usize/isize. Needs a second target check or width-parameterised argument, or an explicit statement.
6. MH success record omits R1 x/y origin fields (offsets 8-11) (model.rs:161-180, tga.rs:92-105); inert metadata, output unaffected; include or narrow the record claim.
7. m15 and m17 fail earlier than their specified safety obligations (m15 an existing proof assertion's premise; m17 index_in_range postcondition after body replaced by index==index), so the logs don't show the claimed OOB/overflow diagnostics.

Nit
8. tga.rs:1847-1855 allocates the bounded 4*N output before checking ID/map availability: an 18-byte 8192x2048 header can request 64 MiB then return RleTruncated/TruncatedImageId. Obeys R6 and the manifest bound; cheap availability checks could avoid it while preserving R14 priority. The single #[verifier::rlimit(80)] changes prover resources, not logical assumptions.

Checked and sound: public decode (no requires, exact MP Ok bytes/Err), ghost-start c0g passes the real cursor on the public path, models independent of tga.rs, no custom axiom/assume/admit/external_body/unsafe, cfg(verus_only) only gates imports, R8/R9/R10/R11/R12/R14 re-derived against model and code, generator --check up to date, no unwrap/expect/panic, allocations bounded by 64 MiB.
Not checked: each of the 165 vectors by hand, a 32-bit verifier run, fuzzing, allocator exhaustion, Verus/vstd soundness, rerun of the 20 mutants.
