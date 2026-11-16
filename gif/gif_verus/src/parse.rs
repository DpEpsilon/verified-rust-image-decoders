//! G2–G9: header, logical screen descriptor, colour tables, the block walk's
//! extensions (G6/G7), image descriptors (G8) and the structural scan of the
//! image data (G9). Nothing here allocates; colour tables are referenced by
//! their offset in the input.
//!
//! Every read goes through `get`/`le16`, which are bounds-checked by the
//! preceding "require n bytes" test (`need`); positions are `usize` sums of
//! values below `MAX_INPUT + 2^16` and are compared, never wrapped.
//!
//! The header path (`ColorTable`, `Header`, `need`/`get`/`le16`,
//! `parse_header`) is inside `verus!`: `parse_header` carries T2's contract,
//! equality with the model `MH` on the whole result and the first error. The
//! block-walk functions below it are still plain Rust (milestone M3 proves
//! them next).

use crate::compose::Rect;
use crate::gif::{Error, Version};
use crate::limits::{MAX_DIMENSION, MAX_FRAMES, MAX_INPUT, MAX_OUTPUT, MAX_TOTAL_PIXELS};
use crate::policy::{
    DisposalUndefined, ExtSizes, FrameBounds, GceConsumers, GceDuplicate, LoopScan, LzwMin, Policy,
};

use vstd::prelude::*;

verus! {

/// A colour table inside the input: `entries` RGB triples at `offset`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorTable {
    pub offset: usize,
    pub entries: u32,
}

/// G2–G4 result.
#[derive(Clone, Copy, Debug)]
pub struct Header {
    pub width: u16,
    pub height: u16,
    pub version: Version,
    pub background_index: u8,
    pub aspect: u8,
    pub global: Option<ColorTable>,
    /// Cursor after the global colour table.
    pub cursor: usize,
}

/// `hdr_matches(b, m, h)`: the executable `Header` record `h` is exactly the
/// model record `m` of `MH(b)`, field for field.
pub open spec fn hdr_matches(b: Seq<u8>, m: crate::model::HeaderM, h: Header) -> bool {
    &&& h.width as int == m.width
    &&& h.height as int == m.height
    &&& crate::model::version_code(h.version) == m.version
    &&& h.background_index as int == m.background_index
    &&& h.aspect as int == m.aspect
    &&& match (m.global, h.global) {
        (None, None) => true,
        (Some(t), Some(c)) => c.entries as int == t.entries && c.offset as int == t.off,
        _ => false,
    }
    &&& h.cursor as int == m.cursor
}

/// `hdr_ok(b, h)`: `h` is the executable record of some successful `MH(b)` —
/// the property `parse_header` establishes, and the premise the reservation
/// prefix needs to relate its `4·W·H` to `Req_new`.
pub open spec fn hdr_ok(b: Seq<u8>, h: Header) -> bool {
    exists|m: crate::model::HeaderM| #[trigger] crate::model::mh(b) == Ok(m) && hdr_matches(b, m, h)
}

/// `hdr_inv(b, h)`: the executable bounds T2 asserts for a successful header,
/// derived from the model and proved from checked values (nothing is assumed).
pub open spec fn hdr_inv(b: Seq<u8>, h: Header) -> bool {
    &&& 1 <= h.width as int <= crate::model::max_dimension()
    &&& 1 <= h.height as int <= crate::model::max_dimension()
    &&& 4 * (h.width as int) * (h.height as int) <= crate::model::max_output()
    &&& match h.global {
        None => h.cursor == 13,
        Some(c) => 2 <= c.entries <= 256 && c.offset == 13 && h.cursor == 13 + 3 * c.entries as int,
    }
    &&& h.cursor <= b.len()
}

/// "Require `n` bytes at `p`": `p + n <= |b|`, compared without overflow.
pub fn need(b: &[u8], p: usize, n: usize) -> (r: bool)
    ensures
        r == crate::model::have(b@, p as int, n as int),
{
    let r = p <= b.len() && n <= b.len() - p;
    proof {
        reveal(crate::model::have);
    }
    r
}

/// `b[i]`, or 0 past the end (every call is preceded by `need`).
pub fn get(b: &[u8], i: usize) -> (r: u8)
    ensures
        i < b@.len() ==> r == b@[i as int],
{
    match b.get(i) {
        Some(v) => *v,
        None => 0,
    }
}

/// LE16 at `i` (every call is preceded by `need(b, i, 2)`).
pub fn le16(b: &[u8], i: usize) -> (r: u16)
    requires
        i + 2 <= b@.len(),
    ensures
        r as int == crate::model::le16(b@, i as int),
{
    let lo = b[i];
    let hi = b[i + 1];
    let r = (lo as u16) | ((hi as u16) << 8);
    proof {
        reveal(crate::model::le16);
        assert(lo == b@[i as int]);
        assert(hi == b@[i as int + 1]);
        assert(r as int == lo as int + 256 * (hi as int)) by (bit_vector)
            requires
                r == ((lo as u16) | ((hi as u16) << 8)),
        ;
        assert(crate::model::le16(b@, i as int) == lo as int + 256 * (hi as int));
    }
    r
}

/// Entries of a colour table with size field `f` (`F & 7`): `2^(f+1)`.
fn table_entries(packed: u8) -> (r: u32)
    ensures
        r as int == crate::model::table_entries(packed),
        2 <= r <= 256,
{
    let f = packed & 7u8;
    proof {
        assert(f == packed & 7u8);
    }
    assert(f <= 7u8) by (bit_vector)
        requires
            f == packed & 7u8,
    ;
    let k = f as u32;
    let r = 1u32 << (k + 1);
    proof {
        reveal(crate::model::table_entries);
        assert(k <= 7);
        assert(k + 1 <= 8);
        crate::model::lemma_shift_pow2(k as int + 1);
        assert(r as int == crate::model::pow2(k as int + 1));
        assert(r as int == crate::model::table_entries(packed));
        crate::model::lemma_table_entries_range(packed);
    }
    r
}

/// G2 header, G3 logical screen descriptor, G4 global colour table (T2): the
/// result equals `MH(b)` as the complete record or as the exact first error,
/// and a success satisfies `hdr_inv`.
pub fn parse_header(b: &[u8]) -> (r: Result<Header, Error>)
    ensures
        match crate::model::mh(b@) {
            Ok(m) => match r {
                Ok(h) => hdr_matches(b@, m, h),
                Err(_) => false,
            },
            Err(e2) => match r {
                Ok(_) => false,
                Err(e) => e == e2,
            },
        },
        match r {
            Ok(h) => hdr_inv(b@, h),
            Err(_) => true,
        },
{
    if b.len() > MAX_INPUT {
        proof {
            reveal(crate::model::mh);
            reveal(crate::model::max_input);
        }
        return Err(Error::FileTooLarge);
    }
    proof {
        reveal(crate::model::mh);
        reveal(crate::model::max_input);
        assert(b@.len() <= crate::model::max_input());
    }
    if !need(b, 0, 13) {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::TruncatedHeader);
    }
    proof {
        reveal(crate::model::mh);
        assert(crate::model::have(b@, 0, 13));
        assert(b@.len() >= 13);
    }
    let c0 = get(b, 0);
    let c1 = get(b, 1);
    let c2 = get(b, 2);
    proof {
        assert(c0 == b@[0]);
        assert(c1 == b@[1]);
        assert(c2 == b@[2]);
    }
    if !(c0 == b'G' && c1 == b'I' && c2 == b'F') {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::BadSignature);
    }
    proof {
        reveal(crate::model::mh);
        assert(b@[0] == 0x47u8 && b@[1] == 0x49u8 && b@[2] == 0x46u8);
    }
    let v3 = get(b, 3);
    let v4 = get(b, 4);
    let v5 = get(b, 5);
    proof {
        assert(v3 == b@[3]);
        assert(v4 == b@[4]);
        assert(v5 == b@[5]);
    }
    let version;
    if v3 == b'8' && v4 == b'7' && v5 == b'a' {
        version = Version::V87a;
    } else if v3 == b'8' && v4 == b'9' && v5 == b'a' {
        version = Version::V89a;
    } else {
        proof {
            reveal(crate::model::mh);
            reveal(crate::model::version_of);
            reveal(crate::model::version_code);
        }
        return Err(Error::UnsupportedVersion);
    }
    proof {
        reveal(crate::model::mh);
        reveal(crate::model::version_of);
        reveal(crate::model::version_code);
        if version == Version::V87a {
            assert(b@[3] == 0x38u8 && b@[4] == 0x37u8 && b@[5] == 0x61u8);
            assert(crate::model::version_of(b@) == Some(Version::V87a));
        } else {
            assert(b@[3] == 0x38u8 && b@[4] == 0x39u8 && b@[5] == 0x61u8);
            assert(crate::model::version_of(b@) == Some(Version::V89a));
        }
        assert(crate::model::version_code(version) == crate::model::version_code(
            crate::model::version_of(b@)->Some_0,
        ));
    }
    let w16 = le16(b, 6);
    let h16 = le16(b, 8);
    let w = w16 as u32;
    let h = h16 as u32;
    let packed = get(b, 10);
    proof {
        reveal(crate::model::mh);
        reveal(crate::model::le16);
        crate::model::lemma_le16_lt(b@, 6);
        crate::model::lemma_le16_lt(b@, 8);
        assert(w as int == crate::model::le16(b@, 6));
        assert(h as int == crate::model::le16(b@, 8));
        assert(w <= 65535);
        assert(h <= 65535);
    }
    if !(1 <= w && w <= MAX_DIMENSION && 1 <= h && h <= MAX_DIMENSION) {
        proof {
            reveal(crate::model::mh);
            reveal(crate::model::max_dimension);
        }
        return Err(Error::InvalidDimensions);
    }
    proof {
        reveal(crate::model::mh);
        reveal(crate::model::max_dimension);
    }
    proof {
        crate::model::lemma_area_cap(w, h);
    }
    // `w*h` fits `u32` because both factors are at most 65,535 (at most
    // 4,294,836,225 < 2^32), so this comparison cannot wrap; the comparison
    // `w*h > MAX_OUTPUT/4` is the model's `4·w·h > MAX_OUTPUT` (G1, G3).
    let wh = w * h;
    if (wh as u64) > MAX_OUTPUT / 4 {
        proof {
            reveal(crate::model::mh);
            reveal(crate::model::max_output);
            reveal(crate::model::max_output_quarter);
            crate::model::lemma_area_cap(w, h);
            assert(wh as int == (w as int) * (h as int));
            assert(MAX_OUTPUT / 4 == crate::model::max_output_quarter()) by (compute_only);
            assert((w as int) * (h as int) > crate::model::max_output_quarter());
        }
        return Err(Error::OutputTooLarge);
    }
    proof {
        reveal(crate::model::mh);
        reveal(crate::model::max_output);
        reveal(crate::model::max_output_quarter);
        crate::model::lemma_area_cap(w, h);
        assert(wh as int == (w as int) * (h as int));
        assert((w as int) * (h as int) <= crate::model::max_output_quarter());
        assert(4 * (w as int) * (h as int) <= crate::model::max_output());
    }
    let mut p: usize = 13;
    let mut global = None;
    if packed & 0x80 != 0 {
        proof {
            reveal(crate::model::flag_gct);
            assert(packed == b@[10]);
        }
        let entries = table_entries(packed);
        let n = 3 * entries as usize;
        proof {
            assert(entries <= 256);
            assert(n == 3 * (entries as int) as usize);
            assert(n <= 768) by {
                assert(entries as int <= 256);
            }
            assert(n as int == 3 * crate::model::table_entries(b@[10]));
        }
        if !need(b, p, n) {
            proof {
                reveal(crate::model::mh);
                assert(!crate::model::have(b@, 13, n as int));
                assert(!crate::model::have(
                    b@,
                    13,
                    3 * crate::model::table_entries(b@[10]),
                ));
            }
            return Err(Error::TruncatedColorTable);
        }
        proof {
            reveal(crate::model::mh);
            assert(crate::model::have(b@, 13, n as int));
            assert(crate::model::have(b@, 13, 3 * crate::model::table_entries(b@[10])));
        }
        global = Some(ColorTable { offset: p, entries });
        p += n;
    } else {
        proof {
            reveal(crate::model::flag_gct);
            assert(packed == b@[10]);
            assert(!crate::model::flag_gct(b@[10]));
        }
    }
    proof {
        reveal(crate::model::mh);
        reveal(crate::model::have);
        reveal(crate::model::table_entries);
        reveal(crate::model::flag_gct);
        reveal(hdr_matches);
        reveal(crate::model::version_code);
        assert(global is None ==> p == 13);
        assert(global is Some ==> p == 13 + 3 * crate::model::table_entries(b@[10]));
        assert(crate::model::mh(b@ ) == Ok(
            crate::model::HeaderM {
                width: crate::model::le16(b@, 6),
                height: crate::model::le16(b@, 8),
                version: crate::model::version_code(
                    crate::model::version_of(b@)->Some_0,
                ),
                background_index: b@[11] as int,
                aspect: b@[12] as int,
                global: if crate::model::flag_gct(b@[10]) {
                    Some(
                        crate::model::ColorTableM {
                            off: 13,
                            entries: crate::model::table_entries(b@[10]),
                        },
                    )
                } else {
                    None
                },
                cursor: if crate::model::flag_gct(b@[10]) {
                    13 + 3 * crate::model::table_entries(b@[10])
                } else {
                    13
                },
            },
        ));
    }
    Ok(Header {
        width: w16,
        height: h16,
        version,
        background_index: get(b, 11),
        aspect: get(b, 12),
        global,
        cursor: p,
    })
}

/// G5's sub-block sequence starting with the size byte at `q`: the cursor after
/// the zero-size terminator and the total payload length, or `err` for a
/// missing byte. G6.3 rule A (the `loop_count` scan) is a separate pass over
/// the same block (`loop_count_of`), so this function's contract is only the
/// structural scan plus the payload total.
fn read_sub_blocks(b: &[u8], q: usize, err: Error) -> (r: Result<(usize, u64), Error>)
    requires
        q <= b.len(),
        b.len() <= crate::limits::MAX_INPUT,
    ensures
        match crate::model::sub_scan(b@, q as int) {
            Some(res) => match r {
                Ok((end, total)) => {
                    &&& end as int == res.1
                    &&& total as int == crate::model::concat_len(res.0)
                    &&& total as int <= b@.len()
                },
                Err(_) => false,
            },
            None => match r {
                Ok(_) => false,
                Err(e) => e == err,
            },
        },
{
    let mut p = q;
    let mut total: u64 = 0;
    let ghost mut scanned: Seq<Seq<u8>> = Seq::empty();
    proof {
        reveal(crate::model::sub_scan);
        reveal(crate::model::sub_pref_at);
        reveal(crate::model::sub_consumed);
        reveal(crate::model::concat_len);
        reveal(crate::model::have);
        assert(b@.len() == b.len());
        crate::model::lemma_sub_pref_init(b@, q as int);
        crate::model::lemma_sub_consumed_init(q as int);
    }
    loop
        invariant
            q <= p <= b.len(),
            b@.len() == b.len(),
            b.len() <= crate::limits::MAX_INPUT,
            crate::model::sub_pref_at(b@, q as int, p as int, scanned),
            crate::model::sub_consumed(q as int, p as int, scanned),
            total as int == crate::model::concat_len(scanned),
        decreases (b.len() - p),
    {
        if !need(b, p, 1) {
            return Err(err);
        }
        let n = get(b, p) as usize;
        if n == 0 {
            proof {
                crate::model::lemma_sub_pref_done(b@, q as int, p as int, scanned);
                assert(total as int == crate::model::concat_len(scanned));
                assert(total as int <= b@.len());
                assert(b@.len() <= usize::MAX);
                assert(p + 1 <= usize::MAX) by (nonlinear_arith)
                    requires
                        p < b@.len(),
                        b@.len() <= usize::MAX,
                ;
            }
            return Ok((p + 1, total));
        }
        if !need(b, p + 1, n) {
            return Err(err);
        }
        let ghost payload: Seq<u8> = b@.subrange(p as int + 1, p as int + 1 + n as int);
        proof {
            assert(n as int == b@[p as int]);
            crate::model::lemma_sub_scan_step(b@, q as int, p as int, scanned, n as int);
            vstd::seq::lemma_seq_subrange_len(b@, p as int + 1, p as int + 1 + n as int);
            assert(payload.len() == n as int);
            crate::model::lemma_concat_len_push(b@, p as int, scanned, n as int);
            assert(total as int + n as int == crate::model::concat_len(scanned.push(payload)));
            assert(total as int + n as int <= b@.len());
            assert(p + 1 + n <= b.len());
            crate::model::lemma_sub_consumed_step(q as int, p as int, scanned, n as int);
            assert(b@.len() <= usize::MAX);
            assert(total as int + n as int <= b@.len());
            assert(total as usize + n <= usize::MAX) by (nonlinear_arith)
                requires
                    total as int + n as int <= b@.len(),
                    b@.len() <= usize::MAX,
            ;
        }
        total += n as u64;
        p = p + 1 + n;
        proof {
            scanned = scanned.push(payload);
        }
    }
}

/// G6.3 rule A as a separate pass: the `loop_count` value the payloads of a
/// sub-block sequence set, or `None` when the sequence cannot be read (the
/// caller has already established that it can, so the value is then irrelevant).
fn loop_count_of(b: &[u8], q: usize, seed: Option<u16>, first_only: bool) -> (r: Result<
    (usize, Option<u16>),
    Error,
>)
    requires
        q <= b.len(),
        b.len() <= crate::limits::MAX_INPUT,
    ensures
        match crate::model::sub_scan(b@, q as int) {
            Some(res) => match r {
                Ok((end, lc)) => {
                    &&& end as int == res.1
                    &&& lc == crate::model::loop_fold(res.0, 0, seed, first_only)
                },
                Err(_) => false,
            },
            None => match r {
                Ok(_) => false,
                Err(e) => e == Error::TruncatedExtension,
            },
        },
{
    let sub = read_sub_blocks(b, q, Error::TruncatedExtension)?;
    let (end, _) = sub;
    let mut lc = seed;
    let mut idx: usize = 0;
    let mut p = q;
    let ghost mut payloads: Seq<Seq<u8>> = Seq::empty();
    let ghost mut hit_value: Option<u16> = None;
    proof {
        reveal(crate::model::sub_scan);
        reveal(crate::model::loop_fold);
        reveal(crate::model::have);
        crate::model::lemma_loop_fold_init(seed);
    }
    loop
        invariant
            q <= p <= b.len(),
            b@.len() == b.len(),
            b.len() <= crate::limits::MAX_INPUT,
            idx as int == payloads.len(),
            crate::model::sub_pref_at(b@, q as int, p as int, payloads),
            crate::model::sub_consumed(q as int, p as int, payloads),
            crate::model::loop_fold(payloads, 0, seed, first_only) == lc,
            crate::model::concat_len(payloads) >= 0,
            match crate::model::sub_scan(b@, p as int) {
                Some(res) => res.1 == end as int,
                None => false,
            },
        decreases (b.len() - p),
    {
        proof {
            assert(b@.len() <= usize::MAX);
        }
        if !need(b, p, 1) {
            return Err(Error::TruncatedExtension);
        }
        let n = get(b, p) as usize;
        if n == 0 {
            proof {
                crate::model::lemma_sub_pref_done(b@, q as int, p as int, payloads);
                crate::model::lemma_sub_scan_unique(b@, q as int, payloads, p as int);
            }
            return Ok((end, lc));
        }
        if !need(b, p + 1, n) {
            return Err(Error::TruncatedExtension);
        }
        let first = get(b, p + 1);
        let ghost payload: Seq<u8> = b@.subrange(p as int + 1, p as int + 1 + n as int);
        proof {
            assert(n as int == b@[p as int]);
            crate::model::lemma_sub_scan_step(b@, q as int, p as int, payloads, n as int);
            reveal(crate::model::loop_hit);
            reveal(crate::model::loop_hit_value);
            vstd::seq::lemma_seq_subrange_len(b@, p as int + 1, p as int + 1 + n as int);
            assert(payload.len() == n as int);
            assert(payload[0] == first);
        }
        let prev = lc;
        let hit = (!first_only || idx == 0) && n == 3 && first == 1;
        if hit {
            let v = le16(b, p + 2);
            proof {
                assert(v as int == crate::model::le16(b@, p as int + 2));
                hit_value = Some(v);
            }
            lc = Some(v);
        }
        proof {
            reveal(crate::model::loop_fold);
            reveal(crate::model::loop_hit);
            reveal(crate::model::loop_hit_value);
            crate::model::lemma_loop_fold_single_tail(payloads, payload, seed, first_only);
            // The executable's `hit` is the model's `loop_hit` at index `idx`.
            assert(crate::model::loop_hit(payload, idx as int, first_only) == hit) by {
                assert(payload.len() == n as int);
                assert(payload[0] == first);
            }
            // On a hit the value is `loop_hit_value(payload)`; the executable
            // wrote `Some(le16(b, p+2))`, captured in `hit_value`.
            if hit {
                let a = payload[1];
                let c = payload[2];
                assert(a == b@[p as int + 2]);
                assert(c == b@[p as int + 3]);
                assert(payload.len() == 3 <==> n == 3);
                assert(lc == crate::model::loop_hit_value(payload)) by {
                    reveal(crate::model::loop_hit_value);
                    assert(lc == hit_value);
                    match hit_value {
                        Some(v) => {
                            assert(v as int == crate::model::le16(b@, p as int + 2));
                            reveal(crate::model::le16);
                            assert(v == ((a as u16) | ((c as u16) << 8)) as u16) by (bit_vector)
                                requires
                                    v as int == a as int + 256 * (c as int),
                            ;
                        },
                        None => {},
                    }
                }
            } else {
                assert(lc == prev);
                assert(!crate::model::loop_hit(payload, idx as int, first_only));
            }
            crate::model::lemma_loop_fold_one_step(payload, idx as int, prev, lc, first_only);
            assert(crate::model::loop_fold(payloads, 0, seed, first_only) == prev);
            // `payloads.push(payload) == payloads + seq![payload]`, so the fold of
            // the extended list is the fold of the singleton from `idx`.
            assert(payloads + seq![payload] == payloads.push(payload));
            assert(crate::model::loop_fold(payloads + seq![payload], 0, seed, first_only)
                == crate::model::loop_fold(seq![payload], idx as int, prev, first_only));
            assert(crate::model::loop_fold(payloads.push(payload), 0, seed, first_only) == lc);
        }
        proof {
            reveal(crate::model::sub_consumed);
            reveal(crate::model::concat_len);
            crate::model::lemma_concat_len_nonneg(payloads);
            // `p - q == concat_len(payloads) + |payloads|`, `p + 1 + n <= |b|`.
            assert(payloads.len() + 1 <= p - q + 1) by (nonlinear_arith)
                requires
                    p - q == crate::model::concat_len(payloads) + payloads.len(),
                    crate::model::concat_len(payloads) >= 0,
            ;
            assert(payloads.len() + 1 <= p + 1) by (nonlinear_arith)
                requires
                    payloads.len() + 1 <= p - q + 1,
                    q >= 0,
            ;
            assert(p + 1 + n <= b@.len());
            assert(payloads.len() + 1 <= b@.len()) by (nonlinear_arith)
                requires
                    payloads.len() + 1 <= p + 1,
                    p + 1 + n <= b@.len(),
                    n >= 1,
            ;
            assert(idx + 1 <= b@.len());
            assert(idx + 1 <= usize::MAX) by (nonlinear_arith)
                requires
                    idx + 1 <= b@.len(),
                    b@.len() <= usize::MAX,
            ;
            crate::model::lemma_sub_consumed_step(q as int, p as int, payloads, n as int);
            crate::model::lemma_concat_len_push(b@, p as int, payloads, n as int);
            assert(crate::model::concat_len(payloads.push(payload)) >= 0) by {
                crate::model::lemma_concat_len_nonneg(payloads.push(payload));
            }
        }
        idx = idx + 1;
        p = p + 1 + n;
        proof {
            payloads = payloads.push(payload);
        }
    }
}


/// A pending Graphic Control Extension (G6.1). `disposal` is the raw 3-bit
/// value (4–7 only under `disposal_undefined = Keep`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gce {
    pub disposal: u8,
    pub user_input: bool,
    pub delay_cs: u16,
    pub transparent: Option<u8>,
}

/// Block-walk state (G5). The cursor lives in the caller (`Frames::p`), so the
/// extension functions return it rather than storing it here.
#[derive(Clone, Copy, Debug)]
pub struct WalkState {
    pub gce: Option<Gce>,
    pub frames_seen: u32,
    pub total_pixels: u64,
    pub loop_count: Option<u16>,
}

impl WalkState {
    pub fn new() -> (st: WalkState)
        ensures st.gce is None,st.frames_seen==0,st.total_pixels==0,st.loop_count is None,
    {
        WalkState { gce: None, frames_seen: 0, total_pixels: 0, loop_count: None }
    }
}

/// The executable `Gce` is the model record: equality.
pub open spec fn gce_matches(g: Option<Gce>, m: Option<crate::model::GceM>) -> bool {
    g == m
}

/// The executable walk state carries the model's fields. The cursor is not
/// among them (the caller owns it), so only the four mutated fields are
/// compared.
pub open spec fn walk_matches(st: WalkState, m: crate::model::WalkM) -> bool {
    &&& st.frames_seen as int == m.frames_seen
    &&& st.total_pixels as int == m.total_pixels
    &&& st.loop_count == m.loop_count
    &&& gce_matches(st.gce, m.gce)
}

/// G6.1's disposal `(G >> 2) & 7`.
pub proof fn lemma_gce_disposal(g: u8)
    ensures
        (((g >> 2u8) & 7u8) as int) == crate::model::gce_disposal(g),
{
    reveal(crate::model::gce_disposal);
}

/// G6.1's user-input flag.
pub proof fn lemma_gce_user_input(g: u8)
    ensures
        (g & 2u8 != 0u8) == crate::model::gce_user_input(g),
{
    reveal(crate::model::gce_user_input);
}

/// G6.1's transparency flag.
pub proof fn lemma_gce_transparent_flag(g: u8)
    ensures
        (g & 1u8 != 0u8) == crate::model::gce_transparent_flag(g),
{
    reveal(crate::model::gce_transparent_flag);
}


/// The model walk state of an exec state at cursor `p`.
pub open spec fn walk_m_at(st: WalkState, p: usize) -> crate::model::WalkM {
    walk_m_p(st, p as int)
}

/// The model walk state with an explicit model cursor.
pub open spec fn walk_m_p(st: WalkState, p: int) -> crate::model::WalkM {
    crate::model::WalkM {
        p,
        gce: st.gce,
        frames_seen: st.frames_seen as int,
        total_pixels: st.total_pixels as int,
        loop_count: st.loop_count,
    }
}

/// The model walk state of an exec state (cursor `0`, which `gce_step` does not
/// read: it takes `q` as a separate argument).
pub open spec fn walk_m_unused(st: WalkState) -> crate::model::WalkM {
    crate::model::WalkM {
        p: 0,
        gce: st.gce,
        frames_seen: st.frames_seen as int,
        total_pixels: st.total_pixels as int,
        loop_count: st.loop_count,
    }
}

/// The model record of an exec `Gce` is the exec `Gce` itself (`model::GceM`
/// is `pub use crate::parse::Gce as GceM`), so this is the identity and no
/// conversion lemma is needed.
pub open spec fn gce_of(g: Option<Gce>) -> Option<crate::model::GceM> {
    g
}

/// `walk_m` is injective on the fields it carries: two exec states with the
/// same model state agree there.
pub open spec fn walk_eq(st1: WalkState, st2: WalkState) -> bool {
    &&& st1.frames_seen == st2.frames_seen
    &&& st1.total_pixels == st2.total_pixels
    &&& st1.loop_count == st2.loop_count
    &&& st1.gce == st2.gce
}

/// The exec state at cursor `e` matches the model state `s2` (`s2.p == e`).
pub open spec fn st_matches(e: usize, st: WalkState, s2: crate::model::WalkM) -> bool {
    e as int == s2.p && st.frames_seen as int == s2.frames_seen && st.total_pixels as int
        == s2.total_pixels && st.loop_count == s2.loop_count && gce_matches(st.gce, s2.gce)
}





/// One model branch of `gce_step`: the duplicate-Graphic-Control rejection.
pub proof fn lemma_gce_step_dup(b: Seq<u8>, pol: Policy, st: WalkState, q: int)
    requires
        st.gce != None,
        pol.gce_duplicate == GceDuplicate::Reject,
    ensures
        crate::model::gce_step(b, pol, walk_m_unused(st), q) == Err(Error::DuplicateGraphicControl),
{
    reveal(crate::model::gce_step);
    reveal(walk_m_unused);
}

/// The missing size byte.
pub proof fn lemma_gce_step_trunc(b: Seq<u8>, pol: Policy, st: WalkState, q: int)
    requires
        st.gce == None || pol.gce_duplicate != GceDuplicate::Reject,
        !crate::model::have(b, q, 1),
    ensures
        crate::model::gce_step(b, pol, walk_m_unused(st), q) == Err(Error::TruncatedExtension),
{
    reveal(crate::model::gce_step);
    reveal(walk_m_unused);
    reveal(crate::model::have);
}

/// A size byte other than 4.
pub proof fn lemma_gce_step_size(b: Seq<u8>, pol: Policy, st: WalkState, q: int)
    requires
        st.gce == None || pol.gce_duplicate != GceDuplicate::Reject,
        crate::model::have(b, q, 1),
        b[q] != 4u8,
    ensures
        crate::model::gce_step(b, pol, walk_m_unused(st), q) == Err(Error::InvalidGraphicControl),
{
    reveal(crate::model::gce_step);
    reveal(walk_m_unused);
}

/// The body bytes missing.
pub proof fn lemma_gce_step_body_trunc(b: Seq<u8>, pol: Policy, st: WalkState, q: int)
    requires
        st.gce == None || pol.gce_duplicate != GceDuplicate::Reject,
        crate::model::have(b, q, 1),
        b[q] == 4u8,
        !crate::model::have(b, q, 6),
    ensures
        crate::model::gce_step(b, pol, walk_m_unused(st), q) == Err(Error::TruncatedExtension),
{
    reveal(crate::model::gce_step);
    reveal(walk_m_unused);
}

/// A terminator byte other than 0.
pub proof fn lemma_gce_step_term(b: Seq<u8>, pol: Policy, st: WalkState, q: int)
    requires
        st.gce == None || pol.gce_duplicate != GceDuplicate::Reject,
        crate::model::have(b, q, 1),
        b[q] == 4u8,
        crate::model::have(b, q, 6),
        b[q + 5] != 0u8,
    ensures
        crate::model::gce_step(b, pol, walk_m_unused(st), q) == Err(Error::InvalidGraphicControl),
{
    reveal(crate::model::gce_step);
    reveal(walk_m_unused);
}

/// An undefined disposal.
pub proof fn lemma_gce_step_disposal(b: Seq<u8>, pol: Policy, st: WalkState, q: int)
    requires
        st.gce == None || pol.gce_duplicate != GceDuplicate::Reject,
        crate::model::have(b, q, 1),
        b[q] == 4u8,
        crate::model::have(b, q, 6),
        b[q + 5] == 0u8,
        (b[q + 1] >> 2u8 & 7u8) as int > 3,
        pol.disposal_undefined == DisposalUndefined::Reject,
    ensures
        crate::model::gce_step(b, pol, walk_m_unused(st), q) == Err(Error::UndefinedDisposal),
{
    reveal(crate::model::gce_step);
    reveal(walk_m_unused);
}

/// The success branch, stated as the exec result `gce_ext` returns.
pub proof fn lemma_gce_step_ok_simple(b: Seq<u8>, pol: Policy, st: WalkState, q: int, gce: Gce)
    requires
        st.gce == None || pol.gce_duplicate != GceDuplicate::Reject,
        crate::model::have(b, q, 1),
        b[q] == 4u8,
        crate::model::have(b, q, 6),
        b[q + 5] == 0u8,
        !((b[q + 1] >> 2u8 & 7u8) as int > 3 && pol.disposal_undefined
            == DisposalUndefined::Reject),
        gce == crate::model::gce_record(b, q),
    ensures
        crate::model::gce_step(b, pol, walk_m_unused(st), q) == Ok(
            crate::model::WalkM {
                p: q + 6,
                gce: Some(gce),
                frames_seen: st.frames_seen as int,
                total_pixels: st.total_pixels as int,
                loop_count: st.loop_count,
            },
        ),
{
    reveal(crate::model::gce_step);
    reveal(walk_m_unused);
    reveal(crate::model::gce_record);
}

/// `gce_ext`'s postcondition, named so each proof block reveals one predicate.
pub open spec fn gce_ext_post(
    b: Seq<u8>,
    pol: Policy,
    st: WalkState,
    q: int,
    r: Result<(usize, WalkState), Error>,
) -> bool {
    match crate::model::gce_step(b, pol, walk_m_unused(st), q) {
        Err(e) => match r {
            Ok(_) => false,
            Err(e2) => e2 == e,
        },
        Ok(s2) => match r {
            Ok((end, st2)) => st_matches(end, st2, s2),
            Err(_) => false,
        },
    }
}

/// G6.1: the Graphic Control Extension body at `q` (after the introducer and
/// label). Returns the cursor after the extension and the new state.
fn gce_ext(b: &[u8], pol: Policy, st: &WalkState, q: usize) -> (r: Result<(usize, WalkState), Error>)
    requires
        q >= 2,
        q <= b.len(),
        b.len() <= crate::limits::MAX_INPUT,
    ensures
        gce_ext_post(b@, pol, *st, q as int, r),
{
    let has_gce = match st.gce {
        Some(_) => true,
        None => false,
    };
    let dup_reject = match pol.gce_duplicate {
        GceDuplicate::Reject => true,
        _ => false,
    };
    proof {
        assert(has_gce == (st.gce is Some));
        assert(dup_reject == (pol.gce_duplicate == GceDuplicate::Reject));
    }
    if has_gce && dup_reject {
        proof {
            reveal(walk_m_unused);
            lemma_gce_step_dup(b@, pol, *st, q as int);
            reveal(gce_ext_post);
        }
        return Err(Error::DuplicateGraphicControl);
    }
    proof {
        reveal(walk_m_unused);
        reveal(gce_ext_post);
    }

    if !need(b, q, 1) {
        proof {
            reveal(crate::model::gce_step);
            reveal(walk_m_unused);
            reveal(gce_of);
            reveal(crate::model::have);
        }
        return Err(Error::TruncatedExtension);
    }
    proof {
        reveal(gce_ext_post);
        reveal(crate::model::gce_step);
        reveal(crate::model::have);
    }
    if get(b, q) != 4 {
        proof {
            reveal(walk_m_unused);
            lemma_gce_step_size(b@, pol, *st, q as int);
            reveal(gce_ext_post);
        }
        return Err(Error::InvalidGraphicControl);
    }
    if !need(b, q, 6) {
        proof {
            reveal(walk_m_unused);
            lemma_gce_step_body_trunc(b@, pol, *st, q as int);
            reveal(gce_ext_post);
        }
        return Err(Error::TruncatedExtension);
    }
    if get(b, q + 5) != 0 {
        proof {
            reveal(walk_m_unused);
            lemma_gce_step_term(b@, pol, *st, q as int);
            reveal(gce_ext_post);
        }
        return Err(Error::InvalidGraphicControl);
    }
    let g = get(b, q + 1);
    proof {
        reveal(crate::model::gce_step);
        reveal(crate::model::gce_disposal);
        reveal(crate::model::gce_user_input);
        reveal(crate::model::gce_transparent_flag);
        assert(g as int == b@[q as int + 1]);
    }
    let disposal = (g >> 2) & 7;
    let undef_reject = match pol.disposal_undefined {
        DisposalUndefined::Reject => true,
        _ => false,
    };
    proof {
        assert(undef_reject == (pol.disposal_undefined == DisposalUndefined::Reject));
    }
    if disposal > 3 && undef_reject {
        proof {
            reveal(walk_m_unused);
            assert(g as int == b@[q as int + 1]);
            assert(disposal == (g >> 2u8 & 7u8));
            assert((g >> 2u8 & 7u8) as int > 3);
            lemma_gce_step_disposal(b@, pol, *st, q as int);
            reveal(gce_ext_post);
        }
        return Err(Error::UndefinedDisposal);
    }
    let gce = Gce {
        disposal,
        user_input: g & 2 != 0,
        delay_cs: le16(b, q + 2),
        transparent: if g & 1 != 0 { Some(get(b, q + 4)) } else { None },
    };
    proof {
        // The record equals the model's `gce_record` at `q`, field for field.
        assert(g as int == b@[q as int + 1]);
        assert(disposal == (g >> 2u8 & 7u8));
        assert((g & 2u8 != 0u8) == (b@[q as int + 1] & 2u8 != 0u8));
        assert((g & 1u8 != 0u8) == (b@[q as int + 1] & 1u8 != 0u8));
        assert(gce.delay_cs as int == crate::model::le16(b@, q as int + 2));
        assert(gce.disposal == crate::model::gce_record(b@, q as int).disposal);
        assert(gce.user_input == crate::model::gce_record(b@, q as int).user_input);
        assert(gce.delay_cs as int == crate::model::le16(b@, q as int + 2));
        assert(gce.delay_cs == crate::model::gce_record(b@, q as int).delay_cs) by {
            reveal(crate::model::gce_record);
        }
        assert(gce.transparent == crate::model::gce_record(b@, q as int).transparent);
        assert(gce == crate::model::gce_record(b@, q as int));
        lemma_gce_step_ok_simple(b@, pol, *st, q as int, gce);
        reveal(gce_ext_post);
        reveal(st_matches);
        reveal(gce_matches);
        reveal(walk_m_unused);
    }
    Ok((
        q + 6,
        WalkState {
            gce: Some(gce),
            frames_seen: st.frames_seen,
            total_pixels: st.total_pixels,
            loop_count: st.loop_count,
        },
    ))
}

/// G6.4 / G6.2: the sub-block sequence at `q` read and skipped.
fn skip_ext(b: &[u8], st: &WalkState, q: usize) -> (r: Result<(usize, WalkState), Error>)
    requires
        q <= b.len(),
        b.len() <= crate::limits::MAX_INPUT,
    ensures
        match crate::model::sub_scan(b@, q as int) {
            None => match r {
                Ok(_) => false,
                Err(e) => e == Error::TruncatedExtension,
            },
            Some(res) => match r {
                Ok((end, st2)) => {
                    &&& end as int == res.1
                    &&& st_eq_keep(st2, *st, end, res.1)
                },
                Err(_) => false,
            },
        },
{
    let (end, _) = read_sub_blocks(b, q, Error::TruncatedExtension)?;
    Ok((
        end,
        WalkState {
            gce: st.gce,
            frames_seen: st.frames_seen,
            total_pixels: st.total_pixels,
            loop_count: st.loop_count,
        },
    ))
}

/// The skipped state keeps every field and takes the model's cursor.
pub open spec fn st_eq_keep(st2: WalkState, st: WalkState, end: usize, mend: int) -> bool {
    &&& end as int == mend
    &&& walk_eq(st2, st)
}


/// `loop_value` of the tail is `loop_fold` of the tail from `None`, which is
/// what `loop_count_of` computes (`loop_count_of(b, q+1+size, None, fo)`).
pub proof fn lemma_loop_value_tail(subs: Seq<Seq<u8>>)
    requires
        subs.len() >= 1,
    ensures
        crate::model::loop_value(
            subs.subrange(1, subs.len() as int),
            false,
        ) == crate::model::loop_fold(subs.subrange(1, subs.len() as int), 0, None, false),
{
    reveal(crate::model::loop_value);
}



/// `sub_scan(b, q)` when the first size byte is `size >= 1`: the payload of
/// that length followed by the rest. This is the unfolding the identifier check
/// needs.
pub open spec fn sub_scan_at(b: Seq<u8>, q: int, size: int) -> Option<(Seq<Seq<u8>>, int)> {
    if crate::model::have(b, q, 1) && b[q] as int == size && size >= 1 && crate::model::have(
        b,
        q + 1,
        size,
    ) {
        match crate::model::sub_scan(b, q + 1 + size) {
            Some((rest, e)) => Some((seq![b.subrange(q + 1, q + 1 + size)] + rest, e)),
            None => None,
        }
    } else {
        crate::model::sub_scan(b, q)
    }
}

/// The unfolding holds: `sub_scan(b, q)` is the first payload plus the tail.
pub proof fn lemma_sub_scan_at(b: Seq<u8>, q: int, size: int)
    requires
        crate::model::have(b, q, 1),
        b[q] as int == size,
        1 <= size,
        crate::model::have(b, q + 1, size),
    ensures
        sub_scan_at(b, q, size) == crate::model::sub_scan(b, q),
{
    reveal(sub_scan_at);
    reveal(crate::model::sub_scan);
    reveal(crate::model::have);
}

/// The `app_step` branches, in the shape `app_ext`'s exits need.

/// The missing size byte.
pub proof fn lemma_app_step_trunc(b: Seq<u8>, pol: Policy, st: WalkState, q: int)
    requires
        !crate::model::have(b, q, 1),
    ensures
        crate::model::app_step(b, pol, walk_m_unused(st), q) == Err(Error::TruncatedExtension),
{
    reveal(crate::model::app_step);
    reveal(crate::model::have);
    reveal(walk_m_unused);
}

/// A size byte other than 11 under `ext_sizes = Exact`.
pub proof fn lemma_app_step_size(b: Seq<u8>, pol: Policy, st: WalkState, q: int)
    requires
        crate::model::have(b, q, 1),
        b[q] != 11u8,
        pol.ext_sizes == ExtSizes::Exact,
    ensures
        crate::model::app_step(b, pol, walk_m_unused(st), q) == Err(
            Error::InvalidApplicationExtension,
        ),
{
    reveal(crate::model::app_step);
    reveal(walk_m_unused);
}

/// A zero size byte: the extension is empty and the cursor moves past it.
pub proof fn lemma_app_step_empty(b: Seq<u8>, pol: Policy, st: WalkState, q: int)
    requires
        crate::model::have(b, q, 1),
        b[q] == 0u8,
        b[q] != 11u8 && pol.ext_sizes == ExtSizes::Exact ==> false,
    ensures
        crate::model::app_step(b, pol, walk_m_unused(st), q) == Ok(
            crate::model::WalkM {
                p: q + 1,
                gce: st.gce,
                frames_seen: st.frames_seen as int,
                total_pixels: st.total_pixels as int,
                loop_count: st.loop_count,
            },
        ),
{
    reveal(crate::model::app_step);
    reveal(crate::model::sub_scan);
    reveal(crate::model::have);
    reveal(crate::model::is_netscape);
    reveal(walk_m_unused);
}

/// The identifier's bytes are absent.
pub proof fn lemma_app_step_id_trunc(b: Seq<u8>, pol: Policy, st: WalkState, q: int, size: int)
    requires
        crate::model::have(b, q, 1),
        b[q] as int == size,
        !(b[q] != 11u8 && pol.ext_sizes == ExtSizes::Exact),
        !crate::model::have(b, q + 1, size),
    ensures
        crate::model::app_step(b, pol, walk_m_unused(st), q) == Err(Error::TruncatedExtension),
{
    reveal(crate::model::app_step);
    reveal(crate::model::sub_scan);
    reveal(crate::model::have);
    reveal(walk_m_unused);
}



/// The `loop_count` `app_step` computes when the identifier payload is
/// `id_payload` (`NETSCAPE2.0` or not) and the sub-blocks after it are `tail`:
/// rule A over `tail` when the identifier matched and no image has been seen,
/// else the incoming value.
pub open spec fn app_step_lc_of(
    id_payload: Seq<u8>,
    tail: Seq<Seq<u8>>,
    frames_seen: int,
    incoming: Option<u16>,
    first_only: bool,
) -> Option<u16> {
    if id_payload == crate::model::bytes_of(&crate::model::NETSCAPE_ID) && frames_seen == 0 {
        match crate::model::loop_fold(tail, 0, incoming, first_only) {
            None => incoming,
            Some(v) => Some(v),
        }
    } else {
        incoming
    }
}

/// `app_step_lc_of` with the identifier and tail read out of `sub_scan`.
pub open spec fn app_step_lc(b: Seq<u8>, pol: Policy, st: WalkState, q: int) -> Option<u16> {
    match crate::model::sub_scan(b, q) {
        None => st.loop_count,
        Some((subs, end)) => {
            if subs.len() >= 1 {
                app_step_lc_of(
                    subs[0],
                    subs.subrange(1, subs.len() as int),
                    st.frames_seen as int,
                    st.loop_count,
                    pol.loop_scan == LoopScan::FirstSubBlockOnly,
                )
            } else {
                st.loop_count
            }
        },
    }
}

/// The `plain_text_step` branches.

/// The missing size byte.
pub proof fn lemma_pt_step_trunc(b: Seq<u8>, pol: Policy, st: WalkState, q: int)
    requires
        !crate::model::have(b, q, 1),
    ensures
        crate::model::plain_text_step(b, pol, walk_m_unused(st), q) == Err(Error::TruncatedExtension),
{
    reveal(crate::model::plain_text_step);
    reveal(crate::model::have);
    reveal(walk_m_unused);
}

/// A size byte other than 12 under `ext_sizes = Exact`.
pub proof fn lemma_pt_step_size(b: Seq<u8>, pol: Policy, st: WalkState, q: int)
    requires
        crate::model::have(b, q, 1),
        b[q] != 12u8,
        pol.ext_sizes == ExtSizes::Exact,
    ensures
        crate::model::plain_text_step(b, pol, walk_m_unused(st), q) == Err(Error::InvalidPlainText),
{
    reveal(crate::model::plain_text_step);
    reveal(walk_m_unused);
}

/// The success branch: the sequence at `q` is skipped.
pub proof fn lemma_pt_step_ok(b: Seq<u8>, pol: Policy, st: WalkState, q: int)
    requires
        crate::model::have(b, q, 1),
        b[q] == 12u8 || pol.ext_sizes != ExtSizes::Exact,
    ensures
        crate::model::plain_text_step(b, pol, walk_m_unused(st), q) == match crate::model::sub_scan(
            b,
            q,
        ) {
            None => Err(Error::TruncatedExtension),
            Some((_, end)) => Ok(
                crate::model::WalkM {
                    p: end,
                    gce: st.gce,
                    frames_seen: st.frames_seen as int,
                    total_pixels: st.total_pixels as int,
                    loop_count: st.loop_count,
                },
            ),
        },
{
    reveal(crate::model::plain_text_step);
    reveal(crate::model::have);
    reveal(walk_m_unused);
}

/// G6.2: the Plain Text Extension at `q`. The 12-byte header is the first
/// sub-block; `ext_sizes = Exact` requires its size byte to be 12. The whole
/// sequence is then skipped (not rendered, N4/Q7).
fn plain_text_ext(b: &[u8], pol: Policy, st: &WalkState, q: usize) -> (r: Result<
    (usize, WalkState),
    Error,
>)
    requires
        q <= b.len(),
        b.len() <= crate::limits::MAX_INPUT,
    ensures
        match crate::model::plain_text_step(b@, pol, walk_m_unused(*st), q as int) {
            Err(e) => match r {
                Ok(_) => false,
                Err(e2) => e2 == e,
            },
            Ok(s2) => match r {
                Ok((end, st2)) => st_matches(end, st2, s2),
                Err(_) => false,
            },
        },
{
    if !need(b, q, 1) {
        proof {
            lemma_pt_step_trunc(b@, pol, *st, q as int);
            reveal(st_matches);
        }
        return Err(Error::TruncatedExtension);
    }
    let exact = match pol.ext_sizes {
        ExtSizes::Exact => true,
        _ => false,
    };
    proof {
        assert(exact == (pol.ext_sizes == ExtSizes::Exact));
    }
    if get(b, q) != 12 && exact {
        proof {
            lemma_pt_step_size(b@, pol, *st, q as int);
            reveal(st_matches);
        }
        return Err(Error::InvalidPlainText);
    }
    proof {
        lemma_pt_step_ok(b@, pol, *st, q as int);
        reveal(st_matches);
    }
    skip_ext(b, st, q)
}

/// G6.3: the Application Extension at `q`. The identifier block is the first
/// sub-block; `ext_sizes = Exact` requires its size byte to be 11. When it is
/// `NETSCAPE2.0` and no image has been seen, G6.3 rule A scans the following
/// sub-blocks for `loop_count`.
fn app_ext(b: &[u8], pol: Policy, st: &WalkState, q: usize) -> (r: Result<(usize, WalkState), Error>)
    requires
        q <= b.len(),
        b.len() <= crate::limits::MAX_INPUT,
    ensures
        app_ext_post(b@, pol, *st, q as int, r),
{
    if !need(b, q, 1) {
        proof {
            reveal(crate::model::app_step);
            reveal(walk_m_unused);
        }
        return Err(Error::TruncatedExtension);
    }
    let exact = match pol.ext_sizes {
        ExtSizes::Exact => true,
        _ => false,
    };
    proof {
        assert(exact == (pol.ext_sizes == ExtSizes::Exact));
    }
    let size = get(b, q) as usize;
    if size != 11 && exact {
        proof {
            lemma_app_step_size(b@, pol, *st, q as int);
        }
        return Err(Error::InvalidApplicationExtension);
    }
    if size == 0 {
        proof {
            lemma_app_step_empty(b@, pol, *st, q as int);
            reveal(st_matches);
        }
        return Ok((
            q + 1,
            WalkState {
                gce: st.gce,
                frames_seen: st.frames_seen,
                total_pixels: st.total_pixels,
                loop_count: st.loop_count,
            },
        ));
    }
    if !need(b, q + 1, size) {
        proof {
            lemma_app_step_id_trunc(b@, pol, *st, q as int, size as int);
        }
        return Err(Error::TruncatedExtension);
    }
    // The identifier is the payload `b[q+1..q+1+size]`.
    // The identifier is the 11 payload bytes `b[q+1..q+12]`; `is_netscape` is
    // whether they equal `NETSCAPE2.0`. The comparison runs only when `size` is
    // 11 (a shorter or longer first sub-block is not the identifier).
    let mut is_netscape = size == 11;
    if is_netscape {
        // `q + 1 + 11 <= |b| <= MAX_INPUT`, so every index this loop forms is at
        // most `MAX_INPUT + 12`, far below `usize::MAX`.
        let blen: usize = b.len();
        proof {
            crate::model::lemma_max_input_fits_umax();
            assert(blen <= crate::limits::MAX_INPUT);
            assert(q + 12 <= crate::limits::MAX_INPUT + 12) by (nonlinear_arith)
                requires
                    q + 1 + 11 <= blen,
                    blen <= crate::limits::MAX_INPUT,
            ;
            crate::model::lemma_max_input_plus12();
            assert(q + 12 <= usize::MAX) by (nonlinear_arith)
                requires
                    q + 12 <= crate::limits::MAX_INPUT + 12,
                    crate::limits::MAX_INPUT + 12 <= usize::MAX,
            ;
        }
        let mut i = 0;
        while i < 11
            invariant
                i <= 11,
                blen == b@.len(),
                q + 1 + 11 <= blen,
                blen <= crate::limits::MAX_INPUT,
                q + 12 <= usize::MAX,
                // The flag is `id_matches` at `i`: every byte compared so far
                // matched.
                is_netscape == crate::model::id_matches(b@, q as int, i as int),
            decreases (11 - i),
        {
            let j: usize = q + 1 + i;
            let byte = get(b, j);
            let want = crate::model::NETSCAPE_ID[i];
            proof {
                crate::model::lemma_id_matches_step(b@, q as int, i as int);
                assert(byte == b@[j as int]);
                assert(want == crate::model::NETSCAPE_ID@[i as int]);
            }
            if byte != want {
                is_netscape = false;
            } else {
                proof {
                    assert(is_netscape == crate::model::id_matches(b@, q as int, i as int + 1));
                }
            }
            i += 1;
        }
        proof {
            crate::model::lemma_id_matches_eq(b@, q as int);
        }
    }
    let first_only = match pol.loop_scan {
        LoopScan::FirstSubBlockOnly => true,
        _ => false,
    };
    let mut lc = st.loop_count;
    let (nsub, nlc) = read_sub_blocks(b, q + 1 + size, Error::TruncatedExtension)?;
    if is_netscape && st.frames_seen == 0 {
        let (_, lc2) = loop_count_of(b, q + 1 + size, st.loop_count, first_only)?;
        lc = lc2;
    }
    proof {
        reveal(crate::model::app_step);
        reveal(crate::model::sub_scan);
        reveal(crate::model::have);
        reveal(crate::model::is_netscape);
        reveal(crate::model::bytes_of);
        reveal(crate::model::NETSCAPE_ID);
        reveal(walk_m_unused);
        // The scan's payload list: `read_sub_blocks` proved `sub_scan(b, q+1+size)`
        // succeeds, and `sub_scan(b, q)` is the identifier payload followed by it.
        let ghost id_len: int = size as int;
        let ghost rest = choose|r: (Seq<Seq<u8>>, int)|
            crate::model::sub_scan(b@, q as int + 1 + id_len) == Some(r);
        lemma_sub_scan_at(b@, q as int, id_len);
        let ghost payload: Seq<u8> = b@.subrange(q as int + 1, q as int + 1 + id_len);
        let ghost subs: Seq<Seq<u8>> = seq![payload] + rest.0;
        assert(crate::model::sub_scan(b@, q as int) == Some((subs, rest.1)));
        assert(subs.len() >= 1);
        vstd::seq::lemma_seq_subrange_len(b@, q as int + 1, q as int + 1 + id_len);
        assert(payload.len() == id_len);
        assert(subs[0] == payload);
        assert(subs.subrange(1, subs.len() as int) == rest.0) by {
            assert(subs.subrange(1, subs.len() as int) =~= rest.0);
        }
        assert(is_netscape == (subs[0] == crate::model::bytes_of(&crate::model::NETSCAPE_ID)));
        crate::model::lemma_app_step_of_scan(
            b@,
            pol,
            walk_m_unused(*st),
            q as int,
            subs,
            rest.1,
            is_netscape,
            first_only,
            lc,
        );
        assert(nsub as int == rest.1);
    }
    Ok((
        nsub,
        WalkState { gce: st.gce, frames_seen: st.frames_seen, total_pixels: st.total_pixels, loop_count: lc },
    ))
}

/// `app_ext`'s postcondition: the exact `app_step` result, `loop_count`
/// included. The body proves it with `lemma_app_step_of_scan` (M5b), so the
/// whole `WalkM` — cursor, `gce`, `frames_seen`, `total_pixels`, `loop_count` —
/// matches. This closes the open `loop_count` obligation the earlier contract
/// left out.
pub open spec fn app_ext_post(
    b: Seq<u8>,
    pol: Policy,
    st: WalkState,
    q: int,
    r: Result<(usize, WalkState), Error>,
) -> bool {
    match crate::model::app_step(b, pol, walk_m_unused(st), q) {
        Err(e) => match r {
            Ok(_) => false,
            Err(e2) => e2 == e,
        },
        Ok(s2) => match r {
            Ok((end, st2)) => st_matches(end, st2, s2),
            Err(_) => false,
        },
    }
}

/// The extension dispatch's postcondition: the exact `ext_step` result, the
/// whole `WalkM` (`loop_count` included). Every branch — Graphic Control, Plain
/// Text, Application and the skip — now matches `st_matches`, so this is the
/// same clause as `gce_ext_post` was for its own step.
pub open spec fn ext_step_post(
    b: Seq<u8>,
    pol: Policy,
    st: WalkState,
    p: int,
    r: Result<(usize, WalkState), Error>,
) -> bool {
    match crate::model::ext_step(b, pol, walk_m_at(st, p as usize)) {
        Err(e) => match r {
            Ok(_) => false,
            Err(e2) => e2 == e,
        },
        Ok(s2) => match r {
            Ok((end, st2)) => st_matches(end, st2, s2),
            Err(_) => false,
        },
    }
}

/// G6/G7: the extension with its introducer at `p`. The scope check (G7)
/// precedes every other check of the extension (G6).
fn parse_extension_verified(b: &[u8], p: usize, st: &WalkState, pol: Policy) -> (r: Result<
    (usize, WalkState),
    Error,
>)
    requires
        p <= b.len(),
        b.len() <= crate::limits::MAX_INPUT,
    ensures
        ext_step_post(b@, pol, *st, p as int, r),
{
    if !need(b, p, 2) {
        proof {
            reveal(crate::model::ext_step);
            reveal(walk_m_at);
        }
        return Err(Error::TruncatedExtension);
    }
    let label = get(b, p + 1);
    let q = p + 2;
    let reject_consumers = match pol.gce_consumers {
        GceConsumers::Reject => true,
        _ => false,
    };
    let has_gce = match st.gce {
        Some(_) => true,
        None => false,
    };
    proof {
        reveal(crate::model::ext_step);
        reveal(walk_m_at);
        reveal(crate::model::have);
        assert(label as int == b@[p as int + 1]);
    }
    if label <= 0x7F && has_gce && reject_consumers {
        proof {
            reveal(crate::model::ext_step);
            reveal(walk_m_at);
            assert(st.gce is Some);
            assert(pol.gce_consumers == GceConsumers::Reject);
        }
        return Err(Error::UnsupportedGraphicControlTarget);
    }
    proof {
        reveal(crate::model::ext_step);
        reveal(walk_m_at);
    }
    if label == 0xF9 {
        return gce_ext(b, pol, st, q);
    }
    if label == 0x01 {
        return plain_text_ext(b, pol, st, q);
    }
    if label == 0xFF {
        proof {
            reveal(crate::model::ext_step);
            reveal(crate::model::app_step);
            reveal(walk_m_at);
            reveal(walk_m_unused);
        }
        return app_ext(b, pol, st, q);
    }
    // G6.4: Comment (0xFE) and unknown labels are skipped.
    proof {
        reveal(crate::model::ext_step);
        reveal(crate::model::skip_step);
        reveal(crate::model::have);
        reveal(walk_m_at);
    }
    skip_ext(b, st, q)
}


} // verus!

verus! {
/// G8 result.
#[derive(Clone, Copy, Debug)]
pub struct FrameHeader {
    pub rect: Rect,
    pub interlaced: bool,
    pub table: ColorTable,
    /// The rectangle lies inside the screen (always true under Strict).
    pub inside: bool,
}

/// G9 result: minimum code size, the first sub-block's size byte, the
/// payload length `|D|` and the cursor after the block terminator.
#[derive(Clone, Copy, Debug)]
pub struct ImageData {
    pub min_code_size: u32,
    pub first: usize,
    pub data_len: u32,
    pub end: usize,
}
} // verus!

verus! {
pub proof fn lemma_sub_scan_bounds(b:Seq<u8>,q:int,subs:Seq<Seq<u8>>,end:int)
    requires 0<=q,crate::model::sub_scan(b,q)==Some((subs,end)),
    ensures q<end<=b.len(),
    decreases (b.len()-q) as nat,
{
    reveal(crate::model::sub_scan);
    if b[q]!=0 {
        let rest=crate::model::sub_scan(b,q+1+b[q] as int).unwrap();
        lemma_sub_scan_bounds(b,q+1+b[q] as int,rest.0,rest.1);
    }
}

pub proof fn lemma_ext_success_progress(b:Seq<u8>,pol:Policy,s:crate::model::WalkM,ns:crate::model::WalkM)
    requires 0<=s.p,crate::model::ext_step(b,pol,s)==Ok(ns),
    ensures s.p<ns.p<=b.len(),ns.frames_seen==s.frames_seen,ns.total_pixels==s.total_pixels,
        s.frames_seen>=1 ==> ns.loop_count==s.loop_count,
{
    let q=s.p+2;
    if b[s.p+1]!=0xf9 {
        match crate::model::sub_scan(b,q) {
            None=>{},Some((subs,end))=>lemma_sub_scan_bounds(b,q,subs,end),
        }
    }
}

pub open spec fn extension_post(b:Seq<u8>,p:usize,pol:Policy,oldst:WalkState,st:WalkState,r:Result<usize,Error>)->bool {
    match crate::model::ext_step(b,pol,walk_m_at(oldst,p)) {
        Err(e)=>r is Err && r.unwrap_err()==e && walk_eq(oldst,st),
        Ok(ms)=>r is Ok && st_matches(r.unwrap(),st,ms) && p<r.unwrap()<=b.len()
            && st.frames_seen==oldst.frames_seen && st.total_pixels==oldst.total_pixels,
    }
}

pub fn parse_extension(b:&[u8],p:usize,st:&mut WalkState,pol:&Policy)->(r:Result<usize,Error>)
    requires p<=b.len(),b.len()<=MAX_INPUT,
    ensures extension_post(b@,p,*pol,*old(st),*final(st),r),
        old(st).frames_seen>=1 ==> final(st).loop_count==old(st).loop_count,
{
    let result=parse_extension_verified(b,p,st,*pol);
    match result {
        Err(e)=>Err(e),
        Ok((end,ns))=> {
            proof {
                let ms=crate::model::ext_step(b@,*pol,walk_m_at(*st,p)).unwrap();
                lemma_ext_success_progress(b@,*pol,walk_m_at(*st,p),ms);
            }
            *st=ns;
            Ok(end)
        },
    }
}
} // verus!

verus! {
pub open spec fn header_model(h:Header)->crate::model::HeaderM {
    crate::model::HeaderM {width:h.width as int,height:h.height as int,version:crate::model::version_code(h.version),
        background_index:h.background_index as int,aspect:h.aspect as int,cursor:h.cursor as int,
        global:match h.global {None=>None,Some(t)=>Some(crate::model::ColorTableM {off:t.offset as int,entries:t.entries as int})}}
}
pub open spec fn frame_header_model(h:FrameHeader)->crate::model::FrameHeaderM {
    crate::model::FrameHeaderM {left:h.rect.left as int,top:h.rect.top as int,w:h.rect.width as int,h:h.rect.height as int,
        interlaced:h.interlaced,table:crate::model::ColorTableM {off:h.table.offset as int,entries:h.table.entries as int},inside:h.inside}
}
pub open spec fn frame_header_ok(b:Seq<u8>,h:FrameHeader,c:usize,st:WalkState)->bool {
    &&& 1<=h.rect.width<=MAX_DIMENSION && 1<=h.rect.height<=MAX_DIMENSION
    &&& h.rect.left<=MAX_DIMENSION && h.rect.top<=MAX_DIMENSION
    &&& 2<=h.table.entries<=256
    &&& h.table.offset as int+3*(h.table.entries as int)<=b.len()
    &&& c<=b.len() && st.frames_seen<MAX_FRAMES
    &&& (st.total_pixels as int)+(h.rect.width as int)*(h.rect.height as int)<=MAX_TOTAL_PIXELS
}
pub open spec fn descriptor_post(b:Seq<u8>,p:usize,hdr:Header,st:WalkState,pol:Policy,r:Result<(FrameHeader,usize),Error>)->bool {
    match r {
        Err(e)=>crate::model::image_step(b,header_model(hdr),pol,walk_m_at(st,p))==Err(e),
        Ok((fh,c))=>frame_header_ok(b,fh,c,st) && p as int+10<=c as int && crate::model::image_step(b,header_model(hdr),pol,walk_m_at(st,p))
            ==crate::model::finish_image(b,pol,walk_m_at(st,p),frame_header_model(fh),(fh.rect.width as int)*(fh.rect.height as int),c as int),
    }
}
pub proof fn lemma_descriptor_area(w:u32,h:u32,total:u64)
    requires w<=MAX_DIMENSION,h<=MAX_DIMENSION,total<=MAX_TOTAL_PIXELS,
    ensures (w as int)*(h as int)<= (MAX_DIMENSION as int)*(MAX_DIMENSION as int),
        (total as int)+(w as int)*(h as int)<=u64::MAX,
{
    assert((w as int)*(h as int)<= (MAX_DIMENSION as int)*(MAX_DIMENSION as int)) by(nonlinear_arith)
        requires 0<=w as int<=MAX_DIMENSION,0<=h as int<=MAX_DIMENSION;
    assert((MAX_TOTAL_PIXELS as int)+(MAX_DIMENSION as int)*(MAX_DIMENSION as int)<=u64::MAX) by(compute);
}

/// Exact G8 descriptor prefix; successful results leave G9 to `scan_image_data`.
pub fn parse_image_descriptor(b:&[u8],p:usize,hdr:&Header,st:&WalkState,pol:&Policy)->(r:Result<(FrameHeader,usize),Error>)
    requires p<=b.len(),b.len()<=MAX_INPUT,hdr_inv(b@,*hdr),st.total_pixels<=MAX_TOTAL_PIXELS,
    ensures descriptor_post(b@,p,*hdr,*st,*pol,r),
{
    hide(crate::model::finish_image);
    if st.frames_seen>=MAX_FRAMES {return Err(Error::TooManyFrames);}
    if !need(b,p,10) {return Err(Error::TruncatedImageDescriptor);}
    let left=le16(b,p+1) as u32;
    let top=le16(b,p+3) as u32;
    let w=le16(b,p+5) as u32;
    let h=le16(b,p+7) as u32;
    let packed=get(b,p+9);
    let mut c=p+10;
    if w==0 || h==0 {return Err(Error::InvalidFrameDimensions);}
    let inside=left+w<=hdr.width as u32 && top+h<=hdr.height as u32;
    let reject=match pol.frame_bounds {FrameBounds::Reject=>true,FrameBounds::Clip=>false};
    if !inside && reject {return Err(Error::FrameOutsideScreen);}
    proof {lemma_descriptor_area(w,h,st.total_pixels);}
    if st.total_pixels+(w as u64)*(h as u64)>MAX_TOTAL_PIXELS {return Err(Error::PixelBudgetExceeded);}
    let mut table=hdr.global;
    if packed&0x80!=0 {
        let entries=table_entries(packed);
        let n=3*entries as usize;
        if !need(b,c,n) {return Err(Error::TruncatedColorTable);}
        table=Some(ColorTable {offset:c,entries});
        c+=n;
    }
    let table=match table {Some(t)=>t,None=>return Err(Error::NoColorTable)};
    Ok((FrameHeader {rect:Rect {left,top,width:w,height:h},interlaced:packed&0x40!=0,table,inside},c))
}
} // verus!

verus! {
/// G9's policy check, made before scanning the image sub-blocks.
pub open spec fn lzw_min_ok(kind:LzwMin,m:int,area:int)->bool {
    match kind {
        LzwMin::Spec=>2<=m<=8,
        LzwMin::Range1to11Checked=>1<=m<=11 && (m!=1 || area==1),
    }
}

pub open spec fn image_data_post(b:Seq<u8>,p:int,pol:Policy,area:int,r:Result<ImageData,Error>)->bool {
    if !crate::model::have(b,p,1) {r is Err && r.unwrap_err()==Error::TruncatedImageData}
    else if !lzw_min_ok(pol.lzw_min,b[p] as int,area) {r is Err && r.unwrap_err()==Error::InvalidLzwMinCodeSize}
    else {match crate::model::sub_scan(b,p+1) {
        None=>r is Err && r.unwrap_err()==Error::TruncatedImageData,
        Some((ps,e))=>match r {
            Err(_)=>false,
            Ok(data)=> {
                &&& data.min_code_size as int==b[p] as int
                &&& data.first as int==p+1
                &&& data.end as int==e
                &&& data.data_len as int==crate::model::concat_len(ps)
                &&& data.data_len as int==crate::model::concat(ps).len()
                &&& data.data_len<=crate::limits::MAX_INPUT
                &&& lzw_min_ok(pol.lzw_min,data.min_code_size as int,area)
            },
        },
    }}
}

/// G9 image data: minimum code size (policy `lzw_min`, `area` = descriptor
/// `w·h`), then the whole sub-block sequence before any code is decoded.
pub fn scan_image_data(b: &[u8], p: usize, pol: &Policy, area: u64) -> (r: Result<ImageData, Error>)
    requires p<=b.len(),b.len()<=crate::limits::MAX_INPUT,
    ensures image_data_post(b@,p as int,*pol,area as int,r),
{
    if !need(b, p, 1) { return Err(Error::TruncatedImageData); }
    let m = get(b, p) as u32;
    match pol.lzw_min {
        LzwMin::Spec => {
            if !(2 <= m && m <= 8) { return Err(Error::InvalidLzwMinCodeSize); }
        }
        LzwMin::Range1to11Checked => {
            if !(1 <= m && m <= 11) { return Err(Error::InvalidLzwMinCodeSize); }
            if m == 1 && area != 1 { return Err(Error::InvalidLzwMinCodeSize); }
        }
    }
    let (end, total) = read_sub_blocks(b, p + 1, Error::TruncatedImageData)?;
    proof {
        let scanned=crate::model::sub_scan(b@,p as int+1).unwrap();
        crate::model::lemma_concat_len(scanned.0,Seq::empty());
    }
    // G9's scan bounds the payload total by the input slice before narrowing.
    let data_len = if total > u32::MAX as u64 { u32::MAX } else { total as u32 };
    Ok(ImageData { min_code_size: m, first: p + 1, data_len, end })
}
} // verus!

verus! {
#[derive(Clone,Copy,Debug)]
pub struct ImageStep {
    pub fh:FrameHeader,
    pub data:ImageData,
    pub gce:Option<Gce>,
}

pub open spec fn data_step_post(b:Seq<u8>,c:usize,pol:Policy,area:u64,oldst:WalkState,st:WalkState,r:Result<(ImageData,Option<Gce>),Error>)->bool {
    match crate::model::data_step(b,pol,walk_m_unused(oldst),c as int,area as int) {
        Err(e)=>r is Err && r.unwrap_err()==e && walk_eq(oldst,st),
        Ok((m,d,ms))=>match r {
            Err(_)=>false,
            Ok((data,g))=>data.min_code_size as int==m && crate::lzw::read_d(b,data.first as int)==d
                && st_matches(data.end,st,ms) && g==oldst.gce && image_data_post(b,c as int,pol,area as int,Ok(data)),
        },
    }
}

pub proof fn lemma_data_model(b:Seq<u8>,c:int,pol:Policy,area:int,st:WalkState,r:Result<ImageData,Error>)
    requires image_data_post(b,c,pol,area,r),
    ensures match r {
        Err(e)=>crate::model::data_step(b,pol,walk_m_unused(st),c,area)==Err(e),
        Ok(data)=>crate::model::data_step(b,pol,walk_m_unused(st),c,area)==Ok((data.min_code_size as int,crate::lzw::read_d(b,data.first as int),
            crate::model::WalkM {p:data.end as int,gce:None,frames_seen:st.frames_seen as int+1,total_pixels:st.total_pixels as int+area,loop_count:st.loop_count})),
    },
{}

/// G9's actual state update follows the complete structural scan.
pub fn parse_data_step(b:&[u8],c:usize,pol:&Policy,area:u64,st:&mut WalkState)->(r:Result<(ImageData,Option<Gce>),Error>)
    requires c<=b.len(),b.len()<=MAX_INPUT,old(st).frames_seen<MAX_FRAMES,
        (old(st).total_pixels as int)+(area as int)<=MAX_TOTAL_PIXELS,
    ensures data_step_post(b@,c,*pol,area,*old(st),*final(st),r),
        final(st).loop_count==old(st).loop_count,
        r is Ok ==> final(st).frames_seen as int==old(st).frames_seen as int+1,
{
    let result=scan_image_data(b,c,pol,area);
    proof {lemma_data_model(b@,c as int,*pol,area as int,*st,result);}
    let data=match result {Ok(data)=>data,Err(e)=>return Err(e)};
    let gce=st.gce;
    st.gce=None;
    st.frames_seen+=1;
    st.total_pixels+=area;
    Ok((data,gce))
}

pub open spec fn image_step_post(b:Seq<u8>,p:usize,hdr:Header,pol:Policy,oldst:WalkState,st:WalkState,r:Result<ImageStep,Error>)->bool {
    match crate::model::image_step(b,header_model(hdr),pol,walk_m_at(oldst,p)) {
        Err(e)=>r is Err && r.unwrap_err()==e && walk_eq(oldst,st),
        Ok((fh,g,m,d,ms))=>match r {
            Err(_)=>false,
            Ok(image)=>frame_header_model(image.fh)==fh && image.gce==g && image.data.min_code_size as int==m
                && crate::lzw::read_d(b,image.data.first as int)==d && st_matches(image.data.end,st,ms)
                && frame_header_ok(b,image.fh,image.data.first,oldst)
                && p<image.data.first
                && image_data_post(b,image.data.first as int-1,pol,(image.fh.rect.width as int)*(image.fh.rect.height as int),Ok(image.data))
                && lzw_min_ok(pol.lzw_min,m,(image.fh.rect.width as int)*(image.fh.rect.height as int))
                && crate::model::sub_scan(b,image.data.first as int) is Some
                && crate::lzw::read_d(b,image.data.first as int).len()==image.data.data_len
                && image.data.data_len<=MAX_INPUT,
        },
    }
}

/// Exact executable G8-G9 image step, including frame/pixel counters and GCE
/// consumption, used by the frame path before disposal, allocation or LZW.
pub fn parse_image_step(b:&[u8],p:usize,hdr:&Header,pol:&Policy,st:&mut WalkState)->(r:Result<ImageStep,Error>)
    requires p<=b.len(),b.len()<=MAX_INPUT,hdr_inv(b@,*hdr),old(st).total_pixels<=MAX_TOTAL_PIXELS,
    ensures image_step_post(b@,p,*hdr,*pol,*old(st),*final(st),r),
        final(st).loop_count==old(st).loop_count,
        r is Ok ==> final(st).frames_seen as int==old(st).frames_seen as int+1,
        final(st).total_pixels<=MAX_TOTAL_PIXELS,
        r is Ok ==> final(st).frames_seen<=MAX_FRAMES,
{
    let ghost start=*st;
    let (fh,c)=parse_image_descriptor(b,p,hdr,st,pol)?;
    proof {lemma_descriptor_area(fh.rect.width,fh.rect.height,st.total_pixels);}
    let area=(fh.rect.width as u64)*(fh.rect.height as u64);
    let (data,gce)=parse_data_step(b,c,pol,area,st)?;
    Ok(ImageStep {fh,data,gce})
}

/// At an image introducer, the executable step is MB's image branch.
pub proof fn lemma_mb_image_branch(b:Seq<u8>,p:usize,hdr:Header,pol:Policy,oldst:WalkState,st:WalkState,r:Result<ImageStep,Error>)
    requires p<b.len(),b[p as int]==0x2c,image_step_post(b,p,hdr,pol,oldst,st,r),
    ensures match crate::model::mb(b,header_model(hdr),pol,walk_m_at(oldst,p)) {
        Err(e)=>r is Err && r.unwrap_err()==e,
        Ok(out)=>match out {
            crate::model::MbOut::Trailer=>false,
            crate::model::MbOut::Image(fh,g,m,d,ms)=>match r {
                Err(_)=>false,Ok(image)=>frame_header_model(image.fh)==fh && image.gce==g && image.data.min_code_size as int==m
                    && crate::lzw::read_d(b,image.data.first as int)==d && st_matches(image.data.end,st,ms),
            },
        },
    },
{}
} // verus!

verus! {
pub proof fn lemma_image_end(b:Seq<u8>,pol:Policy,p:usize,image:ImageStep)
    requires p<image.data.first,
        image_data_post(b,image.data.first as int-1,pol,(image.fh.rect.width as int)*(image.fh.rect.height as int),Ok(image.data)),
    ensures p<image.data.end<=b.len(),
{
    let scan=crate::model::sub_scan(b,image.data.first as int).unwrap();
    lemma_sub_scan_bounds(b,image.data.first as int,scan.0,scan.1);
}

pub open spec fn mb_exec_post(b:Seq<u8>,p0:usize,hdr:Header,pol:Policy,s0:WalkState,p:usize,st:WalkState,r:Result<Option<ImageStep>,Error>)->bool {
    match crate::model::mb(b,header_model(hdr),pol,walk_m_at(s0,p0)) {
        Err(e)=>r is Err && r.unwrap_err()==e,
        Ok(crate::model::MbOut::Trailer)=>r is Ok && r.unwrap() is None,
        Ok(crate::model::MbOut::Image(fh,g,m,d,ms))=>match r {
            Ok(Some(image))=>frame_header_model(image.fh)==fh && image.gce==g && image.data.min_code_size as int==m
                && crate::lzw::read_d(b,image.data.first as int)==d && st_matches(p,st,ms) && p==image.data.end
                && frame_header_ok(b,image.fh,image.data.first,s0)
                && image_data_post(b,image.data.first as int-1,pol,(image.fh.rect.width as int)*(image.fh.rect.height as int),Ok(image.data)),
            _=>false,
        },
    }
}

/// MB's executable block walk: extensions, exact Trailer checks, then one
/// descriptor/data step. The model fuel counts extensions; the executable
/// variant counts bytes still available at the increasing cursor.
pub fn parse_mb(b:&[u8],p:&mut usize,hdr:&Header,pol:&Policy,st:&mut WalkState)->(r:Result<Option<ImageStep>,Error>)
    requires *old(p)<=b.len(),b.len()<=MAX_INPUT,hdr_inv(b@,*hdr),
        old(st).total_pixels<=MAX_TOTAL_PIXELS,old(st).frames_seen<=MAX_FRAMES,
    ensures mb_exec_post(b@,*old(p),*hdr,*pol,*old(st),*final(p),*final(st),r),
        r is Ok && r.unwrap() is Some ==> final(st).frames_seen as int==old(st).frames_seen as int+1,

        old(st).frames_seen>=1 || *old(p)<b.len() && b@[*old(p) as int]!=0x21
            ==> final(st).loop_count==old(st).loop_count,
        *final(p)<=b.len(),final(st).total_pixels<=MAX_TOTAL_PIXELS,final(st).frames_seen<=MAX_FRAMES,
{
    hide(crate::model::mb_fuel);
    let missing_reject=match pol.missing_trailer {crate::policy::MissingTrailer::Reject=>true,_=>false};
    let orphan_reject=match pol.gce_orphan {crate::policy::GceOrphan::Reject=>true,_=>false};
    let trailing_reject=match pol.trailing {crate::policy::Trailing::Reject=>true,_=>false};
    let ghost initial=crate::model::mb(b@,header_model(*hdr),*pol,walk_m_at(*st,*p));
    let ghost mut fuel=(b.len()-*p) as nat;
    loop
        invariant *p<=b.len(),b.len()<=MAX_INPUT,hdr_inv(b@,*hdr),
            st.total_pixels<=MAX_TOTAL_PIXELS,st.frames_seen<=MAX_FRAMES,
            st.frames_seen==old(st).frames_seen,st.total_pixels==old(st).total_pixels,
            fuel>=b.len()-*p,
            missing_reject==(pol.missing_trailer==crate::policy::MissingTrailer::Reject),
            orphan_reject==(pol.gce_orphan==crate::policy::GceOrphan::Reject),
            trailing_reject==(pol.trailing==crate::policy::Trailing::Reject),
            old(st).frames_seen>=1 || *old(p)<b.len() && b@[*old(p) as int]!=0x21
                ==> st.loop_count==old(st).loop_count,
            old(st).frames_seen==0 && *old(p)<b.len() && b@[*old(p) as int]!=0x21 ==> *p==*old(p),
            initial==crate::model::mb(b@,header_model(*hdr),*pol,walk_m_at(*old(st),*old(p))),
            initial==crate::model::mb_fuel(b@,header_model(*hdr),*pol,walk_m_at(*st,*p),fuel),
        decreases b.len()-*p,
    {
        proof {reveal_with_fuel(crate::model::mb_fuel,1);}
        let c=if *p>=b.len() {
            if missing_reject {return Err(Error::MissingTrailer);}
            0x3bu8
        } else {get(b,*p)};
        if c==0x3b {
            let have_gce=match st.gce {Some(_)=>true,None=>false};
            if have_gce && orphan_reject {return Err(Error::GraphicControlWithoutGraphic);}
            if *p<b.len() && b.len()-*p>1 && trailing_reject {return Err(Error::TrailingBytes);}
            return Ok(None);
        }
        if c==0x21 {
            let ghost before=walk_m_at(*st,*p);
            let end=parse_extension(b,*p,st,pol)?;
            proof {
                let ns=crate::model::ext_step(b@,*pol,before).unwrap();
                assert(ns==walk_m_at(*st,end));
                fuel=(fuel-1) as nat;
            }
            *p=end;
            continue;
        }
        if c==0x2c {
            let image=parse_image_step(b,*p,hdr,pol,st)?;
            proof {lemma_image_end(b@,*pol,*p,image);}
            *p=image.data.end;
            return Ok(Some(image));
        }
        return Err(Error::UnknownBlock);
    }
}
} // verus!

verus! {
#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum PrefixStop {AtImage,Trailer}

pub open spec fn mp_exec_post(b:Seq<u8>,p0:usize,pol:Policy,s0:WalkState,p:usize,st:WalkState,r:Result<PrefixStop,Error>)->bool {
    match crate::model::mp_fuel(b,pol,walk_m_at(s0,p0),(b.len()-p0) as nat) {
        Err(e)=>r is Err && r.unwrap_err()==e,
        Ok(ms)=>r is Ok && st_matches(p,st,ms) && match r.unwrap() {
            PrefixStop::AtImage=>p<b.len() && b[p as int]==0x2c,
            PrefixStop::Trailer=>p==b.len() || p<b.len() && b[p as int]==0x3b,
        },
    }
}

/// Constructor walk: extensions and trailer checks, never descriptor/data work.
pub fn parse_mp(b:&[u8],p:&mut usize,pol:&Policy,st:&mut WalkState)->(r:Result<PrefixStop,Error>)
    requires *old(p)<=b.len(),b.len()<=MAX_INPUT,
    ensures mp_exec_post(b@,*old(p),*pol,*old(st),*final(p),*final(st),r),
        *final(p)<=b.len(),final(st).frames_seen==old(st).frames_seen,final(st).total_pixels==old(st).total_pixels,
{
    hide(crate::model::mp_fuel);
    let missing_reject=match pol.missing_trailer {crate::policy::MissingTrailer::Reject=>true,_=>false};
    let orphan_reject=match pol.gce_orphan {crate::policy::GceOrphan::Reject=>true,_=>false};
    let trailing_reject=match pol.trailing {crate::policy::Trailing::Reject=>true,_=>false};
    let ghost initial=crate::model::mp_fuel(b@,*pol,walk_m_at(*st,*p),(b.len()-*p) as nat);
    let ghost mut fuel=(b.len()-*p) as nat;
    loop
        invariant *p<=b.len(),b.len()<=MAX_INPUT,
            st.frames_seen==old(st).frames_seen,st.total_pixels==old(st).total_pixels,
            fuel>=b.len()-*p,
            missing_reject==(pol.missing_trailer==crate::policy::MissingTrailer::Reject),
            orphan_reject==(pol.gce_orphan==crate::policy::GceOrphan::Reject),
            trailing_reject==(pol.trailing==crate::policy::Trailing::Reject),
            initial==crate::model::mp_fuel(b@,*pol,walk_m_at(*old(st),*old(p)),(b.len()-*old(p)) as nat),
            initial==crate::model::mp_fuel(b@,*pol,walk_m_at(*st,*p),fuel),
        decreases b.len()-*p,
    {
        proof {reveal_with_fuel(crate::model::mp_fuel,1);}
        let c=if *p>=b.len() {
            if missing_reject {return Err(Error::MissingTrailer);}
            0x3bu8
        } else {get(b,*p)};
        if c==0x3b {
            let have_gce=match st.gce {Some(_)=>true,None=>false};
            if have_gce && orphan_reject {return Err(Error::GraphicControlWithoutGraphic);}
            if *p<b.len() && b.len()-*p>1 && trailing_reject {return Err(Error::TrailingBytes);}
            return Ok(PrefixStop::Trailer);
        }
        if c==0x21 {
            let ghost before=walk_m_at(*st,*p);
            let end=parse_extension(b,*p,st,pol)?;
            proof {
                let ns=crate::model::ext_step(b@,*pol,before).unwrap();
                assert(ns==walk_m_at(*st,end));fuel=(fuel-1) as nat;
            }
            *p=end;continue;
        }
        if c==0x2c {return Ok(PrefixStop::AtImage);}
        return Err(Error::UnknownBlock);
    }
}

pub proof fn lemma_header_model_matches(b:Seq<u8>,h:Header,m:crate::model::HeaderM)
    requires hdr_matches(b,m,h),
    ensures m==header_model(h),
{}
} // verus!
