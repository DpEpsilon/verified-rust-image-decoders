# ICO/CUR container core (spec stage)

`spec/` holds the frozen ICO-CORE-SPEC 1.0 (217 vectors, independent Python oracle, hashes in `spec/FREEZE.txt`); `THEOREMS-DRAFT.md` is the (frozen) manifest; `REVIEW*.md` are the independent reviews. No Rust implementation or proof exists yet. The core delegates DIB entries to `../dib` in `IcoEntryDibSlack` mode (spec `../dib/spec-1.1-frozen`).
Note: `spec/dib_oracle_pin.py` and the tools still point at the original workspace path of the DIB 1.1 oracle; the same file is `../dib/spec-1.1-frozen/check_vectors.py` (sha256 `214dbf12cd63e8967e1af401297cff5ac16922653d969998e7addc0d2bdd5488`). Edit the path to run the checker elsewhere.
