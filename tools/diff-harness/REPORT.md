# Differential test: icns_verus vs icns_creusot vs Python oracle

Result: 0 divergence groups. Written by a Sonnet agent (its Write tool refused to create this
file, so the report text was saved here from its reply). Logs: out/. Reproduce:
`PATH=/opt/cargo/bin:$PATH cargo build --release && ./target/release/icns_diff [seed] [structured] [bases]`
(about 2.5 min), then `python3 -I oracle_cmp.py 5000`. The 0.5 GB oracle sample dump was deleted
after use, so the oracle comparison cannot be repeated without regenerating it.

## Cases (Rust vs Rust)
| seed | a exhaustive | b structured | c mutations | d vectors + mutations | total |
|---|---|---|---|---|---|
| 2037265300141248775 | 16,876,759 | 30,457 | 303,747 | 459,680 | 17,670,643 |
| 20261107 | 16,876,759 | 60,457 | 477,390 | 459,753 | 17,874,359 |

Oracle (check_vectors.py) sample: 19,155 cases (seed 1) and 20,000 (seed 2); 0 divergences by
either crate. Also checked separately: 12 inputs around the 64 MiB limit and 40 entry-count cases
(4094..4098 entries with valid, malformed or short tails); all agree.

Kinds: (a) all byte strings of length 0..=3, header grids, one-entry grids, short entry areas;
(b) structured random files (legacy, opaque and unknown types, duplicates, signatures full,
partial and bit-flipped, valid/truncated/overrun/trailing RLE, it32 prefixes, mask lengths);
(c) mutations (length fields -1/0/+1/+8/extremes, truncation at every offset, byte replacement,
bit flips, insert/delete); (d) the 111 vectors plus mutations up to 200,000 bytes.

## Self-test of the harness
With a deliberately injected fault (one crate's EntryOutOfBounds renamed) the harness reported
19,193 divergent cases in one group and minimised it to a 16-byte input, so comparison and
minimiser work.

## Limits
- Entry lists over 12 are indexed by sampling (first 6, last 3, n, n+1, u32::MAX); combine pairs
  are limited; 64-bit only, so R1's 32-bit narrowing clause is untested; forged entries out of scope.
- Both crates and the oracle share an author and a spec, so a shared misreading is invisible here.

## Findings
No spec ambiguity shown. Optional extra vectors: a combine whose colour and mask sides differ
with the mask listed first; an is32 payload of exactly 1024 bytes (4N) to pin the RLE reading.
