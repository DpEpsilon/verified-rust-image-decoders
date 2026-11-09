# Draft theorem manifest — TGA core

This is a **draft**, not a freeze. It applies to TGA-CORE-SPEC 1.0 (`/home/ws/projects/tga/spec/SPEC.md`, R1–R17) and is proportioned for one Verus implementer. The notation is mathematical, not mandated Verus syntax. The proof model must be written independently of executable parsing and decoding; testing the executable against the vectors does not prove the model.

## Total reference models

`Result<X> = Ok(X) | Err(e)`, where `e` is exactly one of R15's 18 errors. The model uses mathematical integers for lengths and offsets; every concrete call has a byte slice `b` with `|b|<=usize::MAX`. `OutsideDomain` is an internal proof marker, **not** a nineteenth error. `LE16(b,p)` is defined only after checking that both bytes exist. A result has exact `u32` width/height and exact `4*width*height` RGBA bytes.

`MH(b)` is the total ordered header-and-resource model. Check R1 input cap and 18-byte header first; test type, flag, dimensions, interleave, image depth, map coherence/depth, map range, attribute count and output cap in R14 order. On success it returns a record containing the exact raw header fields, `N`, `sample_bytes` and `map_bytes`; all sizes are computed in natural numbers before narrowing. No absent byte is read. It returns `Err` at the first check failure. This is an independent model of checks, not a call to the implementation's header parser.

`MP(b)` is the complete **total** decoder model. Call `MH`; propagate its error. Test and skip the image ID; check and slice the map; apply R8's exact word/channel/alpha conversion. For raw images test each complete sample before decoding; for RLE images use one cursor and a produced count, checking control presence, overrun, whole-packet bytes, then each sample index in that order. Write each decoded pixel to the R9 oriented output position. Check R12's footer/suffix rule only after pixel `N`. Every error returns immediately. The model's pixel result contains every byte, not just output dimensions. Define `Accepted(b) iff exists w,h,p: MP(b)=Ok(w,h,p)`.

`MR(s,N,u,indexed,first,length,map)` is a total packet model for every finite sample stream `s` and natural `N,u,first,length`, returning packet-decoded samples or the first of `RleTruncated`, `RleOverrun`, `ColorIndexOutOfRange`. It always enforces `count<=N-produced` before reading packet data, validates the **whole** encoded packet before its first palette lookup, and increases `produced` by `count>=1`. The concrete decoder uses only the bounded parameters supplied by successful `MH`; mathematical calls on other parameters remain defined (if `u=0`, return `OutsideDomain`, not an API error). For an indexed lookup, `ColorIndexOutOfRange` is returned at the first invalid produced pixel. `MP` also models raw samples and the post-pixel suffix check directly; it may call the mathematical `MR`, but must not call executable RLE code.

`Safe(f,args)` means the concrete function terminates without panic, unchecked integer overflow/narrowing, out-of-bounds read/write, recursion dependent on input length or unbounded allocation requests, assuming requested allocations succeed and an ordinary process stack exists. This does not turn an out-of-memory abort into a format error. No proof may assume a valid header, valid packet, valid index or a length cap as a premise for a **public** decode call. The parser's successful-header invariants are proved from concrete code and transported to the pixel decoder, not inserted as axioms.

## Required obligations

| ID | Exact quantified claim | Excluded shortcut |
| --- | --- | --- |
| T1 · safety | `forall b, |b|<=usize::MAX: Safe(decode,b)`. Prove input-cap rejection before input-dependent allocation; every read/index/narrow is guarded; successful RGBA allocation requests exactly `4*N<=67,108,864`. | Assuming input under the cap, header complete or any valid image type. |
| T2 · header safety | `forall b, |b|<=usize::MAX: Safe(parse_header,b)` for the actual header routine, if separate; on success its width/height are 1..8,192, `N<=16,777,216`, map-byte count <=262,140, and computed positions fit the input when used. If header logic is inline, prove this as a named decoder invariant. | Postulating checked widths or valid ranges. |
| T3 · packet safety/termination | For every successful concrete header/map prefix and every remaining finite stream `s`, the actual RLE helper terminates without invalid access on all 0..255 controls and requests no pixel buffer beyond `4*N`; the loop variant `N-produced` strictly decreases or returns an error. | Assuming packet bytes exist, run counts fit or samples are in the palette. |
| T4 · conversion/orientation bounds | For each concrete accepted header and every `k<N`, sample conversion reads at most its validated 1..4 sample bytes and a validated map entry; `4*(Y*width+X)+4<=4*N`; the map access lies within its checked slice. Distinct `k` write distinct output positions. | Assuming top-left origin or a zero map first index. |
| T5 · exact header semantics | `forall b, |b|<=usize::MAX: parse_header(b)=MH(b)` as exact success record or exact first error; if inline, the proved header-prefix transition of `decode` equals `MH`. | Proving accepted inputs only or reducing all header errors to one. |
| T6 · exact packet semantics | For every header/map prefix reachable by `MH(b)=Ok`, all finite remaining streams: concrete RLE packet result equals `MR` exactly, including produced samples, consumed cursor and first error; no restriction to valid packets. | Comparing only produced count or only successful streams. |
| T7 · total functional correctness | `forall b, |b|<=usize::MAX: decode(b)=MP(b)` as exact `Result`, including all output bytes, both flips, alpha, map offsets, footer behavior and first error. | Restricting to vector inputs or using executable code as the model. |
| T8 · success and resource bounds | `forall b: decode(b)=Ok(w,h,p) iff MP(b)=Ok(w,h,p)`; in either direction `1<=w,h<=8192`, `|p|=4*w*h<=67,108,864`, and every `p[4i..4i+4]` equals R8's non-premultiplied sample at R9's unique oriented position. | Proving only dimensions/length or a one-way implication. |
| T9 · precedence | `forall b: decode(b)=Err(e) iff MP(b)=Err(e)` for each R15 error, with the header check order and RLE control/overrun/full-packet/index order explicit; then `decode` returns neither pixels nor a second error. | Assuming a single defect or swapping an overrun for a truncation. |
| T10 · termination | For all representable inputs, the header consumes a fixed 18-byte prefix or returns, ID/map checks are finite, raw decoding increments sample index by one, RLE decoding increments produced by 1..128, and output writes perform no input-dependent recursion. | Inferring totality only from a successful fixed vector. |
| T11 · integrated call path | From a **concrete** `decode(b)` call, prove every header-success invariant used by RLE, palette, orientation and footer functions, then derive T7–T9 without assuming a forged header or switching byte slices. | A model equivalence stated only for an impossible precondition. |

T1–T4/T10 cover safety and termination; T5–T9/T11 cover exact behavior, bounds and precedence. A single combined lemma can discharge multiple rows, but the report identifies each row's proved statement. No `assume`, `admit`, `external_body`, trusted wrapper, unverified FFI, unsafe operation or vacuous false precondition may establish an obligation. State verifier, standard-library contract and compiler versions. Allocator failure, process termination and verifier/compiler unsoundness remain outside the claim.

## Non-vacuity and compact vector protocol

The implementation must pass all **118** `/home/ws/projects/tga/spec/VECTORS.json` fixtures using `python3 -I` only as an independent baseline, then establish that its **own mathematical model** agrees with every fixture's independent `expected` field. The **114** expanded inputs at most 4,096 bytes require concrete computation or proof against the model. Do not expand the other four in the verifier: prove L1–L4 below using sequence/repetition algebra and instantiate the vector descriptions. The following **40** named small witnesses must be reported individually (they are not a substitute for checking the other 74):

- Header and precedence: `empty`, `header-short-17`, `unsupported-type-0`, `invalid-map-flag-2`, `dimensions-0-1`, `interleave-64`, `depth-type-2-depth-8`, `indexed-no-map`, `map-range-overflow`, `map-invalid-depth-before-range`, `attribute-2-24-1-0`, `output-too-large`, `type-before-flag-and-dimensions`, `flag-before-dimensions`, `attribute-before-output`, `output-before-id`.
- Payload and errors: `id-short`, `map-short`, `raw-partial-pixel`, `rle-repeat-overrun-no-sample`, `rle-literal-overrun-no-sample`, `rle-repeat-missing-sample`, `index-below-first`, `index-rle-packet-truncation-before-index`, `rle-packet-missing-before-invalid-first-index`, `trailing-incomplete-footer`.
- Successful pixels: `raw-15-bit15-ignored`, `raw-16-alpha-zero`, `raw-32-alpha-zero-ignores-byte`, `gray16-alpha-eight`, `palette-16-attr-1`, `palette-first-65535`, `rle-index16-literal`, `rle-crosses-row`, `origin-top-left`, `origin-top-right`, `origin-bottom-left`, `origin-bottom-right`, `footer-opaque-offsets`, `rle-stops-before-footer`.

Let `H(t,w,h,d,a)` be the R1 18-byte header with image type `t`, dimensions `w,h`, depth `d`, descriptor `a`, and all other fields zero; `S=bytes(0xff,0x7f)`; `C=bytes(0xff,0x00,0xfc)`; `q^n` denotes repetition of finite byte string `q` exactly `n` times. The definitions must be expanded symbolically from header bytes, not asserted as assumptions.

- **L1 input overflow:** `forall b, |b|>67,108,864: MP(b)=Err(FileTooLarge)`. Instantiate `input-over-64m` with a zero byte repeated 67,108,865 times.
- **L2 exact input boundary:** `forall b, |b|=67,108,864 and b[2]=0: MP(b)=Err(UnsupportedImageType)` provided its 18 bytes exist. Instantiate `input-at-64m` with a zero byte repeated 67,108,864 times; this rejects `>=` in place of `>`.
- **L3 maximum greyscale output:** For `b=H(11,8192,2048,8,32) + S^131072`, prove `|b|=262,162`, `MP(b)=Ok(8192,2048,(bytes(0x7f,0x7f,0x7f,0xff))^16777216)` and `|rgba|=67,108,864`. Instantiate `output-at-64m-8192x2048`. Here 128-pixel packets end at row boundaries because 8,192 is divisible by 128; `rle-crosses-row` proves the distinct crossing case. No verifier-level 64 MiB allocation is needed.
- **L4 maximum true-colour output:** For `b=H(10,4096,4096,16,33) + C^131072`, prove `|b|=393,234`, `MP(b)=Ok(4096,4096,(bytes(0xff,0,0,0xff))^16777216)` and `|rgba|=67,108,864`. Instantiate `output-at-64m-4096-square`.

## Deliberate proof-sensitivity mutations

Run each mutation in a disposable copy; restore it before the next. Keep failed **verifier** diagnostics with all T1–T11 enabled. A failed runtime test alone is insufficient. If a removed postcondition is reconstructed by another lemma, inject an actual incorrect sample/write instead.

1. Replace `|b|>MAX_INPUT` with `>=`; L2 and T7/T9 must fail.
2. Swap the image-type and map-flag checks; `type-before-flag-and-dimensions` and T5/T9 must fail.
3. Remove the dimension cap; `dimensions-8193-1` and T5/T8 must fail.
4. Ignore colour-map `first`; `palette-first-65535` and T7 must fail.
5. Use `v&0x8000` as alpha even for attribute count zero; `raw-16-opaque-bit-off` and T7 must fail.
6. Change repeat count from `(control&127)+1` to `control&127`; `rle-true-repeat` and T6/T7 must fail.
7. Check packet availability before packet overrun; `rle-repeat-overrun-no-sample` and T6/T9 must fail.
8. Reset the RLE produced count at each row; `rle-crosses-row` and T6/T7 must fail.
9. Reverse the vertical flip; `origin-bottom-left` and T7/T8 must fail.
10. Accept arbitrary suffix bytes or dereference a bogus footer offset; `trailing-one-byte` or `footer-opaque-offsets`, and T1/T7/T9, must fail.

No mutation is successful if it only makes a fixture test fail while all claimed theorems still verify. An unreachable branch or extra trust assumption is not a valid substitute for proof sensitivity.
