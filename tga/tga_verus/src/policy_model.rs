//! The policy-indexed header and conversion model
//! (THEOREMS-POLICY-DELTA.md "Model and executable boundary").
//!
//! `mh_policy` and `conv2_policy` are the 1.0 models `model::mh` and
//! `model::conv2` re-stated over an arbitrary [`crate::policy::Policy`] record,
//! following POLICY-SPEC.md exactly and written from the specification text.
//!
//! `mh_policy(b, strict_policy())` is `model::mh(b)` and
//! `conv2_policy(.., strict_policy())` is `model::conv2(..)`, proved by the
//! `lemma_*_policy_strict` bridges below, so the existing T5 proof carries over
//! to Strict by unfolding.

use vstd::prelude::*;

use crate::model::*;
use crate::policy::{
    AlphaOptions, AttributeBitsOptions, DepthsOptions, ImageTypeOptions, IndexRangeOptions,
    InterleaveOptions, Map16AlphaOptions, Map5Options, MapHeaderOptions, MapIndexOptions, Policy,
    Rgb5Options,
};

#[cfg(verus_only)]
use crate::policy::{strict_policy, *};

#[cfg(verus_only)]
use crate::tga::Error;

verus! {

// ---------------------------------------------------------------------------
// R8: policy-indexed channel expansion and conversion.
// ---------------------------------------------------------------------------

/// R8 `rgb5 = Round`: `(255c+15) div 31`.
pub open spec fn round5(c: int) -> int {
    (255 * c + 15) / 31
}

/// R8's five-bit triple under the `rgb5` rule.
pub open spec fn rgb555_policy(v: int, rule: Rgb5Options) -> Seq<u8> {
    match rule {
        Rgb5Options::Replicate => rgb555(v),
        Rgb5Options::Round => seq![
            round5(red5(v)) as u8,
            round5(green5(v)) as u8,
            round5(blue5(v)) as u8,
        ],
    }
}

/// R8's five-bit triple under the `map5` rule. `GdkSwapShift` is gdk-pixbuf's
/// assignment: the low five bits to red, bits 5-9 to green, bits 10-14 to blue,
/// each shifted but with no low-bit replication.
pub open spec fn map555_policy(v: int, rule: Map5Options) -> Seq<u8> {
    match rule {
        Map5Options::Replicate => rgb555(v),
        Map5Options::Round => seq![
            round5(red5(v)) as u8,
            round5(green5(v)) as u8,
            round5(blue5(v)) as u8,
        ],
        Map5Options::GdkSwapShift => seq![
            (blue5(v) * 8) as u8,
            (green5(v) * 8) as u8,
            (red5(v) * 8) as u8,
        ],
    }
}

/// R8's 15/16-bit layout under `rgb5`; alpha `a`.
pub open spec fn bytes4_15_policy(v: int, a: u8, rule: Rgb5Options) -> Seq<u8> {
    rgb555_policy(v, rule).push(a)
}

/// R8's alpha rule resolved on the *effective* attribute count `h.attr`: the
/// stored byte carries alpha when `alpha = Stored`, or when `eff = 8`.
pub open spec fn stored_alpha_policy(h: PktH, p: Policy) -> bool {
    p.alpha == AlphaOptions::Stored || h.attr == 8
}

/// R8: convert a validated non-indexed sample under `p`. Same structure as the
/// 1.0 `conv_sample`, with `rgb5` and `alpha` branches.
pub open spec fn conv_sample_policy(h: PktH, s: Seq<u8>, p: Policy) -> Seq<u8> {
    if h.kind == 1 && (h.depth == 15 || h.depth == 16) {
        bytes4_15_policy(le16s(s, 0), 255, p.rgb5)
    } else if h.kind == 1 && h.depth == 24 {
        seq![s[2], s[1], s[0], 255u8]
    } else if h.kind == 1 && h.depth == 32 {
        seq![s[2], s[1], s[0], (if stored_alpha_policy(h, p) { s[3] } else { 255u8 })]
    } else if h.kind == 2 && h.depth == 8 {
        seq![s[0], s[0], s[0], 255u8]
    } else if h.kind == 2 && h.depth == 16 {
        seq![s[0], s[0], s[0], (if stored_alpha_policy(h, p) { s[1] } else { 255u8 })]
    } else {
        // Outside the supported-depth domain (`OutsideDomain`, not an API error).
        seq![0u8, 0, 0, 255]
    }
}

/// R4/R8 `map_index`: the entry number `j` a raw index selects. `Absolute` is
/// 1.0's `index - first`; `ZeroBased` is `index`, `first` ignored. This is the
/// only place `first` can enter an address computation.
pub open spec fn entry_j(index: int, first: int, p: Policy) -> int {
    match p.map_index {
        MapIndexOptions::Absolute => index - first,
        MapIndexOptions::ZeroBased => index,
    }
}

/// R8/R4: the entry-number range test `0 <= j < length` with
/// `j = entry_j(index, first, p)`. Under `Absolute` this is 1.0's
/// `first <= index < first+length`; under `ZeroBased` `first` is absent.
pub open spec fn in_range_policy(h: PktH, index: int, p: Policy) -> bool {
    0 <= entry_j(index, h.map_first, p) < h.map_len
}

/// R8: the 15/16-bit palette entry's alpha under `map16_alpha` (16-bit, count
/// one) and `alpha`.
pub open spec fn map_entry_alpha_policy(h: PktH, v: int, map_depth: int, p: Policy) -> u8 {
    if map_depth == 16 && h.attr == 1 && p.map16_alpha == Map16AlphaOptions::Bit15 {
        if bit15(v) {
            255u8
        } else {
            0u8
        }
    } else {
        255u8
    }
}

/// R8 `index_range = Black`: the substitute of an out-of-range index:
/// `(0,0,0,0)` when the palette entry depth is 32, `(0,0,0,255)` otherwise.
pub open spec fn black_substitute(map_depth: int) -> Seq<u8> {
    seq![0u8, 0u8, 0u8, (if map_depth == 32 { 0u8 } else { 255u8 })]
}

/// R8: convert map entry `index` under `p`. An out-of-range index under
/// `Black` substitutes and reads no map byte; under `Reject` the caller has
/// already returned `ColorIndexOutOfRange`, so the fill is unused.
pub open spec fn conv_map_policy(h: PktH, map: Seq<u8>, index: int, p: Policy) -> Seq<u8> {
    let j = entry_j(index, h.map_first, p);
    if !(0 <= j < h.map_len) {
        if p.index_range == IndexRangeOptions::Black {
            black_substitute(h.map_depth)
        } else {
            seq![0u8, 0, 0, 255]
        }
    } else {
        let off = j * stride_of(h.map_depth);
        if h.map_depth == 15 || h.map_depth == 16 {
            let v = le16s(map, off);
            map555_policy(v, p.map5).push(map_entry_alpha_policy(h, v, h.map_depth, p))
        } else if h.map_depth == 24 {
            seq![map[off + 2], map[off + 1], map[off], 255u8]
        } else if h.map_depth == 32 {
            seq![
                map[off + 2],
                map[off + 1],
                map[off],
                (if stored_alpha_policy(h, p) { map[off + 3] } else { 255u8 }),
            ]
        } else {
            seq![0u8, 0, 0, 255]
        }
    }
}

/// R8: the complete conversion of one sample under `p`: an indexed sample
/// first reads its index and then converts the corresponding map entry.
pub open spec fn conv2_policy(h: PktH, map: Seq<u8>, s: Seq<u8>, p: Policy) -> Seq<u8> {
    if h.kind == 0 {
        conv_map_policy(h, map, sindex(s, h.sample_bytes), p)
    } else {
        conv_sample_policy(h, s, p)
    }
}

// ---------------------------------------------------------------------------
// R1/R2/R3/R4/R5: the policy-indexed header classification.
// ---------------------------------------------------------------------------

/// R1: the image kind of a supported type byte (0 indexed, 1 true-colour,
/// 2 greyscale).
pub open spec fn kind_of_type(t: u8) -> int {
    if is_indexed(t) {
        0
    } else if t == 2u8 || t == 10u8 {
        1
    } else {
        2
    }
}

/// R1 `image_type`: the accepted type byte.
pub open spec fn type_ok_policy(t: u8, p: Policy) -> bool {
    type_ok(t) || p.image_type == ImageTypeOptions::GreyFallback
}

/// R1: the kind of a type byte under `p`. A non-`type_ok` byte is accepted only
/// under `GreyFallback`, and is then greyscale.
pub open spec fn kind_policy(t: u8, p: Policy) -> int {
    if type_ok(t) {
        kind_of_type(t)
    } else {
        2
    }
}

/// R1: is the image RLE-encoded? RLE iff the type is 9/10/11 (a raw fallback
/// type is never RLE).
pub open spec fn rle_policy(t: u8, p: Policy) -> bool {
    t == 9u8 || t == 10u8 || t == 11u8
}

/// R2 `map_header`: does the flag byte satisfy the profile's rule?
pub open spec fn map_flag_ok_policy(b: Seq<u8>, p: Policy) -> bool {
    match p.map_header {
        MapHeaderOptions::Core | MapHeaderOptions::IgnoreAbsent => b[1] <= 1u8,
        MapHeaderOptions::Gdk => true,
        MapHeaderOptions::Image => true,
    }
}

/// R2 `map_header`: is a colour map present?
pub open spec fn map_present_policy(b: Seq<u8>, p: Policy) -> bool {
    match p.map_header {
        MapHeaderOptions::Core | MapHeaderOptions::IgnoreAbsent => b[1] == 1u8,
        MapHeaderOptions::Gdk => b[1] != 0u8,
        MapHeaderOptions::Image => b[1] == 1u8,
    }
}

/// R3 `depths`: the supported depth table of `p`.
pub open spec fn depth_ok_policy(kind: int, d: u8, o: DepthsOptions) -> bool {
    match o {
        DepthsOptions::Core | DepthsOptions::Image => {
            if kind == 0 {
                d == 8u8 || d == 16u8
            } else if kind == 1 {
                d == 15u8 || d == 16u8 || d == 24u8 || d == 32u8
            } else {
                d == 8u8 || d == 16u8
            }
        },
        DepthsOptions::Gdk => {
            if kind == 0 {
                d == 8u8
            } else if kind == 1 {
                d == 16u8 || d == 24u8 || d == 32u8
            } else {
                d == 8u8 || d == 16u8
            }
        },
    }
}

/// R5: the CORE attribute-count set for element class `kind` at depth `d`.
/// The map and true-colour classes share `{15:(0), 16:(0,1), 24:(0), 32:(0,8)}`;
/// the greyscale class is `{8:(0), 16:(0,8)}`. An unknown depth allows only 0
/// (the 1.0 `attr_ok` default).
pub open spec fn core_attr_allowed(kind: int, d: int, count: int) -> bool {
    if kind == 2 {
        if d == 8 {
            count == 0
        } else if d == 16 {
            count == 0 || count == 8
        } else {
            count == 0
        }
    } else {
        if d == 15 || d == 24 {
            count == 0
        } else if d == 16 {
            count == 0 || count == 1
        } else if d == 32 {
            count == 0 || count == 8
        } else {
            count == 0
        }
    }
}

/// R5: the ImageTable attribute-count set: CORE, except that 8-bit grey allows
/// `{0,8}` and 16-bit grey allows only `{8}`.
pub open spec fn image_attr_allowed(kind: int, d: int, count: int) -> bool {
    if kind == 2 && d == 8 {
        count == 0 || count == 8
    } else if kind == 2 && d == 16 {
        count == 8
    } else {
        core_attr_allowed(kind, d, count)
    }
}

/// R5: the count table `attribute_bits` consults (`Lenient` reuses the CORE
/// table to pick its replacement value).
pub open spec fn attr_table_allowed(
    kind: int,
    d: int,
    count: int,
    o: AttributeBitsOptions,
) -> bool {
    match o {
        AttributeBitsOptions::ImageTable => image_attr_allowed(kind, d, count),
        AttributeBitsOptions::Exact => core_attr_allowed(kind, d, count),
        AttributeBitsOptions::Lenient => core_attr_allowed(kind, d, count),
    }
}

/// R5: does the count pass the option's *acceptance* test? `Lenient` accepts
/// every count (a rejected one becomes `eff_attr`); the other two require the
/// table to allow it.
pub open spec fn attr_ok_policy(
    kind: int,
    d: int,
    count: int,
    o: AttributeBitsOptions,
) -> bool {
    match o {
        AttributeBitsOptions::Lenient => true,
        AttributeBitsOptions::Exact => core_attr_allowed(kind, d, count),
        AttributeBitsOptions::ImageTable => image_attr_allowed(kind, d, count),
    }
}

/// R5: the effective attribute count when the record passes R5. For `Exact`
/// and `ImageTable` this is the descriptor count (both tables allow it on a
/// passing record); for `Lenient` a rejected count becomes 8 (when the element
/// has an 8-bit attribute slot) or 0.
pub open spec fn eff_attr(kind: int, d: int, count: int, o: AttributeBitsOptions) -> int {
    if attr_table_allowed(kind, d, count, o) {
        count
    } else if attr_table_allowed(kind, d, 8, o) {
        8
    } else {
        0
    }
}

/// R4: the map coherence/entry-depth failure that returns `InvalidColorMap`,
/// under the `map_header` option.
pub open spec fn map_coherence_bad(
    b: Seq<u8>,
    present: bool,
    kind: int,
    o: MapHeaderOptions,
) -> bool {
    let first = le16(b, 3);
    let length = le16(b, 5);
    let edepth = b[7];
    match o {
        MapHeaderOptions::Core => {
            if !present {
                first != 0 || length != 0 || edepth != 0u8 || kind == 0
            } else {
                length == 0 || !map_entry_depth_ok(edepth)
            }
        },
        MapHeaderOptions::IgnoreAbsent => {
            if !present {
                kind == 0
            } else {
                length == 0 || !map_entry_depth_ok(edepth)
            }
        },
        MapHeaderOptions::Gdk => {
            if !present {
                kind == 0
            } else {
                length != 0 && !map_entry_depth_ok(edepth)
            }
        },
        MapHeaderOptions::Image => {
            if !present {
                kind == 0
            } else {
                !map_entry_depth_ok(edepth)
            }
        },
    }
}

/// R4: the range failure `first + length > 65,536`. Only `Core` and
/// `IgnoreAbsent` test it, and only when a map is present.
pub open spec fn map_range_bad(b: Seq<u8>, present: bool, o: MapHeaderOptions) -> bool {
    match o {
        MapHeaderOptions::Core | MapHeaderOptions::IgnoreAbsent => {
            present && le16(b, 3) + le16(b, 5) > 65536
        },
        MapHeaderOptions::Gdk => false,
        MapHeaderOptions::Image => false,
    }
}

/// R5: the depth that governs the attribute table: the map entry depth for an
/// indexed image, the image depth otherwise.
pub open spec fn attr_depth(b: Seq<u8>, p: Policy) -> int {
    if kind_policy(b[2], p) == 0 {
        b[7] as int
    } else {
        b[16] as int
    }
}

/// R3 `depths = Image`: an indexed image whose index depth exceeds its entry
/// depth, checked immediately after the map checks.
pub open spec fn image_index_depth_bad(
    b: Seq<u8>,
    present: bool,
    kind: int,
    o: DepthsOptions,
) -> bool {
    o == DepthsOptions::Image && kind == 0 && present && (b[16] as int) > (b[7] as int)
}

/// The map-slice length under `p`: `length*ceil(edepth/8)` when a map is
/// present, else 0 (including `Gdk`/`Image` zero-length maps).
pub open spec fn map_bytes_policy(b: Seq<u8>, p: Policy) -> int {
    if map_present_policy(b, p) {
        le16(b, 5) * sample_bytes_of(b[7])
    } else {
        0
    }
}

/// `MH(b,p)`: the policy-indexed R1-R7 header model. The R14 order is kept
/// wherever a check survives: input cap, header length, type, flag, dimensions,
/// interleave, depth, map coherence, map range, `Image` index depth, attribute
/// count, output cap. A relaxed check never moves another check earlier.
pub open spec fn mh_policy(b: Seq<u8>, p: Policy) -> Result<MHModel, Error> {
    if b.len() > max_input() {
        Err(Error::FileTooLarge)
    } else if b.len() < 18 {
        Err(Error::TruncatedHeader)
    } else if !type_ok_policy(b[2], p) {
        Err(Error::UnsupportedImageType)
    } else if !map_flag_ok_policy(b, p) {
        Err(Error::InvalidColorMapFlag)
    } else if !(1 <= le16(b, 12) && le16(b, 12) <= max_dimension())
        || !(1 <= le16(b, 14) && le16(b, 14) <= max_dimension()) {
        Err(Error::InvalidDimensions)
    } else if b[17] & 0xc0u8 != 0u8 && p.interleave == InterleaveOptions::Reject {
        Err(Error::UnsupportedInterleave)
    } else if !depth_ok_policy(kind_policy(b[2], p), b[16], p.depths) {
        Err(Error::UnsupportedPixelDepth)
    } else if map_coherence_bad(b, map_present_policy(b, p), kind_policy(b[2], p), p.map_header) {
        Err(Error::InvalidColorMap)
    } else if map_range_bad(b, map_present_policy(b, p), p.map_header) {
        Err(Error::ColorMapRange)
    } else if image_index_depth_bad(b, map_present_policy(b, p), kind_policy(b[2], p), p.depths) {
        Err(Error::UnsupportedPixelDepth)
    } else if !attr_ok_policy(
        kind_policy(b[2], p),
        attr_depth(b, p),
        (b[17] & 0x0fu8) as int,
        p.attribute_bits,
    ) {
        Err(Error::InvalidAttributeBits)
    } else if 4 * (le16(b, 12) * le16(b, 14)) > max_output() {
        Err(Error::OutputTooLarge)
    } else {
        Ok(
            MHModel {
                id_length: b[0],
                color_map_flag: b[1],
                image_type: b[2],
                map_first: le16(b, 3) as u32,
                map_length: le16(b, 5) as u32,
                map_entry_depth: b[7],
                x_origin: le16(b, 8) as u32,
                y_origin: le16(b, 10) as u32,
                width: le16(b, 12) as u32,
                height: le16(b, 14) as u32,
                pixel_depth: b[16],
                descriptor: b[17],
                n: le16(b, 12) * le16(b, 14),
                sample_bytes: sample_bytes_of(b[16]),
                map_bytes: map_bytes_policy(b, p),
            },
        )
    }
}

// ---------------------------------------------------------------------------
// The header-indexed variants of R2/R4/R5, for the policy executable header
// parser. Each mirrors the model function, reading the field values a parsed
// `Header` already carries rather than the input slice; the `*_flags_eq` lemmas
// below show the two forms agree on the input the header was parsed from.
// ---------------------------------------------------------------------------

/// R2 `map_header` over a raw flag byte.
pub open spec fn map_flag_ok_policy_flags(flag: u8, p: Policy) -> bool {
    match p.map_header {
        MapHeaderOptions::Core | MapHeaderOptions::IgnoreAbsent => flag <= 1u8,
        MapHeaderOptions::Gdk => true,
        MapHeaderOptions::Image => true,
    }
}

/// R2 `map_header`: is a colour map present, from a raw flag byte?
pub open spec fn map_present_policy_flags(flag: u8, p: Policy) -> bool {
    match p.map_header {
        MapHeaderOptions::Core | MapHeaderOptions::IgnoreAbsent => flag == 1u8,
        MapHeaderOptions::Gdk => flag != 0u8,
        MapHeaderOptions::Image => flag == 1u8,
    }
}

/// R4's coherence/entry-depth failure over parsed field values.
pub open spec fn map_coherence_bad_flags(
    first: int,
    length: int,
    edepth: u8,
    present: bool,
    kind: int,
    o: MapHeaderOptions,
) -> bool {
    match o {
        MapHeaderOptions::Core => {
            if !present {
                first != 0 || length != 0 || edepth != 0u8 || kind == 0
            } else {
                length == 0 || !map_entry_depth_ok(edepth)
            }
        },
        MapHeaderOptions::IgnoreAbsent => {
            if !present {
                kind == 0
            } else {
                length == 0 || !map_entry_depth_ok(edepth)
            }
        },
        MapHeaderOptions::Gdk => {
            if !present {
                kind == 0
            } else {
                length != 0 && !map_entry_depth_ok(edepth)
            }
        },
        MapHeaderOptions::Image => {
            if !present {
                kind == 0
            } else {
                !map_entry_depth_ok(edepth)
            }
        },
    }
}

/// R4's range failure `first + length > 65,536` over parsed field values.
pub open spec fn map_range_bad_flags(
    first: int,
    length: int,
    present: bool,
    o: MapHeaderOptions,
) -> bool {
    match o {
        MapHeaderOptions::Core | MapHeaderOptions::IgnoreAbsent => present && first + length > 65536,
        MapHeaderOptions::Gdk => false,
        MapHeaderOptions::Image => false,
    }
}

/// R3 `depths = Image`'s index-depth test over parsed field values.
pub open spec fn image_index_depth_bad_flags(
    present: bool,
    kind: int,
    depth: u8,
    edepth: u8,
    o: DepthsOptions,
) -> bool {
    o == DepthsOptions::Image && kind == 0 && present && (depth as int) > (edepth as int)
}

/// The map slice length under `p`, from a raw flag byte.
pub open spec fn map_bytes_flags(flag: u8, length: int, edepth: u8, p: Policy) -> int {
    if map_present_policy_flags(flag, p) {
        length * sample_bytes_of(edepth)
    } else {
        0
    }
}

/// `mh_policy` restated over parsed field values, for the policy header parser.
/// The R14 order and every guard are the model's own; the `*_flags` variants
/// above are the only difference, and `lemma_mh_policy_eq_of_header` shows this
/// equals `mh_policy` on the input the fields came from.
pub open spec fn mh_policy_of(
    id_length: u8,
    flag: u8,
    image_type: u8,
    map_first: int,
    map_length: int,
    edepth: u8,
    x_origin: int,
    y_origin: int,
    width: int,
    height: int,
    depth: u8,
    descriptor: u8,
    p: Policy,
) -> Result<MHModel, Error> {
    let kind = kind_policy(image_type, p);
    if !type_ok_policy(image_type, p) {
        Err(Error::UnsupportedImageType)
    } else if !map_flag_ok_policy_flags(flag, p) {
        Err(Error::InvalidColorMapFlag)
    } else if !(1 <= width && width <= max_dimension()) || !(1 <= height && height
        <= max_dimension()) {
        Err(Error::InvalidDimensions)
    } else if descriptor & 0xc0u8 != 0u8 && p.interleave == InterleaveOptions::Reject {
        Err(Error::UnsupportedInterleave)
    } else if !depth_ok_policy(kind, depth, p.depths) {
        Err(Error::UnsupportedPixelDepth)
    } else if map_coherence_bad_flags(
        map_first,
        map_length,
        edepth,
        map_present_policy_flags(flag, p),
        kind,
        p.map_header,
    ) {
        Err(Error::InvalidColorMap)
    } else if map_range_bad_flags(map_first, map_length, map_present_policy_flags(flag, p), p.map_header)
    {
        Err(Error::ColorMapRange)
    } else if image_index_depth_bad_flags(
        map_present_policy_flags(flag, p),
        kind,
        depth,
        edepth,
        p.depths,
    ) {
        Err(Error::UnsupportedPixelDepth)
    } else if !attr_ok_policy(
        kind,
        (if kind == 0 {
            edepth as int
        } else {
            depth as int
        }),
        (descriptor & 0x0fu8) as int,
        p.attribute_bits,
    ) {
        Err(Error::InvalidAttributeBits)
    } else if 4 * (width * height) > max_output() {
        Err(Error::OutputTooLarge)
    } else {
        Ok(
            MHModel {
                id_length,
                color_map_flag: flag,
                image_type,
                map_first: map_first as u32,
                map_length: map_length as u32,
                map_entry_depth: edepth,
                x_origin: x_origin as u32,
                y_origin: y_origin as u32,
                width: width as u32,
                height: height as u32,
                pixel_depth: depth,
                descriptor,
                n: width * height,
                sample_bytes: sample_bytes_of(depth),
                map_bytes: map_bytes_flags(flag, map_length, edepth, p),
            },
        )
    }
}

/// `mh_policy` on the input a `Header` was parsed from equals `mh_policy_of` on
/// that header's fields, provided the fields really are the input's bytes.
pub proof fn lemma_mh_policy_eq_of_header(
    b: Seq<u8>,
    id_length: u8,
    flag: u8,
    image_type: u8,
    map_first: int,
    map_length: int,
    edepth: u8,
    x_origin: int,
    y_origin: int,
    width: int,
    height: int,
    depth: u8,
    descriptor: u8,
    p: Policy,
)
    requires
        18 <= b.len(),
        b.len() <= max_input(),
        b[0] == id_length,
        b[1] == flag,
        b[2] == image_type,
        le16(b, 3) == map_first,
        le16(b, 5) == map_length,
        b[7] == edepth,
        le16(b, 8) == x_origin,
        le16(b, 10) == y_origin,
        le16(b, 12) == width,
        le16(b, 14) == height,
        b[16] == depth,
        b[17] == descriptor,
    ensures
        mh_policy(b, p) == mh_policy_of(
            id_length,
            flag,
            image_type,
            map_first,
            map_length,
            edepth,
            x_origin,
            y_origin,
            width,
            height,
            depth,
            descriptor,
            p,
        ),
{
    reveal(mh_policy);
    reveal(mh_policy_of);
    reveal(map_flag_ok_policy);
    reveal(map_flag_ok_policy_flags);
    reveal(map_present_policy);
    reveal(map_present_policy_flags);
    reveal(map_coherence_bad);
    reveal(map_coherence_bad_flags);
    reveal(map_range_bad);
    reveal(map_range_bad_flags);
    reveal(image_index_depth_bad);
    reveal(image_index_depth_bad_flags);
    reveal(map_bytes_policy);
    reveal(map_bytes_flags);
    reveal(max_dimension);
    reveal(max_output);
    reveal(max_input);
    let mof = mh_policy_of(
        id_length,
        flag,
        image_type,
        map_first,
        map_length,
        edepth,
        x_origin,
        y_origin,
        width,
        height,
        depth,
        descriptor,
        p,
    );
    // The guard conditions of the two functions. `18 <= b.len() <= max_input()`
    // rules out the first two arms of `mh_policy` on both sides.
    assert(type_ok_policy(b[2], p) == type_ok_policy(image_type, p));
    assert(map_flag_ok_policy(b, p) == map_flag_ok_policy_flags(flag, p));
    assert(map_present_policy(b, p) == map_present_policy_flags(flag, p));
    assert(kind_policy(b[2], p) == kind_policy(image_type, p));
    assert(depth_ok_policy(kind_policy(b[2], p), b[16], p.depths) == depth_ok_policy(
        kind_policy(image_type, p),
        depth,
        p.depths,
    ));
    assert(map_coherence_bad(b, map_present_policy(b, p), kind_policy(b[2], p), p.map_header)
        == map_coherence_bad_flags(
        map_first,
        map_length,
        edepth,
        map_present_policy_flags(flag, p),
        kind_policy(image_type, p),
        p.map_header,
    ));
    assert(map_range_bad(b, map_present_policy(b, p), p.map_header) == map_range_bad_flags(
        map_first,
        map_length,
        map_present_policy_flags(flag, p),
        p.map_header,
    ));
    assert(image_index_depth_bad(b, map_present_policy(b, p), kind_policy(b[2], p), p.depths)
        == image_index_depth_bad_flags(
        map_present_policy_flags(flag, p),
        kind_policy(image_type, p),
        depth,
        edepth,
        p.depths,
    ));
    assert((if kind_policy(b[2], p) == 0 {
        b[7] as int
    } else {
        b[16] as int
    }) == (if kind_policy(image_type, p) == 0 {
        edepth as int
    } else {
        depth as int
    }));
    assert((b[17] & 0x0fu8) as int == (descriptor & 0x0fu8) as int);
    assert(map_bytes_policy(b, p) == map_bytes_flags(flag, map_length, edepth, p));
    assert(4 * (le16(b, 12) * le16(b, 14)) == 4 * (width * height));
    assert(le16(b, 12) == width && le16(b, 14) == height);
    if !type_ok_policy(image_type, p) {
        assert(mh_policy(b, p) == Err(Error::UnsupportedImageType));
        assert(mof == Err(Error::UnsupportedImageType));
    } else if !map_flag_ok_policy_flags(flag, p) {
        assert(mh_policy(b, p) == Err(Error::InvalidColorMapFlag));
        assert(mof == Err(Error::InvalidColorMapFlag));
    } else if !(1 <= width && width <= max_dimension()) || !(1 <= height && height
        <= max_dimension()) {
        assert(mh_policy(b, p) == Err(Error::InvalidDimensions));
        assert(mof == Err(Error::InvalidDimensions));
    } else if descriptor & 0xc0u8 != 0u8 && p.interleave == InterleaveOptions::Reject {
        assert(mh_policy(b, p) == Err(Error::UnsupportedInterleave));
        assert(mof == Err(Error::UnsupportedInterleave));
    } else if !depth_ok_policy(kind_policy(image_type, p), depth, p.depths) {
        assert(mh_policy(b, p) == Err(Error::UnsupportedPixelDepth));
        assert(mof == Err(Error::UnsupportedPixelDepth));
    } else if map_coherence_bad_flags(
        map_first,
        map_length,
        edepth,
        map_present_policy_flags(flag, p),
        kind_policy(image_type, p),
        p.map_header,
    ) {
        assert(mh_policy(b, p) == Err(Error::InvalidColorMap));
        assert(mof == Err(Error::InvalidColorMap));
    } else if map_range_bad_flags(map_first, map_length, map_present_policy_flags(flag, p), p.map_header)
    {
        assert(mh_policy(b, p) == Err(Error::ColorMapRange));
        assert(mof == Err(Error::ColorMapRange));
    } else if image_index_depth_bad_flags(
        map_present_policy_flags(flag, p),
        kind_policy(image_type, p),
        depth,
        edepth,
        p.depths,
    ) {
        assert(mh_policy(b, p) == Err(Error::UnsupportedPixelDepth));
        assert(mof == Err(Error::UnsupportedPixelDepth));
    } else if !attr_ok_policy(
        kind_policy(image_type, p),
        (if kind_policy(image_type, p) == 0 {
            edepth as int
        } else {
            depth as int
        }),
        (descriptor & 0x0fu8) as int,
        p.attribute_bits,
    ) {
        assert(mh_policy(b, p) == Err(Error::InvalidAttributeBits));
        assert(mof == Err(Error::InvalidAttributeBits));
    } else if 4 * (width * height) > max_output() {
        assert(mh_policy(b, p) == Err(Error::OutputTooLarge));
        assert(mof == Err(Error::OutputTooLarge));
    } else {
        assert(mh_policy(b, p) == Ok(MHModel {
            id_length: b[0],
            color_map_flag: b[1],
            image_type: b[2],
            map_first: le16(b, 3) as u32,
            map_length: le16(b, 5) as u32,
            map_entry_depth: b[7],
            x_origin: le16(b, 8) as u32,
            y_origin: le16(b, 10) as u32,
            width: le16(b, 12) as u32,
            height: le16(b, 14) as u32,
            pixel_depth: b[16],
            descriptor: b[17],
            n: le16(b, 12) * le16(b, 14),
            sample_bytes: sample_bytes_of(b[16]),
            map_bytes: map_bytes_flags(flag, map_length, edepth, p),
        }));
        assert(mof == Ok(MHModel {
            id_length,
            color_map_flag: flag,
            image_type,
            map_first: map_first as u32,
            map_length: map_length as u32,
            map_entry_depth: edepth,
            x_origin: x_origin as u32,
            y_origin: y_origin as u32,
            width: width as u32,
            height: height as u32,
            pixel_depth: depth,
            descriptor,
            n: width * height,
            sample_bytes: sample_bytes_of(depth),
            map_bytes: map_bytes_flags(flag, map_length, edepth, p),
        }));
        assert(mh_policy(b, p) == mof);
    }
}

/// The conversion-relevant projection of a *policy* header record: the 1.0
/// `pkt_of_mh`, except that `attr` is the effective count and `kind` follows
/// `image_type`.
pub open spec fn pkt_of_mh_policy(m: MHModel, p: Policy) -> PktH {
    let kind = kind_policy(m.image_type, p);
    let d = if kind == 0 {
        m.map_entry_depth as int
    } else {
        m.pixel_depth as int
    };
    PktH {
        kind: kind,
        depth: m.pixel_depth as int,
        attr: eff_attr(kind, d, (m.descriptor & 0x0fu8) as int, p.attribute_bits),
        map_first: m.map_first as int,
        map_len: m.map_length as int,
        map_depth: m.map_entry_depth as int,
        n: m.n,
        sample_bytes: m.sample_bytes,
    }
}

/// R5's alpha rule on the *effective* attribute count, as a plain predicate the
/// executable's `stored_alpha_exec` proves equal to `stored_alpha_policy`.
pub open spec fn stored_alpha_policy_attr(attr: int, p: Policy) -> bool {
    p.alpha == AlphaOptions::Stored || attr == 8
}

/// R8's five-bit expansion of one channel under the `rgb5` rule, as the byte
/// the executable writes.
pub open spec fn rgb5_chan(c: int, rule: Rgb5Options) -> u8 {
    match rule {
        Rgb5Options::Replicate => rep5(c) as u8,
        Rgb5Options::Round => round5(c) as u8,
    }
}

/// R8's five-bit expansion of one channel under the `map5` rule, as the byte
/// the executable writes.
pub open spec fn map5_chan(c: int, rule: Map5Options) -> u8 {
    match rule {
        Map5Options::Replicate => rep5(c) as u8,
        Map5Options::Round => round5(c) as u8,
        Map5Options::GdkSwapShift => (c * 8) as u8,
    }
}

/// `rgb555_policy` is `rgb5_chan` on the three fields.
pub proof fn lemma_rgb555_policy_chan(v: int, rule: Rgb5Options)
    ensures
        rgb555_policy(v, rule) == seq![
            rgb5_chan(red5(v), rule),
            rgb5_chan(green5(v), rule),
            rgb5_chan(blue5(v), rule),
        ],
{
    reveal(rgb555_policy);
    reveal(rgb555);
    reveal(rep5);
    reveal(round5);
    reveal(rgb5_chan);
    match rule {
        Rgb5Options::Replicate => {},
        Rgb5Options::Round => {},
    }
}

/// The five-bit field `map555_policy` puts in output slot 0: `GdkSwapShift`
/// takes the low five bits (the blue field) as red, the other rules the high
/// five bits.
pub open spec fn map5_field0(v: int, rule: Map5Options) -> int {
    match rule {
        Map5Options::GdkSwapShift => blue5(v),
        _ => red5(v),
    }
}

/// The field `map555_policy` puts in output slot 2 (the mirror of
/// `map5_field0`).
pub open spec fn map5_field2(v: int, rule: Map5Options) -> int {
    match rule {
        Map5Options::GdkSwapShift => red5(v),
        _ => blue5(v),
    }
}

/// `map555_policy` is `map5_chan` on the three fields, in the policy's output
/// order (`GdkSwapShift` swaps the outer two).
pub proof fn lemma_map555_policy_chan(v: int, rule: Map5Options)
    ensures
        map555_policy(v, rule) == seq![
            map5_chan(map5_field0(v, rule), rule),
            map5_chan(green5(v), rule),
            map5_chan(map5_field2(v, rule), rule),
        ],
{
    reveal(map555_policy);
    reveal(rgb555);
    reveal(rep5);
    reveal(round5);
    reveal(map5_chan);
    reveal(map5_field0);
    reveal(map5_field2);
    match rule {
        Map5Options::Replicate => {},
        Map5Options::Round => {},
        Map5Options::GdkSwapShift => {},
    }
}



/// `map_entry_alpha_exec`'s four cases, one lemma each: a single unfold per
/// lemma keeps the solver inside its budget, and the executable's nested `if`s
/// supply each lemma's preconditions by construction.
pub proof fn lemma_alpha_bit15_set(edepth: u8, attr: u8, v: u16, p: Policy)
    requires
        edepth == 16u8,
        attr == 1u8,
        p.map16_alpha == Map16AlphaOptions::Bit15,
        bit15(v as int),
    ensures
        map_entry_alpha_exec(edepth, attr as int, v as int, 0, p) == 255u8,
{
    reveal(map_entry_alpha_exec);
    assert((attr == 1u8) == (attr as int == 1));
}

pub proof fn lemma_alpha_bit15_clear(edepth: u8, attr: u8, v: u16, p: Policy)
    requires
        edepth == 16u8,
        attr == 1u8,
        p.map16_alpha == Map16AlphaOptions::Bit15,
        !bit15(v as int),
    ensures
        map_entry_alpha_exec(edepth, attr as int, v as int, 0, p) == 0u8,
{
    reveal(map_entry_alpha_exec);
    assert((attr == 1u8) == (attr as int == 1));
}

pub proof fn lemma_alpha_depth_not16(edepth: u8, attr: u8, v: u16, p: Policy)
    requires
        edepth != 16u8,
    ensures
        map_entry_alpha_exec(edepth, attr as int, v as int, 0, p) == 255u8,
{
    reveal(map_entry_alpha_exec);
}

pub proof fn lemma_alpha_attr_not1(edepth: u8, attr: u8, v: u16, p: Policy)
    requires
        attr != 1u8,
    ensures
        map_entry_alpha_exec(edepth, attr as int, v as int, 0, p) == 255u8,
{
    reveal(map_entry_alpha_exec);
    assert((attr == 1u8) == (attr as int == 1));
    assert(attr as int != 1);
}

pub proof fn lemma_alpha_not_bit15rule(edepth: u8, attr: u8, v: u16, p: Policy)
    requires
        p.map16_alpha != Map16AlphaOptions::Bit15,
    ensures
        map_entry_alpha_exec(edepth, attr as int, v as int, 0, p) == 255u8,
{
    reveal(map_entry_alpha_exec);
}







/// R5's alpha rule on the effective attribute count, as a plain predicate.
pub open spec fn stored_alpha_img_attr(attr: int, p: Policy) -> bool {
    p.alpha == AlphaOptions::Stored || attr == 8
}

/// R8/R5's governing depth for an image pixel: the map entry depth for an
/// indexed image, the image depth otherwise.
pub open spec fn gov_depth(t: u8, depth: u8, edepth: u8, p: Policy) -> int {
    if kind_policy(t, p) == 0 {
        edepth as int
    } else {
        depth as int
    }
}

/// R5's effective attribute count of an image pixel.
pub open spec fn eff_count_img(t: u8, depth: u8, edepth: u8, count: int, p: Policy) -> int {
    eff_attr(kind_policy(t, p), gov_depth(t, depth, edepth, p), count, p.attribute_bits)
}

/// R8's `stored_alpha` on the effective count of an image pixel.
pub open spec fn stored_alpha_img(t: u8, depth: u8, edepth: u8, count: int, p: Policy) -> bool {
    p.alpha == AlphaOptions::Stored || eff_count_img(t, depth, edepth, count, p) == 8
}

/// A true-colour or greyscale depth that the table accepts is one of the five
/// supported depths, and never the 8-bit depth for true colour.
pub proof fn lemma_depth_ok_pixel(kind: int, d: u8, o: DepthsOptions)
    requires
        depth_ok_policy(kind, d, o),
        kind == 1 || kind == 2,
    ensures
        d == 8u8 || d == 15u8 || d == 16u8 || d == 24u8 || d == 32u8,
        kind == 1 ==> d != 8u8,
        kind == 2 ==> (d == 8u8 || d == 16u8),
{
    reveal(depth_ok_policy);
    if o == DepthsOptions::Gdk {
        if kind == 1 {
            assert(d == 16u8 || d == 24u8 || d == 32u8);
        } else {
            assert(d == 8u8 || d == 16u8);
        }
    } else {
        if kind == 1 {
            assert(d == 15u8 || d == 16u8 || d == 24u8 || d == 32u8);
        } else {
            assert(d == 8u8 || d == 16u8);
        }
    }
}


/// The alpha rule the executable tests on the effective count is the record's
/// `stored_alpha_policy`.
pub proof fn lemma_stored_alpha_img(h: PktH, p: Policy)
    ensures
        stored_alpha_policy(h, p) == (p.alpha == AlphaOptions::Stored || h.attr == 8),
{
    reveal(stored_alpha_policy);
}

/// `map_bytes_flags` on a present, non-empty map: `length * ceil(edepth/8)`.
pub proof fn lemma_map_bytes_flags_present(flag: u8, length: int, edepth: u8, p: Policy)
    requires
        map_present_policy_flags(flag, p),
        length > 0,
    ensures
        map_bytes_flags(flag, length, edepth, p) == length * sample_bytes_of(edepth),
{
    reveal(map_bytes_flags);
    reveal(map_present_policy_flags);
}

/// `sample_bytes_of` is `stride_of` (the executable's `ceil(d/8)`).
pub proof fn lemma_sample_bytes_of_stride(d: u8)
    ensures
        sample_bytes_of(d) == stride_of(d as int),
{
    crate::model::lemma_stride_eq(d);
}



/// `stored_alpha_policy` is `stored_alpha_policy_attr` on the record's count.
pub proof fn lemma_stored_alpha_policy_attr(h: PktH, p: Policy)
    ensures
        stored_alpha_policy(h, p) == stored_alpha_policy_attr(h.attr, p),
{
    reveal(stored_alpha_policy);
    reveal(stored_alpha_policy_attr);
}

// ---------------------------------------------------------------------------
// The exec-facing conversion specs: what the profiled executable's
// `convert_sample_policy_exec`/`convert_index_policy`/`convert_one_policy`
// prove their results equal. They are the policy `conv2` restricted to the
// indices the caller has already range-checked, so the `Black` branch of
// `conv_map_policy` is unreachable from them.
// ---------------------------------------------------------------------------

/// The policy `conv_map` on an in-range index: `0 <= j < map_len`, so the
/// `Black` substitute is not taken.
pub open spec fn conv_map_policy_ok(h: PktH, map: Seq<u8>, index: int, p: Policy) -> Seq<u8> {
    let j = entry_j(index, h.map_first, p);
    let off = j * stride_of(h.map_depth);
    if h.map_depth == 15 || h.map_depth == 16 {
        let v = le16s(map, off);
        map555_policy(v, p.map5).push(map_entry_alpha_policy(h, v, h.map_depth, p))
    } else if h.map_depth == 24 {
        seq![map[off + 2], map[off + 1], map[off], 255u8]
    } else if h.map_depth == 32 {
        seq![
            map[off + 2],
            map[off + 1],
            map[off],
            (if stored_alpha_policy(h, p) { map[off + 3] } else { 255u8 }),
        ]
    } else {
        seq![0u8, 0, 0, 255]
    }
}

/// `conv_map_policy` on an in-range index is `conv_map_policy_ok`.
pub proof fn lemma_conv_map_policy_in_range(h: PktH, map: Seq<u8>, index: int, p: Policy)
    requires
        0 <= entry_j(index, h.map_first, p) < h.map_len,
        0 <= h.map_depth,
    ensures
        conv_map_policy(h, map, index, p) == conv_map_policy_ok(h, map, index, p),
{
    reveal(conv_map_policy);
    reveal(conv_map_policy_ok);
    reveal(entry_j);
    if h.map_depth != 15 && h.map_depth != 16 {
        assert(0 <= stride_of(h.map_depth)) by (nonlinear_arith)
            requires
                stride_of(h.map_depth) == (h.map_depth + 7) / 8,
                h.map_depth >= 0,
        ;
    }
}

/// The policy 5-bit triple under `map5`, executably: gdk-pixbuf's red/blue
/// swap (`GdkSwapShift`), image-rs's rounding (`Round`) or 1.0's replication.
pub open spec fn map555_exec(b0: int, b1: int, rule: Map5Options) -> Seq<u8> {
    let v = b0 + 256 * b1;
    match rule {
        Map5Options::Replicate => seq![
            rep5(red5(v)) as u8,
            rep5(green5(v)) as u8,
            rep5(blue5(v)) as u8,
        ],
        Map5Options::Round => seq![
            round5(red5(v)) as u8,
            round5(green5(v)) as u8,
            round5(blue5(v)) as u8,
        ],
        Map5Options::GdkSwapShift => seq![
            (blue5(v) * 8) as u8,
            (green5(v) * 8) as u8,
            (red5(v) * 8) as u8,
        ],
    }
}

/// `map555_policy(le16s(map, off), rule)` is `map555_exec` on the entry's two
/// bytes.
pub proof fn lemma_map555_policy_eq(map: Seq<u8>, off: int, rule: Map5Options)
    requires
        0 <= off,
        off + 2 <= map.len(),
    ensures
        map555_policy(le16s(map, off), rule) == map555_exec(map[off] as int, map[off + 1] as int, rule),
{
    reveal(map555_policy);
    reveal(map555_exec);
    reveal(le16s);
    reveal(round5);
    assert(le16s(map, off) == map[off] as int + 256 * (map[off + 1] as int));
}

/// The policy 15/16-bit true-colour pixel under `rgb5`.
pub open spec fn rgb5_exec(b0: int, b1: int, rule: Rgb5Options) -> Seq<u8> {
    let v = b0 + 256 * b1;
    match rule {
        Rgb5Options::Replicate => rgb555(v),
        Rgb5Options::Round => seq![
            round5(red5(v)) as u8,
            round5(green5(v)) as u8,
            round5(blue5(v)) as u8,
        ],
    }
}

/// `rgb555_policy(le16s(s, 0), rule)` is `rgb5_exec` on the sample's bytes.
pub proof fn lemma_rgb5_policy_eq(s: Seq<u8>, rule: Rgb5Options)
    requires
        s.len() >= 2,
    ensures
        rgb555_policy(le16s(s, 0), rule) == rgb5_exec(s[0] as int, s[1] as int, rule),
{
    reveal(rgb555_policy);
    reveal(rgb5_exec);
    reveal(le16s);
    reveal(round5);
    assert(le16s(s, 0) == s[0] as int + 256 * (s[1] as int));
}

/// The 15/16-bit palette entry's alpha under `map16_alpha` (only bit 15 is
/// read) and, for a 32-bit entry, the stored-byte rule.
pub open spec fn map_entry_alpha_exec(
    edepth: u8,
    attr: int,
    v: int,
    a: int,
    p: Policy,
) -> u8 {
    if edepth == 16u8 && attr == 1 && p.map16_alpha == Map16AlphaOptions::Bit15 {
        if bit15(v) {
            255u8
        } else {
            0u8
        }
    } else {
        255u8
    }
}

/// `map_entry_alpha_policy(h, le16s(map,off), depth, p)` is
/// `map_entry_alpha_exec` on the entry's two bytes.
pub proof fn lemma_map_entry_alpha_policy_eq(
    h: PktH,
    map: Seq<u8>,
    off: int,
    edepth: u8,
    p: Policy,
)
    requires
        edepth as int == h.map_depth,
        0 <= off,
        off + 2 <= map.len(),
    ensures
        map_entry_alpha_policy(h, le16s(map, off), edepth as int, p) == map_entry_alpha_exec(
            edepth,
            h.attr,
            le16s(map, off),
            0,
            p,
        ),
{
    reveal(map_entry_alpha_policy);
    reveal(map_entry_alpha_exec);
    reveal(le16s);
}

/// The map-entry stride the executable multiplies by, as an `int`.
pub open spec fn map_stride_of(depth: u8) -> int {
    sample_bytes_of(depth)
}

// ---------------------------------------------------------------------------
// The policy raw phase: a cursor projection and the write invariants the
// profiled executable's raw loop carries.
// ---------------------------------------------------------------------------

/// The cursor-and-error projection of `mp_raw_acc_policy`: the same ordered
/// checks and the same final cursor, without the pixel bookkeeping.
pub open spec fn mp_raw_cur_policy(b: Seq<u8>, p: PktH, pol: Policy, c0: int, k: int)
    -> Result<int, Error>
    decreases (p.n - k) as nat
{
    if k < 0 || k > p.n || p.sample_bytes <= 0 {
        Err(Error::ColorIndexOutOfRange)
    } else if k == p.n {
        Ok(c0 + k * p.sample_bytes)
    } else if c0 + k * p.sample_bytes + p.sample_bytes > b.len() {
        Err(Error::TruncatedPixels)
    } else {
        let smp = b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes);
        if p.kind == 0 && !in_range_policy(p, sindex(smp, p.sample_bytes), pol) && pol.index_range
            == IndexRangeOptions::Reject {
            Err(Error::ColorIndexOutOfRange)
        } else {
            mp_raw_cur_policy(b, p, pol, c0, k + 1)
        }
    }
}

/// `mp_raw_cur_policy` is exactly the cursor/error projection of
/// `mp_raw_acc_policy`, for every starting `k` and accumulator.
pub proof fn lemma_mp_raw_cur_policy_proj(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    c0: int,
    k: int,
    acc: Seq<Seq<u8>>,
)
    ensures
        match mp_raw_acc_policy(b, p, map, pol, c0, k, acc) {
            Ok(r) => mp_raw_cur_policy(b, p, pol, c0, k) == Ok(r.1),
            Err((e, _c, _pr, _px)) => mp_raw_cur_policy(b, p, pol, c0, k) == Err(e),
        },
    decreases (p.n - k) as nat,
{
    if k < 0 || k > p.n || p.sample_bytes <= 0 {
        reveal(mp_raw_acc_policy);
        reveal(mp_raw_cur_policy);
    } else if k == p.n {
        reveal(mp_raw_acc_policy);
        reveal(mp_raw_cur_policy);
    } else if c0 + k * p.sample_bytes + p.sample_bytes > b.len() {
        reveal(mp_raw_acc_policy);
        reveal(mp_raw_cur_policy);
    } else {
        reveal(mp_raw_acc_policy);
        reveal(mp_raw_cur_policy);
        let smp = b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes);
        if p.kind == 0 && !in_range_policy(p, sindex(smp, p.sample_bytes), pol) && pol.index_range
            == IndexRangeOptions::Reject {
        } else {
            lemma_mp_raw_cur_policy_proj(
                b,
                p,
                map,
                pol,
                c0,
                k + 1,
                acc + seq![conv2_policy(p, map, smp, pol)],
            );
        }
    }
}

/// The guard facts a policy raw-cursor `Ok` at `k < n` supplies.
pub proof fn lemma_raw_cur_policy_ok_guard(b: Seq<u8>, p: PktH, pol: Policy, c0: int, k: int)
    requires
        0 <= k < p.n,
        mp_raw_cur_policy(b, p, pol, c0, k) == Ok(c0 + p.n * p.sample_bytes),
    ensures
        p.sample_bytes >= 1,
        c0 + k * p.sample_bytes + p.sample_bytes <= b.len(),
        p.kind == 0 ==> in_range_policy(
            p,
            sindex(
                b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes),
                p.sample_bytes,
            ),
            pol,
        ) || pol.index_range == IndexRangeOptions::Black,
{
    reveal(mp_raw_cur_policy);
    if p.sample_bytes <= 0 {
        assert(mp_raw_cur_policy(b, p, pol, c0, k) == Err(Error::ColorIndexOutOfRange));
    }
    if c0 + k * p.sample_bytes + p.sample_bytes > b.len() {
        assert(mp_raw_cur_policy(b, p, pol, c0, k) == Err(Error::TruncatedPixels));
    }
    let smp = b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes);
    if p.kind == 0 {
        if !in_range_policy(p, sindex(smp, p.sample_bytes), pol) {
            if pol.index_range == IndexRangeOptions::Reject {
                assert(mp_raw_cur_policy(b, p, pol, c0, k) == Err(Error::ColorIndexOutOfRange));
            }
        }
    }
}


/// The raw-cursor model over an in-domain record is `Ok` at `n` exactly when
/// the full policy raw model accepts with `n` pixels.
pub proof fn lemma_mp_raw_policy_cur(b: Seq<u8>, p: PktH, map: Seq<u8>, pol: Policy, c0: int)
    ensures
        match mp_raw_policy(b, p, map, pol, c0) {
            Ok(r) => mp_raw_cur_policy(b, p, pol, c0, 0) == Ok(r.1),
            Err((e, _c, _pr, _px)) => mp_raw_cur_policy(b, p, pol, c0, 0) == Err(e),
        },
{
    reveal(mp_raw_policy);
    lemma_mp_raw_cur_policy_proj(b, p, map, pol, c0, 0, Seq::<Seq<u8>>::empty());
}

/// `pkt_of_policy`'s `n` is the header's `width*height` — placed here so the
/// raw loop can keep it as an invariant fact.
pub open spec fn pkt_of_policy_n_of(h_n: int, w: int, hgt: int) -> bool {
    w * hgt == h_n
}

/// Every sample `j < k` is present and its policy index is in range (or
/// substituted under `Black`): the raw loop's per-sample guard, as a policy
/// predicate.
pub open spec fn raw_sample_ok_policy(b: Seq<u8>, p: PktH, pol: Policy, c0: int, j: int) -> bool {
    &&& c0 + j * p.sample_bytes + p.sample_bytes <= b.len()
    &&& (p.kind == 0 ==> (in_range_policy(
        p,
        sindex(
            b.subrange(c0 + j * p.sample_bytes, c0 + j * p.sample_bytes + p.sample_bytes),
            p.sample_bytes,
        ),
        pol,
    ) || pol.index_range == IndexRangeOptions::Black))
}

/// Every sample `j < k` passes the policy raw guard.
pub open spec fn raw_prefix_ok_policy(b: Seq<u8>, p: PktH, pol: Policy, c0: int, k: int) -> bool {
    0 <= k && forall|j: int| 0 <= j < k ==> #[trigger] raw_sample_ok_policy(b, p, pol, c0, j)
}

/// Assemble `raw_prefix_ok_policy` from its conjuncts.
pub proof fn lemma_raw_prefix_ok_policy_intro(
    b: Seq<u8>,
    p: PktH,
    pol: Policy,
    c0: int,
    k: int,
)
    requires
        0 <= k,
        forall|j: int| 0 <= j < k ==> #[trigger] raw_sample_ok_policy(b, p, pol, c0, j),
    ensures
        raw_prefix_ok_policy(b, p, pol, c0, k),
{
    reveal(raw_prefix_ok_policy);
}

/// `map_entry_depth_ok` gives the entry stride.
pub proof fn lemma_map_entry_depth_stride(edepth: u8)
    requires
        crate::model::map_entry_depth_ok(edepth),
    ensures
        1 <= sample_bytes_of(edepth) <= 4,
{
    reveal(crate::model::map_entry_depth_ok);
    assert(edepth == 15u8 || edepth == 16u8 || edepth == 24u8 || edepth == 32u8);
    crate::model::lemma_sample_bytes_le_4(edepth);
}

/// `in_range_policy` implies a non-empty map.
pub proof fn lemma_in_range_policy_pos(h: PktH, index: int, pol: Policy)
    requires
        in_range_policy(h, index, pol),
    ensures
        h.map_len > 0,
{
    reveal(in_range_policy);
    reveal(entry_j);
}



/// The raw loop's current sample guard, from the `convert_one_policy` `Ok`
/// postcondition and the header: the sample is present (the loop checked it)
/// and its policy index is in range or substituted.
pub proof fn lemma_raw_sample_ok_policy_of_loop(
    b: Seq<u8>,
    p: PktH,
    pol: Policy,
    c0: int,
    k: int,
    indexed: bool,
)
    requires
        indexed == (p.kind == 0),
        0 <= k,
        c0 + k * p.sample_bytes + p.sample_bytes <= b.len(),
        p.sample_bytes >= 1,
        indexed ==> in_range_policy(
            p,
            sindex(
                b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes),
                p.sample_bytes,
            ),
            pol,
        ) || pol.index_range == IndexRangeOptions::Black,
    ensures
        raw_sample_ok_policy(b, p, pol, c0, k),
{
    reveal(raw_sample_ok_policy);
    reveal(in_range_policy);
    reveal(entry_j);
}

/// Unpack `raw_prefix_ok_policy` at one index.
pub proof fn lemma_raw_prefix_ok_policy_at(
    b: Seq<u8>,
    p: PktH,
    pol: Policy,
    c0: int,
    k: int,
    j: int,
)
    requires
        raw_prefix_ok_policy(b, p, pol, c0, k),
        0 <= j < k,
    ensures
        raw_sample_ok_policy(b, p, pol, c0, j),
{
    reveal(raw_prefix_ok_policy);
}

/// One decodable sample extends `raw_prefix_ok_policy`.
pub proof fn lemma_raw_prefix_ok_policy_step(b: Seq<u8>, p: PktH, pol: Policy, c0: int, k: int)
    requires
        raw_prefix_ok_policy(b, p, pol, c0, k),
        0 <= k,
        raw_sample_ok_policy(b, p, pol, c0, k),
    ensures
        raw_prefix_ok_policy(b, p, pol, c0, k + 1),
{
    reveal(raw_prefix_ok_policy);
    assert forall|j: int| 0 <= j < k + 1 implies #[trigger] raw_sample_ok_policy(b, p, pol, c0, j) by {
        if j < k {
            lemma_raw_prefix_ok_policy_at(b, p, pol, c0, k, j);
        }
    }
}

/// The loop-entry state: `k = 0` is vacuous.
pub proof fn lemma_raw_prefix_ok_policy_zero(b: Seq<u8>, p: PktH, pol: Policy, c0: int)
    ensures
        raw_prefix_ok_policy(b, p, pol, c0, 0),
{
    reveal(raw_prefix_ok_policy);
}

/// `mp_raw_acc_policy` on a decodable sample `k` appends one converted pixel.
pub proof fn lemma_mp_raw_acc_policy_step(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    c0: int,
    k: int,
    acc: Seq<Seq<u8>>,
)
    requires
        raw_sample_ok_policy(b, p, pol, c0, k),
        0 <= k < p.n,
        p.sample_bytes >= 1,
    ensures
        mp_raw_acc_policy(b, p, map, pol, c0, k, acc) == mp_raw_acc_policy(
            b,
            p,
            map,
            pol,
            c0,
            k + 1,
            acc + seq![conv2_policy(p, map, b.subrange(
                c0 + k * p.sample_bytes,
                c0 + k * p.sample_bytes + p.sample_bytes,
            ), pol)],
        ),
{
    reveal(mp_raw_acc_policy);
    reveal(raw_sample_ok_policy);
    let smp = b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes);
    if p.kind == 0 {
        if !in_range_policy(p, sindex(smp, p.sample_bytes), pol) {
            assert(pol.index_range == IndexRangeOptions::Black);
        }
    }
}

/// One policy raw iteration's branch: sample `k` missing.
pub proof fn lemma_mp_raw_cur_policy_trunc(b: Seq<u8>, p: PktH, pol: Policy, c0: int, k: int)
    requires
        0 <= k < p.n,
        p.sample_bytes >= 1,
        c0 + k * p.sample_bytes + p.sample_bytes > b.len(),
    ensures
        mp_raw_cur_policy(b, p, pol, c0, k) == Err(Error::TruncatedPixels),
{
    reveal(mp_raw_cur_policy);
}

/// One policy raw iteration's branch: sample `k`'s index is rejected.
pub proof fn lemma_mp_raw_cur_policy_index(b: Seq<u8>, p: PktH, pol: Policy, c0: int, k: int)
    requires
        0 <= k < p.n,
        p.sample_bytes >= 1,
        c0 + k * p.sample_bytes + p.sample_bytes <= b.len(),
        p.kind == 0,
        !in_range_policy(
            p,
            sindex(
                b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes),
                p.sample_bytes,
            ),
            pol,
        ),
        pol.index_range == IndexRangeOptions::Reject,
    ensures
        mp_raw_cur_policy(b, p, pol, c0, k) == Err(Error::ColorIndexOutOfRange),
{
    reveal(mp_raw_cur_policy);
}

/// `mp_raw_cur_policy` is `Ok` at every `k <= n` once it is `Ok` at 0.
pub proof fn lemma_mp_raw_cur_policy_mono(b: Seq<u8>, p: PktH, pol: Policy, c0: int, k: int)
    requires
        mp_raw_cur_policy(b, p, pol, c0, 0) == Ok(c0 + p.n * p.sample_bytes),
        0 <= k <= p.n,
    ensures
        mp_raw_cur_policy(b, p, pol, c0, k) == Ok(c0 + p.n * p.sample_bytes),
    decreases k as nat,
{
    if k > 0 {
        lemma_mp_raw_cur_policy_mono(b, p, pol, c0, k - 1);
        assert(0 <= k - 1 < p.n) by (nonlinear_arith)
            requires
                k > 0,
                k <= p.n,
        ;
        lemma_mp_raw_cur_policy_ok_guard(b, p, pol, c0, k - 1);
        reveal(mp_raw_cur_policy);
    }
}

/// The guard facts a policy raw-cursor `Ok` at `k < n` supplies.
pub proof fn lemma_mp_raw_cur_policy_ok_guard(b: Seq<u8>, p: PktH, pol: Policy, c0: int, k: int)
    requires
        0 <= k < p.n,
        mp_raw_cur_policy(b, p, pol, c0, k) == Ok(c0 + p.n * p.sample_bytes),
    ensures
        p.sample_bytes >= 1,
        c0 + k * p.sample_bytes + p.sample_bytes <= b.len(),
        p.kind == 0 ==> in_range_policy(
            p,
            sindex(
                b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes),
                p.sample_bytes,
            ),
            pol,
        ) || pol.index_range == IndexRangeOptions::Black,
{
    reveal(mp_raw_cur_policy);
    if p.sample_bytes <= 0 {
        assert(mp_raw_cur_policy(b, p, pol, c0, k) == Err(Error::ColorIndexOutOfRange));
    }
    if c0 + k * p.sample_bytes + p.sample_bytes > b.len() {
        assert(mp_raw_cur_policy(b, p, pol, c0, k) == Err(Error::TruncatedPixels));
    }
    let smp = b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes);
    if p.kind == 0 {
        if !in_range_policy(p, sindex(smp, p.sample_bytes), pol) {
            if pol.index_range == IndexRangeOptions::Reject {
                assert(mp_raw_cur_policy(b, p, pol, c0, k) == Err(Error::ColorIndexOutOfRange));
            }
        }
    }
}

/// `raw_prefix_ok_policy` for every prefix of a successful policy raw run.
pub proof fn lemma_raw_prefix_ok_policy_all(b: Seq<u8>, p: PktH, pol: Policy, c0: int, k: int)
    requires
        mp_raw_cur_policy(b, p, pol, c0, 0) == Ok(c0 + p.n * p.sample_bytes),
        0 <= k <= p.n,
    ensures
        raw_prefix_ok_policy(b, p, pol, c0, k),
    decreases k as nat,
{
    if k == 0 {
        reveal(raw_prefix_ok_policy);
    } else {
        // k-1's guard holds (the run is Ok), so `k-1`'s prefix extends.
        lemma_raw_prefix_ok_policy_all(b, p, pol, c0, k - 1);
        assert(0 <= k - 1 < p.n) by (nonlinear_arith)
            requires
                k > 0,
                k <= p.n,
        ;
        lemma_mp_raw_cur_policy_mono(b, p, pol, c0, k - 1);
        lemma_mp_raw_cur_policy_ok_guard(b, p, pol, c0, k - 1);
        assert(raw_sample_ok_policy(b, p, pol, c0, k - 1)) by {
            reveal(raw_sample_ok_policy);
            if p.kind == 0 {
                let smp = b.subrange(
                    c0 + (k - 1) * p.sample_bytes,
                    c0 + (k - 1) * p.sample_bytes + p.sample_bytes,
                );
                if !in_range_policy(p, sindex(smp, p.sample_bytes), pol) {
                    assert(pol.index_range == IndexRangeOptions::Black);
                }
            }
        }
        lemma_raw_prefix_ok_policy_step(b, p, pol, c0, k - 1);
    }
}

/// One `write_pixel` at the oriented offset of sample `kk` preserves every
/// earlier sample's quad: the offsets are distinct (R9's `fcoord` is a
/// bijection), so the four written bytes never overlap an earlier quad.
pub proof fn lemma_write_preserves_policy_quads(
    out0: Seq<u8>,
    out1: Seq<u8>,
    bytes: Seq<u8>,
    pkt: PktH,
    map: Seq<u8>,
    pol: Policy,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    c0: int,
    sb: int,
    kk: int,
    off: int,
)
    requires
        1 <= w,
        1 <= hgt,
        0 <= kk < w * hgt,
        off == 4 * crate::model::fcoord(w, hgt, fx, fy, kk),
        wrote_samples_policy(out0, bytes, pkt, map, pol, w, hgt, fx, fy, c0, sb, kk),
        out0.len() == 4 * (w * hgt),
        out1.len() == out0.len(),
        forall|i: int| 0 <= i < out0.len() && !(off <= i < off + 4) ==> out1[i] == out0[i],
    ensures
        forall|j: int| 0 <= j < kk ==> #[trigger] crate::model::quad_at(
            out1,
            4 * crate::model::fcoord(w, hgt, fx, fy, j),
            conv2_policy(pkt, map, bytes.subrange(c0 + j * sb, c0 + j * sb + sb), pol),
        ),
{
    reveal(wrote_samples_policy);
    reveal(crate::model::quad_at);
    assert forall|j: int| 0 <= j < kk implies #[trigger] crate::model::quad_at(
        out1,
        4 * crate::model::fcoord(w, hgt, fx, fy, j),
        conv2_policy(pkt, map, bytes.subrange(c0 + j * sb, c0 + j * sb + sb), pol),
    ) by {
        {
            let fj = crate::model::fcoord(w, hgt, fx, fy, j);
            let fk = crate::model::fcoord(w, hgt, fx, fy, kk);
            crate::model::lemma_offset_injective(w, hgt, fx, fy, j, kk);
            assert(fj != fk);
            if fj < fk {
                assert(4 * fj + 4 <= 4 * fk) by (nonlinear_arith)
                    requires
                        fj < fk,
                ;
            } else {
                assert(fj > fk);
                assert(4 * fk + 4 <= 4 * fj) by (nonlinear_arith)
                    requires
                        fj > fk,
                ;
            }
            crate::model::lemma_fcoord_range(w, hgt, fx, fy, j);
            assert(0 <= fj < w * hgt);
            assert(crate::model::quad_at(
                out0,
                4 * fj,
                conv2_policy(pkt, map, bytes.subrange(c0 + j * sb, c0 + j * sb + sb), pol),
            ));
            assert forall|i: int| 0 <= i < 4 implies out1[4 * fj + i] == #[trigger] out0[4 * fj
                + i] by {
                assert(off == 4 * fk);
                assert(!(off <= 4 * fj + i < off + 4));
                if fj < fk {
                    assert(4 * fj + i < 4 * fk);
                } else {
                    assert(4 * fj + i >= 4 * fk + 4);
                }
            }
        }
    }
}

/// `wrote_samples_policy` holds vacuously at `k=0`.
pub proof fn lemma_wrote_samples_policy_none(
    out: Seq<u8>,
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    start: int,
    sb: int,
)
    ensures
        wrote_samples_policy(out, s, h, map, pol, w, hgt, fx, fy, start, sb, 0),
{
    reveal(wrote_samples_policy);
}

/// `mp_raw_acc_policy`'s pixel accumulator starts empty.
pub proof fn lemma_mp_raw_acc_policy_start(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    c0: int,
)
    ensures
        mp_raw_acc_policy(b, p, map, pol, c0, 0, Seq::<Seq<u8>>::empty()) == mp_raw_acc_policy(
            b,
            p,
            map,
            pol,
            c0,
            0,
            Seq::<Seq<u8>>::empty(),
        ),
{
}

/// The pixel accumulator at `k` is the `k` input-order conversions.
pub proof fn lemma_acc_conv_policy_len(b: Seq<u8>, p: PktH, map: Seq<u8>, pol: Policy, c0: int)
    requires
        p.n >= 0,
    ensures
        acc_conv_policy(b, p, map, pol, c0, p.n).len() == p.n,
{
    reveal(acc_conv_policy);
    broadcast use vstd::seq::group_seq_lemmas;
}

/// `wrote_samples_policy` at `N` gives the oriented output bytes of the raw
/// phase's pixel sequence.
pub proof fn lemma_wrote_to_orient_policy(
    out: Seq<u8>,
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    c0: int,
)
    requires
        1 <= w,
        1 <= hgt,
        w * hgt == p.n,
        out.len() == 4 * (w * hgt),
        wrote_samples_policy(out, b, p, map, pol, w, hgt, fx, fy, c0, p.sample_bytes, p.n),
    ensures
        out == crate::model::orient_bytes(
            acc_conv_policy(b, p, map, pol, c0, p.n),
            w,
            hgt,
            fx,
            fy,
            w * hgt,
        ),
{
    reveal(wrote_samples_policy);
    reveal(acc_conv_policy);
    reveal(crate::model::orient_bytes);
    broadcast use vstd::seq::group_seq_lemmas;
    assert(out =~= crate::model::orient_bytes(
        acc_conv_policy(b, p, map, pol, c0, p.n),
        w,
        hgt,
        fx,
        fy,
        w * hgt,
    )) by {
        assert forall|i: int| 0 <= i < 4 * (w * hgt) implies out[i] == #[trigger] crate::model::orient_bytes(
            acc_conv_policy(b, p, map, pol, c0, p.n),
            w,
            hgt,
            fx,
            fy,
            w * hgt,
        )[i] by {
            crate::model::lemma_div4_lt(i, w * hgt);
            crate::model::lemma_fcoord_inverse(w, hgt, fx, fy, i / 4);
            let k = crate::model::gcoord(w, hgt, fx, fy, i / 4);
            assert(0 <= k < w * hgt);
            assert(crate::model::quad_at(
                out,
                4 * crate::model::fcoord(w, hgt, fx, fy, k),
                conv2_policy(
                    p,
                    map,
                    b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes),
                    pol,
                ),
            ));
            assert(acc_conv_policy(b, p, map, pol, c0, p.n)[k] == conv2_policy(
                p,
                map,
                b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes),
                pol,
            )) by {
                reveal(acc_conv_policy);
            }
            assert(crate::model::fcoord(w, hgt, fx, fy, k) == i / 4);
            assert(4 * (i / 4) + i % 4 == i) by (nonlinear_arith)
                requires
                    i >= 0,
            ;
            reveal(crate::model::quad_at);
            reveal(crate::model::px_byte);
            if i % 4 == 0 {
            } else if i % 4 == 1 {
            } else if i % 4 == 2 {
            } else {
            }
        }
    }
}

/// A policy RLE stream's pixel quads give the oriented output bytes.
pub proof fn lemma_rle_quad_to_orient_policy(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
)
    requires
        1 <= w,
        1 <= hgt,
        px.len() == w * hgt,
        out.len() == 4 * (w * hgt),
        crate::model::rle_quad_ok(out, px, w, hgt, fx, fy),
    ensures
        out == crate::model::orient_bytes(px, w, hgt, fx, fy, w * hgt),
{
    crate::model::lemma_quad_ok_to_orient(out, px, w, hgt, fx, fy);
}

// ---------------------------------------------------------------------------
// The policy packet stream's cursor projection (the policy analog of 1.0's
// `mr_cur`; `mr_pix_policy` is the pixel projection, and the clip/substitution
// bits are `mr_flags_policy`).
// ---------------------------------------------------------------------------

/// R11: the cursor-and-produced projection of `mr_at_policy`: the same ordered
/// checks, the clip/`Black` decisions and the same final `(cursor, produced)`,
/// without the pixel bookkeeping.
pub open spec fn mr_cur_policy(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
) -> Result<(int, int), MrErr>
    decreases (h.n - prod) as nat
{
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
        Err(MrErr::Outside)
    } else if prod == h.n {
        Ok((cur, prod))
    } else if cur >= s.len() {
        Err(MrErr::Api(Error::RleTruncated))
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        let remaining = h.n - prod;
        let start = cur + 1;
        if count > remaining && pol.rle_overrun == RleOverrunOptions::Reject {
            Err(MrErr::Api(Error::RleOverrun))
        } else {
            let paint = if count < remaining { count } else { remaining };
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                    Err(MrErr::Api(Error::RleTruncated))
                } else {
                    let smp = s.subrange(start, pe);
                    if h.kind == 0 && pkt_reject(h, sindex(smp, h.sample_bytes), pol) {
                        Err(MrErr::Api(Error::ColorIndexOutOfRange))
                    } else {
                        mr_cur_policy(s, h, map, pol, pe, prod + paint)
                    }
                }
            } else {
                let need = if pol.rle_overrun == RleOverrunOptions::ClipSamples {
                    paint * h.sample_bytes
                } else {
                    count * h.sample_bytes
                };
                let pe = start + need;
                if s.len() < pe {
                    Err(MrErr::Api(Error::RleTruncated))
                } else if !lit_ok_policy(s, h, start, paint, pol) {
                    Err(MrErr::Api(Error::ColorIndexOutOfRange))
                } else {
                    mr_cur_policy(s, h, map, pol, pe, prod + paint)
                }
            }
        }
    }
}

/// `mr_cur_policy`'s `Error` arm as a plain equality (the loop's exits).
pub proof fn lemma_mr_cur_policy_ctrl_absent(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
)
    requires
        0 <= prod < h.n,
        cur == s.len(),
    ensures
        mr_cur_policy(s, h, map, pol, cur, prod) == Err(MrErr::Api(Error::RleTruncated)),
{
    reveal(mr_cur_policy);
}

/// The `Reject` overrun exit.
pub proof fn lemma_mr_cur_policy_overrun(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) > h.n - prod,
        pol.rle_overrun == RleOverrunOptions::Reject,
    ensures
        mr_cur_policy(s, h, map, pol, cur, prod) == Err(MrErr::Api(Error::RleOverrun)),
{
    reveal(mr_cur_policy);
}

/// A repeat packet whose single sample is missing.
pub proof fn lemma_mr_cur_policy_rep_missing(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_repeat(s[cur] as int),
        !(rle_count(s[cur] as int) > h.n - prod && pol.rle_overrun == RleOverrunOptions::Reject),
        s.len() < cur + 1 + h.sample_bytes,
    ensures
        mr_cur_policy(s, h, map, pol, cur, prod) == Err(MrErr::Api(Error::RleTruncated)),
{
    reveal(mr_cur_policy);
}

/// A repeat packet whose sample's index is rejected.
pub proof fn lemma_mr_cur_policy_rep_index(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_repeat(s[cur] as int),
        !(rle_count(s[cur] as int) > h.n - prod && pol.rle_overrun == RleOverrunOptions::Reject),
        s.len() >= cur + 1 + h.sample_bytes,
        h.kind == 0,
        pkt_reject(h, sindex(s.subrange(cur + 1, cur + 1 + h.sample_bytes), h.sample_bytes), pol),
    ensures
        mr_cur_policy(s, h, map, pol, cur, prod) == Err(MrErr::Api(Error::ColorIndexOutOfRange)),
{
    reveal(mr_cur_policy);
}

/// A literal packet whose bytes are missing (whole packet under `Reject` and
/// `ClipPacket`, painted samples only under `ClipSamples`).
pub proof fn lemma_mr_cur_policy_lit_missing(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        !rle_repeat(s[cur] as int),
        !(rle_count(s[cur] as int) > h.n - prod && pol.rle_overrun == RleOverrunOptions::Reject),
        s.len() < cur + 1 + (if pol.rle_overrun == RleOverrunOptions::ClipSamples {
            (if rle_count(s[cur] as int) < h.n - prod {
                rle_count(s[cur] as int)
            } else {
                h.n - prod
            }) * h.sample_bytes
        } else {
            rle_count(s[cur] as int) * h.sample_bytes
        }),
    ensures
        mr_cur_policy(s, h, map, pol, cur, prod) == Err(MrErr::Api(Error::RleTruncated)),
{
    reveal(mr_cur_policy);
}

/// A literal packet whose sample `i` index is rejected.
pub proof fn lemma_mr_cur_policy_lit_index(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        !rle_repeat(s[cur] as int),
        !(rle_count(s[cur] as int) > h.n - prod && pol.rle_overrun == RleOverrunOptions::Reject),
        s.len() >= cur + 1 + (if pol.rle_overrun == RleOverrunOptions::ClipSamples {
            (if rle_count(s[cur] as int) < h.n - prod {
                rle_count(s[cur] as int)
            } else {
                h.n - prod
            }) * h.sample_bytes
        } else {
            rle_count(s[cur] as int) * h.sample_bytes
        }),
        !lit_ok_policy(
            s,
            h,
            cur + 1,
            if rle_count(s[cur] as int) < h.n - prod {
                rle_count(s[cur] as int)
            } else {
                h.n - prod
            },
            pol,
        ),
    ensures
        mr_cur_policy(s, h, map, pol, cur, prod) == Err(MrErr::Api(Error::ColorIndexOutOfRange)),
{
    reveal(mr_cur_policy);
}

/// A repeat packet that decodes: the cursor and produced count advance.
pub proof fn lemma_mr_cur_policy_rep_ok(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_repeat(s[cur] as int),
        !(rle_count(s[cur] as int) > h.n - prod && pol.rle_overrun == RleOverrunOptions::Reject),
        s.len() >= cur + 1 + h.sample_bytes,
        h.kind == 0 ==> !pkt_reject(
            h,
            sindex(s.subrange(cur + 1, cur + 1 + h.sample_bytes), h.sample_bytes),
            pol,
        ),
    ensures
        mr_cur_policy(s, h, map, pol, cur, prod) == mr_cur_policy(
            s,
            h,
            map,
            pol,
            cur + 1 + h.sample_bytes,
            prod + (if rle_count(s[cur] as int) < h.n - prod {
                rle_count(s[cur] as int)
            } else {
                h.n - prod
            }),
        ),
{
    reveal(mr_cur_policy);
}

/// A literal packet that decodes.
pub proof fn lemma_mr_cur_policy_lit_ok(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        !rle_repeat(s[cur] as int),
        !(rle_count(s[cur] as int) > h.n - prod && pol.rle_overrun == RleOverrunOptions::Reject),
        s.len() >= cur + 1 + (if pol.rle_overrun == RleOverrunOptions::ClipSamples {
            (if rle_count(s[cur] as int) < h.n - prod {
                rle_count(s[cur] as int)
            } else {
                h.n - prod
            }) * h.sample_bytes
        } else {
            rle_count(s[cur] as int) * h.sample_bytes
        }),
        lit_ok_policy(
            s,
            h,
            cur + 1,
            if rle_count(s[cur] as int) < h.n - prod {
                rle_count(s[cur] as int)
            } else {
                h.n - prod
            },
            pol,
        ),
    ensures
        mr_cur_policy(s, h, map, pol, cur, prod) == mr_cur_policy(
            s,
            h,
            map,
            pol,
            cur + 1 + (if pol.rle_overrun == RleOverrunOptions::ClipSamples {
                (if rle_count(s[cur] as int) < h.n - prod {
                    rle_count(s[cur] as int)
                } else {
                    h.n - prod
                }) * h.sample_bytes
            } else {
                rle_count(s[cur] as int) * h.sample_bytes
            }),
            prod + (if rle_count(s[cur] as int) < h.n - prod {
                rle_count(s[cur] as int)
            } else {
                h.n - prod
            }),
        ),
{
    reveal(mr_cur_policy);
}

/// R11: the ghost error cursor of `mr_at_policy` — where the stream stops.
pub open spec fn mr_gcur_policy(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
) -> Result<int, MrErr>
    decreases (h.n - prod) as nat
{
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
        Err(MrErr::Outside)
    } else if prod == h.n {
        Ok(cur)
    } else if cur >= s.len() {
        Ok(cur)
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        let remaining = h.n - prod;
        let start = cur + 1;
        if count > remaining && pol.rle_overrun == RleOverrunOptions::Reject {
            Ok(cur + 1)
        } else {
            let paint = if count < remaining { count } else { remaining };
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                    Ok(start)
                } else {
                    let smp = s.subrange(start, pe);
                    if h.kind == 0 && pkt_reject(h, sindex(smp, h.sample_bytes), pol) {
                        Ok(pe)
                    } else {
                        mr_gcur_policy(s, h, map, pol, pe, prod + paint)
                    }
                }
            } else {
                let need = if pol.rle_overrun == RleOverrunOptions::ClipSamples {
                    paint * h.sample_bytes
                } else {
                    count * h.sample_bytes
                };
                let pe = start + need;
                if s.len() < pe {
                    Ok(start)
                } else if !lit_ok_policy(s, h, start, paint, pol) {
                    Ok(pe)
                } else {
                    mr_gcur_policy(s, h, map, pol, pe, prod + paint)
                }
            }
        }
    }
}

/// R11's packet split under `pol`, a repeat packet: the pixel stream is the
/// packet's repeated conversions followed by the rest.
pub proof fn lemma_mr_pix_policy_rep(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur0: int,
    prod: int,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod < p.n,
        0 <= p.sample_bytes,
        rle_repeat(b[cur0] as int),
        !(rle_count(b[cur0] as int) > p.n - prod && pol.rle_overrun == RleOverrunOptions::Reject),
        b.len() >= cur0 + 1 + p.sample_bytes,
        p.kind == 0 ==> !pkt_reject(
            p,
            sindex(b.subrange(cur0 + 1, cur0 + 1 + p.sample_bytes), p.sample_bytes),
            pol,
        ),
    ensures
        mr_pix_policy(b, p, map, pol, cur0, prod) == pix_pow(
            conv2_policy(p, map, b.subrange(cur0 + 1, cur0 + 1 + p.sample_bytes), pol),
            if rle_count(b[cur0] as int) < p.n - prod {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod
            },
        ) + mr_pix_policy(
            b,
            p,
            map,
            pol,
            cur0 + 1 + p.sample_bytes,
            prod + (if rle_count(b[cur0] as int) < p.n - prod {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod
            }),
        ),
{
    reveal(mr_pix_policy);
}

/// R11's packet split under `pol`, a literal packet.
pub proof fn lemma_mr_pix_policy_lit(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur0: int,
    prod: int,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod < p.n,
        0 <= p.sample_bytes,
        !rle_repeat(b[cur0] as int),
        !(rle_count(b[cur0] as int) > p.n - prod && pol.rle_overrun == RleOverrunOptions::Reject),
        b.len() >= cur0 + 1 + (if pol.rle_overrun == RleOverrunOptions::ClipSamples {
            (if rle_count(b[cur0] as int) < p.n - prod {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod
            }) * p.sample_bytes
        } else {
            rle_count(b[cur0] as int) * p.sample_bytes
        }),
        lit_ok_policy(
            b,
            p,
            cur0 + 1,
            if rle_count(b[cur0] as int) < p.n - prod {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod
            },
            pol,
        ),
    ensures
        mr_pix_policy(b, p, map, pol, cur0, prod) == lit_out_policy(
            b,
            p,
            map,
            pol,
            cur0 + 1,
            if rle_count(b[cur0] as int) < p.n - prod {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod
            },
        ) + mr_pix_policy(
            b,
            p,
            map,
            pol,
            cur0 + 1 + (if pol.rle_overrun == RleOverrunOptions::ClipSamples {
                (if rle_count(b[cur0] as int) < p.n - prod {
                    rle_count(b[cur0] as int)
                } else {
                    p.n - prod
                }) * p.sample_bytes
            } else {
                rle_count(b[cur0] as int) * p.sample_bytes
            }),
            prod + (if rle_count(b[cur0] as int) < p.n - prod {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod
            }),
        ),
{
    reveal(mr_pix_policy);
}

/// Expose exactly one recursive layer; callers need no conversion unfolding.
proof fn lemma_lit_ok_policy_step(s: Seq<u8>, h: PktH, pol: Policy, start: int, count: int)
    ensures lit_ok_policy(s, h, start, count, pol) == (count <= 0 || (
        (h.kind != 0 || pol.index_range == IndexRangeOptions::Black
            || in_range_policy(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes), pol))
        && lit_ok_policy(s, h, start + h.sample_bytes, count - 1, pol))),
{ reveal(lit_ok_policy); }

proof fn lemma_lit_good_policy_step(s: Seq<u8>, h: PktH, pol: Policy, start: int, count: int)
    requires count > 0, h.kind == 0, pol.index_range == IndexRangeOptions::Reject,
    ensures lit_good_policy(s, h, start, count, pol) ==
        (if in_range_policy(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes), pol) {
            1 + lit_good_policy(s, h, start + h.sample_bytes, count - 1, pol)
        } else { 0 }),
{ reveal(lit_good_policy); }

proof fn lemma_lit_out_policy_step(s: Seq<u8>, h: PktH, map: Seq<u8>, pol: Policy, start: int, count: int)
    ensures lit_out_policy(s, h, map, pol, start, count) ==
        (if count <= 0 { Seq::<Seq<u8>>::empty() } else {
            seq![conv2_policy(h, map, s.subrange(start, start + h.sample_bytes), pol)]
                + lit_out_policy(s, h, map, pol, start + h.sample_bytes, count - 1)
        }),
{ reveal(lit_out_policy); }

proof fn lemma_literal_shift(start: int, stride: int, i: int)
    ensures
        start + stride + (i - 1) * stride == start + i * stride,
        start + stride + i * stride == start + (i + 1) * stride,
        start + i * stride + stride == start + (i + 1) * stride,
{
    assert(start + stride + (i - 1) * stride == start + i * stride) by (nonlinear_arith);
    assert(start + stride + i * stride == start + (i + 1) * stride) by (nonlinear_arith);
    assert(start + i * stride + stride == start + (i + 1) * stride) by (nonlinear_arith);
}

pub proof fn lemma_lit_ok_policy_extend(
    s: Seq<u8>, h: PktH, pol: Policy, start: int, i: int,
)
    requires
        0 <= i,
        lit_ok_policy(s, h, start, i, pol),
        h.kind != 0 || pol.index_range == IndexRangeOptions::Black
            || in_range_policy(h, sindex(s.subrange(
                start + i * h.sample_bytes,
                start + (i + 1) * h.sample_bytes,
            ), h.sample_bytes), pol),
    ensures lit_ok_policy(s, h, start, i + 1, pol),
    decreases i,
{
    hide(lit_ok_policy);
    hide(in_range_policy);
    lemma_literal_shift(start, h.sample_bytes, i);
    lemma_lit_ok_policy_step(s, h, pol, start, i);
    lemma_lit_ok_policy_step(s, h, pol, start, i + 1);
    if i > 0 {
        lemma_lit_ok_policy_extend(s, h, pol, start + h.sample_bytes, i - 1);
    } else {
        lemma_lit_ok_policy_step(s, h, pol, start + h.sample_bytes, 0);
    }
}

pub proof fn lemma_lit_good_policy_first_bad(
    s: Seq<u8>, h: PktH, pol: Policy, start: int, count: int, i: int,
)
    requires
        0 <= i < count,
        h.kind == 0,
        pol.index_range == IndexRangeOptions::Reject,
        lit_ok_policy(s, h, start, i, pol),
        !in_range_policy(h, sindex(s.subrange(
            start + i * h.sample_bytes,
            start + (i + 1) * h.sample_bytes,
        ), h.sample_bytes), pol),
    ensures
        lit_good_policy(s, h, start, count, pol) == i,
        !lit_ok_policy(s, h, start, count, pol),
    decreases i,
{
    hide(lit_ok_policy);
    hide(lit_good_policy);
    hide(in_range_policy);
    lemma_literal_shift(start, h.sample_bytes, i);
    lemma_lit_ok_policy_step(s, h, pol, start, i);
    lemma_lit_ok_policy_step(s, h, pol, start, count);
    lemma_lit_good_policy_step(s, h, pol, start, count);
    if i > 0 {
        assert(in_range_policy(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes), pol));
        lemma_lit_good_policy_first_bad(s, h, pol, start + h.sample_bytes, count - 1, i - 1);
        assert(lit_good_policy(s, h, start + h.sample_bytes, count - 1, pol) == i - 1);
        assert(!lit_ok_policy(s, h, start + h.sample_bytes, count - 1, pol));
        assert(lit_good_policy(s, h, start, count, pol) == i);
        assert(!lit_ok_policy(s, h, start, count, pol));
    } else {
        assert(start + i * h.sample_bytes == start) by (nonlinear_arith)
            requires i == 0;
        assert(start + (i + 1) * h.sample_bytes == start + h.sample_bytes) by (nonlinear_arith)
            requires i == 0;
        assert(!in_range_policy(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes), pol));
        assert(lit_good_policy(s, h, start, count, pol) == 0);
        assert(!lit_ok_policy(s, h, start, count, pol));
    }
}

/// Append one literal sample to the recursively defined pixel prefix.
pub proof fn lemma_lit_out_policy_peel(
    s: Seq<u8>, h: PktH, map: Seq<u8>, pol: Policy, start: int, i: int,
)
    requires 0 <= i,
    ensures lit_out_policy(s, h, map, pol, start, i + 1)
        == lit_out_policy(s, h, map, pol, start, i)
            + seq![conv2_policy(h, map, s.subrange(
                start + i * h.sample_bytes,
                start + (i + 1) * h.sample_bytes,
            ), pol)],
    decreases i,
{
    hide(lit_out_policy);
    hide(conv2_policy);
    lemma_literal_shift(start, h.sample_bytes, i);
    lemma_lit_out_policy_step(s, h, map, pol, start, i);
    lemma_lit_out_policy_step(s, h, map, pol, start, i + 1);
    if i > 0 {
        lemma_lit_out_policy_peel(s, h, map, pol, start + h.sample_bytes, i - 1);
        vstd::seq_lib::lemma_concat_associative(
            seq![conv2_policy(h, map, s.subrange(start, start + h.sample_bytes), pol)],
            lit_out_policy(s, h, map, pol, start + h.sample_bytes, i - 1),
            seq![conv2_policy(h, map, s.subrange(start + i * h.sample_bytes, start + (i + 1) * h.sample_bytes), pol)],
        );
    } else {
        lemma_lit_out_policy_step(s, h, map, pol, start + h.sample_bytes, 0);
        broadcast use vstd::seq_lib::group_seq_lib_default;
    }
}

/// `lit_out_policy`'s prefix form, indexed by sample: equals `lit_out_policy`
/// on the same range but has a definitional `Seq::new`, so the executable loop
/// can peel one sample without a recursive proof.
pub open spec fn lit_seq_policy(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    start: int,
    count: int,
) -> Seq<Seq<u8>> {
    Seq::new(count as nat, |j: int| conv2_policy(h, map, s.subrange(
        start + j * h.sample_bytes,
        start + j * h.sample_bytes + h.sample_bytes,
    ), pol))
}



/// `lit_seq_policy`'s `i+1` prefix is the `i` prefix plus the `i`-th sample.
pub proof fn lemma_lit_seq_policy_peel(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    start: int,
    i: int,
)
    requires
        0 <= i,
    ensures
        lit_seq_policy(s, h, map, pol, start, i + 1) == lit_seq_policy(
            s,
            h,
            map,
            pol,
            start,
            i,
        ) + seq![conv2_policy(h, map, s.subrange(
            start + i * h.sample_bytes,
            start + i * h.sample_bytes + h.sample_bytes,
        ), pol)],
{
    reveal(lit_seq_policy);
    assert(lit_seq_policy(s, h, map, pol, start, i + 1) =~= lit_seq_policy(
        s,
        h,
        map,
        pol,
        start,
        i,
    ) + seq![conv2_policy(h, map, s.subrange(
        start + i * h.sample_bytes,
        start + i * h.sample_bytes + h.sample_bytes,
    ), pol)]) by {
        assert forall|j: int| 0 <= j < i + 1 implies #[trigger] lit_seq_policy(
            s,
            h,
            map,
            pol,
            start,
            i + 1,
        )[j] == (lit_seq_policy(s, h, map, pol, start, i) + seq![conv2_policy(
            h,
            map,
            s.subrange(start + i * h.sample_bytes, start + i * h.sample_bytes + h.sample_bytes),
            pol,
        )])[j] by {
            if j < i {
                assert(lit_seq_policy(s, h, map, pol, start, i)[j] == lit_seq_policy(
                    s,
                    h,
                    map,
                    pol,
                    start,
                    i + 1,
                )[j]);
            }
        }
    }
}

/// `lit_seq_policy` has `count` entries.
pub proof fn lemma_lit_seq_policy_len(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    start: int,
    count: int,
)
    requires
        0 <= count,
    ensures
        lit_seq_policy(s, h, map, pol, start, count).len() == count,
{
    reveal(lit_seq_policy);
    broadcast use vstd::seq::group_seq_lemmas;
}

/// `lit_out_policy` has one entry per sample; `pix_pow` one per copy.
pub proof fn lemma_lit_out_policy_len(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    start: int,
    count: int,
)
    requires
        0 <= count,
    ensures
        lit_out_policy(s, h, map, pol, start, count).len() == count,
    decreases count,
{
    reveal(lit_out_policy);
    if count <= 0 {
    } else {
        lemma_lit_out_policy_len(s, h, map, pol, start + h.sample_bytes, count - 1);
        broadcast use vstd::seq::group_seq_lemmas;
    }
}

/// `lit_good_policy` is at most the count it scans.
pub proof fn lemma_lit_good_policy_le(
    s: Seq<u8>,
    h: PktH,
    start: int,
    count: int,
    pol: Policy,
)
    requires
        0 <= count,
    ensures
        0 <= lit_good_policy(s, h, start, count, pol) <= count,
    decreases count,
{
    reveal(lit_good_policy);
    if count <= 0 {
    } else if h.kind != 0 || pol.index_range == IndexRangeOptions::Black {
    } else {
        lemma_lit_good_policy_le(s, h, start + h.sample_bytes, count - 1, pol);
    }
}
/// `mr_pix_policy` never overruns `N`.
pub proof fn lemma_mr_pix_policy_len(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    c0: int,
    prod0: int,
)
    requires
        0 <= prod0 <= h.n,
        0 <= h.sample_bytes,
    ensures
        mr_pix_policy(s, h, map, pol, c0, prod0).len() + prod0 <= h.n,
        mr_pix_policy(s, h, map, pol, c0, prod0).len() <= h.n,
    decreases (h.n - prod0) as nat,
{
    reveal(mr_pix_policy);
    if prod0 == h.n {
        broadcast use vstd::seq_lib::group_seq_lib_default;
    } else if c0 < 0 || c0 > s.len() {
    } else if c0 >= s.len() {
    } else {
        let control = s[c0] as int;
        let count = rle_count(control);
        if count > h.n - prod0 && pol.rle_overrun == RleOverrunOptions::Reject {
        } else {
            let paint = if count < h.n - prod0 { count } else { h.n - prod0 };
            let start = c0 + 1;
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                } else {
                    let smp = s.subrange(start, pe);
                    if h.kind == 0 && pkt_reject(h, sindex(smp, h.sample_bytes), pol) {
                    } else {
                        crate::model::lemma_pix_pow_len(conv2_policy(h, map, smp, pol), paint);
                        lemma_mr_pix_policy_len(s, h, map, pol, pe, prod0 + paint);
                        broadcast use vstd::seq::group_seq_lemmas;
                    }
                }
            } else {
                let need = if pol.rle_overrun == RleOverrunOptions::ClipSamples {
                    paint * h.sample_bytes
                } else {
                    count * h.sample_bytes
                };
                let pe = start + need;
                lemma_lit_good_policy_le(s, h, start, paint, pol);
                if s.len() < pe {
                } else if !lit_ok_policy(s, h, start, paint, pol) {
                    lemma_lit_out_policy_len(s, h, map, pol, start, lit_good_policy(
                        s,
                        h,
                        start,
                        paint,
                        pol,
                    ));
                    assert(lit_good_policy(s, h, start, paint, pol) <= paint);
                } else {
                    lemma_lit_out_policy_len(s, h, map, pol, start, paint);
                    lemma_mr_pix_policy_len(s, h, map, pol, pe, prod0 + paint);
                    broadcast use vstd::seq::group_seq_lemmas;
                }
            }
        }
    }
}





/// `pkt_post_policy`'s success arm for a repeat packet: the exit `Ok` state,
/// with the model's own split of the stream as the hypothesis.
pub proof fn lemma_pkt_policy_rep_ok(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur0: int,
    prod0: int,
    acc0: Seq<Seq<u8>>,
    acc1: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod0 < p.n,
        p.sample_bytes >= 1,
        rle_repeat(b[cur0] as int),
        !(rle_count(b[cur0] as int) > p.n - prod0 && pol.rle_overrun == RleOverrunOptions::Reject),
        b.len() >= cur0 + 1 + p.sample_bytes,
        p.kind == 0 ==> !pkt_reject(
            p,
            sindex(b.subrange(cur0 + 1, cur0 + 1 + p.sample_bytes), p.sample_bytes),
            pol,
        ),
        mr_pix_policy(b, p, map, pol, cur0, prod0) == pix_pow(
            conv2_policy(p, map, b.subrange(cur0 + 1, cur0 + 1 + p.sample_bytes), pol),
            if rle_count(b[cur0] as int) < p.n - prod0 {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod0
            },
        ) + mr_pix_policy(
            b,
            p,
            map,
            pol,
            cur0 + 1 + p.sample_bytes,
            prod0 + (if rle_count(b[cur0] as int) < p.n - prod0 {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod0
            }),
        ),
        acc1 == acc0 + pix_pow(conv2_policy(p, map, b.subrange(cur0 + 1, cur0 + 1
            + p.sample_bytes), pol), if rle_count(b[cur0] as int) < p.n - prod0 {
            rle_count(b[cur0] as int)
        } else {
            p.n - prod0
        }),
    ensures
        pkt_post_policy(b, p, map, pol, cur0, prod0, cur0 + 1 + p.sample_bytes, prod0 + (if rle_count(
            b[cur0] as int,
        ) < p.n - prod0 {
            rle_count(b[cur0] as int)
        } else {
            p.n - prod0
        }), acc0, acc1, Ok(())),
{
    reveal(pkt_post_policy);
    reveal(mr_flags_policy);
    lemma_mr_cur_policy_rep_ok(b, p, map, pol, cur0, prod0);
    reveal(mr_gcur_policy);
    vstd::seq_lib::lemma_concat_associative(
        acc0,
        pix_pow(conv2_policy(p, map, b.subrange(cur0 + 1, cur0 + 1 + p.sample_bytes), pol), if rle_count(
            b[cur0] as int,
        ) < p.n - prod0 {
            rle_count(b[cur0] as int)
        } else {
            p.n - prod0
        }),
        mr_pix_policy(b, p, map, pol, cur0 + 1 + p.sample_bytes, prod0 + (if rle_count(
            b[cur0] as int,
        ) < p.n - prod0 {
            rle_count(b[cur0] as int)
        } else {
            p.n - prod0
        })),
    );
}

/// `pkt_post_policy`'s success arm for a literal packet.
pub proof fn lemma_pkt_policy_lit_ok(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur0: int,
    prod0: int,
    acc0: Seq<Seq<u8>>,
    acc1: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod0 < p.n,
        p.sample_bytes >= 1,
        !rle_repeat(b[cur0] as int),
        !(rle_count(b[cur0] as int) > p.n - prod0 && pol.rle_overrun == RleOverrunOptions::Reject),
        b.len() >= cur0 + 1 + (if pol.rle_overrun == RleOverrunOptions::ClipSamples {
            (if rle_count(b[cur0] as int) < p.n - prod0 {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod0
            }) * p.sample_bytes
        } else {
            rle_count(b[cur0] as int) * p.sample_bytes
        }),
        lit_ok_policy(
            b,
            p,
            cur0 + 1,
            if rle_count(b[cur0] as int) < p.n - prod0 {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod0
            },
            pol,
        ),
        mr_pix_policy(b, p, map, pol, cur0, prod0) == lit_out_policy(
            b,
            p,
            map,
            pol,
            cur0 + 1,
            if rle_count(b[cur0] as int) < p.n - prod0 {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod0
            },
        ) + mr_pix_policy(
            b,
            p,
            map,
            pol,
            cur0 + 1 + (if pol.rle_overrun == RleOverrunOptions::ClipSamples {
                (if rle_count(b[cur0] as int) < p.n - prod0 {
                    rle_count(b[cur0] as int)
                } else {
                    p.n - prod0
                }) * p.sample_bytes
            } else {
                rle_count(b[cur0] as int) * p.sample_bytes
            }),
            prod0 + (if rle_count(b[cur0] as int) < p.n - prod0 {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod0
            }),
        ),
        acc1 == acc0 + lit_out_policy(
            b,
            p,
            map,
            pol,
            cur0 + 1,
            if rle_count(b[cur0] as int) < p.n - prod0 {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod0
            },
        ),
    ensures
        pkt_post_policy(b, p, map, pol, cur0, prod0, cur0 + 1 + (if pol.rle_overrun
            == RleOverrunOptions::ClipSamples {
            (if rle_count(b[cur0] as int) < p.n - prod0 {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod0
            }) * p.sample_bytes
        } else {
            rle_count(b[cur0] as int) * p.sample_bytes
        }), prod0 + (if rle_count(b[cur0] as int) < p.n - prod0 {
            rle_count(b[cur0] as int)
        } else {
            p.n - prod0
        }), acc0, acc1, Ok(())),
{
    reveal(pkt_post_policy);
    reveal(mr_flags_policy);
    lemma_mr_cur_policy_lit_ok(b, p, map, pol, cur0, prod0);
    reveal(mr_gcur_policy);
    vstd::seq_lib::lemma_concat_associative(
        acc0,
        lit_out_policy(b, p, map, pol, cur0 + 1, if rle_count(b[cur0] as int) < p.n - prod0 {
            rle_count(b[cur0] as int)
        } else {
            p.n - prod0
        }),
        mr_pix_policy(b, p, map, pol, cur0 + 1 + (if pol.rle_overrun
            == RleOverrunOptions::ClipSamples {
            (if rle_count(b[cur0] as int) < p.n - prod0 {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod0
            }) * p.sample_bytes
        } else {
            rle_count(b[cur0] as int) * p.sample_bytes
        }), prod0 + (if rle_count(b[cur0] as int) < p.n - prod0 {
            rle_count(b[cur0] as int)
        } else {
            p.n - prod0
        })),
    );
}

/// The five exit lemmas for `pkt_post_policy`, each closing one branch of
/// `mr_cur_policy`/`mr_gcur_policy` from the same entry state.
pub proof fn lemma_pkt_policy_ctrl_absent(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur0: int,
    prod0: int,
    acc0: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 <= b.len(),
        cur0 >= b.len(),
        0 <= prod0 < p.n,
    ensures
        pkt_post_policy(b, p, map, pol, cur0, prod0, cur0, prod0, acc0, acc0, Err(
            Error::RleTruncated,
        )),
{
    reveal(pkt_post_policy);
    reveal(mr_pix_policy);
    reveal(mr_flags_policy);
    reveal(mr_subst_policy);
    reveal(mr_clip_policy);
    lemma_mr_cur_policy_ctrl_absent(b, p, map, pol, cur0, prod0);
    reveal(mr_gcur_policy);
    broadcast use vstd::seq_lib::group_seq_lib_default;
}

pub proof fn lemma_pkt_policy_overrun(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur0: int,
    prod0: int,
    acc0: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod0 < p.n,
        rle_count(b[cur0] as int) > p.n - prod0,
        pol.rle_overrun == RleOverrunOptions::Reject,
    ensures
        pkt_post_policy(b, p, map, pol, cur0, prod0, cur0 + 1, prod0, acc0, acc0, Err(
            Error::RleOverrun,
        )),
{
    reveal(pkt_post_policy);
    reveal(mr_pix_policy);
    reveal(mr_flags_policy);
    reveal(mr_subst_policy);
    reveal(mr_clip_policy);
    lemma_mr_cur_policy_overrun(b, p, map, pol, cur0, prod0);
    reveal(mr_gcur_policy);
    broadcast use vstd::seq_lib::group_seq_lib_default;
}

pub proof fn lemma_pkt_policy_rep_missing(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur0: int,
    prod0: int,
    acc0: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod0 < p.n,
        rle_repeat(b[cur0] as int),
        !(rle_count(b[cur0] as int) > p.n - prod0 && pol.rle_overrun == RleOverrunOptions::Reject),
        b.len() < cur0 + 1 + p.sample_bytes,
    ensures
        pkt_post_policy(b, p, map, pol, cur0, prod0, cur0 + 1, prod0, acc0, acc0, Err(
            Error::RleTruncated,
        )),
{
    reveal(pkt_post_policy);
    reveal(mr_flags_policy);
    reveal(mr_subst_policy);
    reveal(mr_clip_policy);
    lemma_mr_cur_policy_rep_missing(b, p, map, pol, cur0, prod0);
    reveal(mr_gcur_policy);
    reveal(mr_pix_policy);
    broadcast use vstd::seq_lib::group_seq_lib_default;
}

pub proof fn lemma_pkt_policy_rep_index(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur0: int,
    prod0: int,
    acc0: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod0 < p.n,
        rle_repeat(b[cur0] as int),
        !(rle_count(b[cur0] as int) > p.n - prod0 && pol.rle_overrun == RleOverrunOptions::Reject),
        b.len() >= cur0 + 1 + p.sample_bytes,
        p.kind == 0,
        pkt_reject(p,
            sindex(b.subrange(cur0 + 1, cur0 + 1 + p.sample_bytes), p.sample_bytes),
            pol),
    ensures
        pkt_post_policy(b, p, map, pol, cur0, prod0, cur0 + 1 + p.sample_bytes, prod0, acc0, acc0, Err(
            Error::ColorIndexOutOfRange,
        )),
{
    reveal(pkt_post_policy);
    reveal(mr_flags_policy);
    reveal(mr_subst_policy);
    reveal(mr_clip_policy);
    lemma_mr_cur_policy_rep_index(b, p, map, pol, cur0, prod0);
    reveal(mr_gcur_policy);
    reveal(mr_pix_policy);
    broadcast use vstd::seq_lib::group_seq_lib_default;
}

pub proof fn lemma_pkt_policy_lit_missing(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur0: int,
    prod0: int,
    acc0: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod0 < p.n,
        !rle_repeat(b[cur0] as int),
        !(rle_count(b[cur0] as int) > p.n - prod0 && pol.rle_overrun == RleOverrunOptions::Reject),
        b.len() < cur0 + 1 + (if pol.rle_overrun == RleOverrunOptions::ClipSamples {
            (if rle_count(b[cur0] as int) < p.n - prod0 {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod0
            }) * p.sample_bytes
        } else {
            rle_count(b[cur0] as int) * p.sample_bytes
        }),
    ensures
        pkt_post_policy(b, p, map, pol, cur0, prod0, cur0 + 1, prod0, acc0, acc0, Err(
            Error::RleTruncated,
        )),
{
    reveal(pkt_post_policy);
    reveal(mr_flags_policy);
    reveal(mr_subst_policy);
    reveal(mr_clip_policy);
    lemma_mr_cur_policy_lit_missing(b, p, map, pol, cur0, prod0);
    reveal(mr_gcur_policy);
    reveal(mr_pix_policy);
    broadcast use vstd::seq_lib::group_seq_lib_default;
}

pub proof fn lemma_pkt_policy_lit_index(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur0: int,
    prod0: int,
    acc0: Seq<Seq<u8>>,
    acc1: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod0 < p.n,
        !rle_repeat(b[cur0] as int),
        !(rle_count(b[cur0] as int) > p.n - prod0 && pol.rle_overrun == RleOverrunOptions::Reject),
        b.len() >= cur0 + 1 + (if pol.rle_overrun == RleOverrunOptions::ClipSamples {
            (if rle_count(b[cur0] as int) < p.n - prod0 {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod0
            }) * p.sample_bytes
        } else {
            rle_count(b[cur0] as int) * p.sample_bytes
        }),
        !lit_ok_policy(
            b,
            p,
            cur0 + 1,
            if rle_count(b[cur0] as int) < p.n - prod0 {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod0
            },
            pol,
        ),
        acc1 == acc0 + lit_out_policy(
            b,
            p,
            map,
            pol,
            cur0 + 1,
            lit_good_policy(
                b,
                p,
                cur0 + 1,
                if rle_count(b[cur0] as int) < p.n - prod0 {
                    rle_count(b[cur0] as int)
                } else {
                    p.n - prod0
                },
                pol,
            ),
        ),
    ensures
        pkt_post_policy(b, p, map, pol, cur0, prod0, cur0 + 1 + (if pol.rle_overrun
            == RleOverrunOptions::ClipSamples {
            (if rle_count(b[cur0] as int) < p.n - prod0 {
                rle_count(b[cur0] as int)
            } else {
                p.n - prod0
            }) * p.sample_bytes
        } else {
            rle_count(b[cur0] as int) * p.sample_bytes
        }), prod0, acc0, acc1, Err(Error::ColorIndexOutOfRange)),
{
    reveal(pkt_post_policy);
    reveal(mr_flags_policy);
    reveal(mr_subst_policy);
    reveal(mr_clip_policy);
    lemma_mr_cur_policy_lit_index(b, p, map, pol, cur0, prod0);
    reveal(mr_gcur_policy);
    reveal(mr_pix_policy);
    broadcast use vstd::seq_lib::group_seq_lib_default;
}



/// The packet-helper postcondition under `pol`: on `Ok` the produced count grew
/// and the cursor, ghost cursor and pixel stream continue; on `Err` the model
/// fails with that error from the entry state, its ghost cursor is the exit
/// cursor, and the accumulator is the entry accumulator plus the pixels up to
/// the failure.
pub open spec fn pkt_post_policy(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur0: int,
    prod0: int,
    cur1: int,
    prod1: int,
    acc0: Seq<Seq<u8>>,
    acc1: Seq<Seq<u8>>,
    r: Result<(), Error>,
) -> bool {
    match r {
        Ok(_) => {
            &&& prod0 < prod1
            &&& mr_cur_policy(b, p, map, pol, cur0, prod0) == mr_cur_policy(
                b,
                p,
                map,
                pol,
                cur1,
                prod1,
            )
            &&& mr_gcur_policy(b, p, map, pol, cur0, prod0) == mr_gcur_policy(
                b,
                p,
                map,
                pol,
                cur1,
                prod1,
            )
            &&& acc0 + mr_pix_policy(b, p, map, pol, cur0, prod0) == acc1 + mr_pix_policy(
                b,
                p,
                map,
                pol,
                cur1,
                prod1,
            )
        },
        Err(e) => {
            &&& mr_cur_policy(b, p, map, pol, cur0, prod0) == Err(MrErr::Api(e))
            &&& mr_gcur_policy(b, p, map, pol, cur0, prod0) == Ok(cur1)
            &&& acc0 + mr_pix_policy(b, p, map, pol, cur0, prod0) == acc1
        },
    }
}

/// The loop's clip/substitution flags are the entry stream's, carried step by
/// step; the packet helper's `Err` arm freezes them at the failing packet's
/// own decision.
pub open spec fn pkt_flags_policy(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur0: int,
    prod0: int,
    cur1: int,
    prod1: int,
    clip: bool,
    subst: bool,
) -> bool {
    &&& clip == mr_clip_policy(b, p, map, pol, cur0, prod0)
    &&& subst == mr_subst_policy(b, p, map, pol, cur0, prod0)
}

/// The pixel accumulator of a policy RLE stream from `(cur, prod)`.
pub open spec fn mr_pix_policy_at(s: Seq<u8>, h: PktH, map: Seq<u8>, pol: Policy, cur: int, prod: int)
    -> Seq<Seq<u8>> {
    mr_pix_policy(s, h, map, pol, cur, prod)
}

/// `mr_pix_policy`'s own recursion at one packet, as the packet helper's
/// postcondition needs it.
pub proof fn lemma_mr_pix_policy_rec(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
)
    requires
        0 <= cur <= s.len(),
        0 <= prod <= h.n,
    ensures
        mr_pix_policy(s, h, map, pol, cur, prod) == mr_pix_policy(s, h, map, pol, cur, prod),
{
}

/// The policy clip event of a stream from `(cur, prod)`: some `Clip*` packet
/// exceeds the remaining pixels.
pub open spec fn mr_clip_policy(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
) -> bool {
    mr_flags_policy(s, h, map, pol, cur, prod).0
}

/// The policy substitution event of a stream from `(cur, prod)`.
pub open spec fn mr_subst_policy(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
) -> bool {
    mr_flags_policy(s, h, map, pol, cur, prod).1
}

/// `mr_flags_policy`'s tuple is the pair of the two projections.
pub proof fn lemma_mr_flags_policy_pair(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
)
    ensures
        mr_flags_policy(s, h, map, pol, cur, prod).0 == mr_clip_policy(s, h, map, pol, cur, prod),
        mr_flags_policy(s, h, map, pol, cur, prod).1 == mr_subst_policy(s, h, map, pol, cur, prod),
{
}

/// `mr_cur_policy` is the cursor/error projection of `mr_at_policy`.
pub proof fn lemma_mr_cur_policy_proj(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
    px: Seq<Seq<u8>>,
)
    ensures
        match mr_at_policy(s, h, map, pol, cur, prod, px) {
            Ok(r) => mr_cur_policy(s, h, map, pol, cur, prod) == Ok((r.0, r.1)),
            Err((e, _c, _pr, _px)) => mr_cur_policy(s, h, map, pol, cur, prod) == Err(e),
        },
    decreases (h.n - prod) as nat,
{
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
        reveal(mr_at_policy);
        reveal(mr_cur_policy);
    } else if prod == h.n {
        reveal(mr_at_policy);
        reveal(mr_cur_policy);
    } else if cur >= s.len() {
        reveal(mr_at_policy);
        reveal(mr_cur_policy);
    } else {
        reveal(mr_at_policy);
        reveal(mr_cur_policy);
        let control = s[cur] as int;
        let count = rle_count(control);
        let remaining = h.n - prod;
        let start = cur + 1;
        if count > remaining && pol.rle_overrun == RleOverrunOptions::Reject {
        } else {
            let paint = if count < remaining { count } else { remaining };
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                } else {
                    let smp = s.subrange(start, pe);
                    if h.kind == 0 && pkt_reject(h, sindex(smp, h.sample_bytes), pol) {
                    } else {
                        lemma_mr_cur_policy_proj(
                            s,
                            h,
                            map,
                            pol,
                            pe,
                            prod + paint,
                            px + pix_pow(conv2_policy(h, map, smp, pol), paint),
                        );
                    }
                }
            } else {
                let need = if pol.rle_overrun == RleOverrunOptions::ClipSamples {
                    paint * h.sample_bytes
                } else {
                    count * h.sample_bytes
                };
                let pe = start + need;
                if s.len() < pe {
                } else if !lit_ok_policy(s, h, start, paint, pol) {
                } else {
                    lemma_mr_cur_policy_proj(
                        s,
                        h,
                        map,
                        pol,
                        pe,
                        prod + paint,
                        px + lit_out_policy(s, h, map, pol, start, paint),
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// The policy output-write invariants: what the profiled raw and RLE loops prove
// their writes establish, as the policy `conv2_policy` analog of the 1.0
// `wrote_samples`/`rle_quad_ok`.
// ---------------------------------------------------------------------------

/// After `k` samples, each converted pixel `j<k` sits at R9's oriented byte
/// offset, converted under `pol`.
pub open spec fn wrote_samples_policy(
    out: Seq<u8>,
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    start: int,
    sb: int,
    k: int,
) -> bool {
    forall|j: int| 0 <= j < k ==> #[trigger] crate::model::quad_at(
        out,
        4 * crate::model::fcoord(w, hgt, fx, fy, j),
        conv2_policy(h, map, s.subrange(start + j * sb, start + j * sb + sb), pol),
    )
}

/// Unpack `wrote_samples_policy` at one index.
pub proof fn lemma_wrote_samples_policy_at(
    out: Seq<u8>,
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    start: int,
    sb: int,
    k: int,
    j: int,
)
    requires
        wrote_samples_policy(out, s, h, map, pol, w, hgt, fx, fy, start, sb, k),
        0 <= j < k,
    ensures
        crate::model::quad_at(
            out,
            4 * crate::model::fcoord(w, hgt, fx, fy, j),
            conv2_policy(h, map, s.subrange(start + j * sb, start + j * sb + sb), pol),
        ),
{
    reveal(wrote_samples_policy);
}

/// One write of a new pixel at index `k` extends `wrote_samples_policy`.
pub proof fn lemma_wrote_samples_policy_step(
    out1: Seq<u8>,
    out0: Seq<u8>,
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    start: int,
    sb: int,
    k: int,
)
    requires
        wrote_samples_policy(out0, s, h, map, pol, w, hgt, fx, fy, start, sb, k),
        crate::model::quad_at(
            out1,
            4 * crate::model::fcoord(w, hgt, fx, fy, k),
            conv2_policy(h, map, s.subrange(start + k * sb, start + k * sb + sb), pol),
        ),
        forall|j: int| 0 <= j < k ==> #[trigger] crate::model::quad_at(
            out1,
            4 * crate::model::fcoord(w, hgt, fx, fy, j),
            conv2_policy(h, map, s.subrange(start + j * sb, start + j * sb + sb), pol),
        ),
    ensures
        wrote_samples_policy(out1, s, h, map, pol, w, hgt, fx, fy, start, sb, k + 1),
{
    reveal(wrote_samples_policy);
}




// ---------------------------------------------------------------------------
// The Strict bridges: the policy model at Strict is the 1.0 model.
// ---------------------------------------------------------------------------

pub proof fn lemma_kind_of_type_is_indexed(t: u8)
    ensures
        (kind_of_type(t) == 0) == is_indexed(t),
        kind_of_type(t) == 1 || kind_of_type(t) == 2 || kind_of_type(t) == 0,
{
    reveal(kind_of_type);
    reveal(is_indexed);
}

pub proof fn lemma_type_ok_policy_strict(t: u8, p: Policy)
    requires
        p == strict_policy(),
    ensures
        type_ok_policy(t, p) == type_ok(t),
        kind_policy(t, p) == kind_of_type(t),
{
    reveal(strict_policy);
    reveal(type_ok_policy);
    reveal(kind_policy);
    reveal(kind_of_type);
    reveal(is_indexed);
}

pub proof fn lemma_map_strict(b: Seq<u8>, p: Policy)
    requires
        p == strict_policy(),
        18 <= b.len(),
    ensures
        map_flag_ok_policy(b, p) == (b[1] <= 1u8),
        map_present_policy(b, p) == (b[1] == 1u8),
        map_range_bad(b, map_present_policy(b, p), p.map_header) == (b[1] == 1u8 && cm_range_bad(b)),
{
    reveal(strict_policy);
    reveal(map_flag_ok_policy);
    reveal(map_present_policy);
    reveal(map_range_bad);
    reveal(cm_range_bad);
}

pub proof fn lemma_depth_ok_policy_strict(t: u8, d: u8, p: Policy)
    requires
        p == strict_policy(),
    ensures
        depth_ok_policy(kind_of_type(t), d, p.depths) == depth_ok(t, d),
{
    reveal(strict_policy);
    reveal(depth_ok_policy);
    reveal(depth_ok);
    reveal(kind_of_type);
    reveal(is_indexed);
}

/// At Strict, `map_coherence_bad` with `kind_of_type(b[2])` is 1.0's `cm_bad`
/// (on a flag byte that has already passed R2).
pub proof fn lemma_map_coherence_strict(b: Seq<u8>, p: Policy)
    requires
        p == strict_policy(),
        18 <= b.len(),
        b[1] <= 1u8,
    ensures
        map_coherence_bad(b, b[1] == 1u8, kind_of_type(b[2]), p.map_header) == cm_bad(b),
{
    reveal(strict_policy);
    reveal(map_coherence_bad);
    reveal(cm_bad);
    reveal(map_entry_depth_ok);
    reveal(kind_of_type);
    reveal(is_indexed);
    assert(p.map_header == MapHeaderOptions::Core);
    lemma_kind_of_type_is_indexed(b[2]);
    if b[1] == 0u8 {
        assert(map_coherence_bad(b, b[1] == 1u8, kind_of_type(b[2]), p.map_header) == (le16(b, 3) != 0
            || le16(b, 5) != 0 || b[7] != 0u8 || kind_of_type(b[2]) == 0));
        assert(cm_bad(b) == (le16(b, 3) != 0 || le16(b, 5) != 0 || b[7] != 0u8 || is_indexed(b[2])));
    } else {
        assert(b[1] == 1u8);
        assert(map_coherence_bad(b, b[1] == 1u8, kind_of_type(b[2]), p.map_header) == (le16(b, 5) == 0
            || !map_entry_depth_ok(b[7])));
        assert(cm_bad(b) == (le16(b, 5) == 0 || !map_entry_depth_ok(b[7])));
    }
}

pub proof fn lemma_attr_ok_policy_strict(b: Seq<u8>, p: Policy)
    requires
        p == strict_policy(),
        18 <= b.len(),
        b[1] <= 1u8,
        type_ok(b[2]),
        depth_ok(b[2], b[16]),
        !cm_bad(b),
    ensures
        attr_ok_policy(
            kind_of_type(b[2]),
            (if is_indexed(b[2]) {
                b[7] as int
            } else {
                b[16] as int
            }),
            (b[17] & 0x0fu8) as int,
            p.attribute_bits,
        ) == attr_ok(b),
{
    reveal(strict_policy);
    reveal(attr_ok_policy);
    reveal(attr_ok);
    reveal(core_attr_allowed);
    reveal(map_attr_ok);
    reveal(map_entry_depth_ok);
    reveal(depth_ok);
    reveal(kind_of_type);
    reveal(is_indexed);
    reveal(cm_bad);
    assert(p.attribute_bits == AttributeBitsOptions::Exact);
    if is_indexed(b[2]) {
        // `cm_bad` with flag zero is true for an indexed image (its
        // `is_indexed` clause), so the flag passed as one.
        assert(b[1] != 0u8) by {
            if b[1] == 0u8 {
                assert(cm_bad(b) == (le16(b, 3) != 0 || le16(b, 5) != 0 || b[7] != 0u8
                    || is_indexed(b[2])));
                assert(is_indexed(b[2]));
                assert(cm_bad(b));
            }
        }
        assert(b[1] == 1u8);
        assert(cm_bad(b) == (le16(b, 5) == 0 || !map_entry_depth_ok(b[7])));
        assert(map_entry_depth_ok(b[7]));
    } else if b[2] == 2u8 || b[2] == 10u8 {
        assert(depth_ok(b[2], b[16]));
        assert(b[16] == 15u8 || b[16] == 16u8 || b[16] == 24u8 || b[16] == 32u8);
    } else {
        assert(depth_ok(b[2], b[16]));
        assert(b[16] == 8u8 || b[16] == 16u8);
    }
}

pub proof fn lemma_map_bytes_policy_strict(b: Seq<u8>, p: Policy)
    requires
        p == strict_policy(),
        18 <= b.len(),
    ensures
        map_bytes_policy(b, p) == (if b[1] == 1u8 {
            le16(b, 5) * sample_bytes_of(b[7])
        } else {
            0
        }),
{
    reveal(strict_policy);
    reveal(map_bytes_policy);
    reveal(map_present_policy);
}

/// `MH(b, Strict) = MH(b)`: a definitional reduction, so the T5 proof for the
/// 1.0 `parse_header` carries over to the policy parser by unfolding.
pub proof fn lemma_mh_policy_strict(b: Seq<u8>, p: Policy)
    requires
        p == strict_policy(),
    ensures
        mh_policy(b, p) == mh(b),
{
    reveal(strict_policy);
    reveal(mh_policy);
    reveal(mh);
    if b.len() > max_input() {
    } else if b.len() < 18 {
    } else {
        assert(18 <= b.len());
        lemma_type_ok_policy_strict(b[2], p);
        if !type_ok(b[2]) {
        } else if b[1] > 1u8 {
            lemma_map_strict(b, p);
        } else {
            assert(b[1] <= 1u8);
            lemma_map_strict(b, p);
            if !(1 <= le16(b, 12) && le16(b, 12) <= max_dimension())
                || !(1 <= le16(b, 14) && le16(b, 14) <= max_dimension()) {
            } else if b[17] & 0xc0u8 != 0u8 {
                assert(p.interleave == InterleaveOptions::Reject);
            } else {
                lemma_depth_ok_policy_strict(b[2], b[16], p);
                if !depth_ok(b[2], b[16]) {
                } else {
                    lemma_map_coherence_strict(b, p);
                    if cm_bad(b) {
                    } else if b[1] == 1u8 && cm_range_bad(b) {
                    } else {
                        lemma_attr_ok_policy_strict(b, p);
                        reveal(attr_depth);
                        if !attr_ok(b) {
                        } else if 4 * (le16(b, 12) * le16(b, 14)) > max_output() {
                        } else {
                            lemma_map_bytes_policy_strict(b, p);
                            reveal(image_index_depth_bad);
                            assert(p.depths == DepthsOptions::Core);
                            assert(!image_index_depth_bad(
                                b,
                                map_present_policy(b, p),
                                kind_policy(b[2], p),
                                p.depths,
                            ));
                        }
                    }
                }
            }
        }
    }
}

/// `conv2(.., Strict) = conv2(..)` on the 1.0 conversion domain (an indexed
/// sample whose index is in range, with the map slice of exactly
/// `length*stride` bytes).
pub proof fn lemma_conv2_policy_strict(h: PktH, map: Seq<u8>, s: Seq<u8>, p: Policy)
    requires
        p == strict_policy(),
        0 <= h.sample_bytes,
        0 <= h.map_depth,
        h.kind == 0 ==> h.map_first <= sindex(s, h.sample_bytes) < h.map_first + h.map_len,
        h.kind == 0 ==> map.len() == h.map_len * stride_of(h.map_depth),
    ensures
        conv2_policy(h, map, s, p) == conv2(h, map, s),
{
    reveal(strict_policy);
    reveal(conv2_policy);
    reveal(conv2);
    reveal(conv_sample_policy);
    reveal(conv_sample);
    reveal(conv_map_policy);
    reveal(conv_map);
    reveal(entry_j);
    reveal(in_range_policy);
    reveal(stored_alpha_policy);
    reveal(map555_policy);
    reveal(rgb555_policy);
    reveal(map_entry_alpha_policy);
    reveal(entry_alpha_spec);
    reveal(bytes4_15);
    reveal(rgb555);
    reveal(bit15);
    reveal(stride_of);
    if h.kind == 0 {
        reveal(map_entry_depth_ok);
        assert(entry_j(sindex(s, h.sample_bytes), h.map_first, p) == sindex(s, h.sample_bytes)
            - h.map_first);
        assert(0 <= sindex(s, h.sample_bytes) - h.map_first < h.map_len);
        assert(0 <= stride_of(h.map_depth)) by (nonlinear_arith)
            requires
                stride_of(h.map_depth) == (h.map_depth + 7) / 8,
                h.map_depth >= 0,
        ;
        assert(0 <= entry_j(sindex(s, h.sample_bytes), h.map_first, p));
    }
}

/// At Strict, the policy index test is 1.0's `in_range`.
pub proof fn lemma_in_range_policy_strict(h: PktH, index: int, p: Policy)
    requires
        p == strict_policy(),
    ensures
        in_range_policy(h, index, p) == in_range(h, index),
{
    reveal(strict_policy);
    reveal(in_range_policy);
    reveal(in_range);
    reveal(entry_j);
}

/// The policy header's success bounds (T2 for every retained record).
pub proof fn lemma_mh_policy_ok_bounds(b: Seq<u8>, m: MHModel, p: Policy)
    requires
        mh_policy(b, p) == Ok(m),
    ensures
        mh_bounds(m),
        1 <= m.width as int <= 8192,
        1 <= m.height as int <= 8192,
        m.n <= 16777216,
        0 <= m.map_bytes <= 262140,
        1 <= m.sample_bytes <= 4,
{
    reveal(mh_policy);
    reveal(mh_bounds);
    reveal(max_dimension);
    reveal(max_output);
    reveal(max_input);
    reveal(sample_bytes_of);
    reveal(map_bytes_policy);
    reveal(map_present_policy);
    reveal(map_coherence_bad);
    reveal(map_entry_depth_ok);
    assert(b.len() >= 18);
    let w = le16(b, 12);
    let hgt = le16(b, 14);
    // The successful record is the model's own literal, as in 1.0's lemma.
    assert(m == MHModel {
        id_length: b[0],
        color_map_flag: b[1],
        image_type: b[2],
        map_first: le16(b, 3) as u32,
        map_length: le16(b, 5) as u32,
        map_entry_depth: b[7],
        x_origin: le16(b, 8) as u32,
        y_origin: le16(b, 10) as u32,
        width: le16(b, 12) as u32,
        height: le16(b, 14) as u32,
        pixel_depth: b[16],
        descriptor: b[17],
        n: le16(b, 12) * le16(b, 14),
        sample_bytes: sample_bytes_of(b[16]),
        map_bytes: map_bytes_policy(b, p),
    });
    lemma_le16_lt(b, 12);
    lemma_le16_lt(b, 14);
    lemma_le16_lt(b, 5);
    lemma_le16_lt(b, 7);
    assert(1 <= w <= 8192);
    assert(1 <= hgt <= 8192);
    assert(m.width == w as u32);
    assert(m.height == hgt as u32);
    assert(w as u32 as int == w);
    assert(hgt as u32 as int == hgt);
    assert(m.n == w * hgt);
    assert(4 * (w * hgt) <= 67108864) by (nonlinear_arith)
        requires
            1 <= w <= 8192,
            1 <= hgt <= 8192,
            4 * (w * hgt) <= 67108864,
    ;
    assert(w * hgt <= 16777216) by (nonlinear_arith)
        requires
            1 <= w <= 8192,
            1 <= hgt <= 8192,
            4 * (w * hgt) <= 67108864,
    ;
    assert(m.sample_bytes == sample_bytes_of(b[16]));
        lemma_mh_policy_sample_bytes(b, p);
    assert(1 <= m.sample_bytes <= 4);
    // The map bound: `le16(b,5) <= 65535` always, and `sample_bytes_of(b[7]) <= 4`
    // whenever the product is nonzero (a nonzero length forces a valid entry
    // depth past the coherence check).
    if !map_present_policy(b, p) {
        assert(m.map_bytes == 0);
    } else {
        assert(m.map_bytes == le16(b, 5) * sample_bytes_of(b[7]));
        if le16(b, 5) == 0 {
            assert(m.map_bytes == 0) by (nonlinear_arith)
                requires
                    m.map_bytes == le16(b, 5) * sample_bytes_of(b[7]),
                    le16(b, 5) == 0,
            ;
        } else {
            assert(map_entry_depth_ok(b[7])) by {
                if p.map_header == MapHeaderOptions::Core || p.map_header
                    == MapHeaderOptions::IgnoreAbsent {
                    assert(!(le16(b, 5) == 0 || !map_entry_depth_ok(b[7])));
                } else if p.map_header == MapHeaderOptions::Gdk {
                    assert(!(le16(b, 5) != 0 && !map_entry_depth_ok(b[7])));
                } else {
                    assert(!(!map_entry_depth_ok(b[7])));
                }
            }
            reveal(map_entry_depth_ok);
            assert(b[7] == 15u8 || b[7] == 16u8 || b[7] == 24u8 || b[7] == 32u8);
            lemma_sample_bytes_le_4(b[7]);
            assert(0 <= le16(b, 5) * sample_bytes_of(b[7]) <= 262140) by (nonlinear_arith)
                requires
                    0 <= le16(b, 5) <= 65535,
                    0 <= sample_bytes_of(b[7]) <= 4,
            ;
        }
    }
    assert(0 <= m.map_bytes <= 262140);
    assert(m.width == w as u32);
    assert(m.height == hgt as u32);
    assert(m.n == m.width as int * (m.height as int));
    assert(1 <= m.width as int <= max_dimension());
    assert(1 <= m.height as int <= max_dimension());
    assert(1 <= m.n) by (nonlinear_arith)
        requires
            m.n == w * hgt,
            1 <= w,
            1 <= hgt,
    ;
    assert(4 * m.n <= max_output());
    assert(m.n <= 16777216);
    assert(0 <= m.map_bytes <= 262140);
    assert(1 <= m.sample_bytes <= 4);
    assert(m.id_length as int <= 255);
    assert(18 + (m.id_length as int) + m.map_bytes <= 262413) by (nonlinear_arith)
        requires
            m.id_length as int <= 255,
            m.map_bytes <= 262140,
    ;
}

/// The image depth of a passing policy header is one of the supported depths
/// (R3's table for the record's own kind).
pub proof fn lemma_mh_policy_sample_bytes(b: Seq<u8>, p: Policy)
    requires
        mh_policy(b, p) is Ok,
    ensures
        b[16] == 8u8 || b[16] == 15u8 || b[16] == 16u8 || b[16] == 24u8 || b[16] == 32u8,
        1 <= sample_bytes_of(b[16]) <= 4,
{
    reveal(mh_policy);
    reveal(depth_ok_policy);
    reveal(kind_policy);
    reveal(kind_of_type);
    reveal(is_indexed);
    reveal(sample_bytes_of);
    assert(b.len() >= 18);
    assert(depth_ok_policy(kind_policy(b[2], p), b[16], p.depths));
    assert(b[16] == 8u8 || b[16] == 15u8 || b[16] == 16u8 || b[16] == 24u8 || b[16] == 32u8);
    lemma_sample_bytes_le_4(b[16]);
}

// ---------------------------------------------------------------------------
// R10/R11/R12: the policy-indexed pixel phase and the complete model `MP(b,p)`.
// ---------------------------------------------------------------------------

/// R10: the raw phase under `p`, `k` of the `N` samples already decoded. Unlike
/// 1.0's `mp_raw_acc`, the index is the *policy* index test and the produced
/// pixel is the policy conversion; the guard order is identical.
pub open spec fn mp_raw_acc_policy(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    c0: int,
    k: int,
    acc: Seq<Seq<u8>>,
) -> Result<(Seq<Seq<u8>>, int), (Error, int, int, Seq<Seq<u8>>)>
    decreases (p.n - k) as nat
{
    if k < 0 || k > p.n || p.sample_bytes <= 0 {
        Err((Error::ColorIndexOutOfRange, c0 + k * p.sample_bytes, k, acc))
    } else if k == p.n {
        Ok((acc, c0 + k * p.sample_bytes))
    } else if c0 + k * p.sample_bytes + p.sample_bytes > b.len() {
        Err((Error::TruncatedPixels, c0 + k * p.sample_bytes, k, acc))
    } else {
        let smp = b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes);
        if p.kind == 0 && !in_range_policy(p, sindex(smp, p.sample_bytes), pol)
            && pol.index_range == IndexRangeOptions::Reject
        {
            Err((Error::ColorIndexOutOfRange, c0 + k * p.sample_bytes + p.sample_bytes, k, acc))
        } else {
            mp_raw_acc_policy(b, p, map, pol, c0, k + 1, acc + seq![conv2_policy(p, map, smp, pol)])
        }
    }
}

/// R10 from the start of pixel data under `p`.
pub open spec fn mp_raw_policy(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    c0: int,
) -> Result<(Seq<Seq<u8>>, int), (Error, int, int, Seq<Seq<u8>>)> {
    mp_raw_acc_policy(b, p, map, pol, c0, 0, Seq::empty())
}

/// The policy `lit_ok`: every sample of a literal packet has an in-range index
/// (indexed images, `Reject` only). Under `Black` no index ever fails, because
/// an out-of-range one is substituted rather than rejected.
pub open spec fn lit_ok_policy(s: Seq<u8>, h: PktH, start: int, count: int, pol: Policy) -> bool
    decreases count
{
    if count <= 0 {
        true
    } else if h.kind != 0 || pol.index_range == IndexRangeOptions::Black {
        lit_ok_policy(s, h, start + h.sample_bytes, count - 1, pol)
    } else {
        in_range_policy(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes), pol)
            && lit_ok_policy(s, h, start + h.sample_bytes, count - 1, pol)
    }
}

/// The policy `lit_good`: the number of leading samples whose index is in range
/// (or the whole packet under `Black` or a non-indexed image).
pub open spec fn lit_good_policy(s: Seq<u8>, h: PktH, start: int, count: int, pol: Policy) -> int
    decreases count
{
    if count <= 0 {
        count
    } else if h.kind != 0 || pol.index_range == IndexRangeOptions::Black {
        count
    } else {
        let g = lit_good_policy(s, h, start + h.sample_bytes, count - 1, pol);
        if in_range_policy(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes), pol) {
            1 + g
        } else {
            0
        }
    }
}

/// The policy literal-packet output: the policy conversion of each sample.
pub open spec fn lit_out_policy(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    start: int,
    count: int,
) -> Seq<Seq<u8>>
    decreases count
{
    if count <= 0 {
        Seq::empty()
    } else {
        seq![conv2_policy(h, map, s.subrange(start, start + h.sample_bytes), pol)] + lit_out_policy(
            s,
            h,
            map,
            pol,
            start + h.sample_bytes,
            count - 1,
        )
    }
}

/// The policy clip bit of an RLE packet: the packet's count exceeds the
/// remaining pixels and `rle_overrun` is a `Clip` value, so at least one
/// encoded position is discarded.
pub open spec fn pkt_clip(count: int, remaining: int, pol: Policy) -> bool {
    count > remaining && pol.rle_overrun != RleOverrunOptions::Reject
}

/// The policy substitution bit of an indexed sample: the index is out of range
/// and `index_range = Black` substitutes it at a pixel write.
pub open spec fn pkt_subst(h: PktH, index: int, pol: Policy) -> bool {
    !in_range_policy(h, index, pol) && pol.index_range == IndexRangeOptions::Black
}

/// The policy rejection of an indexed sample: the index is out of range and
/// `index_range = Reject` (1.0's `ColorIndexOutOfRange`).
pub open spec fn pkt_reject(h: PktH, index: int, pol: Policy) -> bool {
    !in_range_policy(h, index, pol) && pol.index_range == IndexRangeOptions::Reject
}

/// The clip/substitution bits and pixels of a policy RLE stream, from
/// `(cur, prod)`: returns `(clip, subst)`.
pub open spec fn mr_flags_policy(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
) -> (bool, bool)
    decreases (h.n - prod) as nat
{
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
        (false, false)
    } else if prod == h.n {
        (false, false)
    } else if cur >= s.len() {
        (false, false)
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        let remaining = h.n - prod;
        let start = cur + 1;
        let clip = pkt_clip(count, remaining, pol);
        if pol.rle_overrun == RleOverrunOptions::Reject && count > remaining {
            (false, false)
        } else {
            let paint = if count < remaining { count } else { remaining };
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                    (clip, false)
                } else {
                    let smp = s.subrange(start, pe);
                    let idx = sindex(smp, h.sample_bytes);
                    if h.kind == 0 && pkt_reject(h, idx, pol) {
                        (clip, false)
                    } else {
                        let here = h.kind == 0 && pkt_subst(h, idx, pol);
                        let rest = mr_flags_policy(s, h, map, pol, pe, prod + paint);
                        (clip || rest.0, here || rest.1)
                    }
                }
            } else {
                let need = if pol.rle_overrun == RleOverrunOptions::ClipSamples {
                    paint * h.sample_bytes
                } else {
                    count * h.sample_bytes
                };
                let pe = start + need;
                if s.len() < pe {
                    (clip, false)
                } else {
                    let (sub, rest) = lit_flags_policy(s, h, pol, start, paint);
                    let tail = mr_flags_policy(s, h, map, pol, pe, prod + paint);
                    (clip || tail.0, sub || tail.1)
                }
            }
        }
    }
}

/// The substitution bit of a literal packet's `count` painted samples: true iff
/// any is an out-of-range index under `Black`.
pub open spec fn lit_flags_policy(s: Seq<u8>, h: PktH, pol: Policy, start: int, count: int)
    -> (bool, bool)
    decreases count
{
    if count <= 0 {
        (false, false)
    } else {
        let idx = sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes);
        let first = h.kind == 0 && pkt_subst(h, idx, pol);
        let rest = lit_flags_policy(s, h, pol, start + h.sample_bytes, count - 1);
        (first || rest.0, first || rest.1)
    }
}

/// R11: one continuous packet stream under `p` (the policy `MR`). The check
/// order is control presence, the overrun test (only under `Reject`), whole- or
/// painted-packet availability, then each sample's index in output order. Unlike
/// 1.0, an over-long packet under `Clip*` paints `min(count, N-produced)` and
/// advances the cursor by the whole (`ClipPacket`) or painted (`ClipSamples`)
/// packet. The ghost error tuple is `MR`'s.
pub open spec fn mr_at_policy(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
    out: Seq<Seq<u8>>,
) -> Result<(int, int, Seq<Seq<u8>>), (MrErr, int, int, Seq<Seq<u8>>)>
    decreases (h.n - prod) as nat
{
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
        Err((MrErr::Outside, cur, prod, out))
    } else if prod == h.n {
        Ok((cur, prod, out))
    } else if cur >= s.len() {
        Err((MrErr::Api(Error::RleTruncated), cur, prod, out))
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        let remaining = h.n - prod;
        let start = cur + 1;
        if count > remaining && pol.rle_overrun == RleOverrunOptions::Reject {
            Err((MrErr::Api(Error::RleOverrun), cur + 1, prod, out))
        } else {
            let paint = if count < remaining { count } else { remaining };
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                    Err((MrErr::Api(Error::RleTruncated), start, prod, out))
                } else {
                    let smp = s.subrange(start, pe);
                    if h.kind == 0 && pkt_reject(h, sindex(smp, h.sample_bytes), pol) {
                        Err((MrErr::Api(Error::ColorIndexOutOfRange), pe, prod, out))
                    } else {
                        mr_at_policy(
                            s,
                            h,
                            map,
                            pol,
                            pe,
                            prod + paint,
                            out + pix_pow(conv2_policy(h, map, smp, pol), paint),
                        )
                    }
                }
            } else {
                let need = if pol.rle_overrun == RleOverrunOptions::ClipSamples {
                    paint * h.sample_bytes
                } else {
                    count * h.sample_bytes
                };
                let pe = start + need;
                if s.len() < pe {
                    Err((MrErr::Api(Error::RleTruncated), start, prod, out))
                } else if !lit_ok_policy(s, h, start, paint, pol) {
                    let g = lit_good_policy(s, h, start, paint, pol);
                    Err((MrErr::Api(Error::ColorIndexOutOfRange), pe, prod + g, out
                        + lit_out_policy(s, h, map, pol, start, g)))
                } else {
                    mr_at_policy(
                        s,
                        h,
                        map,
                        pol,
                        pe,
                        prod + paint,
                        out + lit_out_policy(s, h, map, pol, start, paint),
                    )
                }
            }
        }
    }
}

/// The policy `MR` from the start of packet data.
pub open spec fn mr_policy(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cursor0: int,
    produced0: int,
) -> Result<(int, int, Seq<Seq<u8>>), (MrErr, int, int, Seq<Seq<u8>>)> {
    mr_at_policy(s, h, map, pol, cursor0, produced0, Seq::empty())
}

/// The pixels a policy RLE stream produces from `(cur, prod)`, mirroring the
/// policy `MR`'s pixel accumulation.
pub open spec fn mr_pix_policy(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
) -> Seq<Seq<u8>>
    decreases (h.n - prod) as nat
{
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
        Seq::empty()
    } else if prod == h.n {
        Seq::empty()
    } else if cur >= s.len() {
        Seq::empty()
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        let remaining = h.n - prod;
        let start = cur + 1;
        if pol.rle_overrun == RleOverrunOptions::Reject && count > remaining {
            Seq::empty()
        } else {
            let paint = if count < remaining { count } else { remaining };
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                    Seq::empty()
                } else {
                    let smp = s.subrange(start, pe);
                    if h.kind == 0 && pkt_reject(h, sindex(smp, h.sample_bytes), pol) {
                        Seq::empty()
                    } else {
                        pix_pow(conv2_policy(h, map, smp, pol), paint) + mr_pix_policy(
                            s,
                            h,
                            map,
                            pol,
                            pe,
                            prod + paint,
                        )
                    }
                }
            } else {
                let need = if pol.rle_overrun == RleOverrunOptions::ClipSamples {
                    paint * h.sample_bytes
                } else {
                    count * h.sample_bytes
                };
                let pe = start + need;
                if s.len() < pe {
                    Seq::empty()
                } else if !lit_ok_policy(s, h, start, paint, pol) {
                    lit_out_policy(s, h, map, pol, start, lit_good_policy(s, h, start, paint, pol))
                } else {
                    lit_out_policy(s, h, map, pol, start, paint) + mr_pix_policy(
                        s,
                        h,
                        map,
                        pol,
                        pe,
                        prod + paint,
                    )
                }
            }
        }
    }
}

/// The raw-phase converted pixels of a passing policy header: the policy
/// conversions of the input-order samples (the same sequence as 1.0's
/// `acc_conv`, over the policy conversion).
pub open spec fn acc_conv_policy(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    c0: int,
    k: int,
) -> Seq<Seq<u8>> {
    let sb = p.sample_bytes;
    Seq::new(k as nat, |j: int| conv2_policy(
        p,
        map,
        b.subrange(c0 + j * sb, c0 + j * sb + sb),
        pol,
    ))
}

/// The pixels a passing policy `MP` decodes, in input order.
pub open spec fn mp_pixels_policy(b: Seq<u8>, m: MHModel, pol: Policy, id_end: int)
    -> Seq<Seq<u8>> {
    let p = pkt_of_mh_policy(m, pol);
    let map = b.subrange(id_end, id_end + m.map_bytes);
    let c0 = id_end + m.map_bytes;
    if rle_policy(m.image_type, pol) {
        mr_pix_policy(b, p, map, pol, c0, 0)
    } else {
        acc_conv_policy(b, p, map, pol, c0, m.n)
    }
}

/// R9/R8/R13: the oriented output bytes of a fully decoded policy image.
pub open spec fn mp_assemble_policy(b: Seq<u8>, m: MHModel, pol: Policy, id_end: int)
    -> Seq<u8> {
    orient_bytes(
        mp_pixels_policy(b, m, pol, id_end),
        m.width as int,
        m.height as int,
        m.descriptor & 0x10u8 != 0u8,
        m.descriptor & 0x20u8 != 0u8,
        m.n,
    )
}

/// R12 under `p`: `FooterOnly` is 1.0's suffix test; `Ignore` accepts any
/// suffix.
pub open spec fn mp_suffix_ok_policy(b: Seq<u8>, pol: Policy, cursor: int) -> bool {
    if pol.trailing == TrailingOptions::Ignore {
        true
    } else {
        mp_suffix_ok(b, cursor)
    }
}

/// `MP(b,p)` after a successful `MH(b,p)`: R7's ID and map slices, then R10's
/// raw phase or R11's packet stream, then R12's suffix test, then R9/R8's
/// oriented assembly. Every branch returns immediately with the first error.
pub open spec fn mp_after_policy(b: Seq<u8>, m: MHModel, pol: Policy)
    -> Result<(int, int, Seq<u8>), Error> {
    let id_end = 18 + (m.id_length as int);
    if id_end > b.len() {
        Err(Error::TruncatedImageId)
    } else {
        let map_end = id_end + m.map_bytes;
        if map_end > b.len() {
            Err(Error::TruncatedColorMap)
        } else {
            let map = b.subrange(id_end, map_end);
            let p = pkt_of_mh_policy(m, pol);
            if rle_policy(m.image_type, pol) {
                match mr_policy(b, p, map, pol, map_end, 0) {
                    Err((MrErr::Api(e), _c, _pr, _px)) => Err(e),
                    Err((MrErr::Outside, _c, _pr, _px)) => Err(Error::ColorIndexOutOfRange),
                    Ok((cursor, _produced, _px)) => {
                        if mp_suffix_ok_policy(b, pol, cursor) {
                            Ok((m.width as int, m.height as int, mp_assemble_policy(b, m, pol, id_end)))
                        } else {
                            Err(Error::TrailingBytes)
                        }
                    },
                }
            } else {
                match mp_raw_policy(b, p, map, pol, map_end) {
                    Err((e, _c, _pr, _px)) => Err(e),
                    Ok((_px, cursor)) => {
                        if mp_suffix_ok_policy(b, pol, cursor) {
                            Ok((m.width as int, m.height as int, mp_assemble_policy(b, m, pol, id_end)))
                        } else {
                            Err(Error::TrailingBytes)
                        }
                    },
                }
            }
        }
    }
}

/// `MP(b,p)`, the complete total policy decoder model: `MH(b,p)` first
/// (propagating its error), then the ID/map slices, the pixel phase, the suffix
/// test and the oriented RGBA assembly.
pub open spec fn mp_policy(b: Seq<u8>, pol: Policy) -> Result<(int, int, Seq<u8>), Error> {
    match mh_policy(b, pol) {
        Err(e) => Err(e),
        Ok(m) => mp_after_policy(b, m, pol),
    }
}

/// The `recovered` flag of a policy decode (POLICY-SPEC "The `recovered` flag"):
/// true iff a clip event or a substitution event occurs during a successful
/// decode. Under `Reject` on both fields no event is reachable, so it is false.
pub open spec fn recovered_policy(b: Seq<u8>, pol: Policy) -> bool {
    match mh_policy(b, pol) {
        Err(_) => false,
        Ok(m) => {
            let id_end = 18 + (m.id_length as int);
            if id_end > b.len() || id_end + m.map_bytes > b.len() {
                false
            } else {
                let p = pkt_of_mh_policy(m, pol);
                let map = b.subrange(id_end, id_end + m.map_bytes);
                let c0 = id_end + m.map_bytes;
                if rle_policy(m.image_type, pol) {
                    mr_flags_policy(b, p, map, pol, c0, 0).0 || mr_flags_policy(b, p, map, pol, c0, 0).1
                } else {
                    raw_subst_policy(b, p, pol, c0, m.n)
                }
            }
        },
    }
}

/// The substitution bit of a raw phase's `k` samples.
pub open spec fn raw_subst_policy(b: Seq<u8>, p: PktH, pol: Policy, c0: int, k: int) -> bool
    decreases k
{
    if k <= 0 {
        false
    } else {
        let smp = b.subrange(c0 + (k - 1) * p.sample_bytes, c0 + (k - 1) * p.sample_bytes
            + p.sample_bytes);
        let here = p.kind == 0 && pkt_subst(p, sindex(smp, p.sample_bytes), pol);
        here || raw_subst_policy(b, p, pol, c0, k - 1)
    }
}

/// The Strict bridge for the raw phase: `mp_raw_acc_policy(.., Strict)` is
/// `mp_raw_acc` on a record whose every sample is in the 1.0 conversion domain
/// (each sample's index in range for an indexed image). Split as a small
/// non-recursive step lemma (the reveals of the conversion bridges live there)
/// plus the induction, so no single function exceeds the default solver budget.
pub proof fn lemma_mp_raw_acc_policy_strict_step(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    c0: int,
    k: int,
    acc: Seq<Seq<u8>>,
)
    requires
        pol == strict_policy(),
        0 <= c0,
        0 <= p.sample_bytes,
        0 <= p.map_depth,
        p.kind == 0 ==> map.len() == p.map_len * stride_of(p.map_depth),
        0 <= k,
        k < p.n,
        c0 + k * p.sample_bytes + p.sample_bytes <= b.len(),
        p.kind == 0 ==> in_range(
            p,
            sindex(
                b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes),
                p.sample_bytes,
            ),
        ),
    ensures
        conv2_policy(
            p,
            map,
            b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes),
            pol,
        ) == conv2(
            p,
            map,
            b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes),
        ),
{
    let smp = b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes);
    lemma_in_range_policy_strict(p, sindex(smp, p.sample_bytes), pol);
    lemma_conv2_policy_strict(p, map, smp, pol);
}

pub proof fn lemma_mp_raw_acc_policy_strict(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    pol: Policy,
    c0: int,
    k: int,
    acc: Seq<Seq<u8>>,
)
    requires
        pol == strict_policy(),
        0 <= c0,
        0 <= p.sample_bytes,
        0 <= p.map_depth,
        p.kind == 0 ==> map.len() == p.map_len * stride_of(p.map_depth),
        p.kind == 0 ==> forall|j: int|
            0 <= j < p.n ==> #[trigger] in_range(
                p,
                sindex(
                    b.subrange(c0 + j * p.sample_bytes, c0 + j * p.sample_bytes + p.sample_bytes),
                    p.sample_bytes,
                ),
            ),
    ensures
        mp_raw_acc_policy(b, p, map, pol, c0, k, acc) == mp_raw_acc(b, p, map, c0, k, acc),
    decreases (p.n - k) as nat,
{
    reveal(mp_raw_acc_policy);
    reveal(mp_raw_acc);
    reveal(in_range_policy);
    reveal(entry_j);
    reveal(strict_policy);
    if k < 0 || k > p.n || p.sample_bytes <= 0 {
    } else if k == p.n {
    } else if c0 + k * p.sample_bytes + p.sample_bytes > b.len() {
    } else {
        let smp = b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes);
        assert(conv2_policy(p, map, smp, pol) == conv2(p, map, smp)) by {
            assert(0 <= k) by (nonlinear_arith)
                requires
                    k >= 0,
            ;
            lemma_mp_raw_acc_policy_strict_step(b, p, map, pol, c0, k, acc);
        }
        lemma_mp_raw_acc_policy_strict(b, p, map, pol, c0, k + 1, acc + seq![conv2(
            p,
            map,
            smp,
        )]);
    }
}


// ---------------------------------------------------------------------------
// The Strict bridges for the packet phase and the full model.
// ---------------------------------------------------------------------------

/// The successful 1.0 header and the Strict policy have identical packet fields.
pub proof fn lemma_pkt_of_mh_policy_strict(b: Seq<u8>, m: MHModel)
    requires mh(b) == Ok(m),
    ensures pkt_of_mh_policy(m, strict_policy()) == pkt_of_mh(m),
{
    crate::model::lemma_mh_fields(b, m);
    reveal(pkt_of_mh_policy);
    reveal(pkt_of_mh);
    reveal(strict_policy);
    reveal(kind_policy);
    reveal(kind_of_type);
    reveal(is_indexed);
    reveal(eff_attr);
    reveal(attr_table_allowed);
    reveal(core_attr_allowed);
    reveal(map_attr_ok);
    reveal(attr_ok);
    if is_indexed(m.image_type) {
        assert(m.color_map_flag == 1u8) by {
            reveal(cm_bad);
            reveal(mh);
        }
    }
}

/// The R11 selector and R12 suffix test agree on every Strict input.
pub proof fn lemma_strict_scalar_sites(b: Seq<u8>, t: u8, cursor: int)
    ensures
        rle_policy(t, strict_policy()) == (t == 9u8 || t == 10u8 || t == 11u8),
        mp_suffix_ok_policy(b, strict_policy(), cursor) == mp_suffix_ok(b, cursor),
{
    reveal(rle_policy);
    reveal(strict_policy);
    reveal(mp_suffix_ok_policy);
}

/// The two R10 recursions have the same first error, cursor, and pixels, even
/// when a later sample is truncated or indexed outside the map.
pub proof fn lemma_mp_raw_acc_strict_all(
    b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int, acc: Seq<Seq<u8>>,
)
    requires
        0 <= p.sample_bytes,
        0 <= p.map_depth,
        p.kind == 0 ==> map.len() == p.map_len * stride_of(p.map_depth),
    ensures
        mp_raw_acc_policy(b, p, map, strict_policy(), c0, k, acc)
            == mp_raw_acc(b, p, map, c0, k, acc),
    decreases (p.n - k) as nat,
{
    reveal(mp_raw_acc_policy);
    reveal(mp_raw_acc);
    reveal(strict_policy);
    if k < 0 || k > p.n || p.sample_bytes <= 0 {
    } else if k == p.n {
    } else if c0 + k * p.sample_bytes + p.sample_bytes > b.len() {
    } else {
        let smp = b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes);
        lemma_in_range_policy_strict(p, sindex(smp, p.sample_bytes), strict_policy());
        reveal(pkt_reject);
        if p.kind != 0 || in_range(p, sindex(smp, p.sample_bytes)) {
            lemma_conv2_policy_strict(p, map, smp, strict_policy());
            lemma_mp_raw_acc_strict_all(b, p, map, c0, k + 1, acc + seq![conv2(p, map, smp)]);
        }
    }
}

pub proof fn lemma_mp_raw_strict_all(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int)
    requires
        0 <= p.sample_bytes,
        0 <= p.map_depth,
        p.kind == 0 ==> map.len() == p.map_len * stride_of(p.map_depth),
    ensures
        mp_raw_policy(b, p, map, strict_policy(), c0) == mp_raw(b, p, map, c0),
{
    lemma_mp_raw_acc_strict_all(b, p, map, c0, 0, Seq::empty());
    reveal(mp_raw_policy);
    reveal(mp_raw);
}

/// The API view drops only the ghost cursor, produced count and partial pixels
/// carried by an error; success retains the exact cursor and pixel sequence.
pub open spec fn mr_api_result(
    r: Result<(int, int, Seq<Seq<u8>>), (MrErr, int, int, Seq<Seq<u8>>)>,
) -> Result<(int, int, Seq<Seq<u8>>), MrErr> {
    match r { Ok(v) => Ok(v), Err(t) => Err(t.0) }
}

/// At Strict, both packet models agree on their API error or entire success
/// tuple. This covers errors without demanding equality of unused ghost pixels.
pub proof fn lemma_mr_at_strict_api(
    s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int, out: Seq<Seq<u8>>,
)
    requires
        0 <= h.sample_bytes,
        0 <= h.map_depth,
        h.kind == 0 ==> map.len() == h.map_len * stride_of(h.map_depth),
    ensures
        mr_api_result(mr_at_policy(s, h, map, strict_policy(), cur, prod, out))
            == mr_api_result(mr_at(s, h, map, cur, prod, out)),
    decreases (h.n - prod) as nat,
{
    reveal(mr_at_policy);
    reveal(mr_at);
    reveal(mr_api_result);
    reveal(strict_policy);
    reveal(pkt_reject);
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
    } else if prod == h.n {
    } else if cur >= s.len() {
    } else {
        let count = rle_count(s[cur] as int);
        let remaining = h.n - prod;
        let start = cur + 1;
        if count > remaining {
        } else {
            let paint = if count < remaining { count } else { remaining };
            assert(paint == count);
            if rle_repeat(s[cur] as int) {
                let pe = start + h.sample_bytes;
                if pe <= s.len() {
                    let smp = s.subrange(start, pe);
                    lemma_in_range_policy_strict(h, sindex(smp, h.sample_bytes), strict_policy());
                    if h.kind != 0 || in_range(h, sindex(smp, h.sample_bytes)) {
                        lemma_conv2_policy_strict(h, map, smp, strict_policy());
                        lemma_mr_at_strict_api(s, h, map, pe, prod + count,
                            out + pix_pow(conv2(h, map, smp), count));
                    }
                }
            } else {
                let pe = start + count * h.sample_bytes;
                if pe <= s.len() {
                    lemma_lit_ok_policy_strict(s, h, start, count, strict_policy());
                    if lit_ok(s, h, start, count) {
                        assert(h.kind == 0 ==> forall|j: int|
                            0 <= j < count ==> #[trigger] in_range(h, sindex(
                                s.subrange(start + j * h.sample_bytes,
                                    start + j * h.sample_bytes + h.sample_bytes), h.sample_bytes))) by {
                            if h.kind == 0 {
                                assert forall|j: int| 0 <= j < count implies #[trigger] in_range(h,
                                    sindex(s.subrange(start + j * h.sample_bytes,
                                        start + j * h.sample_bytes + h.sample_bytes), h.sample_bytes)) by {
                                    crate::model::lemma_lit_ok_implies_in_range(s, h, start, count, j);
                                }
                            }
                        }
                        lemma_lit_out_policy_strict(s, h, map, strict_policy(), start, count);
                        lemma_mr_at_strict_api(s, h, map, pe, prod + count,
                            out + lit_out(s, h, map, start, count));
                    } else {
                        lemma_lit_good_policy_strict(s, h, start, count, strict_policy());
                    }
                }
            }
        }
    }
}


/// On a successful RLE stream, agreement of the complete MR success tuple
/// forces agreement of the separate pixel sequences used by MP assembly.
pub proof fn lemma_rle_pixels_strict_on_ok(
    b: Seq<u8>, m: MHModel, c0: int, r: (int, int, Seq<Seq<u8>>),
)
    requires
        mh(b) == Ok(m),
        c0 == 18 + m.id_length as int + m.map_bytes,
        c0 <= b.len(),
        mr(b, pkt_of_mh(m), b.subrange(18 + m.id_length as int, c0), c0, 0) == Ok(r),
        mr_policy(b, pkt_of_mh(m), b.subrange(18 + m.id_length as int, c0),
            strict_policy(), c0, 0) == Ok(r),
    ensures
        mr_pix_policy(b, pkt_of_mh(m), b.subrange(18 + m.id_length as int, c0),
            strict_policy(), c0, 0)
            == mr_pix(b, pkt_of_mh(m), b.subrange(18 + m.id_length as int, c0), c0, 0),
{
    let p = pkt_of_mh(m);
    let map = b.subrange(18 + m.id_length as int, c0);
    lemma_strict_phase_domain(b, m);
    crate::model::lemma_mp_conv_domain(b, m);
    reveal(crate::model::mp_conv_domain);
    reveal(crate::model::pkt_of_domain);
    reveal(mr);
    reveal(mr_policy);
    crate::model::lemma_mr_ok_shape(b, p, map, c0, 0, Seq::empty(), r);
    lemma_mr_state_policy_proj(b, p, map, strict_policy(), c0, 0, Seq::empty());
    broadcast use vstd::seq_lib::group_seq_lib_default;
}

/// A checked 1.0 map has exactly the stride required by the conversion lemma.
pub proof fn lemma_strict_phase_domain(b: Seq<u8>, m: MHModel)
    requires
        mh(b) == Ok(m),
        18 + m.id_length as int + m.map_bytes <= b.len(),
    ensures
        1 <= pkt_of_mh(m).sample_bytes,
        0 <= pkt_of_mh(m).map_depth,
        pkt_of_mh(m).kind == 0 ==> b.subrange(18 + m.id_length as int,
            18 + m.id_length as int + m.map_bytes).len()
                == pkt_of_mh(m).map_len * stride_of(pkt_of_mh(m).map_depth),
{
    crate::model::lemma_mp_conv_domain(b, m);
    crate::model::lemma_mp_map_len(b, m);
    reveal(crate::model::mh_selfcons);
    reveal(crate::model::pkt_of_domain);
    reveal(crate::model::mp_conv_domain);
    reveal(pkt_of_mh);
    if is_indexed(m.image_type) {
        crate::model::lemma_stride_eq(m.map_entry_depth);
    }
}

/// Successful raw decoding implies every converted sample is in range, so both
/// model assemblies use exactly the same sequence of converted pixels.
pub proof fn lemma_raw_pixels_strict_on_ok(b: Seq<u8>, m: MHModel, c0: int)
    requires
        mh(b) == Ok(m),
        c0 == 18 + m.id_length as int + m.map_bytes,
        c0 <= b.len(),
        mp_raw(b, pkt_of_mh(m), b.subrange(18 + m.id_length as int, c0), c0) is Ok,
    ensures
        acc_conv_policy(b, pkt_of_mh(m), b.subrange(18 + m.id_length as int, c0),
            strict_policy(), c0, m.n)
            == acc_conv(b, pkt_of_mh(m), b.subrange(18 + m.id_length as int, c0), c0, m.n),
{
    let p = pkt_of_mh(m);
    let map = b.subrange(18 + m.id_length as int, c0);
    lemma_strict_phase_domain(b, m);
    crate::model::lemma_mp_raw_cur_proj(b, p, map, c0, 0, Seq::empty());
    match mp_raw(b, p, map, c0) {
        Ok((_, end)) => {
            crate::model::lemma_raw_prefix_ok_of_cur_ok(b, p, map, c0, m.n, end);
            reveal(crate::model::raw_prefix_ok);
            reveal(crate::model::raw_sample_ok);
            reveal(acc_conv_policy);
            reveal(acc_conv);
            broadcast use vstd::seq::group_seq_lemmas;
            assert(acc_conv_policy(b, p, map, strict_policy(), c0, m.n)
                =~= acc_conv(b, p, map, c0, m.n)) by {
                assert forall|j: int| 0 <= j < m.n implies
                    acc_conv_policy(b, p, map, strict_policy(), c0, m.n)[j]
                        == acc_conv(b, p, map, c0, m.n)[j] by {
                    let smp = b.subrange(c0 + j * p.sample_bytes,
                        c0 + j * p.sample_bytes + p.sample_bytes);
                    assert(crate::model::raw_sample_ok(b, p, map, c0, j));
                    lemma_conv2_policy_strict(p, map, smp, strict_policy());
                }
            }
        },
        Err(_) => { assert(false); },
    }
}

/// The Strict policy follows 1.0's R14 phase order, including every error arm.
pub proof fn lemma_mp_after_policy_strict(b: Seq<u8>, m: MHModel)
    requires mh(b) == Ok(m),
    ensures mp_after_policy(b, m, strict_policy()) == mp_after(b, m),
{
    lemma_pkt_of_mh_policy_strict(b, m);
    lemma_mh_policy_strict(b, strict_policy());
    let id = 18 + m.id_length as int;
    let c0 = id + m.map_bytes;
    lemma_strict_scalar_sites(b, m.image_type, c0);
    reveal(mp_after_policy);
    reveal(mp_after);
    if id > b.len() {
    } else if c0 > b.len() {
    } else {
        let p = pkt_of_mh(m);
        let map = b.subrange(id, c0);
        lemma_strict_phase_domain(b, m);
        if m.image_type == 9u8 || m.image_type == 10u8 || m.image_type == 11u8 {
            lemma_mr_at_strict_api(b, p, map, c0, 0, Seq::empty());
            reveal(mr);
            reveal(mr_policy);
            match mr(b, p, map, c0, 0) {
                Err(_) => {
                    match mr_policy(b, p, map, strict_policy(), c0, 0) {
                        Err(_) => {},
                        Ok(_) => { assert(false); },
                    }
                },
                Ok(r) => {
                    assert(mr_policy(b, p, map, strict_policy(), c0, 0) == Ok(r));
                    lemma_strict_scalar_sites(b, m.image_type, r.0);
                    if mp_suffix_ok(b, r.0) {
                        lemma_rle_pixels_strict_on_ok(b, m, c0, r);
                        reveal(mp_assemble_policy);
                        reveal(mp_assemble);
                        reveal(mp_pixels_policy);
                        reveal(mp_pixels);
                    }
                },
            }
        } else {
            lemma_mp_raw_strict_all(b, p, map, c0);
            match mp_raw(b, p, map, c0) {
                Err(_) => {},
                Ok((pixels, cursor)) => {
                    lemma_strict_scalar_sites(b, m.image_type, cursor);
                    if mp_suffix_ok(b, cursor) {
                        lemma_raw_pixels_strict_on_ok(b, m, c0);
                        reveal(mp_assemble_policy);
                        reveal(mp_assemble);
                        reveal(mp_pixels_policy);
                        reveal(mp_pixels);
                    }
                },
            }
        }
    }
}

/// The complete Strict model is the original 1.0 model on every finite byte
/// sequence: identical first error or identical dimensions and every RGBA byte.
pub proof fn lemma_mp_policy_strict_is_mp(b: Seq<u8>)
    ensures mp_policy(b, strict_policy()) == mp(b),
{
    lemma_mh_policy_strict(b, strict_policy());
    reveal(mp_policy);
    reveal(mp);
    match mh(b) {
        Err(_) => {},
        Ok(m) => { lemma_mp_after_policy_strict(b, m); },
    }
}

/// At Strict, the policy `lit_ok` is 1.0's `lit_ok`.

pub proof fn lemma_lit_ok_policy_strict(s: Seq<u8>, h: PktH, start: int, count: int, pol: Policy)
    requires
        pol == strict_policy(),
    ensures
        lit_ok_policy(s, h, start, count, pol) == lit_ok(s, h, start, count),
    decreases count,
{
    reveal(lit_ok_policy);
    reveal(lit_ok);
    reveal(strict_policy);
    if count <= 0 {
    } else if h.kind != 0 {
        lemma_lit_ok_policy_strict(s, h, start + h.sample_bytes, count - 1, pol);
    } else {
        lemma_in_range_policy_strict(
            h,
            sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes),
            pol,
        );
        lemma_lit_ok_policy_strict(s, h, start + h.sample_bytes, count - 1, pol);
    }
}

/// At Strict, the policy `lit_good` is 1.0's `lit_good`.
pub proof fn lemma_lit_good_policy_strict(s: Seq<u8>, h: PktH, start: int, count: int, pol: Policy)
    requires
        pol == strict_policy(),
    ensures
        lit_good_policy(s, h, start, count, pol) == lit_good(s, h, start, count),
    decreases count,
{
    reveal(lit_good_policy);
    reveal(lit_good);
    reveal(strict_policy);
    if count <= 0 {
    } else if h.kind != 0 {
    } else {
        lemma_in_range_policy_strict(
            h,
            sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes),
            pol,
        );
        lemma_lit_good_policy_strict(s, h, start + h.sample_bytes, count - 1, pol);
    }
}

/// At Strict (on the 1.0 conversion domain), the policy literal output is 1.0's
/// `lit_out`.
pub proof fn lemma_lit_out_policy_strict(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    start: int,
    count: int,
)
    requires
        pol == strict_policy(),
        0 <= h.sample_bytes,
        0 <= h.map_depth,
        h.kind == 0 ==> map.len() == h.map_len * stride_of(h.map_depth),
        h.kind == 0 ==> forall|j: int|
            0 <= j < count ==> #[trigger] in_range(
                h,
                sindex(
                    s.subrange(start + j * h.sample_bytes, start + j * h.sample_bytes
                        + h.sample_bytes),
                    h.sample_bytes,
                ),
            ),
    ensures
        lit_out_policy(s, h, map, pol, start, count) == lit_out(s, h, map, start, count),
    decreases count,
{
    reveal(lit_out_policy);
    reveal(lit_out);
    reveal(strict_policy);
    reveal(in_range);
    if count <= 0 {
    } else {
        lemma_lit_out_head(s, h, map, pol, start, count);
        lemma_lit_out_tail_shift(s, h, pol, start, count);
        lemma_lit_out_policy_strict(s, h, map, pol, start + h.sample_bytes, count - 1);
    }
}

/// The head sample's conversion is the 1.0 conversion.
proof fn lemma_lit_out_head(s: Seq<u8>, h: PktH, map: Seq<u8>, pol: Policy, start: int, count: int)
    requires
        pol == strict_policy(),
        0 <= h.sample_bytes,
        0 <= h.map_depth,
        h.kind == 0 ==> map.len() == h.map_len * stride_of(h.map_depth),
        0 < count,
        h.kind == 0 ==> forall|j: int|
            0 <= j < count ==> #[trigger] in_range(
                h,
                sindex(
                    s.subrange(start + j * h.sample_bytes, start + j * h.sample_bytes
                        + h.sample_bytes),
                    h.sample_bytes,
                ),
            ),
    ensures
        conv2_policy(h, map, s.subrange(start, start + h.sample_bytes), pol) == conv2(
            h,
            map,
            s.subrange(start, start + h.sample_bytes),
        ),
{
    let smp = s.subrange(start, start + h.sample_bytes);
    if h.kind == 0 {
        assert(in_range(h, sindex(smp, h.sample_bytes))) by {
            assert(0 <= 0 < count);
            assert(in_range(
                h,
                sindex(s.subrange(start + 0 * h.sample_bytes, start + 0 * h.sample_bytes
                    + h.sample_bytes), h.sample_bytes),
            ));
            assert(s.subrange(start + 0 * h.sample_bytes, start + 0 * h.sample_bytes
                + h.sample_bytes) == smp) by {
                assert(start + 0 * h.sample_bytes == start) by (nonlinear_arith);
            }
        }
        lemma_in_range_policy_strict(h, sindex(smp, h.sample_bytes), pol);
    }
    lemma_conv2_policy_strict(h, map, smp, pol);
}

/// The tail's `in_range` quantifier, shifted by one sample, is the outer one.
proof fn lemma_lit_out_tail_shift(s: Seq<u8>, h: PktH, pol: Policy, start: int, count: int)
    requires
        0 <= h.sample_bytes,
        0 < count,
        h.kind == 0 ==> forall|j: int|
            0 <= j < count ==> #[trigger] in_range(
                h,
                sindex(
                    s.subrange(start + j * h.sample_bytes, start + j * h.sample_bytes
                        + h.sample_bytes),
                    h.sample_bytes,
                ),
            ),
    ensures
        h.kind == 0 ==> forall|j: int|
            0 <= j < count - 1 ==> #[trigger] in_range(
                h,
                sindex(
                    s.subrange(start + h.sample_bytes + j * h.sample_bytes, start + h.sample_bytes
                        + j * h.sample_bytes + h.sample_bytes),
                    h.sample_bytes,
                ),
            ),
{
    reveal(in_range);
    if h.kind == 0 {
        assert forall|j: int| 0 <= j < count - 1 implies #[trigger] in_range(
            h,
            sindex(
                s.subrange(start + h.sample_bytes + j * h.sample_bytes, start + h.sample_bytes + j
                    * h.sample_bytes + h.sample_bytes),
                h.sample_bytes,
            ),
        ) by {
            assert(0 <= j + 1 < count);
            assert(s.subrange(start + (j + 1) * h.sample_bytes, start + (j + 1) * h.sample_bytes
                + h.sample_bytes) == s.subrange(start + h.sample_bytes + j * h.sample_bytes, start
                + h.sample_bytes + j * h.sample_bytes + h.sample_bytes)) by (nonlinear_arith);
        };
    }
}


// ---------------------------------------------------------------------------
// Obligation E: the `recovered` flag is exact for every retained record.
//
// `recovered_policy` is *defined* as the disjunction of the closed two-event
// bits (RLE clip, `Black` substitution), so the "iff" of obligation E holds by
// construction. The content is the two corollaries the delta names.
// ---------------------------------------------------------------------------

/// E corollary 1: no clip bit is reachable under `rle_overrun = Reject`.
/// `mr_flags_policy`'s clip term is `pkt_clip`, false when the mode is Reject.
pub proof fn lemma_pkt_clip_reject(count: int, remaining: int, pol: Policy)
    requires
        pol.rle_overrun == RleOverrunOptions::Reject,
    ensures
        !pkt_clip(count, remaining, pol),
{
    reveal(pkt_clip);
}

/// E corollary 2: no substitution bit is reachable under
/// `index_range = Reject`.
pub proof fn lemma_pkt_subst_reject(h: PktH, index: int, pol: Policy)
    requires
        pol.index_range == IndexRangeOptions::Reject,
    ensures
        !pkt_subst(h, index, pol),
{
    reveal(pkt_subst);
}

/// E corollary (the named one): a record with `rle_overrun = Reject` and
/// `index_range = Reject` has `recovered = false` on every input. Strict is such
/// a record, so `decode` is unaffected.
pub proof fn lemma_recovered_policy_reject(b: Seq<u8>, pol: Policy)
    requires
        pol.rle_overrun == RleOverrunOptions::Reject,
        pol.index_range == IndexRangeOptions::Reject,
    ensures
        recovered_policy(b, pol) == false,
{
    reveal(recovered_policy);
    match mh_policy(b, pol) {
        Err(_) => {},
        Ok(m) => {
            let id_end = 18 + (m.id_length as int);
            if id_end > b.len() || id_end + m.map_bytes > b.len() {
            } else {
                let p = pkt_of_mh_policy(m, pol);
                if rle_policy(m.image_type, pol) {
                    lemma_mr_flags_no_events(b, p, b.subrange(id_end, id_end + m.map_bytes), pol, id_end
                        + m.map_bytes, 0);
                } else {
                    lemma_raw_subst_no_events(b, p, pol, id_end + m.map_bytes, m.n);
                }
            }
        },
    }
}


/// Under `Reject` on both fields, `mr_flags_policy` reports no event at every
/// reachable call.
pub proof fn lemma_mr_flags_no_events(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    pol: Policy,
    cur: int,
    prod: int,
)
    requires
        pol.rle_overrun == RleOverrunOptions::Reject,
        pol.index_range == IndexRangeOptions::Reject,
    ensures
        mr_flags_policy(s, h, map, pol, cur, prod).0 == false,
        mr_flags_policy(s, h, map, pol, cur, prod).1 == false,
    decreases (h.n - prod) as nat,
{
    reveal(mr_flags_policy);
    reveal(pkt_clip);
    reveal(pkt_subst);
    reveal(pkt_reject);
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
    } else if prod == h.n {
    } else if cur >= s.len() {
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        let remaining = h.n - prod;
        let start = cur + 1;
        if pol.rle_overrun == RleOverrunOptions::Reject && count > remaining {
        } else {
            let paint = if count < remaining { count } else { remaining };
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                } else {
                    let smp = s.subrange(start, pe);
                    let idx = sindex(smp, h.sample_bytes);
                    let rest = mr_flags_policy(s, h, map, pol, pe, prod + paint);
                    lemma_mr_flags_no_events(s, h, map, pol, pe, prod + paint);
                }
            } else {
                let need = if pol.rle_overrun == RleOverrunOptions::ClipSamples {
                    paint * h.sample_bytes
                } else {
                    count * h.sample_bytes
                };
                let pe = start + need;
                if s.len() < pe {
                } else {
                    let (sub, rest) = lit_flags_policy(s, h, pol, start, paint);
                    lemma_lit_flags_no_events(s, h, pol, start, paint);
                    lemma_mr_flags_no_events(s, h, map, pol, pe, prod + paint);
                }
            }
        }
    }
}

/// The literal-packet flag scan reports no event under `Reject`.
pub proof fn lemma_lit_flags_no_events(s: Seq<u8>, h: PktH, pol: Policy, start: int, count: int)
    requires
        pol.index_range == IndexRangeOptions::Reject,
    ensures
        lit_flags_policy(s, h, pol, start, count).0 == false,
        lit_flags_policy(s, h, pol, start, count).1 == false,
    decreases count,
{
    reveal(lit_flags_policy);
    reveal(pkt_subst);
    if count <= 0 {
    } else {
        lemma_lit_flags_no_events(s, h, pol, start + h.sample_bytes, count - 1);
    }
}

/// The raw substitution scan reports no event under `Reject`.
pub proof fn lemma_raw_subst_no_events(b: Seq<u8>, p: PktH, pol: Policy, c0: int, k: int)
    requires
        pol.index_range == IndexRangeOptions::Reject,
    ensures
        raw_subst_policy(b, p, pol, c0, k) == false,
    decreases k,
{
    reveal(raw_subst_policy);
    reveal(pkt_subst);
    if k <= 0 {
    } else {
        lemma_raw_subst_no_events(b, p, pol, c0, k - 1);
    }
}

/// E for Strict: `recovered_policy(b, strict_policy()) = false` on every input.
pub proof fn lemma_recovered_policy_strict(b: Seq<u8>)
    ensures
        recovered_policy(b, strict_policy()) == false,
{
    reveal(strict_policy);
    lemma_recovered_policy_reject(b, strict_policy());
}


// ---------------------------------------------------------------------------
// Obligation R: `RecommendedBeta` conservativity for Strict-accepted inputs.
//
// RecommendedBeta differs from Strict in exactly four fields: `trailing`,
// `rle_overrun`, `attribute_bits`, `map_header`. On a Strict-accepted input each
// is inert, so the header record, the conversion, and (with the suffix lemma)
// the whole result are unchanged with `recovered = false`.
// ---------------------------------------------------------------------------

/// Site lemma 4 (`trailing`): on an input whose suffix is empty or a footer,
/// `Ignore` gives the same suffix verdict as `FooterOnly`.
pub proof fn lemma_suffix_ignore_on_footer(b: Seq<u8>, cursor: int)
    requires
        mp_suffix_ok(b, cursor),
    ensures
        mp_suffix_ok_policy(b, recommended_beta_policy(), cursor),
{
    reveal(mp_suffix_ok_policy);
    reveal(recommended_beta_policy);
    reveal(mp_suffix_ok);
}

/// Site lemma 1 (`attribute_bits`): `Lenient`'s acceptance test is vacuous and
/// its effective count is Strict's count when Strict's table accepted it.
pub proof fn lemma_eff_attr_lenient_accepted(kind: int, d: int, count: int)
    requires
        core_attr_allowed(kind, d, count),
    ensures
        attr_ok_policy(kind, d, count, AttributeBitsOptions::Lenient),
        eff_attr(kind, d, count, AttributeBitsOptions::Lenient) == count,
{
    reveal(eff_attr);
    reveal(attr_ok_policy);
    reveal(attr_table_allowed);
    reveal(core_attr_allowed);
}

/// Site lemma 2 (`map_header`): Strict-acceptance implies `IgnoreAbsent`
/// acceptance, and the map's presence and slice are unchanged.
pub proof fn lemma_map_recommended_beta_on_strict(
    b: Seq<u8>,
    present: bool,
    kind: bool,
)
    requires
        18 <= b.len(),
        b[1] <= 1u8,
        present == (b[1] == 1u8),
        kind == (kind_of_type(b[2]) == 0),
        !map_coherence_bad(b, present, kind_of_type(b[2]), MapHeaderOptions::Core),
        !map_range_bad(b, present, MapHeaderOptions::Core),
    ensures
        !map_coherence_bad(b, present, kind_of_type(b[2]), MapHeaderOptions::IgnoreAbsent),
        !map_range_bad(b, present, MapHeaderOptions::IgnoreAbsent),
{
    reveal(map_coherence_bad);
    reveal(map_range_bad);
    reveal(map_entry_depth_ok);
    if b[1] == 0u8 {
        // Core's flag-0 check is the conjunction that covers IgnoreAbsent's.
        assert(!(le16(b, 3) != 0 || le16(b, 5) != 0 || b[7] != 0u8 || kind_of_type(b[2]) == 0));
        assert(!(kind_of_type(b[2]) == 0));
    } else {
        assert(b[1] == 1u8);
        assert(!(le16(b, 5) == 0 || !map_entry_depth_ok(b[7])));
    }
}

/// A `map_coherence_bad` under `Core` and under `IgnoreAbsent` differ only in
/// the flag-0 test; with a passing `Core` verdict, both are false or both agree.
pub proof fn lemma_map_coherence_recommended_beta(b: Seq<u8>, present: bool)
    requires
        18 <= b.len(),
        present == (b[1] == 1u8),
    ensures
        map_coherence_bad(b, present, kind_of_type(b[2]), MapHeaderOptions::IgnoreAbsent)
            == (kind_of_type(b[2]) == 0 && b[1] == 0u8)
            || map_coherence_bad(b, present, kind_of_type(b[2]), MapHeaderOptions::IgnoreAbsent)
            == map_coherence_bad(b, present, kind_of_type(b[2]), MapHeaderOptions::Core),
{
    reveal(map_coherence_bad);
    reveal(map_entry_depth_ok);
}

/// The header record equality: on a Strict-accepted input,
/// `mh_policy(b, RecommendedBeta)` is the same `Ok` record as `mh_policy(b, Strict)`.
pub proof fn lemma_mh_policy_recommended_beta_on_strict_ok(b: Seq<u8>, m: MHModel)
    requires
        mh_policy(b, strict_policy()) == Ok(m),
    ensures
        mh_policy(b, recommended_beta_policy()) == Ok(m),
{
    reveal(mh_policy);
    reveal(strict_policy);
    reveal(recommended_beta_policy);
    reveal(type_ok_policy);
    reveal(kind_policy);
    reveal(kind_of_type);
    reveal(is_indexed);
    reveal(map_flag_ok_policy);
    reveal(map_present_policy);
    reveal(depth_ok_policy);
    reveal(map_coherence_bad);
    reveal(map_range_bad);
    reveal(map_entry_depth_ok);
    reveal(image_index_depth_bad);
    reveal(attr_ok_policy);
    reveal(attr_table_allowed);
    reveal(core_attr_allowed);
    reveal(eff_attr);
    reveal(attr_depth);
    reveal(map_bytes_policy);
    assert(b.len() >= 18);
    assert(type_ok(b[2]));
    assert(b[1] <= 1u8);
    assert(1 <= le16(b, 12) && le16(b, 12) <= 8192);
    assert(1 <= le16(b, 14) && le16(b, 14) <= 8192);
    // RecommendedBeta's relaxed fields.
    assert(recommended_beta_policy().depths == DepthsOptions::Core);
    assert(recommended_beta_policy().interleave == InterleaveOptions::Reject);
    assert(recommended_beta_policy().image_type == ImageTypeOptions::Core);
    assert(recommended_beta_policy().map_header == MapHeaderOptions::IgnoreAbsent);
    assert(recommended_beta_policy().attribute_bits == AttributeBitsOptions::Lenient);
    // Shared guards hold for RecommendedBeta because they are Strict's own.
    assert(type_ok_policy(b[2], recommended_beta_policy()));
    assert(kind_policy(b[2], recommended_beta_policy()) == kind_of_type(b[2]));
    assert(map_flag_ok_policy(b, recommended_beta_policy()));
    assert(map_present_policy(b, recommended_beta_policy()) == (b[1] == 1u8));
    assert(depth_ok_policy(
        kind_policy(b[2], recommended_beta_policy()),
        b[16],
        recommended_beta_policy().depths,
    )) by {
        assert(depth_ok_policy(kind_of_type(b[2]), b[16], DepthsOptions::Core));
        assert(depth_ok_policy(kind_of_type(b[2]), b[16], DepthsOptions::Core) == depth_ok(b[2], b[16]));
    }
    assert(!map_coherence_bad(
        b,
        map_present_policy(b, recommended_beta_policy()),
        kind_policy(b[2], recommended_beta_policy()),
        recommended_beta_policy().map_header,
    )) by {
        lemma_map_recommended_beta_on_strict(
            b,
            b[1] == 1u8,
            kind_of_type(b[2]) == 0,
        );
    }
    assert(!map_range_bad(
        b,
        map_present_policy(b, recommended_beta_policy()),
        recommended_beta_policy().map_header,
    )) by {
        reveal(map_range_bad);
        if b[1] == 1u8 {
            assert(!(le16(b, 3) + le16(b, 5) > 65536));
        }
    }
    assert(!image_index_depth_bad(
        b,
        map_present_policy(b, recommended_beta_policy()),
        kind_policy(b[2], recommended_beta_policy()),
        recommended_beta_policy().depths,
    )) by {
        reveal(image_index_depth_bad);
    }
    assert(attr_ok_policy(
        kind_policy(b[2], recommended_beta_policy()),
        attr_depth(b, recommended_beta_policy()),
        (b[17] & 0x0fu8) as int,
        recommended_beta_policy().attribute_bits,
    )) by {
        lemma_eff_attr_lenient_accepted(
            kind_of_type(b[2]),
            (if kind_of_type(b[2]) == 0 {
                b[7] as int
            } else {
                b[16] as int
            }),
            (b[17] & 0x0fu8) as int,
        );
    }
    assert(map_bytes_policy(b, recommended_beta_policy()) == map_bytes_policy(b, strict_policy())) by {
        reveal(map_bytes_policy);
        reveal(map_present_policy);
    }
}

/// The conversion equality: on a strict-accepted record, the RecommendedBeta
/// conversion is the Strict conversion (`conv2_policy(.., Strict) = conv2`).
pub proof fn lemma_conv2_policy_recommended_beta_on_strict(
    h: PktH,
    map: Seq<u8>,
    s: Seq<u8>,
)
    requires
        0 <= h.sample_bytes,
        0 <= h.map_depth,
        h.kind == 0 ==> h.map_first <= sindex(s, h.sample_bytes) < h.map_first + h.map_len,
        h.kind == 0 ==> map.len() == h.map_len * stride_of(h.map_depth),
    ensures
        conv2_policy(h, map, s, recommended_beta_policy()) == conv2_policy(h, map, s, strict_policy()),
{
    reveal(conv2_policy);
    reveal(conv_sample_policy);
    reveal(conv_map_policy);
    reveal(recommended_beta_policy);
    reveal(strict_policy);
    reveal(stored_alpha_policy);
    reveal(map555_policy);
    reveal(rgb555_policy);
    reveal(map_entry_alpha_policy);
    reveal(entry_j);
    reveal(in_range_policy);
    reveal(in_range);
    reveal(bit15);
    reveal(stride_of);
    if h.kind == 0 {
        assert(entry_j(sindex(s, h.sample_bytes), h.map_first, recommended_beta_policy()) == sindex(
            s,
            h.sample_bytes,
        ) - h.map_first);
        assert(0 <= sindex(s, h.sample_bytes) - h.map_first < h.map_len);
        assert(0 <= stride_of(h.map_depth)) by (nonlinear_arith)
            requires
                stride_of(h.map_depth) == (h.map_depth + 7) / 8,
                h.map_depth >= 0,
        ;
    }
}


// ---------------------------------------------------------------------------
// Obligation R, packet phase: on a Strict-accepted input the RecommendedBeta
// packet stream, pixels and flags equal Strict's.
// ---------------------------------------------------------------------------

/// The conversion of any sample is the same under Strict and RecommendedBeta: the
/// two records agree on every field the conversion reads (`rgb5`, `map5`,
/// `alpha`, `map16_alpha`, `map_index`, `index_range`).
pub proof fn lemma_conv2_policy_rec_eq_strict(h: PktH, map: Seq<u8>, s: Seq<u8>)
    ensures
        conv2_policy(h, map, s, recommended_beta_policy()) == conv2_policy(h, map, s, strict_policy()),
{
    reveal(conv2_policy);
    reveal(conv_sample_policy);
    reveal(conv_map_policy);
    reveal(recommended_beta_policy);
    reveal(strict_policy);
    reveal(stored_alpha_policy);
    reveal(map555_policy);
    reveal(rgb555_policy);
    reveal(map_entry_alpha_policy);
    reveal(entry_j);
    reveal(in_range_policy);
}

/// Strict and RecommendedBeta agree on the index-range test (`map_index`,
/// `index_range` equal).
pub proof fn lemma_in_range_rec_eq_strict(h: PktH, index: int)
    ensures
        in_range_policy(h, index, recommended_beta_policy()) == in_range_policy(h, index, strict_policy()),
{
    reveal(in_range_policy);
    reveal(entry_j);
    reveal(recommended_beta_policy);
    reveal(strict_policy);
}

/// The raw substitution scan is the same under both records (`index_range`
/// equal), so a raw image's `recovered` bit is the same.
pub proof fn lemma_raw_subst_rec_eq_strict(b: Seq<u8>, p: PktH, c0: int, k: int)
    ensures
        raw_subst_policy(b, p, recommended_beta_policy(), c0, k) == raw_subst_policy(
            b,
            p,
            strict_policy(),
            c0,
            k,
        ),
    decreases k,
{
    reveal(raw_subst_policy);
    reveal(pkt_subst);
    reveal(recommended_beta_policy);
    reveal(strict_policy);
    if k <= 0 {
    } else {
        lemma_raw_subst_rec_eq_strict(b, p, c0, k - 1);
    }
}

/// The Strict packet stream succeeded: a boolean projection used as the
/// induction hypothesis of the conservativity lemma. `true` iff Strict's `MR`
/// returns `Ok`.
pub open spec fn mr_strict_ok(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
) -> bool {
    match mr_at_policy(s, h, map, strict_policy(), cur, prod, Seq::empty()) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// The raw phase of the two records is identical (no success hypothesis
/// needed: the guards and conversions agree for every input).
pub proof fn lemma_mp_raw_acc_rec_eq_strict(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
    k: int,
    acc: Seq<Seq<u8>>,
)
    ensures
        mp_raw_acc_policy(b, p, map, recommended_beta_policy(), c0, k, acc) == mp_raw_acc_policy(
            b,
            p,
            map,
            strict_policy(),
            c0,
            k,
            acc,
        ),
    decreases (p.n - k) as nat,
{
    reveal(mp_raw_acc_policy);
    reveal(pkt_reject);
    reveal(recommended_beta_policy);
    reveal(strict_policy);
    if k < 0 || k > p.n || p.sample_bytes <= 0 {
    } else if k == p.n {
    } else if c0 + k * p.sample_bytes + p.sample_bytes > b.len() {
    } else {
        let smp = b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes);
        lemma_conv2_policy_rec_eq_strict(p, map, smp);
        lemma_mp_raw_acc_rec_eq_strict(b, p, map, c0, k + 1, acc + seq![conv2_policy(
            p,
            map,
            smp,
            recommended_beta_policy(),
        )]);
    }
}


/// `lit_out_policy` is the same under the two records (the conversions agree).
pub proof fn lemma_lit_out_rec_eq_strict(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    start: int,
    count: int,
)
    ensures
        lit_out_policy(s, h, map, recommended_beta_policy(), start, count) == lit_out_policy(
            s,
            h,
            map,
            strict_policy(),
            start,
            count,
        ),
    decreases count,
{
    reveal(lit_out_policy);
    if count <= 0 {
    } else {
        lemma_conv2_policy_rec_eq_strict(h, map, s.subrange(start, start + h.sample_bytes));
        lemma_lit_out_rec_eq_strict(s, h, map, start + h.sample_bytes, count - 1);
    }
}

/// `lit_ok_policy` is the same under the two records.
pub proof fn lemma_lit_ok_rec_eq_strict(s: Seq<u8>, h: PktH, start: int, count: int)
    ensures
        lit_ok_policy(s, h, start, count, recommended_beta_policy()) == lit_ok_policy(
            s,
            h,
            start,
            count,
            strict_policy(),
        ),
    decreases count,
{
    reveal(lit_ok_policy);
    reveal(recommended_beta_policy);
    reveal(strict_policy);
    if count <= 0 {
    } else {
        lemma_in_range_rec_eq_strict(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes));
        lemma_lit_ok_rec_eq_strict(s, h, start + h.sample_bytes, count - 1);
    }
}

/// `lit_good_policy` is the same under the two records.
pub proof fn lemma_lit_good_rec_eq_strict(s: Seq<u8>, h: PktH, start: int, count: int)
    ensures
        lit_good_policy(s, h, start, count, recommended_beta_policy()) == lit_good_policy(
            s,
            h,
            start,
            count,
            strict_policy(),
        ),
    decreases count,
{
    reveal(lit_good_policy);
    reveal(recommended_beta_policy);
    reveal(strict_policy);
    if count <= 0 {
    } else {
        lemma_in_range_rec_eq_strict(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes));
        lemma_lit_good_rec_eq_strict(s, h, start + h.sample_bytes, count - 1);
    }
}

/// A successful Strict `MR` at `(cur, prod)` did not take the overrun branch,
/// so the packet's count fits the remaining pixels.
proof fn lemma_strict_mr_ok_no_overrun(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    out: Seq<Seq<u8>>,
    r: (int, int, Seq<Seq<u8>>),
)
    requires
        mr_at_policy(s, h, map, strict_policy(), cur, prod, out) == Ok(r),
        0 <= cur < s.len(),
        0 <= prod < h.n,
    ensures
        rle_count(s[cur] as int) <= h.n - prod,
{
    reveal(mr_at_policy);
    reveal(strict_policy);
    if rle_count(s[cur] as int) > h.n - prod {
        assert(mr_at_policy(s, h, map, strict_policy(), cur, prod, out) == Err((
            MrErr::Api(Error::RleOverrun),
            cur + 1,
            prod,
            out,
        )));
    }
}

/// Obligation R, packet phase: on an input whose Strict packet stream succeeds
/// at `(cur, prod)`, the RecommendedBeta stream returns the identical `Ok`.
pub proof fn lemma_mr_at_rec_eq_strict_on_ok(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    out: Seq<Seq<u8>>,
    r: (int, int, Seq<Seq<u8>>),
)
    requires
        mr_at_policy(s, h, map, strict_policy(), cur, prod, out) == Ok(r),
    ensures
        mr_at_policy(s, h, map, recommended_beta_policy(), cur, prod, out) == Ok(r),
    decreases (h.n - prod) as nat,
{
    reveal(mr_at_policy);
    reveal(recommended_beta_policy);
    reveal(strict_policy);
    reveal(pkt_clip);
    reveal(pkt_reject);
    reveal(rle_count);
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
    } else if prod == h.n {
    } else if cur >= s.len() {
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        let remaining = h.n - prod;
        let start = cur + 1;
        lemma_strict_mr_ok_no_overrun(s, h, map, cur, prod, out, r);
        let paint = if count < remaining { count } else { remaining };
        assert(paint == count) by {
            if count < remaining {
            } else {
                assert(paint == remaining);
                assert(count == remaining) by (nonlinear_arith)
                    requires
                        count <= remaining,
                        !(count < remaining),
                ;
            }
        }
        if rle_repeat(control) {
            let pe = start + h.sample_bytes;
            if s.len() < pe {
                assert(false);
            } else {
                let smp = s.subrange(start, pe);
                lemma_conv2_policy_rec_eq_strict(h, map, smp);
                assert(out + pix_pow(conv2_policy(h, map, smp, recommended_beta_policy()), paint)
                    == out + pix_pow(conv2_policy(h, map, smp, strict_policy()), paint));
                lemma_mr_at_rec_eq_strict_on_ok(
                    s,
                    h,
                    map,
                    pe,
                    prod + paint,
                    out + pix_pow(conv2_policy(h, map, smp, recommended_beta_policy()), paint),
                    r,
                );
            }
        } else {
            let need = count * h.sample_bytes;
            let pe = start + need;
            if s.len() < pe {
                assert(false);
            } else {
                lemma_lit_ok_rec_eq_strict(s, h, start, paint);
                lemma_lit_out_rec_eq_strict(s, h, map, start, paint);
                if !lit_ok_policy(s, h, start, paint, strict_policy()) {
                    assert(false);
                } else {
                    assert(out + lit_out_policy(s, h, map, recommended_beta_policy(), start, paint)
                        == out + lit_out_policy(s, h, map, strict_policy(), start, paint));
                    lemma_mr_at_rec_eq_strict_on_ok(
                        s,
                        h,
                        map,
                        pe,
                        prod + paint,
                        out + lit_out_policy(s, h, map, recommended_beta_policy(), start, paint),
                        r,
                    );
                }
            }
        }
    }
}



// ---------------------------------------------------------------------------
// Obligation R, end to end: `forall b: MP(b,Strict)=Ok(w,h,p) ==>
// MP(b,RecommendedBeta)=Ok(w,h,p) with recovered=false`.
//
// The four site lemmas (suffix, `eff_attr`, `map_coherence_bad`, map present)
// and the record equalities are above. The packet stream needs two facts on a
// Strict-successful stream: the pixel sequence is unchanged and no recovery event
// fires. Each is a per-packet induction: the packet's branch and its fit are
// fixed by the hypothesis (`lemma_strict_mr_ok_no_overrun`), and the two records
// agree on the conversion, the index test and the `Lenient` effective count.
// ---------------------------------------------------------------------------

/// The conversion-relevant projection of the same accepted header record is the
/// same under Strict and RecommendedBeta. `eff_attr` is the identical function for
/// `Exact` and `Lenient` (`attr_table_allowed` maps both to `core_attr_allowed`),
/// and every other field the projection reads is equal.
pub proof fn lemma_pkt_of_mh_policy_rec_eq_strict(m: MHModel)
    ensures
        pkt_of_mh_policy(m, recommended_beta_policy()) == pkt_of_mh_policy(m, strict_policy()),
{
    reveal(pkt_of_mh_policy);
    reveal(kind_policy);
    reveal(type_ok_policy);
    reveal(eff_attr);
    reveal(attr_table_allowed);
    reveal(core_attr_allowed);
    reveal(strict_policy);
    reveal(recommended_beta_policy);
}

/// Obligation R, packet phase, pixels: on an input whose Strict packet stream
/// succeeds at `(cur, prod)` with accumulator `out`, the RecommendedBeta pixel
/// sequence from the same state equals Strict's. Each step's branch and painted
/// count are pinned by the exact Strict `Ok`, so one unfolding per packet
/// suffices and no single body sees the whole stream.
pub proof fn lemma_mr_pix_rec_eq_strict_on_ok(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    out: Seq<Seq<u8>>,
    r: (int, int, Seq<Seq<u8>>),
)
    requires
        mr_at_policy(s, h, map, strict_policy(), cur, prod, out) == Ok(r),
    ensures
        mr_pix_policy(s, h, map, recommended_beta_policy(), cur, prod) == mr_pix_policy(
            s,
            h,
            map,
            strict_policy(),
            cur,
            prod,
        ),
    decreases (h.n - prod) as nat,
{
    reveal(mr_at_policy);
    reveal(mr_pix_policy);
    reveal(recommended_beta_policy);
    reveal(strict_policy);
    reveal(pkt_reject);
    reveal(rle_count);
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
    } else if prod == h.n {
    } else if cur >= s.len() {
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        let remaining = h.n - prod;
        let start = cur + 1;
        lemma_strict_mr_ok_no_overrun(s, h, map, cur, prod, out, r);
        let paint = if count < remaining {
            count
        } else {
            remaining
        };
        assert(paint == count) by {
            if count < remaining {
            } else {
                assert(paint == remaining);
                assert(count == remaining) by (nonlinear_arith)
                    requires
                        count <= remaining,
                        !(count < remaining),
                ;
            }
        }
        if rle_repeat(control) {
            let pe = start + h.sample_bytes;
            if s.len() < pe {
            } else {
                let smp = s.subrange(start, pe);
                lemma_conv2_policy_rec_eq_strict(h, map, smp);
                lemma_mr_pix_rec_eq_strict_on_ok(
                    s,
                    h,
                    map,
                    pe,
                    prod + paint,
                    out + pix_pow(conv2_policy(h, map, smp, strict_policy()), paint),
                    r,
                );
            }
        } else {
            let need = count * h.sample_bytes;
            let pe = start + need;
            if s.len() < pe {
            } else {
                lemma_lit_ok_rec_eq_strict(s, h, start, paint);
                lemma_lit_out_rec_eq_strict(s, h, map, start, paint);
                if !lit_ok_policy(s, h, start, paint, strict_policy()) {
                    assert(false);
                } else {
                    lemma_mr_pix_rec_eq_strict_on_ok(
                        s,
                        h,
                        map,
                        pe,
                        prod + paint,
                        out + lit_out_policy(s, h, map, strict_policy(), start, paint),
                        r,
                    );
                }
            }
        }
    }
}

/// Obligation R, packet phase, `recovered`: on an input whose Strict packet
/// stream succeeds at `(cur, prod)`, the RecommendedBeta flag scan reports no event.
/// The packet fits (`lemma_strict_mr_ok_no_overrun`), so RecommendedBeta's
/// `ClipPacket` never fires; `index_range = Reject` is RecommendedBeta's value, so
/// every substitution bit is false and the literal scan is `lemma_lit_flags_no_events`.
pub proof fn lemma_mr_flags_rec_false_on_strict_ok(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    out: Seq<Seq<u8>>,
    r: (int, int, Seq<Seq<u8>>),
)
    requires
        mr_at_policy(s, h, map, strict_policy(), cur, prod, out) == Ok(r),
    ensures
        mr_flags_policy(s, h, map, recommended_beta_policy(), cur, prod) == (false, false),
    decreases (h.n - prod) as nat,
{
    reveal(mr_at_policy);
    reveal(mr_flags_policy);
    reveal(recommended_beta_policy);
    reveal(strict_policy);
    reveal(pkt_clip);
    reveal(pkt_subst);
    reveal(pkt_reject);
    reveal(rle_count);
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
    } else if prod == h.n {
    } else if cur >= s.len() {
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        let remaining = h.n - prod;
        let start = cur + 1;
        lemma_strict_mr_ok_no_overrun(s, h, map, cur, prod, out, r);
        let paint = if count < remaining {
            count
        } else {
            remaining
        };
        assert(paint == count) by {
            if count < remaining {
            } else {
                assert(paint == remaining);
                assert(count == remaining) by (nonlinear_arith)
                    requires
                        count <= remaining,
                        !(count < remaining),
                ;
            }
        }
        assert(pkt_clip(count, remaining, recommended_beta_policy()) == false) by {
            reveal(pkt_clip);
        }
        if rle_repeat(control) {
            let pe = start + h.sample_bytes;
            if s.len() < pe {
            } else {
                let smp = s.subrange(start, pe);
                assert(pkt_reject(h, sindex(smp, h.sample_bytes), recommended_beta_policy())
                    == pkt_reject(h, sindex(smp, h.sample_bytes), strict_policy())) by {
                    reveal(pkt_reject);
                    lemma_in_range_rec_eq_strict(h, sindex(smp, h.sample_bytes));
                }
                if h.kind == 0 && pkt_reject(h, sindex(smp, h.sample_bytes), recommended_beta_policy()) {
                    assert(false);
                } else {
                    assert(!pkt_subst(h, sindex(smp, h.sample_bytes), recommended_beta_policy())) by {
                        reveal(pkt_subst);
                    }
                    lemma_mr_flags_rec_false_on_strict_ok(
                        s,
                        h,
                        map,
                        pe,
                        prod + paint,
                        out + pix_pow(conv2_policy(h, map, smp, strict_policy()), paint),
                        r,
                    );
                }
            }
        } else {
            let need = count * h.sample_bytes;
            let pe = start + need;
            if s.len() < pe {
            } else {
                lemma_lit_flags_no_events(s, h, recommended_beta_policy(), start, paint);
                lemma_mr_flags_rec_false_on_strict_ok(
                    s,
                    h,
                    map,
                    pe,
                    prod + paint,
                    out + lit_out_policy(s, h, map, strict_policy(), start, paint),
                    r,
                );
            }
        }
    }
}

/// `acc_conv_policy` is the same under the two records (the conversions agree on
/// every sample, unconditionally).
pub proof fn lemma_acc_conv_policy_rec_eq_strict(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
    k: int,
)
    ensures
        acc_conv_policy(b, p, map, recommended_beta_policy(), c0, k) == acc_conv_policy(
            b,
            p,
            map,
            strict_policy(),
            c0,
            k,
        ),
{
    reveal(acc_conv_policy);
    assert(acc_conv_policy(b, p, map, recommended_beta_policy(), c0, k) =~= acc_conv_policy(
        b,
        p,
        map,
        strict_policy(),
        c0,
        k,
    )) by {
        assert forall|j: int| 0 <= j < k implies acc_conv_policy(
            b,
            p,
            map,
            recommended_beta_policy(),
            c0,
            k,
        )[j] == acc_conv_policy(b, p, map, strict_policy(), c0, k)[j] by {
            lemma_conv2_policy_rec_eq_strict(
                p,
                map,
                b.subrange(c0 + j * p.sample_bytes, c0 + j * p.sample_bytes + p.sample_bytes),
            );
        }
    }
}

/// The pixels a successful strict-packet-stream `MP` decodes are the same under
/// the two records (the `rle_pix`/`acc_conv` phase at the common record `p`).
pub proof fn lemma_mp_pixels_rec_eq_strict_on_strict_ok(
    b: Seq<u8>,
    m: MHModel,
    id_end: int,
    cursor: int,
    produced: int,
    px: Seq<Seq<u8>>,
)
    requires
        mh_policy(b, strict_policy()) == Ok(m),
        id_end == 18 + (m.id_length as int),
        rle_policy(m.image_type, strict_policy()) ==> mr_policy(
            b,
            pkt_of_mh_policy(m, strict_policy()),
            b.subrange(id_end, id_end + m.map_bytes),
            strict_policy(),
            id_end + m.map_bytes,
            0,
        ) == Ok((cursor, produced, px)),
    ensures
        mp_pixels_policy(b, m, recommended_beta_policy(), id_end) == mp_pixels_policy(
            b,
            m,
            strict_policy(),
            id_end,
        ),
{
    reveal(mp_pixels_policy);
    reveal(rle_policy);
    lemma_pkt_of_mh_policy_rec_eq_strict(m);
    let p = pkt_of_mh_policy(m, strict_policy());
    let map = b.subrange(id_end, id_end + m.map_bytes);
    let c0 = id_end + m.map_bytes;
    if rle_policy(m.image_type, strict_policy()) {
        lemma_mr_pix_rec_eq_strict_on_ok(b, p, map, c0, 0, Seq::empty(), (cursor, produced, px));
    } else {
        lemma_acc_conv_policy_rec_eq_strict(b, p, map, c0, m.n);
    }
}

/// `mp_assemble_policy` at the two records is equal once their pixel sequences
/// are (the orientation arguments are the record's own fields).
pub proof fn lemma_assemble_eq_from_pixels(b: Seq<u8>, m: MHModel, id_end: int)
    requires
        mp_pixels_policy(b, m, recommended_beta_policy(), id_end) == mp_pixels_policy(
            b,
            m,
            strict_policy(),
            id_end,
        ),
    ensures
        mp_assemble_policy(b, m, recommended_beta_policy(), id_end) == mp_assemble_policy(
            b,
            m,
            strict_policy(),
            id_end,
        ),
{
    reveal(mp_assemble_policy);
}

/// `recovered_policy(b, RecommendedBeta)` in the RLE branch, given that the
/// RecommendedBeta flag scan reports no event: one unfold, so the flag expression
/// resolves without pulling the packet stream into the body.
proof fn lemma_recovered_rle_false(
    b: Seq<u8>,
    m: MHModel,
    cursor: int,
    produced: int,
    px: Seq<Seq<u8>>,
)
    requires
        mh_policy(b, recommended_beta_policy()) == Ok(m),
        rle_policy(m.image_type, recommended_beta_policy()),
        mr_flags_policy(
            b,
            pkt_of_mh_policy(m, recommended_beta_policy()),
            b.subrange(18 + (m.id_length as int), 18 + (m.id_length as int) + m.map_bytes),
            recommended_beta_policy(),
            18 + (m.id_length as int) + m.map_bytes,
            0,
        ) == (false, false),
    ensures
        recovered_policy(b, recommended_beta_policy()) == false,
{
    reveal(recovered_policy);
    reveal(rle_policy);
    reveal(recommended_beta_policy);
}

/// Under Strict, the RLE branch of `mp_after_policy` returns `Ok` only when the
/// suffix passes: one unfold pins the suffix fact from the exact `Ok` result.
proof fn lemma_after_rle_ok_suffix(
    b: Seq<u8>,
    m: MHModel,
    cursor: int,
    produced: int,
    px: Seq<Seq<u8>>,
)
    requires
        mh_policy(b, strict_policy()) == Ok(m),
        rle_policy(m.image_type, strict_policy()),
        mr_policy(
            b,
            pkt_of_mh_policy(m, strict_policy()),
            b.subrange(18 + (m.id_length as int), 18 + (m.id_length as int) + m.map_bytes),
            strict_policy(),
            18 + (m.id_length as int) + m.map_bytes,
            0,
        ) == Ok((cursor, produced, px)),
        mp_after_policy(b, m, strict_policy()) is Ok,
    ensures
        mp_suffix_ok(b, cursor),
{
    reveal(mp_after_policy);
    reveal(mr_policy);
    reveal(rle_policy);
    reveal(strict_policy);
    reveal(mp_suffix_ok_policy);
    if !mp_suffix_ok(b, cursor) {
        assert(mp_after_policy(b, m, strict_policy()) == Err(Error::TrailingBytes));
    }
}

/// The shape of `mp_after_policy` at a record whose RLE packet stream returns
/// `Ok((cursor, produced, px))`: one unfold, so the branch and the (trivially
/// true under `Ignore`) suffix test resolve to the assembled image.
proof fn lemma_after_rle_shape(
    b: Seq<u8>,
    m: MHModel,
    cursor: int,
    produced: int,
    px: Seq<Seq<u8>>,
)
    requires
        mh_policy(b, strict_policy()) == Ok(m),
        rle_policy(m.image_type, strict_policy()),
        mr_policy(
            b,
            pkt_of_mh_policy(m, strict_policy()),
            b.subrange(18 + (m.id_length as int), 18 + (m.id_length as int) + m.map_bytes),
            strict_policy(),
            18 + (m.id_length as int) + m.map_bytes,
            0,
        ) == Ok((cursor, produced, px)),
        mp_suffix_ok(b, cursor),
    ensures
        mp_after_policy(b, m, strict_policy()) == Ok((
            m.width as int,
            m.height as int,
            mp_assemble_policy(b, m, strict_policy(), 18 + (m.id_length as int)),
        )),
{
    reveal(mp_after_policy);
    reveal(mr_policy);
    reveal(rle_policy);
    reveal(strict_policy);
    reveal(mp_suffix_ok_policy);
}

/// `mp_after_policy` at RecommendedBeta in the RLE branch, given the RecommendedBeta
/// packet stream's `Ok`: one unfold of the RecommendedBeta record.
proof fn lemma_after_rle_shape_recommended_beta(
    b: Seq<u8>,
    m: MHModel,
    cursor: int,
    produced: int,
    px: Seq<Seq<u8>>,
)
    requires
        mh_policy(b, recommended_beta_policy()) == Ok(m),
        rle_policy(m.image_type, recommended_beta_policy()),
        mr_policy(
            b,
            pkt_of_mh_policy(m, recommended_beta_policy()),
            b.subrange(18 + (m.id_length as int), 18 + (m.id_length as int) + m.map_bytes),
            recommended_beta_policy(),
            18 + (m.id_length as int) + m.map_bytes,
            0,
        ) == Ok((cursor, produced, px)),
    ensures
        mp_after_policy(b, m, recommended_beta_policy()) == Ok((
            m.width as int,
            m.height as int,
            mp_assemble_policy(b, m, recommended_beta_policy(), 18 + (m.id_length as int)),
        )),
{
    reveal(mp_after_policy);
    reveal(mr_policy);
    reveal(rle_policy);
    reveal(recommended_beta_policy);
    reveal(mp_suffix_ok_policy);
}

/// Obligation R, RLE case of the ID/map/pixel/suffix phase: a Strict-accepted
/// record whose Strict packet stream returns `Ok(t)` gives, under RecommendedBeta,
/// the identical `Ok(r)` with `recovered = false`. The two shape lemmas fix the
/// assembled image; the pixel equality and the flag scan are the per-packet
/// inductions above.
proof fn lemma_mp_after_rle_rec_eq_strict_on_ok(
    b: Seq<u8>,
    m: MHModel,
    cursor: int,
    produced: int,
    px: Seq<Seq<u8>>,
    r: (int, int, Seq<u8>),
)
    requires
        mh_policy(b, strict_policy()) == Ok(m),
        rle_policy(m.image_type, strict_policy()),
        mr_policy(
            b,
            pkt_of_mh_policy(m, strict_policy()),
            b.subrange(18 + (m.id_length as int), 18 + (m.id_length as int) + m.map_bytes),
            strict_policy(),
            18 + (m.id_length as int) + m.map_bytes,
            0,
        ) == Ok((cursor, produced, px)),
        mp_after_policy(b, m, strict_policy()) == Ok(r),
    ensures
        mp_after_policy(b, m, recommended_beta_policy()) == Ok(r),
        recovered_policy(b, recommended_beta_policy()) == false,
{
    reveal(rle_policy);
    reveal(strict_policy);
    lemma_pkt_of_mh_policy_rec_eq_strict(m);
    lemma_mh_policy_recommended_beta_on_strict_ok(b, m);
    let id_end = 18 + (m.id_length as int);
    let p = pkt_of_mh_policy(m, strict_policy());
    let map = b.subrange(id_end, id_end + m.map_bytes);
    let c0 = id_end + m.map_bytes;
    lemma_after_rle_ok_suffix(b, m, cursor, produced, px);
    lemma_after_rle_shape(b, m, cursor, produced, px);
    lemma_mr_at_rec_eq_strict_on_ok(b, p, map, c0, 0, Seq::empty(), (cursor, produced, px));
    lemma_mp_pixels_rec_eq_strict_on_strict_ok(b, m, id_end, cursor, produced, px);
    lemma_assemble_eq_from_pixels(b, m, id_end);
    lemma_after_rle_shape_recommended_beta(b, m, cursor, produced, px);
    lemma_mr_flags_rec_false_on_strict_ok(b, p, map, c0, 0, Seq::empty(), (cursor, produced, px));
    lemma_recovered_rle_false(b, m, cursor, produced, px);
}

/// Obligation R, raw case of the ID/map/pixel/suffix phase. `index_range =
/// Reject` on RecommendedBeta makes the raw substitution scan false
/// (`lemma_raw_subst_no_events`), and the raw phase is identical under the two
/// records (`lemma_mp_raw_acc_rec_eq_strict`).
pub proof fn lemma_mp_after_raw_rec_eq_strict_on_ok(b: Seq<u8>, m: MHModel, r: (int, int, Seq<u8>))
    requires
        mh_policy(b, strict_policy()) == Ok(m),
        !rle_policy(m.image_type, strict_policy()),
        mp_after_policy(b, m, strict_policy()) == Ok(r),
    ensures
        mp_after_policy(b, m, recommended_beta_policy()) == Ok(r),
        recovered_policy(b, recommended_beta_policy()) == false,
{
    reveal(mp_after_policy);
    reveal(recovered_policy);
    reveal(mp_raw_policy);
    reveal(rle_policy);
    reveal(strict_policy);
    reveal(recommended_beta_policy);
    reveal(mp_suffix_ok_policy);
    reveal(raw_subst_policy);
    lemma_pkt_of_mh_policy_rec_eq_strict(m);
    let id_end = 18 + (m.id_length as int);
    let p = pkt_of_mh_policy(m, strict_policy());
    let map = b.subrange(id_end, id_end + m.map_bytes);
    let c0 = id_end + m.map_bytes;
    assert(id_end + m.map_bytes <= b.len()) by {
        if id_end + m.map_bytes > b.len() {
            assert(mp_after_policy(b, m, strict_policy()) == Err(Error::TruncatedColorMap));
            assert(false);
        }
    }
    match mp_raw_policy(b, p, map, strict_policy(), c0) {
        Err(_) => {
            assert(false);
        },
        Ok((_px, cursor)) => {
            assert(r == (m.width as int, m.height as int, mp_assemble_policy(b, m, strict_policy(), id_end)));
            lemma_mp_raw_acc_rec_eq_strict(b, p, map, c0, 0, Seq::empty());
            assert(mp_raw_policy(b, p, map, recommended_beta_policy(), c0) == Ok((_px, cursor)));
            lemma_mp_pixels_rec_eq_strict_on_strict_ok(b, m, id_end, cursor, m.n, _px);
            lemma_assemble_eq_from_pixels(b, m, id_end);
            assert(mp_suffix_ok_policy(b, recommended_beta_policy(), cursor));
            lemma_raw_subst_no_events(b, p, recommended_beta_policy(), c0, m.n);
        },
    }
}

/// Obligation R at the ID/map/pixel/suffix phase: the two branch lemmas behind
/// one dispatch on the record's RLE flag.
pub proof fn lemma_mp_after_policy_rec_eq_strict_on_ok(b: Seq<u8>, m: MHModel, r: (int, int, Seq<u8>))
    requires
        mh_policy(b, strict_policy()) == Ok(m),
        mp_after_policy(b, m, strict_policy()) == Ok(r),
    ensures
        mp_after_policy(b, m, recommended_beta_policy()) == Ok(r),
        recovered_policy(b, recommended_beta_policy()) == false,
{
    reveal(rle_policy);
    reveal(strict_policy);
    if rle_policy(m.image_type, strict_policy()) {
        reveal(mp_after_policy);
        reveal(mr_policy);
        let id_end = 18 + (m.id_length as int);
        let p = pkt_of_mh_policy(m, strict_policy());
        let map = b.subrange(id_end, id_end + m.map_bytes);
        let c0 = id_end + m.map_bytes;
        assert(mp_after_policy(b, m, strict_policy()) == match mr_policy(b, p, map, strict_policy(), c0, 0)
            {
                Err(_) => mp_after_policy(b, m, strict_policy()),
                Ok((cursor, _produced, _px)) => Ok((
                    m.width as int,
                    m.height as int,
                    mp_assemble_policy(b, m, strict_policy(), id_end),
                )),
            }) by {
            reveal(mp_after_policy);
        }
        match mr_policy(b, p, map, strict_policy(), c0, 0) {
            Err(_) => {
                assert(false);
            },
            Ok((cursor, produced, px)) => {
                lemma_mp_after_rle_rec_eq_strict_on_ok(b, m, cursor, produced, px, r);
            },
        }
    } else {
        lemma_mp_after_raw_rec_eq_strict_on_ok(b, m, r);
    }
}
/// Obligation R (THEOREMS-POLICY-DELTA.md): every input Strict accepts decodes
/// to the identical `(width, height, RGBA)` under RecommendedBeta, with
/// `recovered = false`.
pub proof fn lemma_mp_policy_recommended_beta_on_strict_ok(b: Seq<u8>, r: (int, int, Seq<u8>))
    requires
        mp_policy(b, strict_policy()) == Ok(r),
    ensures
        mp_policy(b, recommended_beta_policy()) == Ok(r),
        recovered_policy(b, recommended_beta_policy()) == false,
{
    reveal(mp_policy);
    reveal(mh_policy);
    reveal(strict_policy);
    reveal(recommended_beta_policy);
    match mh_policy(b, strict_policy()) {
        Err(_) => {
            assert(false);
        },
        Ok(m) => {
            lemma_mh_policy_recommended_beta_on_strict_ok(b, m);
            lemma_mp_after_policy_rec_eq_strict_on_ok(b, m, r);
        },
    }
}

/// OR-composition of the two closed events over one successful packet.
pub open spec fn event_step_policy(
    b: Seq<u8>, h: PktH, map: Seq<u8>, pol: Policy, c0: int, p0: int, c1: int, p1: int,
    clip0: bool, sub0: bool, clip1: bool, sub1: bool,
) -> bool {
    &&& (clip0 || mr_flags_policy(b, h, map, pol, c0, p0).0)
        == (clip1 || mr_flags_policy(b, h, map, pol, c1, p1).0)
    &&& (sub0 || mr_flags_policy(b, h, map, pol, c0, p0).1)
        == (sub1 || mr_flags_policy(b, h, map, pol, c1, p1).1)
}

pub proof fn lemma_packet_events(
    b: Seq<u8>, h: PktH, map: Seq<u8>, pol: Policy, cur: int, prod: int,
    count: int, paint: int, end: int, sub: bool,
)
    requires
        0 <= cur < b.len(), 0 <= prod < h.n, 0 <= h.sample_bytes,
        count == rle_count(b[cur] as int),
        !(count > h.n - prod && pol.rle_overrun == RleOverrunOptions::Reject),
        paint == (if count < h.n - prod { count } else { h.n - prod }),
        end == cur + 1 + (if rle_repeat(b[cur] as int) { h.sample_bytes }
            else if pol.rle_overrun == RleOverrunOptions::ClipSamples { paint * h.sample_bytes }
            else { count * h.sample_bytes }),
        end <= b.len(),
        rle_repeat(b[cur] as int) ==> (h.kind != 0 || !pkt_reject(h,
            sindex(b.subrange(cur + 1, end), h.sample_bytes), pol)),
        sub == (if rle_repeat(b[cur] as int) {
            h.kind == 0 && pkt_subst(h, sindex(b.subrange(cur + 1, end), h.sample_bytes), pol)
        } else { lit_flags_policy(b, h, pol, cur + 1, paint).0 }),
    ensures
        mr_flags_policy(b, h, map, pol, cur, prod).0 ==
            (count > h.n - prod || mr_flags_policy(b, h, map, pol, end, prod + paint).0),
        mr_flags_policy(b, h, map, pol, cur, prod).1 ==
            (sub || mr_flags_policy(b, h, map, pol, end, prod + paint).1),
{ reveal(mr_flags_policy); }

proof fn lemma_lit_flags_step(b: Seq<u8>, h: PktH, pol: Policy, start: int, count: int)
    ensures lit_flags_policy(b, h, pol, start, count).0 == (count > 0 && (
        (h.kind == 0 && pkt_subst(h, sindex(b.subrange(start, start + h.sample_bytes), h.sample_bytes), pol))
        || lit_flags_policy(b, h, pol, start + h.sample_bytes, count - 1).0)),
{ reveal(lit_flags_policy); }

pub proof fn lemma_lit_flags_extend(b: Seq<u8>, h: PktH, pol: Policy, start: int, i: int)
    requires i >= 0,
    ensures lit_flags_policy(b, h, pol, start, i + 1).0 ==
        (lit_flags_policy(b, h, pol, start, i).0 ||
            (h.kind == 0 && pkt_subst(h, sindex(b.subrange(start + i * h.sample_bytes,
                start + (i + 1) * h.sample_bytes), h.sample_bytes), pol))),
    decreases i,
{
    hide(lit_flags_policy);
    hide(pkt_subst);
    lemma_literal_shift(start, h.sample_bytes, i);
    lemma_lit_flags_step(b, h, pol, start, i);
    lemma_lit_flags_step(b, h, pol, start, i + 1);
    if i > 0 {
        lemma_lit_flags_extend(b, h, pol, start + h.sample_bytes, i - 1);
    } else {
        lemma_lit_flags_step(b, h, pol, start + h.sample_bytes, 0);
    }
}

/// Cursor/error projection used to connect the executable pixel phases to MP.
pub open spec fn policy_phase(b: Seq<u8>, m: MHModel, pol: Policy) -> Result<int, Error> {
    let id = 18 + m.id_length as int;
    let c0 = id + m.map_bytes;
    let map = b.subrange(id, c0);
    let h = pkt_of_mh_policy(m, pol);
    if rle_policy(m.image_type, pol) {
        match mr_cur_policy(b, h, map, pol, c0, 0) {
            Ok((c, _)) => Ok(c),
            Err(MrErr::Api(e)) => Err(e),
            Err(MrErr::Outside) => Err(Error::ColorIndexOutOfRange),
        }
    } else { mp_raw_cur_policy(b, h, pol, c0, 0) }
}

pub proof fn lemma_policy_phase(b: Seq<u8>, m: MHModel, pol: Policy)
    requires
        18 + m.id_length as int <= b.len(),
        0 <= m.map_bytes,
        18 + m.id_length as int + m.map_bytes <= b.len(),
    ensures mp_after_policy(b, m, pol) == match policy_phase(b, m, pol) {
        Err(e) => Err(e),
        Ok(c) => if mp_suffix_ok_policy(b, pol, c) {
            Ok((m.width as int, m.height as int, mp_assemble_policy(b, m, pol, 18 + m.id_length as int)))
        } else { Err(Error::TrailingBytes) },
    },
{
    let id = 18 + m.id_length as int;
    let c0 = id + m.map_bytes;
    let map = b.subrange(id, c0);
    let h = pkt_of_mh_policy(m, pol);
    if rle_policy(m.image_type, pol) {
        lemma_mr_cur_policy_proj(b, h, map, pol, c0, 0, Seq::empty());
        reveal(mr_policy);
    } else { lemma_mp_raw_policy_cur(b, h, map, pol, c0); }
    reveal(policy_phase);
    reveal(mp_after_policy);
}

/// The full MR tuple agrees with its cursor, partial-pixel and error-cursor projections.
pub proof fn lemma_mr_state_policy_proj(
    s: Seq<u8>, h: PktH, map: Seq<u8>, pol: Policy, cur: int, prod: int, acc: Seq<Seq<u8>>,
)
    requires 0 <= cur <= s.len(), 0 <= prod <= h.n, 1 <= h.sample_bytes,
    ensures match mr_at_policy(s, h, map, pol, cur, prod, acc) {
        Ok((c, pr, px)) => mr_cur_policy(s, h, map, pol, cur, prod) == Ok((c, pr))
            && px == acc + mr_pix_policy(s, h, map, pol, cur, prod)
            && pr == prod + mr_pix_policy(s, h, map, pol, cur, prod).len(),
        Err((e, c, pr, px)) => mr_cur_policy(s, h, map, pol, cur, prod) == Err(e)
            && mr_gcur_policy(s, h, map, pol, cur, prod) == Ok(c)
            && px == acc + mr_pix_policy(s, h, map, pol, cur, prod)
            && pr == prod + mr_pix_policy(s, h, map, pol, cur, prod).len(),
    },
    decreases h.n - prod,
{
    hide(conv2_policy);
    lemma_mr_cur_policy_proj(s, h, map, pol, cur, prod, acc);
    reveal(mr_at_policy); reveal(mr_pix_policy); reveal(mr_gcur_policy);
    broadcast use vstd::seq_lib::group_seq_lib_default;
    if prod < h.n && cur < s.len() {
        let count = rle_count(s[cur] as int);
        let paint = if count < h.n - prod { count } else { h.n - prod };
        let start = cur + 1;
        if !(count > h.n - prod && pol.rle_overrun == RleOverrunOptions::Reject) {
            if rle_repeat(s[cur] as int) {
                let end = start + h.sample_bytes;
                if end <= s.len() {
                    let sample = s.subrange(start, end);
                    if h.kind != 0 || !pkt_reject(h, sindex(sample, h.sample_bytes), pol) {
                        let here = pix_pow(conv2_policy(h, map, sample, pol), paint);
                        crate::model::lemma_pix_pow_len(conv2_policy(h, map, sample, pol), paint);
                        lemma_mr_state_policy_proj(s, h, map, pol, end, prod + paint, acc + here);
                        vstd::seq_lib::lemma_concat_associative(acc, here, mr_pix_policy(s, h, map, pol, end, prod + paint));
                    }
                }
            } else {
                let need = if pol.rle_overrun == RleOverrunOptions::ClipSamples { paint } else { count };
                let end = start + need * h.sample_bytes;
                if end <= s.len() {
                    if !lit_ok_policy(s, h, start, paint, pol) {
                        lemma_lit_good_policy_le(s, h, start, paint, pol);
                        lemma_lit_out_policy_len(s, h, map, pol, start, lit_good_policy(s, h, start, paint, pol));
                    } else {
                        let here = lit_out_policy(s, h, map, pol, start, paint);
                        lemma_lit_out_policy_len(s, h, map, pol, start, paint);
                        lemma_mr_state_policy_proj(s, h, map, pol, end, prod + paint, acc + here);
                        vstd::seq_lib::lemma_concat_associative(acc, here, mr_pix_policy(s, h, map, pol, end, prod + paint));
                    }
                }
            }
        }
    }
}

/// T2/T4 map-address bounds for every retained policy, including ZeroBased.
pub proof fn t2_t4_policy_map_address(h: PktH, pol: Policy, index: int, stride: int)
    requires in_range_policy(h, index, pol), 0 <= h.map_len <= 65535, 1 <= stride <= 4,
    ensures
        0 <= entry_j(index, h.map_first, pol) * stride,
        (entry_j(index, h.map_first, pol) + 1) * stride <= h.map_len * stride <= 262140,
        pol.map_index == MapIndexOptions::ZeroBased ==> entry_j(index, h.map_first, pol) == index,
        pol.map_index == MapIndexOptions::Absolute ==> index >= h.map_first
            && entry_j(index, h.map_first, pol) == index - h.map_first,
{
    reveal(in_range_policy);
    let j = entry_j(index, h.map_first, pol);
    assert(0 <= j * stride && (j + 1) * stride <= h.map_len * stride && h.map_len * stride <= 262140)
        by (nonlinear_arith) requires 0 <= j < h.map_len, h.map_len <= 65535, 1 <= stride <= 4;
}

} // verus!
