# Output-buffer initialisation after a fallible reservation

Crate: `src/main.rs` (no dependencies). Raw data: `timing.tsv` (every round), `summary.md` (generated tables for all sizes).
Setup: rustc/std 1.99.1, `cargo build --release --offline`, process pinned with `taskset -c 2`, machine idle beforehand (only the agent daemons and tmux running). gVisor sandbox, so absolute page-fault costs may differ from bare metal.
Method: 7 rounds interleaved (the candidate order rotates each round); per cell the reps are 32 (64 MiB), 512 (4 MiB), 8192 (256 KiB), 50000 (1 KiB); each op = allocate + fill [+ write every byte] + drop (the `munmap`/`free` is inside the timed region). "Write-all" is `out[4*i..4*i+4].copy_from_slice(&px)` over `n/4` pixels. Table values are medians of the 7 per-round means; min/max in `summary.md`.
The sandbox CPU clock ticks at 10 ms granularity, so the CPU column is coarse and agrees with wall within that tick everywhere it is resolvable (e.g. 64 MiB: A 46.9 ms CPU vs 47.8 ms wall); wall is quoted below.

## 64 MiB (4*4096*4096), ms per op

| candidate | fill only | spread (min-max) | fill + write-all | spread (min-max) | peak RSS fill / fill+write (MiB) |
|---|---|---|---|---|---|
| A push loop (current guard) | 47.76 | 46.9-52.3 | 53.80 | 48.5-63.5 | 66.9 / 67.0 |
| B `vec![0u8; n]` (calloc) | 0.044 | 0.043-0.056 | 8.28 | 7.7-13.1 | 5.7 / 68.4 |
| C `resize(n, 0)` | 6.61 | 6.0-12.3 | 9.83 | 8.9-19.3 | 67.2 / 66.8 |
| D1 `extend_from_slice(&[0;4096])` chunks | 6.84 | 6.3-9.6 | 9.63 | 8.7-16.8 | 67.1 / 68.7 |
| D2 same, 64 KiB chunks | 7.07 | 5.8-10.8 | 9.71 | 8.5-12.5 | 68.6 / 67.2 |
| E `extend(repeat(0).take(n))` | 6.46 | 6.0-14.9 | 9.48 | 9.0-17.3 | 67.6 / 67.4 |
| F `resize_with(n, \|\| 0)` | 6.65 | 5.9-12.0 | 9.76 | 8.3-19.2 | 68.2 / 67.5 |

## Smaller sizes, wall per op (fill+write-all in parentheses)

| candidate | 4 MiB | 256 KiB | 1 KiB (4*16*16) |
|---|---|---|---|
| A | 2.53 ms (2.66 ms) | 160 us (167 us) | 632 ns (655 ns) |
| B | 51.7 us (190 us) | 2.6 us (11.0 us) | 32 ns (54 ns) |
| C | 50.0 us (190 us) | 2.6 us (11.0 us) | 25 ns (52 ns) |
| D1 | 52.9 us (190 us) | 2.7 us (11.2 us) | 26 ns (53 ns) |
| D2 | 49.7 us (190 us) | 2.5 us (11.2 us) | 26 ns (53 ns) |
| E | 50.7 us (190 us) | 2.5 us (10.9 us) | 26 ns (54 ns) |
| F | 50.1 us (190 us) | 2.5 us (11.0 us) | 27 ns (52 ns) |

At 4 MiB and below, every non-A candidate including B is within noise of the others: glibc recycles the just-freed block, so the pages are already resident and a zero fill (`memset`, 80 GB/s at 4 MiB in cache) costs about 50 us. Only A, with a per-byte `push` that the compiler does not turn into `memset`, is 12-50x slower there.

## Allocator events, 64 MiB, one buffer (counting `GlobalAlloc`, std 1.99.1)

| candidate | alloc | realloc | dealloc |
|---|---|---|---|
| A, C, D1, D2, E, F | 1 (not zeroed) | 0 | 1 |
| B | 1 (via `alloc_zeroed`) | 0 | 1 |

So within already-reserved capacity none of `resize`, `extend_from_slice`, `extend(repeat.take)`, `resize_with` reallocates on this toolchain (all end with `len == n`, `capacity >= n`). This is observed behaviour, not a documented guarantee (below).

## Interpretation

- **Which candidate recovers the old time?** On the realistic metric (fill + write-all) B takes 8.28 ms and C/D/E/F take 9.5-9.8 ms. The push loop takes 53.8 ms, so replacing it saves 44 ms per 64 MiB decode (it removes about 5/6 of the cost); this matches the 37 ms extra seen in the A/B decode. The remaining gap to calloc is 1.2-1.6 ms (15-19% of the 8.28 ms fill+write, about 2% of the old 64 MiB decode if it took ~70 ms; the decode time is not measured here). Candidates C-F are indistinguishable from each other at every size (their medians sit within each other's min-max spread).
- **Lazy-zero effect.** Confirmed. Fill-only makes calloc look 150x faster than C (44 us vs 6.6 ms) because untouched pages are not zeroed and not even mapped (peak RSS 5.7 MiB vs 67 MiB). Once every byte is written, calloc's cost moves into the first-touch page faults: B goes 0.04 -> 8.28 ms, while C goes 6.61 -> 9.83 ms (the write pass over already-faulted pages costs 3.2 ms; over lazily-faulted pages 8.2 ms, so faulting is about 5 ms of the 8.28 ms). Fill-only comparisons are therefore misleading for 64 MiB; use fill+write.
- **Peak RSS.** All candidates reach ~67-69 MiB once the buffer is touched; only B fill-only stays at 5.7 MiB. A decoder that writes every byte (every pixel of the output) pays the same memory either way.
- **Is a documented-only candidate fast enough?** Yes: see the guarantee table. The only one of A-F that relies solely on documented no-realloc behaviour is A, and it is the slow one. D1/D2 (`extend_from_slice` in chunks) match C-F in speed, but their no-realloc property is not documented either. Nothing in C-F is slower than another, so the choice among them is decided purely by what the proof can rely on.

## Which std guarantee each candidate relies on (`alloc/src/vec/mod.rs`, 1.99.1)

- `try_reserve_exact` (doc, line ~1533): "capacity will be greater than or equal to `self.len() + additional` if it returns `Ok(())`". All candidates except B rely on this.
- Struct docs (line ~371): "`push` and `insert` will never (re)allocate if the reported capacity is sufficient ... the reported capacity is completely accurate, and can be relied on ... **Bulk insertion methods *may* reallocate, even when not necessary.**" This is the sentence that matters:
  - A relies on: capacity >= n after `try_reserve_exact` plus "`push` never reallocates when `len < capacity`". Both documented. This is why the guard uses it.
  - C (`resize`), D (`extend_from_slice`), E (`extend`), F (`resize_with`) are all bulk insertion methods. The docs permit them to reallocate even with sufficient capacity, so the "exactly one allocation" fact is only an implementation detail (measured: 1 alloc, 0 realloc here; they do not document it). A reallocation would not break the zero-content postcondition but would make the guard's allocation log (one entry per `try_zeroed`) and the infallibility claim (a realloc could abort on OOM via `handle_alloc_error`; `resize` also "Panics if the new capacity exceeds `isize::MAX`") rest on undocumented behaviour. For `resize`/`resize_with`/`extend_from_slice` the docs say only that they extend the vector (panic-on-overflow in the `# Panics` sections); none mentions capacity.
  - B (`vec![x; n]`): doc guarantee (line ~384) that it "requests an allocation of the exact size needed for precisely n elements from the allocator". It is infallible (aborts on OOM), which is why it is not usable for a fallible guard.
- Safe-Rust alternative that is documented and fast: none found in this set. The cheapest way to keep the documented-only argument is to keep `push` semantics but make the loop cheap (not measured here, outside the requested candidates), or to take the capacity postcondition from `try_reserve_exact`, then do one `resize` and have the postcondition checked at runtime: `if v.capacity() != cap_before { /* realloc happened */ }`. That check uses only documented `capacity` accuracy ("the reported capacity is completely accurate") and makes the "no realloc" assumption a runtime-validated fact rather than a trusted one.

## Recommendation

Change the guard's fill from the `push` loop to `v.resize(n, 0)` (C), the simplest of the equal-speed bulk fills: saves about 44 ms per 64 MiB decode (53.8 -> 9.8 ms fill+write-all), leaving ~1.5 ms over calloc. To keep the "only documented behaviour is trusted" policy, record `let cap = v.capacity();` after `try_reserve_exact`, call `resize`, and treat `v.capacity() != cap` (or `len != n`) as failure/unreachable (documented capacity accuracy makes this check sound), instead of assuming no realloc. This keeps the proof's trusted base to `try_reserve_exact`'s capacity postcondition, `capacity()` accuracy, and `resize`'s documented postcondition (len == new_len, new elements == value). If proof effort for `resize` is too high, the push loop remains the only candidate whose no-realloc is documented, at ~5x the cost.
