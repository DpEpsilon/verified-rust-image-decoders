//! Formal reference model for the GIF core (milestone M3): the header model
//! `MH` (SPEC G2–G4), the block-walk model `MB` (G5–G9, without pixels) and the
//! constructor's prefix walk `MP` (G15).
//!
//! This file is written *independently of the executable code*: every definition
//! below is a transcription of `../spec-draft/SPEC.md` in mathematical integers
//! over `Seq<u8>`. Nothing here calls, mirrors the control flow of, or is
//! derived from `crate::parse`/`crate::gif`. The executable carries
//! postconditions phrased in terms of these definitions; the definitions
//! themselves are never justified by running the executable.
//!
//! Arithmetic is on mathematical integers (no narrowing, no overflow). Every
//! guarded read checks availability first (`have`), and a model error is one of
//! G16's 33 executable `Error` names.
//!
//! Later milestones add the rest of the model table of `../THEOREMS.md`: `ML`
//! (G10, in `gif/lzw_proof`), `IR` (G11), `MC` (G12), `MS`/`MF` (G14/G15) and
//! `plays` (G20).

use vstd::prelude::*;

#[cfg(verus_only)]
use crate::gif::Error;
#[cfg(verus_only)]
use crate::gif::Version;
#[cfg(verus_only)]
use crate::policy::{
    DisposalUndefined, ExtSizes, FrameBounds, GceConsumers, GceDuplicate, GceOrphan, LoopScan,
    LzwMin, MissingTrailer, Policy, Trailing,
};

verus! {

// ---------------------------------------------------------------------------
// G1 limits, as the named constants of `limits.rs` (never a numeral).
// ---------------------------------------------------------------------------

/// G2: `MAX_INPUT` bytes. Read from `limits.rs`, the one place the value lives.
pub open spec fn max_input() -> int {
    crate::limits::MAX_INPUT as int
}

/// G3: screen width and height, `MAX_DIMENSION`.
pub open spec fn max_dimension() -> int {
    crate::limits::MAX_DIMENSION as int
}

/// G3: `4·W·H <= MAX_OUTPUT`.
pub open spec fn max_output() -> int {
    crate::limits::MAX_OUTPUT as int
}

/// G8: image descriptors per stream, `MAX_FRAMES`.
pub open spec fn max_frames() -> int {
    crate::limits::MAX_FRAMES as int
}

/// G8: the sum of `w·h`, `MAX_TOTAL_PIXELS`.
pub open spec fn max_total_pixels() -> int {
    crate::limits::MAX_TOTAL_PIXELS as int
}

/// G10's table size (a format constant of Appendix F, not a cap).
pub open spec fn lzw_codes() -> int {
    4096
}

// ---------------------------------------------------------------------------
// Conventions
// ---------------------------------------------------------------------------

/// Conventions (LE16): `b[p] + 256*b[p+1]`, on mathematical integers. Callers
/// only invoke it after `have(b, p, 2)`.
pub open spec fn le16(b: Seq<u8>, p: int) -> int {
    b[p] as int + 256 * (b[p + 1] as int)
}

/// Conventions: "require `n` bytes at `p`" — `p + n <= |b|`.
pub open spec fn have(b: Seq<u8>, p: int, n: int) -> bool {
    p + n <= b.len()
}

/// `2^n` for small `n`, by recursion (Verus spec `int` has no shift operator).
pub open spec fn pow2(n: int) -> int
    decreases n,
{
    if n <= 0 {
        1
    } else {
        2 * pow2(n - 1)
    }
}

/// G3/G4/G8: the entry count of a colour table with size field `f & 7`, `2^(f+1)`.
/// The size field is the low three bits of the packed byte, on `u8`, so no
/// bitwise operator on `int` is needed.
pub open spec fn table_entries(packed: u8) -> int {
    pow2((packed & 7u8) as int + 1)
}

/// G3: the global colour-table flag, bit 7 of the packed byte.
pub open spec fn flag_gct(packed: u8) -> bool {
    packed & 0x80u8 != 0u8
}

/// G8/G10: bit 6 of the image-descriptor packed byte.
pub open spec fn flag_interlace(packed: u8) -> bool {
    packed & 0x40u8 != 0u8
}

/// G8: the local colour-table flag, bit 7 of the packed byte.
pub open spec fn flag_lct(packed: u8) -> bool {
    packed & 0x80u8 != 0u8
}

/// G6.1: disposal `(G >> 2) & 7` of the Graphic Control packed byte.
pub open spec fn gce_disposal(g: u8) -> int {
    ((g >> 2u8) & 7u8) as int
}

/// G6.1: the user-input flag, bit 1.
pub open spec fn gce_user_input(g: u8) -> bool {
    g & 2u8 != 0u8
}

/// G6.1: the transparency flag, bit 0.
pub open spec fn gce_transparent_flag(g: u8) -> bool {
    g & 1u8 != 0u8
}

/// The bytes of a `[u8]` literal, as a `Seq<u8>` (used for the signature and the
/// NETSCAPE2.0 identifier).
pub open spec fn bytes_of(s: &'static [u8]) -> Seq<u8> {
    s@
}

// ---------------------------------------------------------------------------
// G2: header, G3: logical screen descriptor, G4: global colour table
// ---------------------------------------------------------------------------

/// A colour table inside the input: `entries` RGB triples at `off`.
pub struct ColorTableM {
    pub off: int,
    pub entries: int,
}

/// The `MH(b)` success record (G2–G4). `version` is `0` for `87a`, `1` for
/// `89a`; `cursor` is the position after the global colour table.
pub struct HeaderM {
    pub width: int,
    pub height: int,
    pub version: int,
    pub background_index: int,
    pub aspect: int,
    pub global: Option<ColorTableM>,
    pub cursor: int,
}

/// G2: the version the stream declares, or none. Case-sensitive.
pub open spec fn version_of(b: Seq<u8>) -> Option<Version> {
    if b[3] == 0x38u8 && b[4] == 0x37u8 && b[5] == 0x61u8 {
        Some(Version::V87a)
    } else if b[3] == 0x38u8 && b[4] == 0x39u8 && b[5] == 0x61u8 {
        Some(Version::V89a)
    } else {
        None
    }
}

/// G2's spelling as an integer, for the model record.
pub open spec fn version_code(v: Version) -> int {
    match v {
        Version::V87a => 0,
        Version::V89a => 1,
    }
}

/// `MH(b)`: G2's `|b| > MAX_INPUT` test first, then the 13-byte header, the
/// signature, the version, G3's dimensions and area cap, and G4's global table.
/// Exactly the first error of that order, or the success record.
pub open spec fn mh(b: Seq<u8>) -> Result<HeaderM, Error> {
    if b.len() > max_input() {
        Err(Error::FileTooLarge)
    } else if !have(b, 0, 13) {
        Err(Error::TruncatedHeader)
    } else if !(b[0] == 0x47u8 && b[1] == 0x49u8 && b[2] == 0x46u8) {
        Err(Error::BadSignature)
    } else {
        match version_of(b) {
            None => Err(Error::UnsupportedVersion),
            Some(v) => {
                let w = le16(b, 6);
                let h = le16(b, 8);
                if !(1 <= w && w <= max_dimension() && 1 <= h && h <= max_dimension()) {
                    Err(Error::InvalidDimensions)
                } else if 4 * w * h > max_output() {
                    Err(Error::OutputTooLarge)
                } else if flag_gct(b[10]) {
                    let entries = table_entries(b[10]);
                    if !have(b, 13, 3 * entries) {
                        Err(Error::TruncatedColorTable)
                    } else {
                        Ok(
                            HeaderM {
                                width: w,
                                height: h,
                                version: version_code(v),
                                background_index: b[11] as int,
                                aspect: b[12] as int,
                                global: Some(ColorTableM { off: 13, entries }),
                                cursor: 13 + 3 * entries,
                            },
                        )
                    }
                } else {
                    Ok(
                        HeaderM {
                            width: w,
                            height: h,
                            version: version_code(v),
                            background_index: b[11] as int,
                            aspect: b[12] as int,
                            global: None,
                            cursor: 13,
                        },
                    )
                }
            },
        }
    }
}

// ---------------------------------------------------------------------------
// G5: the sub-block sequence (the structural scan of G9)
// ---------------------------------------------------------------------------

/// G9: `D`, the concatenation of the payloads of a sub-block sequence.
pub open spec fn concat(s: Seq<Seq<u8>>) -> Seq<u8>
    decreases s.len(),
{
    if s.len() == 0 {
        Seq::empty()
    } else {
        s[0] + concat(s.subrange(1, s.len() as int))
    }
}

/// `concat`'s length is the total payload length.
pub proof fn lemma_concat_len(s: Seq<Seq<u8>>, t: Seq<Seq<u8>>)
    ensures
        concat(s + t) == concat(s) + concat(t),
        concat(s).len() == concat_len(s),
    decreases s.len(),
{
    reveal(concat);
    reveal(concat_len);
    if s.len() > 0 {
        lemma_concat_len(s.subrange(1, s.len() as int), t);
        assert((s + t).subrange(1, (s + t).len() as int) == s.subrange(1, s.len() as int) + t) by {
            assert((s + t).subrange(1, (s + t).len() as int) =~= s.subrange(1, s.len() as int) + t);
        }
    } else {
        assert(s + t == t) by {
            assert(s + t =~= t);
        }
    }
}

/// `concat_len` is additive over sequence concatenation: the payload byte count
/// of a longer sub-block list is the sum of the two parts. The `decode` loop's
/// log invariant splits the finished codes' payload list at every code, so it
/// calls this once per iteration.
pub proof fn lemma_concat_len_add(s: Seq<Seq<u8>>, t: Seq<Seq<u8>>)
    ensures
        concat_len(s + t) == concat_len(s) + concat_len(t),
    decreases s.len(),
{
    reveal(concat_len);
    if s.len() == 0 {
        assert(s + t =~= t);
    } else {
        assert((s + t)[0] == s[0]);
        assert((s + t).subrange(1, (s + t).len() as int) =~= s.subrange(1, s.len() as int) + t);
        lemma_concat_len_add(s.subrange(1, s.len() as int), t);
    }
}

/// The total payload length, as a recursion on the payload list.
pub open spec fn concat_len(s: Seq<Seq<u8>>) -> int
    decreases s.len(),
{
    if s.len() == 0 {
        0
    } else {
        s[0].len() + concat_len(s.subrange(1, s.len() as int))
    }
}

/// G5's sub-block sequence starting with the size byte at `q`: the payloads in
/// order and the cursor after the zero-size terminator, or `None` when a
/// required byte is absent. A size byte of 0 always terminates, including as
/// the first sub-block (N10). ICNS's `walk` has this shape
/// (`icns/icns_verus/src/model.rs:535`).
pub open spec fn sub_scan(b: Seq<u8>, q: int) -> Option<(Seq<Seq<u8>>, int)>
    decreases (b.len() - q) as nat,
{
    if !have(b, q, 1) {
        None
    } else {
        let n = b[q] as int;
        if n == 0 {
            Some((Seq::empty(), q + 1))
        } else if !have(b, q + 1, n) {
            None
        } else {
            match sub_scan(b, q + 1 + n) {
                None => None,
                Some((rest, e)) => Some((seq![b.subrange(q + 1, q + 1 + n)] + rest, e)),
            }
        }
    }
}

/// The `sub_scan` loop state: the scan from `q` opens with the payloads `acc`
/// and continues from `p`, whose tail it carries. This is `walk_pref`'s shape
/// (`icns/icns_verus/src/model.rs:1123`).
pub open spec fn sub_pref_at(b: Seq<u8>, q: int, p: int, acc: Seq<Seq<u8>>) -> bool {
    match sub_scan(b, p) {
        None => sub_scan(b, q) is None,
        Some((rest, e)) => sub_scan(b, q) == Some((acc + rest, e)),
    }
}

/// The scan's exact accounting, which bounds the executable accumulator: a scan
/// from `q` that has read `acc` payloads stands at cursor `p` with
/// `p - q == concat_len(acc) + acc.len()` — one size byte per payload plus the
/// payloads themselves.
pub open spec fn sub_consumed(q: int, p: int, acc: Seq<Seq<u8>>) -> bool {
    p - q == concat_len(acc) + acc.len()
}

/// One accepted sub-block step.
pub proof fn lemma_sub_scan_step(b: Seq<u8>, q: int, p: int, acc: Seq<Seq<u8>>, n: int)
    requires
        sub_pref_at(b, q, p, acc),
        have(b, p, 1),
        b[p] as int == n,
        1 <= n,
        have(b, p + 1, n),
    ensures
        sub_pref_at(b, q, p + 1 + n, acc.push(b.subrange(p + 1, p + 1 + n))),
{
    reveal(sub_pref_at);
    reveal(sub_scan);
    reveal(have);
    let payload = b.subrange(p + 1, p + 1 + n);
    match sub_scan(b, p + 1 + n) {
        None => {
            assert(sub_scan(b, p) is None);
        },
        Some((rest, e)) => {
            assert(sub_scan(b, p) == Some((seq![payload] + rest, e)));
            assert(acc + (seq![payload] + rest) == (acc.push(payload)) + rest) by {
                assert(acc.push(payload) == acc + seq![payload]);
                assert((acc + seq![payload]) + rest =~= acc + (seq![payload] + rest));
            }
            assert(acc.push(payload) + rest == acc + (seq![payload] + rest));
        },
    }
}

/// The initial state.
pub proof fn lemma_sub_pref_init(b: Seq<u8>, q: int)
    ensures
        sub_pref_at(b, q, q, Seq::empty()),
{
    reveal(sub_pref_at);
    match sub_scan(b, q) {
        None => {},
        Some((rest, e)) => {
            assert(Seq::<Seq<u8>>::empty() + rest == rest) by {
                assert(Seq::<Seq<u8>>::empty() + rest =~= rest);
            }
        },
    }
}

/// The final state: the size byte at `p` is 0.
pub proof fn lemma_sub_pref_done(b: Seq<u8>, q: int, p: int, acc: Seq<Seq<u8>>)
    requires
        sub_pref_at(b, q, p, acc),
        have(b, p, 1),
        b[p] == 0u8,
    ensures
        sub_scan(b, q) == Some((acc, p + 1)),
{
    reveal(sub_pref_at);
    reveal(sub_scan);
    reveal(have);
    assert(sub_scan(b, p) == Some((Seq::<Seq<u8>>::empty(), p + 1)));
    assert(acc + Seq::<Seq<u8>>::empty() == acc) by {
        assert(acc + Seq::<Seq<u8>>::empty() =~= acc);
    }
}

/// `concat_len` is non-negative.
pub proof fn lemma_concat_len_nonneg(s: Seq<Seq<u8>>)
    ensures
        0 <= concat_len(s),
    decreases s.len(),
{
    reveal(concat_len);
    if s.len() > 0 {
        lemma_concat_len_nonneg(s.subrange(1, s.len() as int));
    }
}

/// The accounting holds initially.
pub proof fn lemma_sub_consumed_init(q: int)
    ensures
        sub_consumed(q, q, Seq::empty()),
{
    reveal(sub_consumed);
    reveal(concat_len);
}

/// The accounting steps: appending the payload at `p` of length `n` advances
/// the cursor by `1 + n` and the accounting by the same amount.
pub proof fn lemma_sub_consumed_step(q: int, p: int, acc: Seq<Seq<u8>>, n: int)
    requires
        sub_consumed(q, p, acc),
        0 <= n,
    ensures
        sub_consumed(q, p + 1 + n, push_len(acc, n)),
{
    reveal(sub_consumed);
    reveal(concat_len);
    reveal(push_len);
    lemma_concat_len_push_len(acc, n);
}

/// The concrete form of `lemma_concat_len_push_len`: appending the real payload
/// at `p` grows `concat_len` by its length and the cursor accounting by `1 + n`.
pub proof fn lemma_concat_len_push(b: Seq<u8>, p: int, acc: Seq<Seq<u8>>, n: int)
    requires
        0 <= p,
        0 <= n,
        have(b, p + 1, n),
    ensures
        concat_len(acc.push(b.subrange(p + 1, p + 1 + n))) == concat_len(acc) + n,
{
    reveal(concat_len);
    reveal(have);
    assert(have(b, p + 1, n));
    assert(p + 1 + n <= b.len());
    assert(0 <= p + 1);
    let payload = b.subrange(p + 1, p + 1 + n);
    assert(payload.len() == n) by {
        vstd::seq::lemma_seq_subrange_len(b, p + 1, p + 1 + n);
    }
    lemma_concat_len_push_len(acc, n);
    // `concat_len` depends only on the payload lengths, not their contents.
    lemma_concat_len_ext(acc.push(payload), push_len(acc, n));
}

pub proof fn lemma_concat_len_ext(s: Seq<Seq<u8>>, t: Seq<Seq<u8>>)
    requires
        s.len() == t.len(),
        forall|i: int| 0 <= i < s.len() ==> #[trigger] s[i].len() == t[i].len(),
    ensures
        concat_len(s) == concat_len(t),
    decreases s.len(),
{
    reveal(concat_len);
    if s.len() > 0 {
        lemma_concat_len_ext(s.subrange(1, s.len() as int), t.subrange(1, t.len() as int));
    }
}

/// `acc` with a payload of length `n` appended (contents irrelevant to the
/// accounting).
pub open spec fn push_len(acc: Seq<Seq<u8>>, n: int) -> Seq<Seq<u8>> {
    acc.push(Seq::new(n as nat, |_i: int| 0u8))
}

/// `concat_len` grows by a payload's length when it is appended.
pub proof fn lemma_concat_len_push_len(acc: Seq<Seq<u8>>, n: int)
    requires
        0 <= n,
    ensures
        concat_len(push_len(acc, n)) == concat_len(acc) + n,
        push_len(acc, n).len() == acc.len() + 1,
    decreases acc.len(),
{
    reveal(concat_len);
    reveal(push_len);
    let payload = Seq::new(n as nat, |_i: int| 0u8);
    assert(payload.len() == n);
    if acc.len() == 0 {
        assert(acc.push(payload) == seq![payload]) by {
            assert(acc.push(payload) =~= seq![payload]);
        }
        assert(concat_len(seq![payload]) == n) by {
            assert(seq![payload][0] == payload);
            assert(seq![payload].subrange(1, 1) == Seq::<Seq<u8>>::empty());
            assert(concat_len(Seq::<Seq<u8>>::empty()) == 0);
        }
    } else {
        lemma_concat_len_push_len(acc.subrange(1, acc.len() as int), n);
        assert(acc.push(payload).subrange(1, acc.push(payload).len() as int) == acc.subrange(
            1,
            acc.len() as int,
        ).push(payload)) by {
            assert(acc.push(payload).subrange(1, acc.push(payload).len() as int) =~= acc.subrange(
                1,
                acc.len() as int,
            ).push(payload));
        }
    }
}

// ---------------------------------------------------------------------------
// MB: the block walk (G5–G9, no pixels)
// ---------------------------------------------------------------------------

/// A Graphic Control Extension (G6.1): the executable `parse::Gce` record
/// itself, so the model and the executable compare it with `==` and no
/// conversion lemma is needed. `disposal` is the raw 3-bit value (4–7 only
/// under `disposal_undefined = Keep`).
pub use crate::parse::Gce as GceM;

/// G8: the image descriptor and local colour table.
pub struct FrameHeaderM {
    pub left: int,
    pub top: int,
    pub w: int,
    pub h: int,
    pub interlaced: bool,
    /// The active table (local if present, else global) and its offset.
    pub table: ColorTableM,
    /// The rectangle lies inside the screen (always true under `Reject`).
    pub inside: bool,
}

/// The block-walk state (G5, with the cursor `p`).
pub struct WalkM {
    pub p: int,
    pub gce: Option<GceM>,
    pub frames_seen: int,
    pub total_pixels: int,
    pub loop_count: Option<u16>,
}

/// The result of one `MB` step: the Trailer, or one image with its frame header,
/// its (recorded) Graphic Control, its minimum code size, the ghost
/// concatenated payload `D` and the state after the terminator.
pub enum MbOut {
    Trailer,
    Image(FrameHeaderM, Option<GceM>, int, Seq<u8>, WalkM),
}

/// G5 step 2: at the Trailer (or at EOF under `missing_trailer = AtEof`), the
/// pending-Graphic-Control check precedes the trailing-bytes check (G16).
pub open spec fn trailer_step(b: Seq<u8>, pol: Policy, s: WalkM) -> Result<MbOut, Error> {
    if matches!(s.gce, Some(_)) && pol.gce_orphan == GceOrphan::Reject {
        Err(Error::GraphicControlWithoutGraphic)
    } else if s.p < b.len() && s.p + 1 < b.len() && pol.trailing == Trailing::Reject {
        Err(Error::TrailingBytes)
    } else {
        Ok(MbOut::Trailer)
    }
}

/// G6.1: the Graphic Control Extension body at `q` (after the introducer and
/// label), or the first error.
/// The Graphic Control record `gce_step` builds from the packed byte at `q + 1`.
pub open spec fn gce_record(b: Seq<u8>, q: int) -> GceM {
    GceM {
        disposal: b[q + 1] >> 2u8 & 7u8,
        user_input: b[q + 1] & 2u8 != 0u8,
        delay_cs: le16(b, q + 2) as u16,
        transparent: if b[q + 1] & 1u8 != 0u8 {
            Some(b[q + 4])
        } else {
            None
        },
    }
}

pub open spec fn gce_step(b: Seq<u8>, pol: Policy, s: WalkM, q: int) -> Result<WalkM, Error> {
    if matches!(s.gce, Some(_)) && pol.gce_duplicate == GceDuplicate::Reject {
        Err(Error::DuplicateGraphicControl)
    } else if !have(b, q, 1) {
        Err(Error::TruncatedExtension)
    } else if b[q] != 4u8 {
        Err(Error::InvalidGraphicControl)
    } else if !have(b, q, 6) {
        Err(Error::TruncatedExtension)
    } else if b[q + 5] != 0u8 {
        Err(Error::InvalidGraphicControl)
    } else {
        let g = b[q + 1];
        let disposal = g >> 2u8 & 7u8;
        if disposal as int > 3 && pol.disposal_undefined == DisposalUndefined::Reject {
            Err(Error::UndefinedDisposal)
        } else {
            Ok(
                WalkM {
                    p: q + 6,
                    gce: Some(gce_record(b, q)),
                    frames_seen: s.frames_seen,
                    total_pixels: s.total_pixels,
                    loop_count: s.loop_count,
                },
            )
        }
    }
}

/// G6.2: the Plain Text Extension at `q`.
pub open spec fn plain_text_step(b: Seq<u8>, pol: Policy, s: WalkM, q: int) -> Result<WalkM, Error> {
    if !have(b, q, 1) {
        Err(Error::TruncatedExtension)
    } else if b[q] != 12u8 && pol.ext_sizes == ExtSizes::Exact {
        Err(Error::InvalidPlainText)
    } else {
        match sub_scan(b, q) {
            None => Err(Error::TruncatedExtension),
            Some((_, end)) => Ok(
                WalkM {
                    p: end,
                    gce: s.gce,
                    frames_seen: s.frames_seen,
                    total_pixels: s.total_pixels,
                    loop_count: s.loop_count,
                },
            ),
        }
    }
}

/// G6.3: the Application Extension at `q`, including rule A's loop scan.
pub open spec fn app_step(b: Seq<u8>, pol: Policy, s: WalkM, q: int) -> Result<WalkM, Error> {
    if !have(b, q, 1) {
        Err(Error::TruncatedExtension)
    } else if b[q] != 11u8 && pol.ext_sizes == ExtSizes::Exact {
        Err(Error::InvalidApplicationExtension)
    } else {
        match sub_scan(b, q) {
            None => Err(Error::TruncatedExtension),
            Some((subs, end)) => {
                let lc = if is_netscape(subs) && s.frames_seen == 0 {
                    let scanned = loop_value(
                        subs.subrange(1, subs.len() as int),
                        pol.loop_scan == LoopScan::FirstSubBlockOnly,
                    );
                    match scanned {
                        None => s.loop_count,
                        Some(v) => Some(v),
                    }
                } else {
                    s.loop_count
                };
                Ok(
                    WalkM {
                        p: end,
                        gce: s.gce,
                        frames_seen: s.frames_seen,
                        total_pixels: s.total_pixels,
                        loop_count: lc,
                    },
                )
            },
        }
    }
}

/// G6.3: the identifier an Application Extension must carry to be a loop source.
pub const NETSCAPE_ID: [u8; 11] = *b"NETSCAPE2.0";

/// Whether the `i` payload bytes at `q + 1` equal the first `i` bytes of
/// `NETSCAPE2.0`, the predicate `app_ext`'s byte-by-byte comparison loop
/// maintains.
pub open spec fn id_matches(b: Seq<u8>, q: int, i: int) -> bool {
    forall|k: int| 0 <= k < i ==> #[trigger] b[q + 1 + k] == NETSCAPE_ID@[k]
}

/// Bridging `id_matches` at 11 to the payload's equality with the constant:
/// `id_matches(b, q, 11)` holds iff the 11-byte subrange at `q + 1` is
/// `NETSCAPE2.0`. This is the form `is_netscape` reads.
pub proof fn lemma_id_matches_eq(b: Seq<u8>, q: int)
    requires
        0 <= q,
        q + 12 <= b.len(),
    ensures
        id_matches(b, q, 11) == (b.subrange(q + 1, q + 12) == NETSCAPE_ID@),
{
    let id = NETSCAPE_ID@;
    let sub = b.subrange(q + 1, q + 12);
    vstd::seq::lemma_seq_subrange_len(b, q + 1, q + 12);
    assert(sub.len() == 11);
    assert(id.len() == 11);
    if id_matches(b, q, 11) {
        assert forall|k: int| 0 <= k < sub.len() implies #[trigger] sub[k] == id[k] by {
            vstd::seq::lemma_seq_subrange_index(b, q + 1, q + 12, k);
            assert(0 <= k < 11);
            assert(sub[k] == b[q + 1 + k]);
            assert(b[q + 1 + k] == id[k]);
        }
        assert(sub =~= id);
    } else {
        assert(exists|k: int| 0 <= k < 11 && b[q + 1 + k] != id[k]) by {
            if !exists|k: int| 0 <= k < 11 && b[q + 1 + k] != id[k] {
                assert(forall|k: int| 0 <= k < 11 ==> b[q + 1 + k] == id[k]);
                assert(id_matches(b, q, 11));
            }
        }
        let k = choose|k: int| 0 <= k < 11 && b[q + 1 + k] != id[k];
        vstd::seq::lemma_seq_subrange_index(b, q + 1, q + 12, k);
        assert(sub[k] == b[q + 1 + k]);
        assert(sub != id);
    }
}

/// Whether the first payload is exactly `NETSCAPE2.0`.
pub open spec fn is_netscape(s: Seq<Seq<u8>>) -> bool {
    s.len() >= 1 && s[0] == bytes_of(&NETSCAPE_ID)
}





/// G6.3 rule A, as the oracle's `for s in subs` runs it: a left-to-right fold in
/// which each 3-byte payload whose first byte is 1 replaces the value, so the
/// last such payload wins. `off` is the absolute index of `s[0]` within the
/// sub-block sequence, which is what `first_only` consults: with
/// `loop_scan = FirstSubBlockOnly` only the payload at index 0 can set the
/// value.
pub open spec fn loop_fold(
    s: Seq<Seq<u8>>,
    off: int,
    acc: Option<u16>,
    first_only: bool,
) -> Option<u16>
    decreases s.len(),
{
    if s.len() == 0 {
        acc
    } else {
        let h = s[0];
        loop_fold(
            s.subrange(1, s.len() as int),
            off + 1,
            if loop_hit(h, off, first_only) {
                loop_hit_value(h)
            } else {
                acc
            },
            first_only,
        )
    }
}

/// The fold, starting from no value.
pub open spec fn loop_value(s: Seq<Seq<u8>>, first_only: bool) -> Option<u16> {
    loop_fold(s, 0, None, first_only)
}

/// A fold started from a `Some` seed is always `Some`: the accumulator only ever
/// becomes `loop_hit_value(h)` (a `Some`) or stays as it was.
pub proof fn lemma_fold_some_seed(s: Seq<Seq<u8>>, off: int, acc: Option<u16>, first_only: bool)
    requires
        acc is Some,
    ensures
        loop_fold(s, off, acc, first_only) is Some,
    decreases s.len(),
{
    reveal(loop_fold);
    if s.len() == 0 {
        assert(loop_fold(s, off, acc, first_only) == acc);
    } else {
        let h = s[0];
        let hit = loop_hit(h, off, first_only);
        let hv = loop_hit_value(h);
        reveal(loop_hit_value);
        let acc1 = if hit {
            hv
        } else {
            acc
        };
        if hit {
            assert(hv is Some);
        }
        assert(acc1 is Some);
        lemma_fold_some_seed(s.subrange(1, s.len() as int), off + 1, acc1, first_only);
    }
}

/// The fold from a seed `acc` equals the fold from `None`, except that a `None`
/// result is `acc` (the seed survives when no payload hits). This is the identity
/// G6.3 rule A needs: `app_step` folds from `None` and `app_ext` folds from the
/// incoming `loop_count`, and G18 makes the two agree.
pub proof fn lemma_loop_fold_seed(s: Seq<Seq<u8>>, off: int, acc: Option<u16>, first_only: bool)
    ensures
        loop_fold(s, off, acc, first_only) == match loop_fold(s, off, None, first_only) {
            None => acc,
            Some(v) => Some(v),
        },
    decreases s.len(),
{
    reveal(loop_fold);
    if s.len() == 0 {
        assert(loop_fold(s, off, acc, first_only) == acc);
        assert(loop_fold(s, off, None, first_only) == None);
    } else {
        let h = s[0];
        let t = s.subrange(1, s.len() as int);
        let hit = loop_hit(h, off, first_only);
        let hv = loop_hit_value(h);
        let acc1 = if hit {
            hv
        } else {
            acc
        };
        let none1 = if hit {
            hv
        } else {
            None
        };
        reveal(loop_hit_value);
        assert(loop_fold(s, off, acc, first_only) == loop_fold(t, off + 1, acc1, first_only));
        assert(loop_fold(s, off, None, first_only) == loop_fold(t, off + 1, none1, first_only));
        if hit {
            assert(none1 == hv && acc1 == hv);
            assert(hv is Some);
            lemma_fold_some_seed(t, off + 1, hv, first_only);
            assert(loop_fold(t, off + 1, hv, first_only) is Some);
        } else {
            assert(none1 == None && acc1 == acc);
            lemma_loop_fold_seed(t, off + 1, acc, first_only);
        }
    }
}

/// `app_step`'s success case, exposed in the shape `app_ext` computes it (T6,
/// G6.3 rule A). From the `sub_scan` unfolding — the identifier payload
/// `subs[0]`, the tail `subs.subrange(1, |subs|)`, the end cursor `e` — and the
/// rule-A value `lc` the executable's `loop_count_of` returns
/// (`loop_fold(tail, 0, s.loop_count, first_only)`, or `s.loop_count` when the
/// identifier did not match), the model's `app_step` returns exactly the
/// `WalkM` with cursor `e` and `loop_count = lc`. This is the missing model
/// lemma that closes `app_ext`'s open `loop_count` obligation (STACK.md).
pub proof fn lemma_app_step_of_scan(
    b: Seq<u8>,
    pol: Policy,
    s: WalkM,
    q: int,
    subs: Seq<Seq<u8>>,
    e: int,
    id_match: bool,
    first_only: bool,
    lc: Option<u16>,
)
    requires
        0 <= q,
        have(b, q, 1),
        1 <= b[q] as int,
        have(b, q + 1, b[q] as int),
        pol.ext_sizes == ExtSizes::Exact ==> b[q] as int == 11,
        sub_scan(b, q) == Some((subs, e)),
        subs.len() >= 1,
        subs[0] == b.subrange(q + 1, q + 1 + (b[q] as int)),
        sub_scan(b, q + 1 + (b[q] as int)) == Some((subs.subrange(1, subs.len() as int), e)),
        id_match == (subs[0] == bytes_of(&NETSCAPE_ID)),
        first_only == (pol.loop_scan == LoopScan::FirstSubBlockOnly),
        lc == if id_match && s.frames_seen == 0 {
            loop_fold(subs.subrange(1, subs.len() as int), 0, s.loop_count, first_only)
        } else {
            s.loop_count
        },
    ensures
        app_step(b, pol, s, q) == Ok(
            WalkM {
                p: e,
                gce: s.gce,
                frames_seen: s.frames_seen,
                total_pixels: s.total_pixels,
                loop_count: lc,
            },
        ),
{
    reveal(have);
    let tail = subs.subrange(1, subs.len() as int);
    assert(is_netscape(subs) == id_match) by {
        reveal(is_netscape);
        assert(bytes_of(&NETSCAPE_ID) == NETSCAPE_ID@);
    }
    reveal(loop_value);
    assert(loop_value(tail, first_only) == loop_fold(tail, 0, None, first_only));
    lemma_loop_fold_seed(tail, 0, s.loop_count, first_only);
    let scanned = loop_value(tail, first_only);
    let model_lc = if is_netscape(subs) && s.frames_seen == 0 {
        match scanned {
            None => s.loop_count,
            Some(v) => Some(v),
        }
    } else {
        s.loop_count
    };
    assert(model_lc == lc) by {
        if is_netscape(subs) && s.frames_seen == 0 {
            assert(scanned == loop_fold(tail, 0, None, first_only));
            match loop_fold(tail, 0, None, first_only) {
                None => {
                    assert(scanned == None);
                    assert(loop_fold(tail, 0, s.loop_count, first_only) == s.loop_count);
                },
                Some(v) => {
                    assert(scanned == Some(v));
                    assert(loop_fold(tail, 0, s.loop_count, first_only) == Some(v));
                },
            }
        }
    }
    reveal(app_step);
    assert(have(b, q, 1));
    let bq: u8 = b[q];
    if pol.ext_sizes == ExtSizes::Exact {
        assert(bq as int == 11);
    }
    assert(!(bq != 11u8 && pol.ext_sizes == ExtSizes::Exact));
    match sub_scan(b, q) {
        None => {
            assert(false);
        },
        Some((subs2, end)) => {
            assert(subs2 == subs);
            assert(end == e);
            assert(is_netscape(subs2) == is_netscape(subs));
            assert(subs2.subrange(1, subs2.len() as int) == tail);
            assert(model_lc == if is_netscape(subs2) && s.frames_seen == 0 {
                match loop_value(subs2.subrange(1, subs2.len() as int), pol.loop_scan
                    == LoopScan::FirstSubBlockOnly) {
                    None => s.loop_count,
                    Some(v) => Some(v),
                }
            } else {
                s.loop_count
            });
        },
    }
}

/// The executable's rule-A test for the payload `h` at index `off`, in exactly
/// the form `read_sub_blocks` evaluates it, so the model and the executable
/// agree without a separate equivalence argument.
pub open spec fn loop_hit(h: Seq<u8>, off: int, first_only: bool) -> bool {
    (!first_only || off == 0) && h.len() == 3 && h[0] == 1u8
}

/// The value a hit sets.
pub open spec fn loop_hit_value(h: Seq<u8>) -> Option<u16> {
    Some(((h[1] as u16) | ((h[2] as u16) << 8)) as u16)
}

/// Every position a walk over an input of at most `MAX_INPUT` bytes forms is at
/// most `MAX_INPUT + 2^16`, which fits a `usize` on either target. `MAX_INPUT`
/// is 268,435,456 and the crate's positions are `usize` sums of values below
/// `MAX_INPUT + 2^16` (`parse.rs` module comment), well below `2^32`.
pub proof fn lemma_pos_fits(p: usize, n: usize)
    requires
        p <= crate::limits::MAX_INPUT,
        n <= 65536,
        usize::MAX as u64 >= 0xffff_ffffu64,
    ensures
        p + n <= usize::MAX,
{
    assert(crate::limits::MAX_INPUT == 268435456);
    assert(268435456 + 65536 == 268500992);
    assert(p + n <= 268500992) by (nonlinear_arith)
        requires
            p <= 268435456,
            n <= 65536,
    ;
    // On both supported targets `usize` is at least 32 bits, so `usize::MAX` is
    // at least 4,294,967,295 and 268,500,992 fits.
    assert(268500992 <= 0xffff_ffffu64) by (bit_vector);
    assert(p + n <= usize::MAX) by (nonlinear_arith)
        requires
            p + n <= 268500992,
            268500992 <= 0xffff_ffffu64,
            usize::MAX as u64 >= 0xffff_ffffu64,
    ;
}

/// The first byte of `b` is `first` when the payload begins there.
pub proof fn lemma_payload_head(b: Seq<u8>, p: int, n: int)
    requires
        0 <= p,
        have(b, p + 1, n),
        1 <= n,
    ensures
        b.subrange(p + 1, p + 1 + n)[0] == b[p + 1],
{
    reveal(have);
    assert(0 <= p + 1);
    assert(p + 1 + n <= b.len());
    assert(p + 1 <= p + 1 + n);
    assert(p + 1 <= b.len());
    vstd::seq::lemma_seq_subrange_len(b, p + 1, p + 1 + n);
    vstd::seq::lemma_seq_subrange_index(b, p + 1, p + 1 + n, 0);
}

/// The fold of nothing is the seed.
pub proof fn lemma_loop_fold_init(seed: Option<u16>)
    ensures
        loop_fold(Seq::<Seq<u8>>::empty(), 0, seed, false) == seed,
        loop_fold(Seq::<Seq<u8>>::empty(), 0, seed, true) == seed,
{
    reveal(loop_fold);
}

/// The loop scan's one step, in the shape the executable checks it: the payload
/// `h` at index `off` with the payload's first byte `first`; on the executable's
/// condition the value becomes `v` (`Some(le16(b, p+2))`), else it stays.
pub proof fn lemma_loop_fold_one_step(
    h: Seq<u8>,
    off: int,
    prev: Option<u16>,
    next: Option<u16>,
    first_only: bool,
)
    requires
        0 <= off,
        loop_hit(h, off, first_only) ==> next == loop_hit_value(h),
        !loop_hit(h, off, first_only) ==> next == prev,
    ensures
        loop_fold(seq![h], off, prev, first_only) == next,
{
    reveal(loop_fold);
    reveal(loop_hit);
    reveal(loop_hit_value);
    assert(seq![h].subrange(1, 1) == Seq::<Seq<u8>>::empty()) by {
        assert(seq![h].subrange(1, 1) =~= Seq::<Seq<u8>>::empty());
    }
    assert(seq![h][0] == h);
    assert(seq![h].len() == 1);
    if loop_hit(h, off, first_only) {
        assert(loop_fold(seq![h], off, prev, first_only) == loop_fold(
            Seq::<Seq<u8>>::empty(),
            off + 1,
            loop_hit_value(h),
            first_only,
        ));
        assert(loop_fold(Seq::<Seq<u8>>::empty(), off + 1, loop_hit_value(h), first_only)
            == loop_hit_value(h));
    } else {
        assert(loop_fold(seq![h], off, prev, first_only) == loop_fold(
            Seq::<Seq<u8>>::empty(),
            off + 1,
            prev,
            first_only,
        ));
        assert(loop_fold(Seq::<Seq<u8>>::empty(), off + 1, prev, first_only) == prev);
    }
}

/// The loop payload's value when it is a 3-byte id-1 block: `Some(LE16)` of its
/// last two bytes. Stated on plain bytes so the executable's `le16` binds to it.
pub open spec fn loop_value3(x: u8, y: u8, z: u8) -> Option<u16> {
    Some(((y as u16) | ((z as u16) << 8)) as u16)
}

pub proof fn lemma_loop_value3(x: u8, y: u8, z: u8, v: u16)
    requires
        v == ((y as u16) | ((z as u16) << 8)) as u16,
    ensures
        loop_value3(x, y, z) == Some(v),
{
    reveal(loop_value3);
}

/// The scan's result is unique: `sub_pref_at` with a finished scan pins the
/// payload list and the cursor.
pub proof fn lemma_sub_scan_unique(b: Seq<u8>, q: int, acc: Seq<Seq<u8>>, p: int)
    requires
        sub_pref_at(b, q, p, acc),
        have(b, p, 1),
        b[p] == 0u8,
    ensures
        sub_scan(b, q) == Some((acc, p + 1)),
{
    lemma_sub_pref_done(b, q, p, acc);
}

/// Folding a single payload from a list's end agrees with folding the list.
pub proof fn lemma_loop_fold_single_tail(acc: Seq<Seq<u8>>, h: Seq<u8>, seed: Option<u16>,
    first_only: bool)
    ensures
        acc + seq![h] == acc.push(h),
        loop_fold(acc + seq![h], 0, seed, first_only) == loop_fold(
            seq![h],
            acc.len() as int,
            loop_fold(acc, 0, seed, first_only),
            first_only,
        ),
        loop_fold(acc.push(h), 0, seed, first_only) == loop_fold(
            seq![h],
            acc.len() as int,
            loop_fold(acc, 0, seed, first_only),
            first_only,
        ),
{
    reveal(loop_fold);
    assert(acc + seq![h] == acc.push(h));
    lemma_loop_fold_append(acc, seq![h], 0, seed, first_only);
}

/// The fold splits at the end of `s`: folding `s ++ p` from `off` is folding `p`
/// from `off + |s|` with the fold of `s` as its starting value.
pub proof fn lemma_loop_fold_append(s: Seq<Seq<u8>>, p: Seq<Seq<u8>>, off: int, acc: Option<
    u16,
>, first_only: bool)
    ensures
        loop_fold(s + p, off, acc, first_only) == loop_fold(
            p,
            off + s.len(),
            loop_fold(s, off, acc, first_only),
            first_only,
        ),
    decreases s.len(),
{
    reveal(loop_fold);
    if s.len() == 0 {
        assert(s + p == p) by {
            assert(s + p =~= p);
        }
    } else {
        let h = s[0];
        let acc2 = if loop_hit(h, off, first_only) {
            loop_hit_value(h)
        } else {
            acc
        };
        lemma_loop_fold_append(s.subrange(1, s.len() as int), p, off + 1, acc2, first_only);
        assert((s + p).subrange(1, (s + p).len() as int) == s.subrange(1, s.len() as int) + p) by {
            assert((s + p).subrange(1, (s + p).len() as int) =~= s.subrange(1, s.len() as int)
                + p);
        }
        assert(off + s.len() == off + 1 + s.subrange(1, s.len() as int).len());
    }
}

/// G6.4: a Comment (0xFE) or an unknown label, whose sub-block sequence is
/// read and skipped. Every record this milestone implements has
/// `unknown_extension = Skip` (Strict's and RecommendedBeta's value, the
/// `UnknownExtension` enum's only variant), so the sequence is always read;
/// P2's `unknown_extension = Reject` record returns `UnknownBlock` here for a
/// label other than the Comment one.
pub open spec fn skip_step(b: Seq<u8>, s: WalkM, q: int) -> Result<WalkM, Error> {
    match sub_scan(b, q) {
        None => Err(Error::TruncatedExtension),
        Some((_, end)) => Ok(
            WalkM {
                p: end,
                gce: s.gce,
                frames_seen: s.frames_seen,
                total_pixels: s.total_pixels,
                loop_count: s.loop_count,
            },
        ),
    }
}

/// G6 and G7: the extension with its introducer at `s.p`, and the scope check
/// that precedes every other check of the extension.
pub open spec fn ext_step(b: Seq<u8>, pol: Policy, s: WalkM) -> Result<WalkM, Error> {
    if !have(b, s.p, 2) {
        Err(Error::TruncatedExtension)
    } else {
        let label = b[s.p + 1] as int;
        let q = s.p + 2;
        if label <= 0x7F && matches!(s.gce, Some(_)) && pol.gce_consumers == GceConsumers::Reject {
            Err(Error::UnsupportedGraphicControlTarget)
        } else if label == 0xF9 {
            gce_step(b, pol, s, q)
        } else if label == 0x01 {
            plain_text_step(b, pol, s, q)
        } else if label == 0xFF {
            app_step(b, pol, s, q)
        } else {
            skip_step(b, s, q)
        }
    }
}

/// G9: the minimum-code-size byte at `c` and the sub-block sequence after it.
/// `area` is the descriptor's `w·h`.
pub open spec fn data_step(b: Seq<u8>, pol: Policy, s: WalkM, c: int, area: int) -> Result<
    (int, Seq<u8>, WalkM),
    Error,
> {
    if !have(b, c, 1) {
        Err(Error::TruncatedImageData)
    } else {
        let m = b[c] as int;
        let m_ok = match pol.lzw_min {
            LzwMin::Spec => 2 <= m && m <= 8,
            LzwMin::Range1to11Checked => 1 <= m && m <= 11 && (m != 1 || area == 1),
        };
        if !m_ok {
            Err(Error::InvalidLzwMinCodeSize)
        } else {
            match sub_scan(b, c + 1) {
                None => Err(Error::TruncatedImageData),
                Some((subs, end)) => Ok(
                    (
                        m,
                        concat(subs),
                        WalkM {
                            p: end,
                            gce: None,
                            frames_seen: s.frames_seen + 1,
                            total_pixels: s.total_pixels + area,
                            loop_count: s.loop_count,
                        },
                    ),
                ),
            }
        }
    }
}

/// G8: the image descriptor at `s.p` (the separator) and the local colour table.
pub open spec fn image_step(b: Seq<u8>, hdr: HeaderM, pol: Policy, s: WalkM) -> Result<
    (FrameHeaderM, Option<GceM>, int, Seq<u8>, WalkM),
    Error,
> {
    if s.frames_seen >= max_frames() {
        Err(Error::TooManyFrames)
    } else if !have(b, s.p, 10) {
        Err(Error::TruncatedImageDescriptor)
    } else {
        let left = le16(b, s.p + 1);
        let top = le16(b, s.p + 3);
        let w = le16(b, s.p + 5);
        let h = le16(b, s.p + 7);
        let packed = b[s.p + 9];
        let c = s.p + 10;
        let area = w * h;
        if w == 0 || h == 0 {
            Err(Error::InvalidFrameDimensions)
        } else {
            let inside = left + w <= hdr.width && top + h <= hdr.height;
            if !inside && pol.frame_bounds == FrameBounds::Reject {
                Err(Error::FrameOutsideScreen)
            } else if s.total_pixels + area > max_total_pixels() {
                Err(Error::PixelBudgetExceeded)
            } else if flag_lct(packed) {
                let entries = table_entries(packed);
                if !have(b, c, 3 * entries) {
                    Err(Error::TruncatedColorTable)
                } else {
                    let fh = FrameHeaderM {
                        left,
                        top,
                        w,
                        h,
                        interlaced: flag_interlace(packed),
                        table: ColorTableM { off: c, entries },
                        inside,
                    };
                    finish_image(b, pol, s, fh, area, c + 3 * entries)
                }
            } else {
                match hdr.global {
                    None => Err(Error::NoColorTable),
                    Some(t) => {
                        let fh = FrameHeaderM {
                            left,
                            top,
                            w,
                            h,
                            interlaced: flag_interlace(packed),
                            table: t,
                            inside,
                        };
                        finish_image(b, pol, s, fh, area, c)
                    },
                }
            }
        }
    }
}

/// G9: the image-data step, sharing the active-table choice of `image_step`.
pub open spec fn finish_image(b: Seq<u8>, pol: Policy, s: WalkM, fh: FrameHeaderM, area: int, c: int)
    -> Result<(FrameHeaderM, Option<GceM>, int, Seq<u8>, WalkM), Error>
{
    match data_step(b, pol, s, c, area) {
        Err(e) => Err(e),
        Ok((m, d, s2)) => Ok((fh, s.gce, m, d, s2)),
    }
}

/// `MB(b, H, s)`: one step of the block walk from `s`. `fuel` is `|b| - s.p` in
/// every reachable state; each extension consumes at least one byte, so the
/// bound is never reached before the walk leaves the block loop.
pub open spec fn mb_fuel(b: Seq<u8>, hdr: HeaderM, pol: Policy, s: WalkM, fuel: nat) -> Result<
    MbOut,
    Error,
>
    decreases fuel,
{
    if fuel == 0 || s.p >= b.len() {
        if pol.missing_trailer == MissingTrailer::Reject {
            Err(Error::MissingTrailer)
        } else {
            trailer_step(b, pol, s)
        }
    } else if b[s.p] == 0x3Bu8 {
        trailer_step(b, pol, s)
    } else if b[s.p] == 0x21u8 {
        match ext_step(b, pol, s) {
            Err(e) => Err(e),
            Ok(s2) => mb_fuel(b, hdr, pol, s2, (fuel - 1) as nat),
        }
    } else if b[s.p] == 0x2Cu8 {
        match image_step(b, hdr, pol, s) {
            Err(e) => Err(e),
            Ok((fh, gce, m, d, s2)) => Ok(MbOut::Image(fh, gce, m, d, s2)),
        }
    } else {
        Err(Error::UnknownBlock)
    }
}

/// `MB(b, H, s)`: one step of the block walk.
pub open spec fn mb(b: Seq<u8>, hdr: HeaderM, pol: Policy, s: WalkM) -> Result<MbOut, Error> {
    mb_fuel(b, hdr, pol, s, (b.len() - s.p) as nat)
}

// ---------------------------------------------------------------------------
// MP: the constructor's prefix walk (G15)
// ---------------------------------------------------------------------------

/// G1's `Info`, as the model record.
pub struct InfoM {
    pub width: int,
    pub height: int,
    pub version: int,
    pub background_index: int,
    pub aspect: int,
    pub loop_count: Option<u16>,
}

/// G15's prefix walk: `MB`'s extension steps until the cursor reaches the first
/// image separator (0x2C) or the walk ends at the Trailer (or at EOF under
/// `missing_trailer = AtEof`), where the trailer checks still run. `fuel` is
/// `|b| - s.p` in every reachable state.
pub open spec fn mp_fuel(b: Seq<u8>, pol: Policy, s: WalkM, fuel: nat) -> Result<WalkM, Error>
    decreases fuel,
{
    if fuel == 0 || s.p >= b.len() {
        if pol.missing_trailer == MissingTrailer::Reject {
            Err(Error::MissingTrailer)
        } else {
            match trailer_step(b, pol, s) {
                Err(e) => Err(e),
                Ok(_) => Ok(s),
            }
        }
    } else if b[s.p] == 0x3Bu8 {
        match trailer_step(b, pol, s) {
            Err(e) => Err(e),
            Ok(_) => Ok(s),
        }
    } else if b[s.p] == 0x21u8 {
        match ext_step(b, pol, s) {
            Err(e) => Err(e),
            Ok(s2) => mp_fuel(b, pol, s2, (fuel - 1) as nat),
        }
    } else if b[s.p] == 0x2Cu8 {
        Ok(s)
    } else {
        Err(Error::UnknownBlock)
    }
}

/// `MP(b)`: `MH`, then the prefix walk. `Ok((info, s0))` carries G1's `Info`
/// and the walk state at the first image separator or at the end.
pub open spec fn mp(b: Seq<u8>, pol: Policy) -> Result<(InfoM, WalkM), Error> {
    match mh(b) {
        Err(e) => Err(e),
        Ok(hdr) => {
            let s = WalkM {
                p: hdr.cursor,
                gce: None,
                frames_seen: 0,
                total_pixels: 0,
                loop_count: None,
            };
            match mp_fuel(b, pol, s, (b.len() - s.p) as nat) {
                Err(e) => Err(e),
                Ok(s0) => Ok(
                    (
                        InfoM {
                            width: hdr.width,
                            height: hdr.height,
                            version: hdr.version,
                            background_index: hdr.background_index,
                            aspect: hdr.aspect,
                            loop_count: s0.loop_count,
                        },
                        s0,
                    ),
                ),
            }
        },
    }
}

// ---------------------------------------------------------------------------
// Malformedness (the structural half of G18's "input is borrowed")
// ---------------------------------------------------------------------------

/// `Malformed(b)`: `MH(b)` does not succeed. A malformed input yields no image
/// and no readable frame, so the pixels of any slice agreeing with `b` before
/// the header are irrelevant.
pub open spec fn malformed(b: Seq<u8>) -> bool {
    match mh(b) {
        Err(_) => true,
        Ok(_) => false,
    }
}

// ---------------------------------------------------------------------------
// Arithmetic facts about `MH` (the content of T2's bounds)
// ---------------------------------------------------------------------------

/// `MH`'s success record satisfies the derived bounds: `1 <= W, H <=
/// MAX_DIMENSION`, `|b| <= MAX_INPUT`, a table of `2^(f+1)` entries between 2
/// and 256 whose bytes are present, and the cursor inside the input.
pub open spec fn mh_bounds(b: Seq<u8>, m: HeaderM) -> bool {
    &&& 1 <= m.width <= max_dimension()
    &&& 1 <= m.height <= max_dimension()
    &&& 4 * m.width * m.height <= max_output()
    &&& b.len() <= max_input()
    &&& match m.global {
        None => m.cursor == 13,
        Some(t) => 2 <= t.entries <= 256 && t.off == 13 && m.cursor == 13 + 3 * t.entries
            && have(b, 13, 3 * t.entries),
    }
    &&& m.cursor <= b.len()
    &&& 0 <= m.background_index < 256
    &&& 0 <= m.aspect < 256
    &&& (m.version == 0 || m.version == 1)
}

/// `pow2(n)` for `0 <= n <= 8` is between 1 and 256.
pub proof fn lemma_pow2_range(n: int)
    requires
        0 <= n <= 8,
    ensures
        1 <= pow2(n) <= 256,
    decreases n,
{
    reveal_with_fuel(pow2, 9);
    if n == 0 {
        assert(pow2(0) == 1);
    } else {
        lemma_pow2_range(n - 1);
        assert(pow2(n) == 2 * pow2(n - 1));
    }
}

/// The size field `packed & 7` is at most 7, so `+ 1` is in `1..=8`.
pub proof fn lemma_size_field(packed: u8)
    ensures
        0 <= (packed & 7u8) as int <= 7,
        1 <= (packed & 7u8) as int + 1 <= 8,
{
    assert((packed & 7u8) as int <= 7) by (bit_vector);
}

/// `pow2` is positive and monotone on the non-negative integers.
pub proof fn lemma_pow2_pos(n: int)
    requires
        0 <= n,
    ensures
        1 <= pow2(n),
    decreases n,
{
    reveal(pow2);
    if n > 0 {
        lemma_pow2_pos(n - 1);
    }
}

/// `MH(b)` returns its record in range for every input.
pub proof fn lemma_mh_ok_bounds(b: Seq<u8>, m: HeaderM)
    requires
        mh(b) == Ok(m),
    ensures
        mh_bounds(b, m),
{
    reveal(mh);
    reveal(version_of);
    reveal(table_entries);
    reveal(have);
    reveal(le16);
    reveal(flag_gct);
    reveal(version_code);
    if !(b.len() > max_input()) && have(b, 0, 13) && b[0] == 0x47u8 && b[1] == 0x49u8 && b[2]
        == 0x46u8 {
        lemma_version_of_len(b);
        match version_of(b) {
            None => {},
            Some(v) => {
                if 1 <= le16(b, 6) && le16(b, 6) <= max_dimension() && 1 <= le16(b, 8) && le16(b, 8)
                    <= max_dimension() && 4 * le16(b, 6) * le16(b, 8) <= max_output() {
                    lemma_size_field(b[10]);
                    lemma_pow2_range((b[10] & 7u8) as int + 1);
                    if flag_gct(b[10]) {
                        assert(have(b, 13, 3 * table_entries(b[10])));
                        assert(m.cursor == 13 + 3 * table_entries(b[10]));
                        assert(m.cursor <= b.len()) by {
                            assert(have(b, 13, 3 * table_entries(b[10]))) ;
                        }
                        assert(3 * table_entries(b[10]) <= b.len() - 13) by {
                            assert(13 + 3 * table_entries(b[10]) <= b.len());
                        }
                    } else {
                        assert(m.cursor == 13);
                        assert(m.cursor <= b.len());
                    }
                }
            },
        }
    }
}

/// `version_of` is only read when the six version bytes exist.
pub proof fn lemma_version_of_len(b: Seq<u8>)
    requires
        have(b, 0, 13),
    ensures
        true,
{
    reveal(have);
}

/// `MH(b)` succeeds only when the 13 header bytes are present.
pub proof fn lemma_mh_ok_len(b: Seq<u8>, m: HeaderM)
    requires
        mh(b) == Ok(m),
    ensures
        13 <= b.len() <= max_input(),
{
    reveal(mh);
    reveal(have);
    lemma_mh_ok_bounds(b, m);
}

/// The table entry count is between 2 and 256 for every size field.
pub proof fn lemma_table_entries_range(packed: u8)
    ensures
        2 <= table_entries(packed) <= 256,
{
    reveal(table_entries);
    assert((packed & 7u8) as int <= 7) by (bit_vector);
    lemma_pow2_range((packed & 7u8) as int + 1);
}

/// `1u32 << n` equals `pow2(n)` for the small `n` a size field can give.
pub proof fn lemma_shift_pow2(n: int)
    requires
        0 <= n <= 8,
    ensures
        (1u32 << (n as u32)) as int == pow2(n),
    decreases n,
{
    reveal_with_fuel(pow2, 9);
    if n == 0 {
        assert(pow2(0) == 1);
        assert(1u32 << 0u32 == 1u32) by (compute_only);
    } else {
        lemma_shift_pow2(n - 1);
        assert(pow2(n) == 2 * pow2(n - 1));
        let m = n as u32;
        assert(m >= 1);
        let mm = (m - 1) as u32;
        assert(1u32 << m == (1u32 << mm) * 2u32) by (bit_vector)
            requires
                m <= 8,
                mm == m - 1,
        ;
    }
}

/// `area_cap` (THEOREMS.md's `area_cap_u64`): with `w, h <= 65535` the product
/// `w*h` fits `u32` (at most 4,294,836,225), so the executable's comparison
/// `w*h > MAX_OUTPUT/4` cannot wrap and is equivalent to the model's
/// `4·w·h > MAX_OUTPUT` (`MAX_OUTPUT` is a multiple of 4).
pub open spec fn max_output_quarter() -> int {
    max_output() / 4
}

pub proof fn lemma_area_cap(w: u32, h: u32)
    requires
        w <= 65535,
        h <= 65535,
    ensures
        (w as int) * (h as int) <= 4294836225,
        (w as int) * (h as int) > max_output_quarter() ==> 4 * (w as int) * (h as int)
            > max_output(),
        4 * (w as int) * (h as int) > max_output() ==> (w as int) * (h as int)
            > max_output_quarter(),
{
    reveal(max_output);
    reveal(max_output_quarter);
    assert((w as int) * (h as int) <= 4294836225) by (nonlinear_arith)
        requires
            w <= 65535,
            h <= 65535,
    ;
    assert(max_output_quarter() == 16777216);
    assert(max_output() == 67108864);
    let a = (w as int) * (h as int);
    assert(a > max_output_quarter() ==> 4 * a > max_output()) by (nonlinear_arith)
        requires
            max_output() == 67108864,
            max_output_quarter() == 16777216,
            a == (w as int) * (h as int),
    ;
    assert(0 <= a) by (nonlinear_arith)
        requires
            a == (w as int) * (h as int),
            w <= 65535,
            h <= 65535,
    ;
    assert(4 * (w as int) * (h as int) == 4 * a) by (nonlinear_arith)
        requires
            a == (w as int) * (h as int),
    ;
    assert(4 * a > max_output() ==> a > max_output_quarter()) by (nonlinear_arith)
        requires
            max_output() == 67108864,
            max_output_quarter() == 16777216,
            a == (w as int) * (h as int),
            0 <= a,
    ;
}

/// `4·W·H <= MAX_OUTPUT` (the checked G3 bound) fits a `usize` on either target
/// and equals the mathematical product. The bound itself comes from `MH`
/// (`mh_bounds`); on a 32-bit target it is what rules out the overflow.
pub proof fn lemma_4wh_usize(w: u32, h: u32)
    requires
        1 <= w <= 65535,
        1 <= h <= 65535,
        4 * (w as int) * (h as int) <= max_output(),
    ensures
        4 * (w as usize) * (h as usize) <= usize::MAX,
        4 * (w as usize) * (h as usize) == 4 * (w as int) * (h as int),
        4 * (w as int) * (h as int) >= 4,
{
    assert(4 * (w as usize) * (h as usize) == 4 * (w as int) * (h as int)) by (bit_vector);
    assert(4 * (w as int) * (h as int) <= max_output());
    assert(4 * (w as usize) * (h as usize) <= max_output() as usize);
    assert(4 * (w as int) * (h as int) >= 4) by (nonlinear_arith)
        requires
            w >= 1,
            h >= 1,
    ;
}

/// `p + 2` does not overflow a `usize` when the caller has checked
/// `p + 1 + n <= |b|` with `n == 3`: a `usize` sum bounded by a real slice
/// length cannot wrap.
pub proof fn lemma_add3_no_overflow(p: usize, n: usize)
    requires
        3 <= n,
        p + 1 + n <= usize::MAX,
    ensures
        p + 2 <= usize::MAX,
{
}

/// A `usize` sum bounded by a real slice length cannot wrap.
pub proof fn lemma_slice_bound(len: usize, x: usize)
    requires
        x <= len,
    ensures
        x <= usize::MAX,
{
}

/// `MAX_INPUT + 2^16` fits a `usize` on both supported targets. Stated as
/// `268500992 < usize::MAX` with `u64` arithmetic: a `usize` of 32 bits or more
/// has `usize::MAX >= 2^32 - 1 = 4,294,967,295 > 268,500,992`.
pub proof fn lemma_max_input_fits_umax()
    ensures
        crate::limits::MAX_INPUT == 268435456,
        crate::limits::MAX_INPUT + 65536 <= usize::MAX,
{
    assert(crate::limits::MAX_INPUT == 268435456);
    assert(crate::limits::MAX_INPUT <= usize::MAX);
    assert(crate::limits::MAX_INPUT + 65536 <= usize::MAX);
}

/// `MAX_INPUT + 12` fits a `usize` on both supported targets (TGA's idiom,
/// `tga.rs:2636`: a plain `<= usize::MAX` assert on a small literal).
pub proof fn lemma_max_input_plus12()
    ensures
        crate::limits::MAX_INPUT + 12 <= usize::MAX,
        crate::limits::MAX_INPUT <= usize::MAX,
{
    assert(crate::limits::MAX_INPUT == 268435456);
    assert(crate::limits::MAX_INPUT <= usize::MAX);
    assert(crate::limits::MAX_INPUT + 12 <= usize::MAX);
}

/// `p + 1` does not overflow a `usize` when `p < len`: the DIB/TGA idiom
/// (`dib.rs:853`, `tga.rs:1352`), stated over the slice length so both
/// `bit_vector` and linear arithmetic apply.
pub proof fn lemma_succ_no_overflow_usize(len: usize, p: usize)
    requires
        p < len,
        len <= crate::limits::MAX_INPUT + 65536,
    ensures
        p + 1 <= usize::MAX,
        len <= usize::MAX,
{
    assert(p + 1 <= len) by (bit_vector)
        requires
            p < len,
    ;
    lemma_max_input_fits_umax();
    assert(len <= usize::MAX) by (nonlinear_arith)
        requires
            len <= crate::limits::MAX_INPUT + 65536,
            crate::limits::MAX_INPUT + 65536 <= usize::MAX,
    ;
}

/// `q + 1 + i` for `q <= MAX_INPUT` and `i <= 11` fits a `usize`.
pub proof fn lemma_pos_fits_two(q: usize, n: usize)
    requires
        q <= 61440,
        n <= 12,
        usize::MAX as u64 >= 0xffff_ffffu64,
    ensures
        q + n <= usize::MAX,
{
    assert(q + n <= 61452) by (nonlinear_arith)
        requires
            q <= 61440,
            n <= 12,
    ;
    assert(61452 <= usize::MAX) by (nonlinear_arith)
        requires
            61452 <= 0xffff_ffffu64,
            usize::MAX as u64 >= 0xffff_ffffu64,
    ;
}

/// `p + n` does not overflow a `usize` when `n` bytes are present at `p`.
pub proof fn lemma_add_no_overflow(p: usize, n: usize)
    requires
        n <= usize::MAX - p,
    ensures
        p + n <= usize::MAX,
{
}

/// `have` and `crate::model::le16` are read only inside their guards.
pub proof fn lemma_le16_lt(b: Seq<u8>, p: int)
    requires
        p + 2 <= b.len(),
    ensures
        0 <= le16(b, p) < 65536,
{
    reveal(le16);
    assert(b[p] as int >= 0);
    assert(b[p] as int <= 255);
    assert(b[p + 1] as int >= 0);
    assert(b[p + 1] as int <= 255);
    assert(le16(b, p) <= 65535) by (nonlinear_arith)
        requires
            le16(b, p) == b[p] as int + 256 * (b[p + 1] as int),
            b[p] as int >= 0,
            b[p] as int <= 255,
            b[p + 1] as int >= 0,
            b[p + 1] as int <= 255,
    ;
    assert(le16(b, p) >= 0) by (nonlinear_arith)
        requires
            le16(b, p) == b[p] as int + 256 * (b[p + 1] as int),
            0 <= b[p] as int,
            0 <= b[p + 1] as int,
    ;
}

/// The two identifier bytes are equal as `Seq<u8>` values.
pub proof fn lemma_bytes_of_eq()
    ensures
        bytes_of(&NETSCAPE_ID) == NETSCAPE_ID@,
{
    assert(bytes_of(&NETSCAPE_ID) =~= NETSCAPE_ID@);
}

/// `have` at a `usize` position agrees with the executable's `need`.
pub proof fn lemma_have_need(b: Seq<u8>, p: int, n: int)
    requires
        0 <= p,
        0 <= n,
        p + n <= usize::MAX,
        b.len() <= usize::MAX,
    ensures
        have(b, p, n) <==> (p as usize <= b.len() && n as usize <= b.len() - (p as usize)),
{
    reveal(have);
}

// ---------------------------------------------------------------------------
// L-BITS: the payload a sub-block reader walks (T6, G10.2)
// ---------------------------------------------------------------------------

/// The concatenated payload's length, `|D|` of G9, read from the scan at the
/// size byte `q`: `concat_len` of `sub_scan`'s payload list, or `-1` when the
/// sequence cannot be read. `-1` is unreachable at a call whose `sub_scan`
/// succeeds; every lemma below guards on the `Some` case.
pub open spec fn payload_len(b: Seq<u8>, q: int) -> int {
    match sub_scan(b, q) {
        None => -1,
        Some((ps, _)) => concat_len(ps),
    }
}

/// Byte `i` (0-based) of the concatenated payload of the sub-block sequence
/// whose size byte is at `q` — the byte a reader walking the sequence fetches
/// as its `i`-th payload byte. The recursion follows `sub_scan`: within the
/// first payload it is `b[q + 1 + i]`; past it, `i` shifts by that payload's
/// length and the walk resumes at the next size byte. On a guarded read
/// (`have`) and `0 <= i < payload_len(b, q)` the walk stays in bounds; the
/// out-of-range branches return 0 and are never read.
pub open spec fn payload_byte(b: Seq<u8>, q: int, i: int) -> u8
    decreases (b.len() - q) as nat,
{
    if !have(b, q, 1) {
        0u8
    } else {
        let n = b[q] as int;
        if n == 0 {
            0u8
        } else if !have(b, q + 1, n) {
            0u8
        } else if i < n {
            b[q + 1 + i]
        } else {
            payload_byte(b, q + 1 + n, i - n)
        }
    }
}

/// `concat` of a one-element sequence is that element.
pub proof fn lemma_concat_one(p: Seq<u8>)
    ensures
        concat(seq![p]) == p,
{
    reveal(concat);
    assert(seq![p].subrange(1, 1) == Seq::<Seq<u8>>::empty()) by {
        assert(seq![p].subrange(1, 1) =~= Seq::<Seq<u8>>::empty());
    }
    assert(concat(Seq::<Seq<u8>>::empty()) == Seq::<u8>::empty()) by {
        reveal(concat);
    }
}

/// `concat` splits over `+`.
pub proof fn lemma_concat_append(s: Seq<Seq<u8>>, t: Seq<Seq<u8>>)
    ensures
        concat(s + t) == concat(s) + concat(t),
{
    lemma_concat_len(s, t);
}

/// The concat/length facts of `lemma_concat_append` at `seq![p] + t`, in one
/// place so `payload_byte`'s induction does not re-derive them.
pub proof fn lemma_concat_append_one(p: Seq<u8>, t: Seq<Seq<u8>>)
    ensures
        concat(seq![p] + t) == p + concat(t),
        concat_len(seq![p] + t) == p.len() + concat_len(t),
{
    lemma_concat_one(p);
    lemma_concat_append(seq![p], t);
    lemma_concat_len(seq![p], t);
    assert(concat(seq![p]) == p);
    assert(concat(seq![p]).len() == concat_len(seq![p]));
    // `concat_len(seq![p] + t)` unfolds to `(seq![p]+t)[0].len() + concat_len(t)`.
    reveal(concat_len);
    let u = seq![p] + t;
    assert(u.len() >= 1);
    assert(u[0] == p);
    assert(u.subrange(1, u.len() as int) == t) by {
        assert(u.subrange(1, u.len() as int) =~= t);
    }
    assert(concat_len(u) == u[0].len() + concat_len(u.subrange(1, u.len() as int)));
}

/// One payload read: within the first payload the walk reads `b[q + 1 + i]`.
pub proof fn lemma_payload_byte_head(b: Seq<u8>, q: int, i: int, n: int)
    requires
        have(b, q, 1),
        b[q] as int == n,
        1 <= n,
        have(b, q + 1, n),
        0 <= i < n,
    ensures
        payload_byte(b, q, i) == b[q + 1 + i],
{
    reveal(payload_byte);
    reveal(have);
}

/// One payload skip: past the first payload the walk resumes at the next size
/// byte with the index shifted by that payload's length.
pub proof fn lemma_payload_byte_tail(b: Seq<u8>, q: int, i: int, n: int)
    requires
        have(b, q, 1),
        b[q] as int == n,
        1 <= n,
        have(b, q + 1, n),
        n <= i,
    ensures
        payload_byte(b, q, i) == payload_byte(b, q + 1 + n, i - n),
{
    reveal(payload_byte);
    reveal(have);
}

/// L-BITS's index identity: the `i`-th payload byte of the sequence at `q` is
/// the `i`-th byte of its concatenation `D`. Proved by the induction
/// `sub_scan` itself runs, one accepted sub-block at a time.
pub proof fn lemma_payload_byte_concat(b: Seq<u8>, q: int, ps: Seq<Seq<u8>>, e: int)
    requires
        0 <= q,
        sub_scan(b, q) == Some((ps, e)),
    ensures
        forall|i: int| 0 <= i < concat_len(ps) ==> #[trigger] payload_byte(b, q, i) == concat(ps)[i],
    decreases (b.len() - q) as nat,
{
    reveal(payload_byte);
    reveal(sub_scan);
    reveal(have);
    reveal(concat);
    reveal(concat_len);
    if !have(b, q, 1) {
        assert(false);
    } else {
        let n = b[q] as int;
        if n == 0 {
            assert(ps == Seq::<Seq<u8>>::empty()) by {
                assert(sub_scan(b, q) == Some((Seq::<Seq<u8>>::empty(), q + 1)));
            }
            assert(concat_len(ps) == 0);
        } else if !have(b, q + 1, n) {
            assert(false);
        } else {
            let payload = b.subrange(q + 1, q + 1 + n);
            vstd::seq::lemma_seq_subrange_len(b, q + 1, q + 1 + n);
            assert(payload.len() == n);
            match sub_scan(b, q + 1 + n) {
                None => {
                    assert(sub_scan(b, q) == None) by {
                        assert(b[q] as int == n);
                    }
                    assert(false);
                },
                Some((rest, e2)) => {
                    assert(sub_scan(b, q) == Some((seq![payload] + rest, e2))) by {
                        assert(b[q] as int == n);
                    }
                    assert(ps == seq![payload] + rest);
                    assert(e == e2);
                    lemma_concat_append_one(payload, rest);
                    assert(concat(ps) == concat(seq![payload] + rest));
                    assert(concat_len(ps) == concat_len(seq![payload] + rest));
                    assert(concat(ps) == payload + concat(rest));
                    assert(concat_len(ps) == payload.len() + concat_len(rest));
                    lemma_payload_byte_concat(b, q + 1 + n, rest, e2);
                    assert(0 <= q + 1 + n);
                    assert forall|i: int| 0 <= i < concat_len(ps) implies #[trigger] payload_byte(
                        b,
                        q,
                        i,
                    ) == concat(ps)[i] by {
                        if i < n {
                            lemma_payload_byte_head(b, q, i, n);
                            assert(payload[i] == b[q + 1 + i]) by {
                                vstd::seq::lemma_seq_subrange_index(b, q + 1, q + 1 + n, i);
                            }
                            vstd::seq::lemma_seq_add_index1(payload, concat(rest), i);
                        } else {
                            lemma_payload_byte_tail(b, q, i, n);
                            assert(0 <= i - n < concat_len(rest));
                            assert(payload_byte(b, q + 1 + n, i - n) == concat(rest)[i - n]);
                            lemma_concat_len(rest, Seq::<Seq<u8>>::empty());
                            assert(concat(rest).len() == concat_len(rest));
                            assert(payload.len() == n);
                            assert(payload.len() <= i < payload.len() + concat(rest).len());
                            vstd::seq::lemma_seq_add_index2(payload, concat(rest), i);
                            assert(concat(rest)[i - payload.len()] == concat(rest)[i - n]);
                        }
                    }
                },
            }
        }
    }
}

// ---------------------------------------------------------------------------
// L-BITS's reader state: the executable bit reader walks the sequence one
// payload byte at a time and must know, after `i` payload bytes, where it
// stands. `blk_at` gives that as a triple.
// ---------------------------------------------------------------------------

/// The reader's position after `i` payload bytes of the sequence at `q`, as
/// `(size_off, left, byte_off)`: `size_off` is the index of the current
/// sub-block's size byte, `left` the payload bytes it still holds (`left >= 1`
/// on the reachable domain), `byte_off` the index of the `i`-th payload byte.
pub open spec fn blk_at(b: Seq<u8>, q: int, i: int) -> (int, int, int)
    decreases (b.len() - q) as nat,
{
    if !have(b, q, 1) {
        (q, 0, q)
    } else {
        let n = b[q] as int;
        if n == 0 {
            (q, 0, q + 1)
        } else if !have(b, q + 1, n) {
            (q, n, q + 1)
        } else if i < n {
            (q, n - i, q + 1 + i)
        } else {
            blk_at(b, q + 1 + n, i - n)
        }
    }
}

/// The `i`-th payload byte sits at `blk_at`'s third component, so `payload_byte`
/// is a plain read there.
pub proof fn lemma_blk_at_byte(b: Seq<u8>, q: int, i: int, ps: Seq<Seq<u8>>, e: int)
    requires
        0 <= q,
        sub_scan(b, q) == Some((ps, e)),
        0 <= i < concat_len(ps),
    ensures
        payload_byte(b, q, i) == b[blk_at(b, q, i).2],
    decreases (b.len() - q) as nat,
{
    reveal(payload_byte);
    reveal(blk_at);
    reveal(sub_scan);
    reveal(have);
    reveal(concat_len);
    if !have(b, q, 1) {
        assert(false);
    } else {
        let n = b[q] as int;
        if n == 0 {
            assert(concat_len(ps) == 0);
            assert(false);
        } else if !have(b, q + 1, n) {
            assert(false);
        } else {
            let payload = b.subrange(q + 1, q + 1 + n);
            vstd::seq::lemma_seq_subrange_len(b, q + 1, q + 1 + n);
            assert(payload.len() == n);
            match sub_scan(b, q + 1 + n) {
                None => {
                    assert(false);
                },
                Some((rest, e2)) => {
                    assert(ps == seq![payload] + rest);
                    lemma_concat_append_one(payload, rest);
                    assert(concat_len(ps) == payload.len() + concat_len(rest));
                    if i < n {
                        assert(payload_byte(b, q, i) == b[q + 1 + i]);
                        assert(blk_at(b, q, i) == (q, n - i, q + 1 + i));
                    } else {
                        let bi = blk_at(b, q + 1 + n, i - n);
                        assert(blk_at(b, q, i) == bi);
                        assert(0 <= i - n < concat_len(rest));
                        lemma_blk_at_byte(b, q + 1 + n, i - n, rest, e2);
                        assert(payload_byte(b, q + 1 + n, i - n) == b[bi.2]);
                        assert(payload_byte(b, q, i) == payload_byte(b, q + 1 + n, i - n));
                    }
                },
            }
        }
    }
}

/// One reader step: after `i` payload bytes, `blk_at(b, q, i).1` is the current
/// sub-block's remaining payload bytes, and it is at least 2 while `i + 1` is
/// still inside the same payload. Crossing a sub-block boundary the reader skips
/// the size byte at `byte_off + 1` and lands at `byte_off + 2` with `left` the
/// new size byte.
pub proof fn lemma_blk_at_step(b: Seq<u8>, q: int, i: int, ps: Seq<Seq<u8>>, e: int)
    requires
        0 <= q,
        sub_scan(b, q) == Some((ps, e)),
        0 <= i < concat_len(ps) - 1,
    ensures
        blk_at(b, q, i).1 >= 1,
        blk_at(b, q, i).1 >= 2 ==> blk_at(b, q, i + 1) == (
            blk_at(b, q, i).0,
            blk_at(b, q, i).1 - 1,
            blk_at(b, q, i).2 + 1,
        ),
        blk_at(b, q, i).1 == 1 ==> {
            &&& blk_at(b, q, i + 1).0 == blk_at(b, q, i).2 + 1
            &&& blk_at(b, q, i + 1).1 == b[blk_at(b, q, i).2 + 1] as int
            &&& blk_at(b, q, i + 1).2 == blk_at(b, q, i).2 + 2
        },
    decreases (b.len() - q) as nat,
{
    reveal(payload_byte);
    reveal(blk_at);
    reveal(sub_scan);
    reveal(have);
    reveal(concat_len);
    if !have(b, q, 1) {
        assert(false);
    } else {
        let n = b[q] as int;
        if n == 0 {
            assert(false);
        } else if !have(b, q + 1, n) {
            assert(false);
        } else {
            let payload = b.subrange(q + 1, q + 1 + n);
            vstd::seq::lemma_seq_subrange_len(b, q + 1, q + 1 + n);
            assert(payload.len() == n);
            match sub_scan(b, q + 1 + n) {
                None => {
                    assert(false);
                },
                Some((rest, e2)) => {
                    assert(ps == seq![payload] + rest);
                    lemma_concat_append_one(payload, rest);
                    assert(concat_len(ps) == payload.len() + concat_len(rest));
                    if i + 1 < n {
                        assert(blk_at(b, q, i) == (q, n - i, q + 1 + i));
                        assert(blk_at(b, q, i + 1) == (q, n - i - 1, q + 1 + i + 1));
                    } else if i + 1 == n {
                        assert(blk_at(b, q, i) == (q, n - i, q + 1 + i));
                        assert(blk_at(b, q, i).1 == 1);
                        assert(blk_at(b, q, i + 1) == blk_at(b, q + 1 + n, 0));
                        assert(blk_at(b, q, i + 1).2 == blk_at(b, q, i).2 + 2);
                    } else {
                        let bi = blk_at(b, q + 1 + n, i - n);
                        assert(blk_at(b, q, i) == bi);
                        assert(blk_at(b, q, i + 1) == blk_at(b, q + 1 + n, i - n + 1));
                        assert(0 <= i - n < concat_len(rest) - 1);
                        lemma_blk_at_step(b, q + 1 + n, i - n, rest, e2);
                        assert(bi.1 >= 1);
                    }
                },
            }
        }
    }
}

/// One step of the identifier comparison: `id_matches` at `i + 1` is
/// `id_matches` at `i` and the `i`-th byte matching. This is the loop
/// invariant `app_ext`'s comparison maintains.
pub proof fn lemma_id_matches_step(b: Seq<u8>, q: int, i: int)
    requires
        0 <= i,
        q + 1 + i < b.len(),
    ensures
        id_matches(b, q, i + 1) == (id_matches(b, q, i) && b[q + 1 + i] == NETSCAPE_ID@[i]),
{
    reveal(id_matches);
    if id_matches(b, q, i + 1) {
        assert forall|k: int| 0 <= k < i implies #[trigger] b[q + 1 + k] == NETSCAPE_ID@[k] by {};
        assert(b[q + 1 + i] == NETSCAPE_ID@[i]);
    }
    if id_matches(b, q, i) && b[q + 1 + i] == NETSCAPE_ID@[i] {
        assert forall|k: int| 0 <= k < i + 1 implies #[trigger] b[q + 1 + k] == NETSCAPE_ID@[k] by {
            if k == i {
            }
        };
    }
}


// ---------------------------------------------------------------------------
// L-BITS's bit and code view of the payload (G10), in the form the executable
// bit reader must produce.
// ---------------------------------------------------------------------------

/// G10's bit `j` of the concatenated payload: bit `j % 8` of payload byte
/// `j / 8`, least significant bit first.
pub open spec fn payload_bit(b: Seq<u8>, q: int, j: int) -> int {
    ((payload_byte(b, q, j / 8) >> (j % 8) as u8) & 1u8) as int
}

/// G10's code in `width` bits at payload bit position `pos`, least significant
/// bit first. This is `lzw_proof`'s `code_at` recursion over the sub-block
/// payload instead of a contiguous slice.
pub open spec fn payload_code(b: Seq<u8>, q: int, pos: int, width: int, i: int) -> int
    decreases width - i,
{
    if i >= width {
        0
    } else {
        payload_bit(b, q, pos + i) + 2 * payload_code(b, q, pos, width, i + 1)
    }
}

/// G10's `bit_at` over a contiguous `D` (`lzw_proof`'s `model::bit_at`).
pub open spec fn concat_bit(d: Seq<u8>, j: int) -> int {
    ((d[j / 8] >> (j % 8) as u8) & 1u8) as int
}

/// G10's `code_at` over a contiguous `D` (`lzw_proof`'s `model::code_at`).
pub open spec fn concat_code(d: Seq<u8>, pos: int, width: int, i: int) -> int
    decreases width - i,
{
    if i >= width {
        0
    } else {
        concat_bit(d, pos + i) + 2 * concat_code(d, pos, width, i + 1)
    }
}

/// L-BITS's bit identity: bit `j` of the sub-block payload equals bit `j` of
/// the concatenation `D`. Immediate from `lemma_payload_byte_concat`.
pub proof fn lemma_payload_bit_concat(b: Seq<u8>, q: int, ps: Seq<Seq<u8>>, e: int, j: int)
    requires
        0 <= q,
        sub_scan(b, q) == Some((ps, e)),
        0 <= j < 8 * concat_len(ps),
    ensures
        payload_bit(b, q, j) == concat_bit(concat(ps), j),
{
    reveal(payload_bit);
    reveal(concat_bit);
    lemma_concat_len_nonneg(ps);
    assert(0 <= j / 8) by (nonlinear_arith)
        requires
            0 <= j,
    ;
    assert(j / 8 < concat_len(ps)) by (nonlinear_arith)
        requires
            j < 8 * concat_len(ps),
    ;
    lemma_payload_byte_concat(b, q, ps, e);
    assert(payload_byte(b, q, j / 8) == concat(ps)[j / 8]);
}

/// L-BITS's code identity: reading `width` bits at payload position `pos`
/// equals `code_at` reading them from the concatenation `D`. Stated at the
/// recursion cursor `i` so the induction is structural.
pub proof fn lemma_payload_code_concat(
    b: Seq<u8>,
    q: int,
    ps: Seq<Seq<u8>>,
    e: int,
    pos: int,
    width: int,
    i: int,
)
    requires
        0 <= q,
        0 <= pos,
        0 <= i <= width,
        sub_scan(b, q) == Some((ps, e)),
        pos + width <= 8 * concat_len(ps),
    ensures
        payload_code(b, q, pos, width, i) == concat_code(concat(ps), pos, width, i),
    decreases width - i,
{
    reveal(payload_code);
    reveal(concat_code);
    if i >= width {
        assert(payload_code(b, q, pos, width, i) == 0);
        assert(concat_code(concat(ps), pos, width, i) == 0);
    } else {
        lemma_payload_bit_concat(b, q, ps, e, pos + i);
        lemma_payload_code_concat(b, q, ps, e, pos, width, i + 1);
    }
}



/// L-BITS at a code read (T6's statement in the form `lzw::decode` cites it):
/// the `width`-bit code at reader bit position `pos` over the sub-block payload
/// of the sequence at `q` equals the code G10's model reads from the
/// concatenated payload `D` at the same position. Together with
/// `BitReader::read`'s postcondition — it returns `payload_code` at the
/// position before the read — this makes every code the executable reads the
/// code `ML`'s `code_at(D, pos, width, 0)` computes.
pub proof fn lemma_lbits_read(b: Seq<u8>, q: int, ps: Seq<Seq<u8>>, e: int, pos: int, width: int)
    requires
        0 <= q,
        sub_scan(b, q) == Some((ps, e)),
        0 <= pos,
        0 <= width,
        pos + width <= 8 * concat_len(ps),
    ensures
        payload_code(b, q, pos, width, 0) == concat_code(concat(ps), pos, width, 0),
{
    lemma_payload_code_concat(b, q, ps, e, pos, width, 0);
}

// ---------------------------------------------------------------------------
// L-BITS's executable reader state. `lzw::BitReader` defers a sub-block's size
// byte until it needs that block's first payload byte, so between blocks it
// sits at a size byte with `left_in_block == 0`. `rd_state` mirrors that.
// ---------------------------------------------------------------------------

/// The executable reader's `(pos, left_in_block)` after `k` payload bytes, on
/// the sequence whose first size byte is at `q`. `pos` is the next byte
/// `fetch_byte` dereferences; `left_in_block == 0` means the next fetch reads a
/// size byte first.
pub open spec fn rd_state(b: Seq<u8>, q: int, k: int) -> (int, int)
    decreases k,
{
    if k <= 0 {
        (q, 0)
    } else {
        let (p, l) = rd_state(b, q, k - 1);
        if l == 0 {
            let s = if have(b, p, 1) {
                b[p] as int
            } else {
                0
            };
            (p + 2, if s > 0 {
                s - 1
            } else {
                0
            })
        } else {
            (p + 1, l - 1)
        }
    }
}

/// The byte index the `k`-th payload byte (`k >= 0`) is read from: the byte
/// `fetch_byte` returns on that call.
pub open spec fn rd_next_off(b: Seq<u8>, q: int, k: int) -> int {
    let (p, l) = rd_state(b, q, k);
    if l == 0 {
        p + 1
    } else {
        p
    }
}

/// Both fields of `rd_state` are non-negative.
pub proof fn lemma_rd_nonneg(b: Seq<u8>, q: int, k: int)
    requires
        0 <= q,
        0 <= k,
    ensures
        rd_state(b, q, k).0 >= q,
        rd_state(b, q, k).1 >= 0,
    decreases k,
{
    reveal(rd_state);
    if k > 0 {
        lemma_rd_nonneg(b, q, k - 1);
    }
}

/// The reader and `blk_at` agree at a payload index: inside a block they name
/// the same next byte and hold the same remaining count; at a boundary the
/// reader's next byte is `blk_at`'s current payload start, whose size is
/// `blk_at`'s count.
pub open spec fn rd_rel(b: Seq<u8>, q: int, k: int) -> bool {
    let (p, l) = rd_state(b, q, k);
    let (so, sl, bo) = blk_at(b, q, k);
    &&& (l == 0 ==> p + 1 == bo && sl == b[p] as int)
    &&& (l >= 1 ==> p == bo && l == sl)
}

/// The reader walk agrees with `blk_at` at every payload byte before the scan's
/// end: with `k` payload bytes fetched, either the reader sits inside a block
/// (same cursor and count as `blk_at`) or at a block boundary (its next size
/// byte is `blk_at`'s current payload start, whose size is `blk_at`'s count).
/// Hence `rd_next_off` and `blk_at`'s third component name the same byte.
pub proof fn lemma_rd_rel(b: Seq<u8>, q: int, k: int, ps: Seq<Seq<u8>>, e: int)
    requires
        0 <= q,
        sub_scan(b, q) == Some((ps, e)),
        0 <= k <= concat_len(ps) - 1,
        forall|j: int| 0 <= j <= k ==> have(b, rd_state(b, q, j).0, 1),
        forall|j: int| 0 <= j <= k ==> rd_state(b, q, j).1 == 0 ==> b[rd_state(b, q, j).0] as int >= 1
            && have(b, rd_state(b, q, j).0 + 1, b[rd_state(b, q, j).0] as int),
    ensures
        rd_rel(b, q, k),
        rd_next_off(b, q, k) == blk_at(b, q, k).2,
    decreases k,
{
    reveal(rd_state);
    reveal(rd_next_off);
    reveal(blk_at);
    reveal(rd_rel);
    reveal(have);
    reveal(sub_scan);
    reveal(concat_len);
    if k == 0 {
        let n = b[q] as int;
        assert(concat_len(ps) >= 1);
        assert(n >= 1);
        assert(ps.len() >= 1);
        assert(blk_at(b, q, 0) == (q, n, q + 1));
        assert(rd_state(b, q, 0) == (q, 0));
        assert(rd_rel(b, q, 0));
        assert(rd_next_off(b, q, 0) == q + 1);
    } else {
        lemma_rd_rel(b, q, k - 1, ps, e);
        let (p, l) = rd_state(b, q, k - 1);
        let (so, sl, bo) = blk_at(b, q, k - 1);
        assert(rd_rel(b, q, k - 1));
        lemma_rd_nonneg(b, q, k - 1);
        // One `fetch_byte` step.
        assert(rd_state(b, q, k) == (if l == 0 {
            (p + 2, if b[p] as int > 0 {
                b[p] as int - 1
            } else {
                0
            })
        } else {
            (p + 1, l - 1)
        })) by {
            reveal(rd_state);
        }
        // One `blk_at` step.
        assert(k - 1 < concat_len(ps) - 1);
        lemma_blk_at_step(b, q, k - 1, ps, e);
        assert(blk_at(b, q, k - 1).1 >= 1);
        if blk_at(b, q, k - 1).1 >= 2 {
            assert(blk_at(b, q, k) == (so, sl - 1, bo + 1));
        } else {
            assert(blk_at(b, q, k) == (bo + 1, b[bo + 1] as int, bo + 2));
        }
        assert(rd_rel(b, q, k));
        // The four cases of the reader step against the four of `blk_at`.
        if l == 0 {
            // At a boundary: `rd_rel(k-1)` gives `p + 1 == bo` and `sl == b[p]`.
            assert(blk_at(b, q, k - 1).2 == p + 1);
            assert(blk_at(b, q, k - 1).1 == b[p] as int);
            if b[p] as int == 1 {
                assert(rd_state(b, q, k) == (p + 2, 0));
                assert(rd_next_off(b, q, k) == p + 3);
                assert(blk_at(b, q, k) == (p + 2, b[p + 2] as int, p + 3));
            } else {
                assert(b[p] as int >= 2);
                assert(rd_state(b, q, k) == (p + 2, b[p] as int - 1));
                assert(rd_next_off(b, q, k) == p + 2);
                assert(blk_at(b, q, k) == (blk_at(b, q, k - 1).0, b[p] as int - 1, p + 2));
            }
        } else if l == 1 {
            assert(rd_state(b, q, k) == (p + 1, 0));
            assert(rd_next_off(b, q, k) == p + 2);
            assert(blk_at(b, q, k - 1).1 == 1);
            assert(blk_at(b, q, k - 1).2 == p);
            assert(blk_at(b, q, k) == (p + 1, b[p + 1] as int, p + 2));
        } else {
            assert(l > 1);
            assert(rd_state(b, q, k) == (p + 1, l - 1));
            assert(rd_next_off(b, q, k) == p + 1);
            assert(blk_at(b, q, k - 1).1 == l);
            assert(blk_at(b, q, k - 1).1 > 1);
            assert(blk_at(b, q, k - 1).2 == p);
            assert(blk_at(b, q, k).2 == p + 1);
        }
        assert(rd_next_off(b, q, k) == blk_at(b, q, k).2);
    }
}


// ---------------------------------------------------------------------------
// L-BITS's reader step condition (`rd_step_ok`): the facts `fetch_byte` needs
// at each call to return `payload_byte` and step the reader state.
// ---------------------------------------------------------------------------

/// The reader step is well defined at `k` payload bytes fetched: the byte
/// `fetch_byte` dereferences is present, that byte is `payload_byte(b, q, k)`,
/// and, when the reader is at a sub-block boundary (`left_in_block == 0`), the
/// size byte it reads there is positive with its payload present.
pub open spec fn rd_step_ok(b: Seq<u8>, q: int, k: int) -> bool {
    let (p, l) = rd_state(b, q, k);
    &&& have(b, p, 1)
    &&& have(b, rd_next_off(b, q, k), 1)
    &&& payload_byte(b, q, k) == b[rd_next_off(b, q, k)]
    &&& (l == 0 ==> b[p] as int >= 1 && have(b, p + 1, b[p] as int))
}

/// Within one sub-block: after `j` payload bytes of a block whose size byte is
/// at `q` (`b[q] == n`, `1 <= n <= |b| - q - 1`), the reader holds the next byte
/// at `q + 1 + j` with `n - j` bytes left in that block.
pub proof fn lemma_rd_state_within_first(b: Seq<u8>, q: int, j: int, n: int)
    requires
        have(b, q, 1),
        b[q] as int == n,
        1 <= n,
        have(b, q + 1, n),
        1 <= j <= n,
    ensures
        rd_state(b, q, j) == (q + 1 + j, n - j),
    decreases j,
{
    reveal(rd_state);
    reveal(have);
    if j == 1 {
        assert(rd_state(b, q, 0) == (q, 0));
        assert(rd_state(b, q, 1) == (q + 2, n - 1));
    } else {
        lemma_rd_state_within_first(b, q, j - 1, n);
        assert(rd_state(b, q, j - 1) == (q + j, n - (j - 1)));
        assert(n - (j - 1) >= 1);
        assert(rd_state(b, q, j) == (q + j + 1, n - j));
    }
}

/// Crossing the sub-block boundary: the reader's state after `n` payload bytes
/// of the block at `q` equals its state after `k` payload bytes of the sequence
/// whose size byte follows that payload (`q + 1 + n`) — the deferral walks on.
pub proof fn lemma_rd_state_shift(b: Seq<u8>, q: int, k: int, n: int)
    requires
        have(b, q, 1),
        b[q] as int == n,
        1 <= n,
        have(b, q + 1, n),
        0 <= k,
    ensures
        rd_state(b, q + 1 + n, k) == rd_state(b, q, n + k),
    decreases k,
{
    reveal(rd_state);
    reveal(have);
    lemma_rd_state_within_first(b, q, n, n);
    assert(rd_state(b, q, n) == (q + 1 + n, 0));
    assert(rd_state(b, q + 1 + n, 0) == (q + 1 + n, 0));
    if k == 0 {
    } else {
        lemma_rd_state_shift(b, q, k - 1, n);
        assert(rd_state(b, q + 1 + n, k - 1) == rd_state(b, q, n + k - 1));
        assert(rd_state(b, q + 1 + n, k) == rd_state(b, q, n + k));
    }
}

/// L-BITS's step condition holds at every payload index below `|D|`: the
/// structural induction over the sub-block sequence (the recursion `sub_scan`
/// and `payload_byte` run), so `fetch_byte` may be called at any index it
/// reaches. This is the hypothesis the executable reader's contracts carry.
pub proof fn lemma_rd_step_ok_all(b: Seq<u8>, q: int, ps: Seq<Seq<u8>>, e: int)
    requires
        0 <= q,
        sub_scan(b, q) == Some((ps, e)),
    ensures
        forall|j: int| 0 <= j < concat_len(ps) ==> #[trigger] rd_step_ok(b, q, j),
    decreases (b.len() - q) as nat,
{
    reveal(rd_step_ok);
    reveal(rd_state);
    reveal(rd_next_off);
    reveal(payload_byte);
    reveal(sub_scan);
    reveal(have);
    reveal(concat_len);
    if !have(b, q, 1) {
        assert(false);
    } else {
        let n = b[q] as int;
        if n == 0 {
            assert(concat_len(ps) == 0);
        } else if !have(b, q + 1, n) {
            assert(false);
        } else {
            let payload = b.subrange(q + 1, q + 1 + n);
            vstd::seq::lemma_seq_subrange_len(b, q + 1, q + 1 + n);
            assert(payload.len() == n);
            match sub_scan(b, q + 1 + n) {
                None => {
                    assert(false);
                },
                Some((rest, e2)) => {
                    assert(ps == seq![payload] + rest);
                    lemma_concat_append_one(payload, rest);
                    assert(concat_len(ps) == n + concat_len(rest));
                    lemma_rd_step_ok_all(b, q + 1 + n, rest, e2);
                    assert forall|j: int| 0 <= j < concat_len(ps) implies #[trigger] rd_step_ok(
                        b,
                        q,
                        j,
                    ) by {
                        if j == 0 {
                            assert(rd_state(b, q, 0) == (q, 0));
                            assert(rd_next_off(b, q, 0) == q + 1);
                            lemma_payload_byte_head(b, q, 0, n);
                            assert(payload_byte(b, q, 0) == b[q + 1]);
                        } else if j < n {
                            lemma_rd_state_within_first(b, q, j, n);
                            lemma_payload_byte_head(b, q, j, n);
                            assert(rd_state(b, q, j) == (q + 1 + j, n - j));
                            assert(rd_next_off(b, q, j) == q + 1 + j);
                            assert(payload_byte(b, q, j) == b[q + 1 + j]);
                        } else {
                            lemma_rd_state_shift(b, q, j - n, n);
                            lemma_payload_byte_tail(b, q, j, n);
                            assert(0 <= j - n < concat_len(rest));
                            assert(rd_step_ok(b, q + 1 + n, j - n));
                            assert(rd_state(b, q, j) == rd_state(b, q + 1 + n, j - n));
                            assert(rd_next_off(b, q, j) == rd_next_off(b, q + 1 + n, j - n));
                            assert(payload_byte(b, q, j) == payload_byte(b, q + 1 + n, j - n));
                        }
                    }
                },
            }
        }
    }
}

// ---------------------------------------------------------------------------
// L-BITS's accumulator view: what the executable `BitReader.acc`/`nacc` mean.
// ---------------------------------------------------------------------------

/// Low bit `j` of a `u32`, on `u32` so `bit_vector` can see it.
pub open spec fn bit32(x: u32, j: u32) -> u32 {
    (x >> j) & 1u32
}

/// The value of bits `[i, width)` of `x`, lowest first — the same recursion
/// `payload_code` runs, so it is `payload_code` up to the bit equality.
/// The low-`n` bit mask as a `u32` value (the `u32` parameter keeps it a
/// finite-width expression for `bit_vector`).
pub open spec fn mask32(n: u32) -> u32 {
    ((1u32 << n) - 1u32) as u32
}

/// Low bit `j` of `x`; the `u32` cast keeps the shift inside `bit_vector`.
pub open spec fn bit32i(x: u32, j: int) -> int {
    ((x >> (j as u32)) & 1u32) as int
}

/// The value of bits `[i, width)` of `x` (ints, like `payload_code`).
pub open spec fn low_bits_val(x: u32, width: int, i: int) -> int
    decreases width - i,
{
    if i >= width {
        0
    } else {
        bit32i(x, i) + 2 * low_bits_val(x, width, i + 1)
    }
}

/// The masked value equals the bit recursion: the crux identity for
/// `BitReader::read`. Induction on the remaining width, one `bit_vector` step
/// with the `u32` quantities bound as locals.
pub proof fn lemma_mask_eq_low_bits(x: u32, width: int, i: int)
    requires
        0 <= i <= width <= 32,
    ensures
        ((x >> (i as u32)) & mask32((width - i) as u32)) as int == low_bits_val(x, width, i),
    decreases width - i,
{
    reveal(low_bits_val);
    reveal(bit32i);
    reveal(mask32);
    if i >= width {
        let iu: u32 = i as u32;
        assert((width - i) as u32 == 0u32);
        assert(mask32(0u32) == 0u32) by (bit_vector);
        assert(((x >> iu) & mask32(0u32)) as int == 0) by (bit_vector)
            requires
                mask32(0u32) == 0u32,
        ;
    } else {
        let iu: u32 = i as u32;
        let nu: u32 = (width - i) as u32;
        let lo: u32 = (x >> iu) & 1u32;
        let hi: u32 = (x >> (iu + 1u32)) & mask32((nu - 1) as u32);
        assert(hi as int == low_bits_val(x, width, i + 1)) by {
            assert(((x >> ((i + 1) as u32)) & mask32((width - i - 1) as u32)) as int
                == low_bits_val(x, width, i + 1)) by {
                lemma_mask_eq_low_bits(x, width, i + 1);
            }
            assert((i + 1) as u32 == iu + 1u32);
            assert((width - i - 1) as u32 == nu - 1u32);
        }
        assert(((x >> iu) & mask32(nu)) as int == lo as int + 2 * (hi as int)) by (bit_vector)
            requires
                iu < 32,
                nu >= 1,
                nu <= 32,
                lo == ((x >> iu) & 1u32),
                hi == ((x >> (iu + 1u32)) & mask32((nu - 1) as u32)),
                mask32(nu) == (((1u32 << nu) - 1u32) as u32),
                mask32((nu - 1) as u32) == (((1u32 << ((nu - 1) as u32)) - 1u32) as u32),
        ;
        assert(lo as int == bit32i(x, i));
    }
}

/// The accumulated read: with the accumulator's low `nacc` bits holding the
/// payload bits at `[8·fetched − nacc, 8·fetched)`, `read(width)` returns
/// `payload_code` at the reader's position and the shifted accumulator holds
/// the payload bits from the new position.
pub open spec fn acc_view(b: Seq<u8>, q: int, fetched: int, nacc: int, acc: u32) -> bool {
    &&& 0 <= nacc <= 32
    &&& (acc >> (nacc as u32)) == 0u32
    &&& forall|j: int| 0 <= j < nacc ==> #[trigger] bit32(acc, j as u32) as int == payload_bit(
        b,
        q,
        8 * fetched - nacc + j,
    )
}


/// `low_bits_val` is `payload_code` when the two agree bit by bit: one
/// induction, since both are the same recursion.
pub proof fn lemma_low_bits_val_is_payload_code(
    b: Seq<u8>,
    q: int,
    pos: int,
    width: int,
    i: int,
    x: u32,
)
    requires
        0 <= i <= width,
        forall|j: int| i <= j < width ==> #[trigger] bit32i(x, j) == payload_bit(b, q, pos + j),
    ensures
        low_bits_val(x, width, i) == payload_code(b, q, pos, width, i),
    decreases width - i,
{
    reveal(low_bits_val);
    reveal(payload_code);
    reveal(bit32i);
    if i >= width {
    } else {
        lemma_low_bits_val_is_payload_code(b, q, pos, width, i + 1, x);
        assert(bit32i(x, i) == payload_bit(b, q, pos + i));
    }
}

/// Step 1 of L-BITS's executable half: `acc | (byte << nacc)` extends
/// `acc_view` by eight bits, when `byte` is the next payload byte.
pub proof fn lemma_acc_push(b: Seq<u8>, q: int, fetched: int, nacc: int, acc: u32, byte: u8)
    requires
        acc_view(b, q, fetched, nacc, acc),
        nacc + 8 <= 32,
        byte as int == payload_byte(b, q, fetched),
    ensures
        acc_view(b, q, fetched + 1, nacc + 8, acc | ((byte as u32) << (nacc as u32))),
{
    reveal(acc_view);
    reveal(bit32);
    let nu: u32 = nacc as u32;
    let acc2: u32 = acc | ((byte as u32) << nu);
    assert(acc2 >> (nu + 8u32) == 0u32) by (bit_vector)
        requires
            acc2 == (acc | ((byte as u32) << nu)),
            acc >> nu == 0u32,
            nu + 8u32 <= 32u32,
    ;
    assert forall|j: int| 0 <= j < nacc + 8 implies #[trigger] bit32(acc2, j as u32) as int
        == payload_bit(b, q, 8 * (fetched + 1) - (nacc + 8) + j) by {
        let ju: u32 = j as u32;
        if j < nacc {
            assert(bit32(acc2, ju) == bit32(acc, ju)) by (bit_vector)
                requires
                    acc2 == (acc | ((byte as u32) << nu)),
                    ju < nu,
                    nu <= 24,
            ;
            assert(bit32(acc, j as u32) as int == payload_bit(b, q, 8 * fetched - nacc + j));
            assert(8 * (fetched + 1) - (nacc + 8) + j == 8 * fetched - nacc + j);
        } else {
            let du: u32 = (j - nacc) as u32;
            assert(du == (ju - nu) as u32);
            assert(ju == (nu + du) as u32);
            assert(bit32(acc, ju) == 0u32) by (bit_vector)
                requires
                    ju >= nu,
                    acc >> nu == 0u32,
                    nu <= 24,
            ;
            assert(bit32(acc2, ju) == bit32(byte as u32, du)) by (bit_vector)
                requires
                    acc2 == (acc | ((byte as u32) << nu)),
                    ju == (nu + du) as u32,
                    du < 8u32,
                    nu <= 24,
                    bit32(acc, ju) == 0u32,
            ;
            assert(((byte >> du) & 1u8) == ((byte >> (du & 7u32)) & 1u8)) by (bit_vector)
                requires
                    du < 8u32,
            ;
            assert(bit32(byte as u32, du) == (byte >> du) & 1u8);
            reveal(payload_bit);
            assert((8 * fetched + (j - nacc)) / 8 == fetched);
            assert((8 * fetched + (j - nacc)) % 8 == j - nacc);
            assert(payload_byte(b, q, fetched) == byte);
            assert(bit32(byte as u32, du) as int == payload_bit(b, q, 8 * fetched + (j - nacc)));
            assert(8 * (fetched + 1) - (nacc + 8) + j == 8 * fetched + (j - nacc));
        }
    }
}


/// Step 2 of L-BITS's executable half: reading `width` bits at the reader's
/// position. `acc & mask32(width)` is `payload_code` at bit position
/// `8·fetched − nacc`, and the shifted accumulator keeps the view at the next
/// position (`fetched` unchanged, `nacc` reduced by `width`).
pub proof fn lemma_acc_take(b: Seq<u8>, q: int, fetched: int, nacc: int, acc: u32, width: int)
    requires
        acc_view(b, q, fetched, nacc, acc),
        1 <= width <= 12,
        width <= nacc,
    ensures
        (acc & mask32(width as u32)) as int == payload_code(b, q, 8 * fetched - nacc, width, 0),
        acc_view(b, q, fetched, nacc - width, acc >> (width as u32)),
{
    reveal(acc_view);
    reveal(bit32);
    reveal(bit32i);
    let wu: u32 = (width - 0) as u32;
    let nu: u32 = nacc as u32;
    let du: u32 = (nacc - width) as u32;
    assert(du == nu - wu);
    assert(acc >> (nacc as u32) == 0u32);
    assert((acc >> wu) >> du == 0u32) by (bit_vector)
        requires
            du == nu - wu,
            acc >> nu == 0u32,
            wu <= nu,
            nu <= 32,
    ;
    lemma_mask_eq_low_bits(acc, width, 0);
    assert((acc >> (0 as u32)) == acc) by (bit_vector);
    assert((acc & mask32(wu)) as int == low_bits_val(acc, width, 0)) by {
        assert(((acc >> (0 as u32)) & mask32((width - 0) as u32)) as int == low_bits_val(
            acc,
            width,
            0,
        ));
        assert((width - 0) as u32 == wu);
    }
    assert forall|j: int| 0 <= j < width implies #[trigger] bit32i(acc, j) == payload_bit(
        b,
        q,
        8 * fetched - nacc + j,
    ) by {
        assert(bit32i(acc, j) == bit32(acc, j as u32) as int);
    }
    lemma_low_bits_val_is_payload_code(b, q, 8 * fetched - nacc, width, 0, acc);
    assert forall|j: int| 0 <= j < nacc - width implies #[trigger] bit32(
        acc >> wu,
        j as u32,
    ) as int == payload_bit(b, q, 8 * fetched - (nacc - width) + j) by {
        let ju: u32 = j as u32;
        let su: u32 = (ju + wu) as u32;
        assert(ju as int == j);
        assert((su as int) == (ju as int) + (wu as int));
        assert((su as int) < 32);
        assert(bit32(acc >> wu, ju) == bit32(acc, su)) by (bit_vector)
            requires
                su == ju + wu,
                su < 32u32,
        ;
        assert(bit32(acc, su) as int == payload_bit(b, q, 8 * fetched - nacc + (j + width)));
        assert(8 * fetched - (nacc - width) + j == 8 * fetched - nacc + (j + width));
    }
}

} // verus!
