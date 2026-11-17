//! M4b: the G12 compositor. Drawing one decoded index at a time onto the canvas,
//! the G12.3 disposals, and the `Saved` buffer.
//!
//! Canvas layout (as `gif_verus/src/compose.rs`): `W·H` RGBA8 pixels, top row
//! first; pixel `(x, y)` with `x < W`, `y < H` is at byte offset `4·(y·W + x)`,
//! below `4·W·H <= MAX_OUTPUT`. All coordinate arithmetic is on values up to
//! 131,070 (`left + w`, `top + h`) and is never narrowed; offsets are formed only
//! after the bounds are checked.
//!
//! `Saved` has the canvas's shape: the rectangle of a disposal-3 frame is copied
//! into the same offsets of `Saved` and copied back by the restore, so the
//! restore reads exactly the bytes the save wrote.
//!
//! The interface the LZW core feeds (for M5): `draw_index` is called once per
//! decoded index in raster order, with `st.q` increasing from 0. `st.q < w·h` is
//! the LZW core's invariant: G10.2 emits indices only while `q < A`, and reports
//! `TooManyPixels` beyond `A`. The row cache keeps the interlace row per stream
//! row, so it is computed once per row rather than once per pixel.

use vstd::prelude::*;
#[cfg(verus_only)]
use vstd::arithmetic::div_mod::{
    lemma_basic_div, lemma_div_pos_is_pos, lemma_fundamental_div_mod, lemma_mod_bound,
};
#[cfg(verus_only)]
use vstd::arithmetic::mul::{
    lemma_mul_inequality, lemma_mul_is_associative, lemma_mul_is_commutative,
    lemma_mul_is_distributive_add, lemma_mul_is_distributive_add_other_way,
};
#[cfg(verus_only)]
use crate::mc::{
    draw_run, draw_run_at, draw_step, lemma_mc_unfold, max_dimension, max_output, mc, mc_disposed,
    mc_from, mc_saved, pixel_is_clear, pixel_is_entry, pixel_word, pixels_well_formed, target_cell,
    target_x, target_y, write_pixel, FrameSpec,
};
use crate::interlace::interlace_row;
#[cfg(verus_only)]
use crate::interlace::{lemma_ir_bijective, lemma_ir_eq_iv, lemma_iv_nat, lemma_iv_total};
#[cfg(verus_only)]
use crate::interlace::{lemma_div_add_mul as div_add_mul, lemma_div_mul_le};

verus! {

// ---------------------------------------------------------------------------
// G16's error for this module.
// ---------------------------------------------------------------------------

/// The only G16 error G12.2 raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComposeError {
    ColorIndexOutOfRange,
}

// ---------------------------------------------------------------------------
// Frame rectangle, frame record and the canvas side conditions.
// ---------------------------------------------------------------------------

/// A frame rectangle as the descriptor gives it (`w, h >= 1`, and `left + w`,
/// `top + h` at most `2·MAX_DIMENSION`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

/// The frame's G11/G12 fields: `c` is the entry count of the active colour table
/// (`3·c` bytes), the transparent index is absent when G6.1's flag is clear, and
/// `disposal` is 0..3 on every reachable state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Frame {
    pub rect: Rect,
    pub interlaced: bool,
    pub c: u32,
    pub transparent: Option<u8>,
    pub disposal: u32,
}

impl Rect {
    pub open spec fn spec_rect(self) -> crate::mc::Rect {
        crate::mc::Rect {
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
        }
    }
}

impl Frame {
    pub open spec fn spec_frame(self) -> FrameSpec {
        FrameSpec {
            rect: crate::mc::Rect {
                left: self.rect.left,
                top: self.rect.top,
                width: self.rect.width,
                height: self.rect.height,
            },
            interlaced: self.interlaced,
            c: self.c as int,
            transparent: self.transparent,
            disposal: self.disposal as int,
        }
    }
}

/// What a reachable frame gives the compositor: positive sides inside G1's range,
/// `C` inside a colour table's reach (G4's table is at most 256 entries), a
/// disposal in `0..=3` (G6.1 rejects 4–7), and both coordinate sums inside
/// `2·MAX_DIMENSION` without narrowing (THEOREMS' `rect_sum_unnarrowed`).
pub open spec fn frame_ok(fr: Frame) -> bool {
    &&& 1 <= fr.rect.width <= max_dimension()
    &&& 1 <= fr.rect.height <= max_dimension()
    &&& 1 <= fr.c <= 256
    &&& fr.disposal <= 3
    &&& (fr.rect.left as int) + (fr.rect.width as int) <= 2 * max_dimension()
    &&& (fr.rect.top as int) + (fr.rect.height as int) <= 2 * max_dimension()
}

/// The canvas the compositor works on: `4·W·H` bytes with `W, H` inside G1's
/// range, so G3's `4·W·H <= MAX_OUTPUT` holds.
pub open spec fn canvas_ok(canvas: &[u8], sw: u32, sh: u32) -> bool {
    &&& 1 <= sw <= max_dimension()
    &&& 1 <= sh <= max_dimension()
    &&& canvas@.len() == 4 * (sw as int) * (sh as int)
    &&& 4 * (sw as int) * (sh as int) <= max_output()
}

/// `table` is the active colour table: `3·C` bytes.
pub open spec fn table_ok(table: &[u8], fr: Frame) -> bool {
    table@.len() == 3 * (fr.c as int)
}

// ---------------------------------------------------------------------------
// The drawing state, and the per-index interface for the LZW core (M5).
// ---------------------------------------------------------------------------

/// G12.2's per-frame drawing state: the next stream position `q`. The image row
/// is computed from `q` when a pixel is written (one division per written pixel;
/// a per-row cache is a later optimisation, and the LZW core's `q` is the only
/// state the drawing needs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawState {
    pub q: u32,
}

/// A fresh state for a frame.
pub fn new_state() -> (s: DrawState)
    ensures
        s.q == 0u32,
{
    DrawState { q: 0 }
}

/// G12.2: draw the index `idx` at stream position `st.q`, for the frame `fr`, the
/// active table `table` and a `sw × sh` screen. One call per decoded index, in
/// raster order. `st.q < w·h` is the caller's invariant (the LZW core emits
/// indices only while `q < A`, and reports `TooManyPixels` beyond `A`).
///
/// The result is `Err(ColorIndexOutOfRange)` when the index is not the frame's
/// transparent one and is `>= fr.c`; the transparent index is tested first, so a
/// transparent index `>= C` writes nothing (N12). The canvas at return is exactly
/// `draw_step`'s, and on an error it is unchanged (G12.4: an error exposes no
/// partial write of this index).
pub fn draw_index(
    canvas: &mut [u8],
    st: &mut DrawState,
    fr: Frame,
    table: &[u8],
    sw: u32,
    sh: u32,
    idx: u8,
) -> (r: Result<(), ComposeError>)
    requires
        canvas_ok(old(canvas), sw, sh),
        frame_ok(fr),
        table_ok(table, fr),
        (old(st).q as int) < (fr.rect.width as int) * (fr.rect.height as int),
    ensures
        final(canvas)@.len() == old(canvas)@.len(),
        final(st).q == old(st).q + 1,
        match draw_step(old(canvas)@, fr.spec_frame(), table@, sw, sh, old(st).q as int, idx) {
            Ok(cv2) => r.is_ok() && final(canvas)@ == cv2,
            Err(e) => r == Err(e) && final(canvas)@ == old(canvas)@,
        },
{
    let q = st.q;
    let ghost qv: int = q as int;
    proof {
        lemma_cap_product_u32();
        assert((fr.rect.width as int) <= max_dimension());
        assert((fr.rect.height as int) <= max_dimension());
        lemma_mul_inequality(fr.rect.width as int, max_dimension(), fr.rect.height as int);
        assert((fr.rect.width as int) * (fr.rect.height as int) <= max_dimension() * max_dimension());
        assert(qv + 1 <= 4294967295);
    }
    st.q = q + 1;
    let ghost frame = fr.spec_frame();
    // G12.2: the transparent index leaves the pixel, even at `>= C` (N12).
    if let Some(t) = fr.transparent {
        if idx == t {
            proof {
                assert(draw_step(old(canvas)@, frame, table@, sw, sh, qv, idx) == Ok(old(canvas)@));
            }
            return Ok(());
        }
    }
    if (idx as u32) >= fr.c {
        proof {
            assert(draw_step(old(canvas)@, frame, table@, sw, sh, qv, idx)
                == Err(ComposeError::ColorIndexOutOfRange));
        }
        return Err(ComposeError::ColorIndexOutOfRange);
    }
    let w = fr.rect.width;
    let r: u32 = q / w;
    let col: u32 = q % w;
    proof {
        lemma_row_below(qv, w as int, fr.rect.height as int);
        assert((r as int) < (fr.rect.height as int));
    }
    let row_y: u32 = if fr.interlaced {
        interlace_row(r, fr.rect.height)
    } else {
        r
    };
    let x: u32 = fr.rect.left + col;
    let y: u32 = fr.rect.top + row_y;
    proof {
        lemma_row_below(qv, w as int, fr.rect.height as int);
        assert((qv / (w as int)) == r as int);
        assert((qv % (w as int)) == col as int);
        assert((row_y as int) == target_y(frame, qv) - (fr.rect.top as int)) by {
            if fr.interlaced {
                lemma_ir_eq_iv(r as int, fr.rect.height as int);
            }
        }
        assert((x as int) == target_x(frame, qv));
        assert((y as int) == target_y(frame, qv));
        lemma_cap_sum_u32(fr.rect.left as int, col as int);
        lemma_cap_sum_u32(fr.rect.top as int, row_y as int);
        assert((fr.rect.left as int) + (col as int) <= 4294967295);
        assert((fr.rect.top as int) + (row_y as int) <= 4294967295);
    }
    if x >= sw || y >= sh {
        proof {
            assert(target_cell(frame, sw, sh, qv) == None);
            assert(draw_step(old(canvas)@, frame, table@, sw, sh, qv, idx) == Ok(old(canvas)@));
        }
        return Ok(());
    }
    proof {
        assert((y as int) < (sh as int));
        assert((x as int) < (sw as int));
        assert((sw as int) <= max_dimension());
        assert((sh as int) <= max_dimension());
        lemma_mul_inequality(y as int, max_dimension(), sw as int);
        lemma_sw_sh_prod_u32(sw as int, sh as int);
        lemma_cell_offset_in_bounds(y as int, x as int, sw as int, sh as int);
        assert((y as int) * (sw as int) + (x as int) + 1 <= 4 * (sw as int) * (sh as int));
    }
    let o: usize = 4 * (y as usize * sw as usize + x as usize);
    proof {
        assert((x as int) < (sw as int));
        assert((y as int) < (sh as int));
        assert(target_cell(frame, sw, sh, qv) == Some((y as int) * (sw as int) + (x as int)));
        lemma_cell_offset_in_bounds(y as int, x as int, sw as int, sh as int);
        assert((o as int) == 4 * ((y as int) * (sw as int) + (x as int)));
    }
    let k: usize = 3 * (idx as usize);
    canvas[o] = table[k];
    canvas[o + 1] = table[k + 1];
    canvas[o + 2] = table[k + 2];
    canvas[o + 3] = 255u8;
    proof {
        let cv = old(canvas)@;
        let tb = table@;
        assert((idx as int) < (fr.c as int));
        assert(3 * (idx as int) + 3 <= 3 * (fr.c as int));
        assert(k as int == 3 * (idx as int));
        assert(tb[3 * (idx as int)] == pixel_word(tb, idx as int)[0]);
        assert(tb[3 * (idx as int) + 1] == pixel_word(tb, idx as int)[1]);
        assert(tb[3 * (idx as int) + 2] == pixel_word(tb, idx as int)[2]);
        assert(pixel_word(tb, idx as int) == seq![
            tb[3 * (idx as int)],
            tb[3 * (idx as int) + 1],
            tb[3 * (idx as int) + 2],
            255u8,
        ]);
        assert(write_pixel(cv, o as int, tb, idx as int) == cv.update(o as int, tb[3 * (idx as int)]).update(
            o as int + 1,
            tb[3 * (idx as int) + 1],
        ).update(o as int + 2, tb[3 * (idx as int) + 2]).update(o as int + 3, 255u8));
        assert(draw_step(cv, frame, tb, sw, sh, qv, idx)
            == Ok(write_pixel(cv, 4 * ((y as int) * (sw as int) + (x as int)), tb, idx as int)));
        assert(final(canvas)@ == write_pixel(cv, o as int, tb, idx as int));
    }
    Ok(())
}

/// An unchecked RGBA byte count can exceed u32 even for LE16-domain sides.
/// Once G3 checks MAX_OUTPUT, the canvas byte offsets fit usize on both targets;
/// the product of the two screen dimensions itself fits u32.
pub proof fn lemma_sw_sh_prod_u32(sw: int, sh: int)
    requires
        0 <= sw <= max_dimension(),
        0 <= sh <= max_dimension(),
    ensures
        sw * sh <= 4294967295,
        sw * sh <= (u16::MAX as int) * (u16::MAX as int),
{
    crate::limits::caps_ok();
    lemma_mul_inequality(sw, max_dimension(), sh);
    assert(sw * sh <= max_dimension() * max_dimension());
    assert(max_dimension() * max_dimension() <= (u16::MAX as int) * (u16::MAX as int))
        by(nonlinear_arith) requires 0 <= max_dimension() <= u16::MAX;
}

/// `left + col` and `top + row_y` fit in a `u32` on every reachable state:
/// `left <= 2·MAX_DIMENSION` and `col < MAX_DIMENSION`.
pub proof fn lemma_cap_sum_u32(a: int, b: int)
    requires
        0 <= a <= 2 * max_dimension(),
        0 <= b < max_dimension(),
    ensures
        a + b <= 4294967295,
{
    crate::limits::caps_ok();
}

/// `⌊q/w⌋ < h` for `q < w·h` and `w >= 1`: the interlace row is always defined.
pub proof fn lemma_row_below(q: int, w: int, h: int)
    requires
        1 <= w,
        1 <= h,
        0 <= q < w * h,
    ensures
        0 <= q / w < h,
{
    if q / w >= h {
        lemma_mul_inequality(h, q / w, w);
        assert((q / w) * w >= h * w);
        lemma_mul_is_commutative(h, w);
        assert(h * w == w * h);
        lemma_div_mul_le(q, w);
        lemma_mul_is_commutative(q / w, w);
    }
}

/// `w·⌊q/w⌋ == q − q % w`: the row's start, so `q − row_start` is the column.
pub proof fn lemma_row_start_mod(q: int, w: int)
    requires
        1 <= w,
    ensures
        (q / w) * w == q - q % w,
{
    lemma_fundamental_div_mod(q, w);
    lemma_mul_is_commutative(w, q / w);
}

/// THEOREMS' `canvas_offset_bound`: pixel `(x, y)` inside a `sw × sh` canvas has
/// its four bytes inside the canvas, strictly: `4·(y·sw + x) + 3 < 4·sw·sh`.
pub proof fn lemma_cell_offset_in_bounds(y: int, x: int, sw: int, sh: int)
    requires
        0 <= y < sh,
        0 <= x < sw,
        0 <= sh,
    ensures
        4 * (y * sw + x) + 3 < 4 * sw * sh,
{
    lemma_mul_is_distributive_add_other_way(sw, y, 1);
    assert((y + 1) * sw == y * sw + 1 * sw) by {
        assert((y + 1) * sw == y * sw + 1 * sw);
    }
    assert(y * sw + x + 1 <= y * sw + sw);
    lemma_mul_inequality(y * sw + x + 1, (y + 1) * sw, 1);
    assert(y * sw + x + 1 <= (y + 1) * sw);
    lemma_mul_inequality(y + 1, sh, sw);
    assert((y + 1) * sw <= sh * sw);
    lemma_mul_is_commutative(sh, sw);
    assert((y + 1) * sw <= sw * sh);
    assert((y * sw + x + 1) <= sw * sh);
    lemma_mul_inequality(y * sw + x + 1, sw * sh, 4);
    assert(4 * (y * sw + x + 1) <= 4 * (sw * sh));
    lemma_mul_is_associative(4, sw, sh);
    assert(4 * (sw * sh) == 4 * sw * sh);
    assert(4 * (y * sw + x) + 3 < 4 * (y * sw + x + 1));
}


/// `MAX_DIMENSION·MAX_DIMENSION` fits in a `u32`: `4·MAX_DIMENSION²` is under
/// four times `u32::MAX` (THEOREMS' `area_cap_u64`), and the product itself,
/// `4,294,836,225`, is under `u32::MAX = 4,294,967,295`.
pub proof fn lemma_cap_product_u32()
    ensures
        max_dimension() * max_dimension() <= 4294967295,
        max_dimension() * max_dimension() <= (u16::MAX as int) * (u16::MAX as int),
{
    crate::limits::caps_ok();
    assert(max_dimension() * max_dimension() <= (u16::MAX as int) * (u16::MAX as int))
        by(nonlinear_arith) requires 0 <= max_dimension() <= u16::MAX;
}

/// `w·⌊q/w⌋ <= q`, the bound that keeps the row start inside `q`.
pub proof fn lemma_row_start_le_q(q: int, w: int)
    requires
        0 <= q,
        1 <= w,
    ensures
        (q / w) * w <= q,
{
    lemma_div_mul_le(q, w);
    lemma_mul_is_commutative(q / w, w);
}

// ---------------------------------------------------------------------------
// G12.3's rectangle operations: the disposal-2 fill, and the disposal-3 save and
// restore. `Saved` has the canvas's shape, so a rectangle's pixels sit at the
// same byte offsets of both buffers.
// ---------------------------------------------------------------------------

/// The clipped rectangle of `r` in a `sw × sh` screen, as the executable
/// computes it: `(left, x1, top, y1)` with `x1 = min(left + width, sw)` and
/// `y1 = min(top + height, sh)`. The executable keeps the *unclipped* `left` and
/// `top`, because the descriptor's coordinates are unsigned: a rectangle starting
/// at or beyond an edge has `x1 <= x0` or `y1 <= y0`, an empty range.
pub fn clip_of(r: Rect, sw: u32, sh: u32) -> (c: (u32, u32, u32, u32))
    requires
        r.width >= 1,
        r.height >= 1,
        (r.left as int) + (r.width as int) <= 2 * max_dimension(),
        (r.top as int) + (r.height as int) <= 2 * max_dimension(),
    ensures
        crate::mc::clip(r.spec_rect(), sw, sh) == (c.0 as int, c.1 as int, c.2 as int, c.3 as int),
        c.1 as int <= sw as int,
        c.3 as int <= sh as int,
        c.0 == r.left, c.2 == r.top,
        c.1 as int <= r.left as int + r.width as int,
        c.3 as int <= r.top as int + r.height as int,
        c.0 as int <= c.1 as int || c.1 as int <= c.0 as int,
{
    proof {
        assert((r.left as int) + (r.width as int) <= 131070);
        assert((r.top as int) + (r.height as int) <= 131070);
    }
    let x1: u32 = if r.left + r.width <= sw {
        r.left + r.width
    } else {
        sw
    };
    let y1: u32 = if r.top + r.height <= sh {
        r.top + r.height
    } else {
        sh
    };
    (r.left, x1, r.top, y1)
}

/// The byte offset of the row's first pixel at column `x` is inside the canvas
/// when `x <= sw` and `y < sh`: `4·(y·sw + x) <= 4·sw·sh == |canvas|`.
pub proof fn lemma_row_offset_bound(y: int, x: int, sw: int, sh: int)
    requires
        0 <= x <= sw,
        0 <= y < sh,
    ensures
        4 * (y * sw + x) <= 4 * sw * sh,
{
    lemma_mul_is_distributive_add_other_way(sw, y, 1);
    assert(y * sw + sw == (y + 1) * sw);
    lemma_mul_inequality(y + 1, sh, sw);
    lemma_mul_is_commutative(sh, sw);
    assert((y + 1) * sw <= sw * sh);
    crate::limits::caps_ok();
    assert(y * sw + x <= y * sw + sw);
    lemma_mul_is_associative(4, sw, sh);
}

/// `clear_rect`'s and `copy_rect`'s empty clipped rectangle writes nothing.
pub proof fn lemma_clip_span(x0: int, x1: int, y0: int, y1: int)
    requires
        x1 <= x0 || y1 <= y0,
    ensures
        !(x0 < x1 && y0 < y1),
{
}


// ---------------------------------------------------------------------------
// G12.3's rectangle operations, against the per-byte model above.
// ---------------------------------------------------------------------------

/// The four bytes of cell `(y, x)` are `[4·(y·sw + x), 4·(y·sw + x) + 4)` for an
/// in-range `x < sw`. Used to turn "byte `i` is one of this cell's bytes" into
/// "the cell's column is `x`".
pub proof fn lemma_cell_bytes(i: int, y: int, x: int, sw: int)
    requires
        0 <= y,
        0 <= x < sw,
        1 <= sw,
        0 <= i,
    ensures
        (crate::mc::byte_row(i, sw) == y && crate::mc::byte_col(i, sw) == x) <==> (
        4 * (y * sw + x) <= i < 4 * (y * sw + x) + 4),
{
    if 4 * (y * sw + x) <= i < 4 * (y * sw + x) + 4 {
        lemma_cell_range(i, y, x, x + 1, sw);
    }
    if crate::mc::byte_row(i, sw) == y && crate::mc::byte_col(i, sw) == x {
        assert(4 * (y * sw + x) <= i < 4 * (y * sw + x) + 4) by {
            reveal(crate::mc::byte_row);
            reveal(crate::mc::byte_col);
            assert(4 * (i / 4) <= i < 4 * (i / 4) + 4) by {
                lemma_fundamental_div_mod(i, 4);
            }
            assert(i / 4 == y * sw + x) by {
                assert(i / 4 / sw == y);
                assert(i / 4 % sw == x);
                assert(i / 4 == sw * ((i / 4) / sw) + (i / 4) % sw) by {
                    lemma_fundamental_div_mod(i / 4, sw);
                }
                lemma_mul_is_commutative(sw, (i / 4) / sw);
            }
        }
    }
}

/// The bytes of the row `y` column range `[x0, x1)` are exactly the byte indices
/// whose row is `y` and whose column lies in the range.
pub proof fn lemma_cell_range(i: int, y: int, x0: int, x1: int, sw: int)
    requires
        0 <= y,
        0 <= x0 <= x1 <= sw,
        1 <= sw,
        4 * (y * sw + x0) <= i < 4 * (y * sw + x1),
    ensures
        crate::mc::byte_row(i, sw as int) == y,
        x0 <= crate::mc::byte_col(i, sw as int) < x1,
{
    reveal(crate::mc::byte_row);
    reveal(crate::mc::byte_col);
    let z0 = y * sw + x0;
    let z1 = y * sw + x1;
    assert(i / 4 >= z0) by {
        if i / 4 < z0 {
            lemma_mul_inequality(i / 4 + 1, z0, 4);
            lemma_mul_is_distributive_add_other_way(4, i / 4, 1);
            assert(4 * (i / 4 + 1) == 4 * (i / 4) + 4);
        }
    }
    assert(i / 4 < z1) by {
        if i / 4 >= z1 {
            lemma_mul_inequality(z1, i / 4, 4);
            assert(4 * (i / 4) >= 4 * z1);
        }
    }
    assert(x0 <= i / 4 - y * sw < sw) by {
        assert(i / 4 - y * sw >= x0);
        assert(i / 4 - y * sw < z1 - y * sw);
        lemma_mul_is_distributive_add_other_way(sw, y, 1);
        assert(z1 - y * sw == x1);
    }
    lemma_div_slab(i / 4, y, sw);
    assert((i / 4) / sw == y);
    assert((i / 4) % sw == i / 4 - y * sw);
    assert(x0 <= (i / 4) % sw < x1) by {
        assert((i / 4) % sw == i / 4 - y * sw);
    }
}

/// `p/sw == y` when `y·sw <= p < y·sw + sw`: division on a slab.
pub proof fn lemma_div_slab(p: int, y: int, sw: int)
    requires
        0 <= y,
        1 <= sw,
        0 <= p - y * sw < sw,
    ensures
        p / sw == y,
        p % sw == p - y * sw,
{
    assert(p == y * sw + (p - y * sw));
    div_add_mul(y, p - y * sw, sw);
    lemma_basic_div(p - y * sw, sw);
    lemma_fundamental_div_mod(p, sw);
    lemma_mul_is_commutative(y, sw);
}

/// `clear_cols`-shaped effect, as the model's `Seq::new` state: the bytes written
/// by the inner loop's run over `[a, z)` of row `y`.
pub open spec fn cleared_upto(
    cv: Seq<u8>,
    sw: u32,
    x0: int,
    x1: int,
    y0: int,
    y1: int,
    y: int,
    x: int,
) -> Seq<u8> {
    Seq::new(
        cv.len(),
        |i: int|
            if (y0 <= crate::mc::byte_row(i, sw as int) && crate::mc::byte_row(i, sw as int) < y
                && x0 <= crate::mc::byte_col(i, sw as int) && crate::mc::byte_col(i, sw as int) < x1) || (
            crate::mc::byte_row(i, sw as int) == y && x0 <= crate::mc::byte_col(i, sw as int)
                && crate::mc::byte_col(i, sw as int) < x) {
                0u8
            } else {
                cv[i]
            },
    )
}

/// The same, for the disposal-3 copy: the bytes written take `src`'s values.
pub open spec fn copied_upto(
    dst: Seq<u8>,
    src: Seq<u8>,
    sw: u32,
    x0: int,
    x1: int,
    y0: int,
    y1: int,
    y: int,
    x: int,
) -> Seq<u8> {
    Seq::new(
        dst.len(),
        |i: int|
            if (y0 <= crate::mc::byte_row(i, sw as int) && crate::mc::byte_row(i, sw as int) < y
                && x0 <= crate::mc::byte_col(i, sw as int) && crate::mc::byte_col(i, sw as int) < x1) || (
            crate::mc::byte_row(i, sw as int) == y && x0 <= crate::mc::byte_col(i, sw as int)
                && crate::mc::byte_col(i, sw as int) < x) {
                src[i]
            } else {
                dst[i]
            },
    )
}

/// At the end of both loops the processed bytes are exactly the clipped
/// rectangle, so `cleared_upto` is `clear_rect`.
pub proof fn lemma_cleared_upto_full(cv: Seq<u8>, sw: u32, sh: u32, r: Rect)
    ensures
        cleared_upto(
            cv,
            sw,
            crate::mc::clip(r.spec_rect(), sw, sh).0,
            crate::mc::clip(r.spec_rect(), sw, sh).1,
            crate::mc::clip(r.spec_rect(), sw, sh).2,
            crate::mc::clip(r.spec_rect(), sw, sh).3,
            crate::mc::clip(r.spec_rect(), sw, sh).3,
            crate::mc::clip(r.spec_rect(), sw, sh).0,
        ) == crate::mc::clear_rect(cv, sw, sh, r.spec_rect()),
{
    reveal(crate::mc::clear_rect);
    reveal(crate::mc::in_clipped);
    assert forall|i: int| 0 <= i < cv.len() implies (
        if (crate::mc::clip(r.spec_rect(), sw, sh).2 <= crate::mc::byte_row(i, sw as int)
            && crate::mc::byte_row(i, sw as int) < crate::mc::clip(r.spec_rect(), sw, sh).3
            && crate::mc::clip(r.spec_rect(), sw, sh).0 <= crate::mc::byte_col(i, sw as int)
            && crate::mc::byte_col(i, sw as int) < crate::mc::clip(r.spec_rect(), sw, sh).1) || (
        crate::mc::byte_row(i, sw as int) == crate::mc::clip(r.spec_rect(), sw, sh).3
            && crate::mc::clip(r.spec_rect(), sw, sh).0 <= crate::mc::byte_col(i, sw as int)
            && crate::mc::byte_col(i, sw as int) < crate::mc::clip(r.spec_rect(), sw, sh).0) {
            0u8
        } else {
            cv[i]
        }
    ) == (if crate::mc::in_clipped(i, sw, sh, r.spec_rect()) { 0u8 } else { cv[i] }) by {
    }
}

/// The same for the copy.
pub proof fn lemma_copied_upto_full(dst: Seq<u8>, src: Seq<u8>, sw: u32, sh: u32, r: Rect)
    ensures
        copied_upto(
            dst,
            src,
            sw,
            crate::mc::clip(r.spec_rect(), sw, sh).0,
            crate::mc::clip(r.spec_rect(), sw, sh).1,
            crate::mc::clip(r.spec_rect(), sw, sh).2,
            crate::mc::clip(r.spec_rect(), sw, sh).3,
            crate::mc::clip(r.spec_rect(), sw, sh).3,
            crate::mc::clip(r.spec_rect(), sw, sh).0,
        ) == crate::mc::copy_rect(dst, src, sw, sh, r.spec_rect()),
{
    reveal(crate::mc::copy_rect);
    reveal(crate::mc::in_clipped);
}

/// The `usize` products of a row offset: with the canvas's byte count `len`
/// inside `MAX_OUTPUT`, `y·sw`, `4·(y·sw)`, `4·(y·sw) + 4·x` and `4·x` all fit
/// `usize`, so the executable's widening cannot wrap (THEOREMS'
/// `canvas_offset_bound`, in the `usize` domain the offsets are formed in).
pub proof fn lemma_usize_products(y: int, x: int, sw: int, sh: int, len: nat)
    requires
        0 <= y < sh,
        0 <= x <= sw,
        0 <= sw,
        0 <= sh,
        len == 4 * sw * sh,
        len <= max_output(),
    ensures
        y * sw <= crate::model::max_output_quarter(),
        y * sw + x <= crate::model::max_output_quarter(),
        y * sw + x < max_output(),
        4 * (y * sw + x) <= max_output(),
        4 * (y * sw + x) <= len,
        y * sw + x <= usize::MAX,
        4 * (y * sw + x) <= usize::MAX,
{
    crate::limits::caps_ok();
    assert(y * sw + x <= y * sw + sw);
    lemma_mul_is_distributive_add_other_way(sw, y, 1);
    assert(y * sw + sw == (y + 1) * sw);
    lemma_mul_inequality(y + 1, sh, sw);
    lemma_mul_is_commutative(sh, sw);
    assert((y + 1) * sw <= sw * sh);
    assert(y * sw + x <= sw * sh);
    assert(sw * sh <= crate::model::max_output_quarter()) by {
        assert(4 * (sw * sh) <= max_output()) by {
            lemma_mul_is_associative(4, sw, sh);
        }
    }
    assert(y * sw <= crate::model::max_output_quarter());
    lemma_mul_inequality(y * sw + x, sw * sh, 4);
    lemma_mul_is_associative(4, sw, sh);
    assert(4 * (y * sw + x) <= 4 * sw * sh);
    assert(y * sw + x <= crate::model::max_output_quarter());
    assert(y * sw + x < max_output());
    assert(4 * (y * sw + x) <= max_output());
    assert(4 * (y * sw + x) <= len);
    assert(usize::MAX >= max_output());
}

/// Every byte index of a row range is inside the canvas: `x <= sw`, `y < sh` give
/// `4·(y·sw + x) <= 4·sw·sh`.
pub proof fn lemma_row_bytes_inside(y: int, x: int, sw: int, sh: int)
    requires
        0 <= y < sh,
        0 <= x <= sw,
        0 <= sh,
    ensures
        4 * (y * sw + x) <= 4 * sw * sh,
{
    crate::limits::caps_ok();
    assert(y * sw + x <= y * sw + sw);
    lemma_mul_is_distributive_add_other_way(sw, y, 1);
    assert(y * sw + sw == (y + 1) * sw);
    lemma_mul_inequality(y + 1, sh, sw);
    lemma_mul_is_commutative(sh, sw);
    assert((y + 1) * sw <= sw * sh);
    lemma_mul_inequality(y * sw + x, sw * sh, 4);
    lemma_mul_is_associative(4, sw, sh);
}

/// An empty clipped rectangle writes nothing: `clear_rect` and `copy_rect` are
/// the identity, so a rectangle entirely beyond the screen is the Q18 regression.
pub proof fn lemma_clip_empty_identity(dst: Seq<u8>, src: Seq<u8>, sw: u32, sh: u32, r: Rect)
    requires
        crate::mc::clip_is_empty(sw, sh, r.spec_rect()),
    ensures
        crate::mc::clear_rect(dst, sw, sh, r.spec_rect()) == dst,
        crate::mc::copy_rect(dst, src, sw, sh, r.spec_rect()) == dst,
{
    reveal(crate::mc::clip_is_empty);
    reveal(crate::mc::in_clipped);
    reveal(crate::mc::clear_rect);
    reveal(crate::mc::copy_rect);
    assert forall|i: int| 0 <= i < dst.len() implies !#[trigger] crate::mc::in_clipped(
        i,
        sw,
        sh,
        r.spec_rect(),
    ) by {
    }
}

/// G12.3 disposal 2: set every pixel of the frame's rectangle, clipped to the
/// screen, to `(0,0,0,0)`. Writes nothing when the clipped rectangle is empty
/// (Q18): the row loop runs over no row.
#[allow(unused_variables)]
pub fn clear_rect(canvas: &mut [u8], sw: u32, sh: u32, r: Rect) -> (work: Ghost<nat>)
    requires
        canvas_ok(canvas, sw, sh),
        r.width >= 1,
        r.height >= 1,
        (r.left as int) + (r.width as int) <= 2 * max_dimension(),
        (r.top as int) + (r.height as int) <= 2 * max_dimension(),
    ensures
        work@ as int <= 2 * ((r.width as int) * (r.height as int)),
        final(canvas)@ == crate::mc::clear_rect(old(canvas)@, sw, sh, r.spec_rect()),
        final(canvas)@.len()==old(canvas)@.len(),
{
    let ghost mut steps = 0nat;
    let (x0, x1, y0, y1) = clip_of(r, sw, sh);
    let ghost cv0 = old(canvas)@;
    let ghost len = cv0.len();
    proof {
        reveal(canvas_ok);
        assert(x1 as int <= sw as int);
        assert(y1 as int <= sh as int);
    }
    if x1 <= x0 || y1 <= y0 {
        proof {
            reveal(crate::mc::clip_is_empty);
            lemma_clip_empty_identity(cv0, cv0, sw, sh, r);
        }
        return Ghost(steps);
    }
    let mut y: u32 = y0;
    while y < y1
        invariant
            y0 <= y <= y1,
            steps as int == (y as int-y0 as int) * (1 + x1 as int-x0 as int),
            x1 as int-x0 as int <= r.width as int,
            y1 as int-y0 as int <= r.height as int,
            r.width>=1, r.height>=1,
            y1 as int <= sh as int,
            x0 as int <= x1 as int,
            x1 as int <= sw as int,
            sw as int >= 1,
            sh as int >= 1,
            cv0.len() == 4 * (sw as int) * (sh as int),
            cv0.len() <= max_output(),
            canvas@.len() == len,
            canvas@
                == cleared_upto(cv0, sw, x0 as int, x1 as int, y0 as int, y1 as int, y as int, x0 as int),
        decreases y1 - y,
    {
        proof {
            lemma_usize_products(y as int, x1 as int, sw as int, sh as int, cv0.len());
        }
        let row_px: usize = (y as usize) * (sw as usize);
        let a: usize = 4 * (row_px + (x0 as usize));
        let z: usize = 4 * (row_px + (x1 as usize));
        proof {
            assert((row_px as int) == (y as int) * (sw as int));
            assert((a as int) == 4 * ((y as int) * (sw as int) + (x0 as int)));
            assert((z as int) == 4 * ((y as int) * (sw as int) + (x1 as int)));
            assert((z as int) <= cv0.len());
            assert((a as int) <= (z as int));
        }
        proof { steps = steps + 1; }
        let mut x: u32 = x0;
        while x < x1
            invariant
                x0 <= x <= x1,
                steps as int == (y as int-y0 as int) * (1+x1 as int-x0 as int) + 1 + x as int-x0 as int,
                y0<=y,
                x1 as int-x0 as int <= r.width as int,
                y1 as int-y0 as int <= r.height as int,
                r.width>=1, r.height>=1,
                y < y1,
                y1 as int <= sh as int,
                x1 as int <= sw as int,
                sw as int >= 1,
                sh as int >= 1,
                cv0.len() == 4 * (sw as int) * (sh as int),
                cv0.len() <= max_output(),
                canvas@.len() == len,
                canvas@
                    == cleared_upto(
                    cv0,
                    sw,
                    x0 as int,
                    x1 as int,
                    y0 as int,
                    y1 as int,
                    y as int,
                    x as int,
                ),
            decreases x1 - x,
        {
            proof {
                lemma_usize_products(y as int, (x + 1) as int, sw as int, sh as int, cv0.len());
                assert((y as int) * (sw as int) <= crate::model::max_output_quarter());
                lemma_mul_is_commutative(y as int, sw as int);
            }
            let rpx: usize = (y as usize) * (sw as usize);
            proof {
                assert((rpx as int) == (y as int) * (sw as int)) by {
                    lemma_mul_is_commutative(y as int, sw as int);
                }
            }
            let cell_off: usize = rpx + (x as usize);
            proof {
                assert((cell_off as int) == (rpx as int) + (x as int));
                assert(4 * (cell_off as int) <= max_output());
                assert(4 * (cell_off as int) <= usize::MAX);
            }
            let o: usize = 4 * cell_off;
            proof {
                assert((o as int) == 4 * ((y as int) * (sw as int) + (x as int)));
                assert((o as int) + 4 <= cv0.len());
                assert((x as int) < (x1 as int));
                let f = cleared_upto(
                    cv0,
                    sw,
                    x0 as int,
                    x1 as int,
                    y0 as int,
                    y1 as int,
                    y as int,
                    x as int,
                );
                let g = cleared_upto(
                    cv0,
                    sw,
                    x0 as int,
                    x1 as int,
                    y0 as int,
                    y1 as int,
                    y as int,
                    (x + 1) as int,
                );
                assert forall|i: int| 0 <= i < len implies canvas@.update(o as int, 0u8).update(
                    o as int + 1,
                    0u8,
                ).update(o as int + 2, 0u8).update(o as int + 3, 0u8)[i] == g[i] by {
                    reveal(crate::mc::byte_row);
                    reveal(crate::mc::byte_col);
                    assert(o as int == 4 * ((y as int) * (sw as int) + (x as int)));
                    lemma_cell_bytes(i, y as int, x as int, sw as int);
                }
            }
            canvas[o] = 0;
            canvas[o + 1] = 0;
            canvas[o + 2] = 0;
            canvas[o + 3] = 0;
            proof { steps = steps + 1; }
            x += 1;
        }
        proof {
            assert(steps as int == (y as int + 1-y0 as int) * (1+x1 as int-x0 as int)) by(nonlinear_arith)
                requires steps as int == (y as int-y0 as int) * (1+x1 as int-x0 as int) + 1+x1 as int-x0 as int;
        }
        y += 1;
    }
    proof {
        lemma_cleared_upto_full(cv0, sw, sh, r);
    }    proof {
        assert(steps as int <= 2 * ((r.width as int) * (r.height as int))) by(nonlinear_arith)
            requires steps as int == (y1 as int-y0 as int) * (1+x1 as int-x0 as int),
                0<=x1 as int-x0 as int<=r.width as int,0<=y1 as int-y0 as int<=r.height as int,
                r.width>=1,r.height>=1;
    }
    Ghost(steps)
}
/// The disposal-3 save (`canvas → Saved`) and restore (`Saved → canvas`): copy
/// the clipped rectangle's bytes from `src` to the same offsets of `dst`.
///
/// The preconditions are the borrow discipline SPEC G18's `Saved` region needs:
/// `src` is a different region from `dst`, and both have the canvas's shape. The
/// save is `copy_rect(saved, canvas, ...)`; the restore is the same call with the
#[allow(unused_variables)]
/// arguments swapped, which is why one proof covers both.
pub fn copy_rect(dst: &mut [u8], src: &[u8], sw: u32, sh: u32, r: Rect) -> (work: Ghost<nat>)
    requires
        canvas_ok(dst, sw, sh),
        src@.len() == 4 * (sw as int) * (sh as int),
        r.width >= 1,
        r.height >= 1,
        (r.left as int) + (r.width as int) <= 2 * max_dimension(),
        (r.top as int) + (r.height as int) <= 2 * max_dimension(),
    ensures
        work@ as int <= 2 * ((r.width as int) * (r.height as int)),
        final(dst)@ == crate::mc::copy_rect(old(dst)@, src@, sw, sh, r.spec_rect()),
        final(dst)@.len()==old(dst)@.len(),
{
    let ghost mut steps = 0nat;
    let (x0, x1, y0, y1) = clip_of(r, sw, sh);
    let ghost dv0 = old(dst)@;
    let ghost len = dv0.len();
    proof {
        reveal(canvas_ok);
        assert(x1 as int <= sw as int);
        assert(y1 as int <= sh as int);
    }
    if x1 <= x0 || y1 <= y0 {
        proof {
            reveal(crate::mc::clip_is_empty);
            lemma_clip_empty_identity(dv0, src@, sw, sh, r);
        }
        return Ghost(steps);
    }
    let mut y: u32 = y0;
    while y < y1
        invariant
            y0 <= y <= y1,
            steps as int == (y as int-y0 as int) * (1 + x1 as int-x0 as int),
            x1 as int-x0 as int <= r.width as int,
            y1 as int-y0 as int <= r.height as int,
            r.width>=1, r.height>=1,
            y1 as int <= sh as int,
            x0 as int <= x1 as int,
            x1 as int <= sw as int,
            sw as int >= 1,
            sh as int >= 1,
            dv0.len() == 4 * (sw as int) * (sh as int),
            src@.len() == 4 * (sw as int) * (sh as int),
            dv0.len() <= max_output(),
            dst@.len() == len,
            dst@
                == copied_upto(dv0, src@, sw, x0 as int, x1 as int, y0 as int, y1 as int, y as int, x0 as int),
        decreases y1 - y,
    {
        proof {
            lemma_usize_products(y as int, x1 as int, sw as int, sh as int, dv0.len());
        }
        let row_px: usize = (y as usize) * (sw as usize);
        let a: usize = 4 * (row_px + (x0 as usize));
        let z: usize = 4 * (row_px + (x1 as usize));
        proof {
            assert((row_px as int) == (y as int) * (sw as int));
            assert((a as int) == 4 * ((y as int) * (sw as int) + (x0 as int)));
            assert((z as int) == 4 * ((y as int) * (sw as int) + (x1 as int)));
            assert((z as int) <= dv0.len());
            assert((a as int) <= (z as int));
        }
        proof { steps = steps + 1; }
        let mut x: u32 = x0;
        while x < x1
            invariant
                x0 <= x <= x1,
                steps as int == (y as int-y0 as int) * (1+x1 as int-x0 as int) + 1 + x as int-x0 as int,
                y0<=y,
                x1 as int-x0 as int <= r.width as int,
                y1 as int-y0 as int <= r.height as int,
                r.width>=1, r.height>=1,
                y < y1,
                y1 as int <= sh as int,
                x1 as int <= sw as int,
                sw as int >= 1,
                sh as int >= 1,
                dv0.len() == 4 * (sw as int) * (sh as int),
                src@.len() == 4 * (sw as int) * (sh as int),
                dv0.len() <= max_output(),
                dst@.len() == len,
                dst@
                    == copied_upto(
                    dv0,
                    src@,
                    sw,
                    x0 as int,
                    x1 as int,
                    y0 as int,
                    y1 as int,
                    y as int,
                    x as int,
                ),
            decreases x1 - x,
        {
            proof {
                lemma_usize_products(y as int, (x + 1) as int, sw as int, sh as int, dv0.len());
                assert((y as int) * (sw as int) <= crate::model::max_output_quarter());
                lemma_mul_is_commutative(y as int, sw as int);
            }
            let rpx: usize = (y as usize) * (sw as usize);
            proof {
                assert((rpx as int) == (y as int) * (sw as int)) by {
                    lemma_mul_is_commutative(y as int, sw as int);
                }
            }
            let cell_off: usize = rpx + (x as usize);
            proof {
                assert((cell_off as int) == (rpx as int) + (x as int));
                assert(4 * (cell_off as int) <= max_output());
                assert(4 * (cell_off as int) <= usize::MAX);
            }
            let o: usize = 4 * cell_off;
            proof {
                assert((o as int) == 4 * ((y as int) * (sw as int) + (x as int)));
                assert((o as int) + 4 <= dv0.len());
                assert((x as int) < (x1 as int));
                let g = copied_upto(
                    dv0,
                    src@,
                    sw,
                    x0 as int,
                    x1 as int,
                    y0 as int,
                    y1 as int,
                    y as int,
                    (x + 1) as int,
                );
                assert forall|i: int| 0 <= i < len implies dst@.update(o as int, src@[o as int]).update(
                    o as int + 1,
                    src@[o as int + 1],
                ).update(o as int + 2, src@[o as int + 2]).update(o as int + 3, src@[o as int + 3])[i]
                    == g[i] by {
                    reveal(crate::mc::byte_row);
                    reveal(crate::mc::byte_col);
                    assert(o as int == 4 * ((y as int) * (sw as int) + (x as int)));
                    lemma_cell_bytes(i, y as int, x as int, sw as int);
                }
            }
            dst[o] = src[o];
            dst[o + 1] = src[o + 1];
            dst[o + 2] = src[o + 2];
            dst[o + 3] = src[o + 3];
            proof { steps = steps + 1; }
            x += 1;
        }
        proof {
            assert(steps as int == (y as int + 1-y0 as int) * (1+x1 as int-x0 as int)) by(nonlinear_arith)
                requires steps as int == (y as int-y0 as int) * (1+x1 as int-x0 as int) + 1+x1 as int-x0 as int;
        }
        y += 1;
    }
    proof {
        lemma_copied_upto_full(dv0, src@, sw, sh, r);
    }    proof {
        assert(steps as int <= 2 * ((r.width as int) * (r.height as int))) by(nonlinear_arith)
            requires steps as int == (y1 as int-y0 as int) * (1+x1 as int-x0 as int),
                0<=x1 as int-x0 as int<=r.width as int,0<=y1 as int-y0 as int<=r.height as int,
                r.width>=1,r.height>=1;
    }
    Ghost(steps)
}


// ---------------------------------------------------------------------------
// G12.3's disposal of the previous frame, and G12.2's full frame transition.
// ---------------------------------------------------------------------------

/// G12.3: apply the previous frame's disposal before the next frame is drawn.
/// Disposals 0 and 1 leave the canvas; 2 clears the (clipped) rectangle; 3
/// restores the `Saved` rows. `prev` is `None` before the first frame and after a
/// frame with no Graphic Control (whose disposal is 0).
pub fn dispose(
    canvas: &mut [u8],
    saved: &[u8],
    sw: u32,
    sh: u32,
    prev: Option<Frame>,
) -> (u: ())
    requires
        canvas_ok(canvas, sw, sh),
        saved@.len() == 4 * (sw as int) * (sh as int),
        match prev {
            None => true,
            Some(p) => frame_ok(p),
        },
    ensures
        final(canvas)@
            == crate::mc::dispose_prev(
            old(canvas)@,
            saved@,
            sw,
            sh,
            match prev {
                None => None,
                Some(p) => Some(p.spec_frame()),
            },
        ),
{
    match prev {
        None => {
            proof {
                reveal(crate::mc::dispose_prev);
            }
        }
        Some(p) => {
            if p.disposal == 2 {
                clear_rect(canvas, sw, sh, p.rect);
                proof {
                    reveal(crate::mc::dispose_prev);
                }
            } else if p.disposal == 3 {
                copy_rect(canvas, saved, sw, sh, p.rect);
                proof {
                    reveal(crate::mc::dispose_prev);
                }
            } else {
                proof {
                    reveal(crate::mc::dispose_prev);
                }
            }
        }
    }
}

/// G12.2's save of the frame's rectangle into `Saved`, when its disposal is 3.
pub fn save_rect(saved: &mut [u8], canvas: &[u8], sw: u32, sh: u32, fr: Frame) -> (u: ())
    requires
        canvas_ok(saved, sw, sh),
        canvas@.len() == 4 * (sw as int) * (sh as int),
        frame_ok(fr),
    ensures
        final(saved)@
            == if fr.disposal == 3 {
            crate::mc::copy_rect(old(saved)@, canvas@, sw, sh, fr.rect.spec_rect())
        } else {
            old(saved)@
        },
{
    if fr.disposal == 3 {
        copy_rect(saved, canvas, sw, sh, fr.rect);
    }
}

/// G12.2's drawing loop for one frame: every index of `idx` in raster order.
pub fn draw_all(canvas: &mut [u8], idx: &[u8], fr: Frame, table: &[u8], sw: u32, sh: u32) -> (r: Result<
    (),
    ComposeError,
>)
    requires
        canvas_ok(canvas, sw, sh),
        frame_ok(fr),
        table_ok(table, fr),
        idx@.len() == (fr.rect.width as int) * (fr.rect.height as int),
    ensures
        final(canvas)@.len() == old(canvas)@.len(),
        final(canvas)@ == crate::mc::draw_run(old(canvas)@, fr.spec_frame(), table@, sw, sh, idx@).0,
        match r {
            Ok(()) => crate::mc::draw_run(old(canvas)@, fr.spec_frame(), table@, sw, sh, idx@).1
                is None,
            Err(e) => crate::mc::draw_run(old(canvas)@, fr.spec_frame(), table@, sw, sh, idx@).1
                == Some(e),
        },
{
    let mut st = new_state();
    let mut q: usize = 0;
    proof {
        reveal(crate::mc::draw_run);
        reveal(crate::mc::draw_run_at);
        assert(old(canvas)@ == old(canvas)@);
    }
    while q < idx.len()
        invariant
            q <= idx.len(),
            idx@.len() == (fr.rect.width as int) * (fr.rect.height as int),
            st.q as int == q as int,
            canvas_ok(canvas, sw, sh),
            frame_ok(fr),
            table_ok(table, fr),
            old(canvas)@.len() == 4 * (sw as int) * (sh as int),
            canvas@.len() == 4 * (sw as int) * (sh as int),
            canvas@.len() == old(canvas)@.len(),
            // The fold identity the loop maintains: the whole run from the start is
            // the run from the current canvas and position.
            crate::mc::draw_run(old(canvas)@, fr.spec_frame(), table@, sw, sh, idx@) == crate::mc::draw_run_at(
                canvas@,
                fr.spec_frame(),
                table@,
                sw,
                sh,
                idx@,
                q as int,
            ),
        decreases idx.len() - q,
    {
        proof {
            reveal(crate::mc::draw_run_at);
            reveal(crate::mc::draw_run);
        }
        let ghost pre = canvas@;
        proof {
            reveal(crate::mc::draw_run_at);
            reveal(crate::mc::draw_run);
            assert((q as int) < idx@.len());
            assert(crate::mc::draw_run(old(canvas)@, fr.spec_frame(), table@, sw, sh, idx@) == crate::mc::draw_run_at(
                pre,
                fr.spec_frame(),
                table@,
                sw,
                sh,
                idx@,
                q as int,
            ));
        }
        let rr = draw_index(canvas, &mut st, fr, table, sw, sh, idx[q]);
        let ghost post = canvas@;
        proof {
            reveal(crate::mc::draw_run_at);
            reveal(crate::mc::draw_run);
            crate::mc::lemma_draw_run_at_step(pre, fr.spec_frame(), table@, sw, sh, idx@, q as int);
        }
        match rr {
            Err(e) => {
                proof {
                    let step = draw_step(pre, fr.spec_frame(), table@, sw, sh, q as int, idx@[q as int]);
                    assert(step == Err(e));
                    assert(canvas@ == pre);
                    assert(crate::mc::draw_run_at(pre, fr.spec_frame(), table@, sw, sh, idx@, q as int)
                        == (pre, Option::<ComposeError>::Some(e))) by {
                        reveal(crate::mc::draw_run_at);
                    }
                    assert(crate::mc::draw_run(old(canvas)@, fr.spec_frame(), table@, sw, sh, idx@).0 == pre);
                    assert(crate::mc::draw_run(old(canvas)@, fr.spec_frame(), table@, sw, sh, idx@).1
                        == Option::<ComposeError>::Some(e));
                    assert(final(canvas)@ == canvas@);
                    assert(final(canvas)@ == crate::mc::draw_run(old(canvas)@, fr.spec_frame(), table@, sw, sh, idx@).0) by {
                        reveal(crate::mc::draw_run);
                    }
                    assert(canvas@.len() == 4 * (sw as int) * (sh as int));
                    assert(old(canvas)@ == old(canvas)@);
                    assert(final(canvas)@ == crate::mc::draw_run(old(canvas)@, fr.spec_frame(), table@, sw, sh, idx@).0);
                    assert(crate::mc::draw_run(old(canvas)@, fr.spec_frame(), table@, sw, sh, idx@).1
                        == Option::<ComposeError>::Some(e));
                    assert(final(canvas)@.len() == old(canvas)@.len());
                }
                return Err(e);
            }
            Ok(()) => {}
        }
        proof {
            let step = draw_step(pre, fr.spec_frame(), table@, sw, sh, q as int, idx@[q as int]);
            assert(step == Ok(post));
            assert(crate::mc::draw_run_at(pre, fr.spec_frame(), table@, sw, sh, idx@, q as int)
                == crate::mc::draw_run_at(
                post,
                fr.spec_frame(),
                table@,
                sw,
                sh,
                idx@,
                (q + 1) as int,
            ));
            assert(crate::mc::draw_run(old(canvas)@, fr.spec_frame(), table@, sw, sh, idx@)
                == crate::mc::draw_run_at(
                post,
                fr.spec_frame(),
                table@,
                sw,
                sh,
                idx@,
                (q + 1) as int,
            ));
        }
        q += 1;
        proof {
            reveal(crate::mc::draw_run_at);
            reveal(crate::mc::draw_run);
        }
    }
    proof {
        reveal(crate::mc::draw_run_at);
        reveal(crate::mc::draw_run);
        assert(crate::mc::draw_run_at(canvas@, fr.spec_frame(), table@, sw, sh, idx@, q as int)
            == (canvas@, Option::<ComposeError>::None));
    }
    Ok(())
}

/// `MC`: the whole frame transition of THEOREMS.md. Applies the previous frame's
/// disposal (G12.3), saves this frame's rectangle into `Saved` when its disposal
/// is 3 (G12.2, before its first write), then draws the frame's index sequence
/// through `IR` and the active table (G12.2).
///
/// The `Saved` reservation of SPEC G18 belongs to the caller (the call that draws
/// the first disposal-3 frame); this function receives both buffers.
pub fn composite(
    canvas: &mut [u8],
    saved: &mut [u8],
    idx: &[u8],
    fr: Frame,
    table: &[u8],
    sw: u32,
    sh: u32,
    prev: Option<Frame>,
) -> (r: Result<(), ComposeError>)
    requires
        canvas_ok(canvas, sw, sh),
        canvas_ok(saved, sw, sh),
        frame_ok(fr),
        table_ok(table, fr),
        idx@.len() == (fr.rect.width as int) * (fr.rect.height as int),
        match prev {
            None => true,
            Some(p) => frame_ok(p),
        },
    ensures
        final(canvas)@.len() == old(canvas)@.len(),
        final(saved)@.len() == old(saved)@.len(),
        final(canvas)@
            == crate::mc::mc(
            old(canvas)@,
            old(saved)@,
            sw,
            sh,
            match prev {
                None => None,
                Some(p) => Some(p.spec_frame()),
            },
            fr.spec_frame(),
            table@,
            idx@,
        ).0,
        final(saved)@
            == crate::mc::mc(
            old(canvas)@,
            old(saved)@,
            sw,
            sh,
            match prev {
                None => None,
                Some(p) => Some(p.spec_frame()),
            },
            fr.spec_frame(),
            table@,
            idx@,
        ).1,
        match r {
            Ok(()) => crate::mc::mc(
                old(canvas)@,
                old(saved)@,
                sw,
                sh,
                match prev {
                    None => None,
                    Some(p) => Some(p.spec_frame()),
                },
                fr.spec_frame(),
                table@,
                idx@,
            ).2 is None,
            Err(e) => crate::mc::mc(
                old(canvas)@,
                old(saved)@,
                sw,
                sh,
                match prev {
                    None => None,
                    Some(p) => Some(p.spec_frame()),
                },
                fr.spec_frame(),
                table@,
                idx@,
            ).2 == Some(e),
        },
{
    proof {
        reveal(crate::mc::mc);
    }
    let ghost cv0 = old(canvas)@;
    let ghost sv0 = old(saved)@;
    let ghost pv = match prev {
        None => None,
        Some(p) => Some(p.spec_frame()),
    };
    dispose(canvas, saved, sw, sh, prev);
    proof {
        reveal(crate::mc::mc_disposed);
        assert(canvas@ == crate::mc::mc_disposed(cv0, sv0, sw, sh, pv));
    }
    let ghost disposed = canvas@;
    save_rect(saved, canvas, sw, sh, fr);
    proof {
        reveal(crate::mc::mc_saved);
        assert(saved@ == crate::mc::mc_saved(cv0, sv0, sw, sh, pv, fr.spec_frame()));
    }
    let ghost saved2 = saved@;
    proof {
        reveal(crate::mc::mc);
        reveal(crate::mc::mc_from);
    }
    proof {
        reveal(crate::mc::mc);
        reveal(crate::mc::mc_from);
        reveal(crate::mc::mc_disposed);
        reveal(crate::mc::mc_saved);
        reveal(crate::mc::draw_run);
        lemma_mc_unfold(cv0, sv0, sw, sh, pv, fr.spec_frame(), table@, idx@);
    }
    let res = draw_all(canvas, idx, fr, table, sw, sh);
    proof {
        reveal(crate::mc::mc);
        reveal(crate::mc::mc_from);
        reveal(crate::mc::mc_disposed);
        reveal(crate::mc::mc_saved);
        reveal(crate::mc::draw_run);
        lemma_mc_unfold(cv0, sv0, sw, sh, pv, fr.spec_frame(), table@, idx@);
        match res {
            Ok(()) => {}
            Err(_) => {}
        }
    }
    res
}

// ---------------------------------------------------------------------------
// The output pixel clause and the frame-sequence statement of T13/T7/T8.
// ---------------------------------------------------------------------------

/// The G12.2 admissibility of one emitted index: the transparent one (which
/// writes nothing, even at `>= C`), or one below `C` (which is looked up).
pub open spec fn index_ok(fr: FrameSpec, v: u8) -> bool {
    fr.transparent == Some(v) || (v as int) < fr.c
}

/// The initial canvas (G12.1) satisfies the output clause: every pixel is
/// `(0,0,0,0)`.
pub proof fn lemma_initial_pixels_well_formed(sw: u32, sh: u32, ts: Set<Seq<u8>>)
    requires
        1 <= sw <= max_dimension(),
        1 <= sh <= max_dimension(),
    ensures
        pixels_well_formed(Seq::new((4 * (sw as int) * (sh as int)) as nat, |i: int| 0u8), ts),
{
    reveal(pixels_well_formed);
    reveal(pixel_is_clear);
    let cv = Seq::new((4 * (sw as int) * (sh as int)) as nat, |j: int| 0u8);
    assert forall|i: int| 0 <= i < cv.len() / 4 implies #[trigger] pixel_is_clear(cv, i) by {
    }
}

/// `write_pixel`'s effect on the output clause: the four bytes written at a cell
/// are the entry's opaque word, so that cell satisfies the clause afterwards, and
/// every other cell keeps its property from the canvas before the write.
pub proof fn lemma_write_pixel_pixels(
    cv: Seq<u8>,
    table: Seq<u8>,
    sw: u32,
    sh: u32,
    cell: int,
    v: int,
    fr: FrameSpec,
    q: int,
    ts: Set<Seq<u8>>,
)
    requires
        pixels_well_formed(cv, ts),
        ts.contains(table),
        0 <= v,
        3 * v + 2 < table.len(),
        4 * cell + 3 < cv.len(),
        0 <= cell,
        target_cell(fr, sw, sh, q) == Some(cell),
    ensures
        pixels_well_formed(write_pixel(cv, 4 * cell, table, v), ts),
{
    reveal(pixels_well_formed);
    reveal(pixel_is_entry);
    reveal(pixel_is_clear);
    reveal(write_pixel);
    reveal(pixel_word);
    let cv2 = write_pixel(cv, 4 * cell, table, v);
    assert forall|i: int| 0 <= i < cv2.len() / 4 implies ({
        ||| (exists|tb: Seq<u8>, w: int|
            ts.contains(tb) && 0 <= w && 3 * w + 2 < tb.len() && #[trigger] pixel_is_entry(cv2, tb, i, w))
        ||| #[trigger] pixel_is_clear(cv2, i)
    }) by {
        if i == cell {
            assert(pixel_is_entry(cv2, table, i, v));
        } else {
            assert(cv2[4 * i] == cv[4 * i] && cv2[4 * i + 1] == cv[4 * i + 1] && cv2[4 * i + 2]
                == cv[4 * i + 2] && cv2[4 * i + 3] == cv[4 * i + 3]) by {
                assert(4 * cell <= 4 * i || 4 * cell >= 4 * i + 4) by {
                    if 4 * cell > 4 * i && 4 * cell < 4 * i + 4 {
                        assert(false);
                    }
                }
            }
            if exists|tb: Seq<u8>, w: int|
                ts.contains(tb) && 0 <= w && 3 * w + 2 < tb.len() && pixel_is_entry(cv, tb, i, w) {
                let (tb, w) = choose|tb: Seq<u8>, w: int|
                    ts.contains(tb) && 0 <= w && 3 * w + 2 < tb.len() && pixel_is_entry(cv, tb, i, w);
                assert(pixel_is_entry(cv, tb, i, w));
                assert(pixel_is_entry(cv2, tb, i, w));
            }
            if pixel_is_clear(cv, i) {
                assert(pixel_is_clear(cv2, i));
            }
        }
    }
}

/// The cell `target_cell` names is `target_y·sw + target_x`, with both
/// coordinates inside the screen: the executable's clipping leaves only such
/// cells, and `frame_ok` plus the frames-inside-screen precondition bound them.
pub proof fn lemma_target_cell_shape(fr: FrameSpec, sw: u32, sh: u32, q: int, cell: int)
    requires
        target_cell(fr, sw, sh, q) == Some(cell),
        1 <= fr.rect.width,
        1 <= fr.rect.height,
        fr.rect.height as int <= max_dimension(),
        0 <= fr.rect.left,
        0 <= fr.rect.top,
        (fr.rect.left as int) + (fr.rect.width as int) <= sw as int,
        (fr.rect.top as int) + (fr.rect.height as int) <= sh as int,
        0 <= q < (fr.rect.width as int) * (fr.rect.height as int),
    ensures
        cell == target_y(fr, q) * (sw as int) + target_x(fr, q),
        0 <= target_x(fr, q) < sw as int,
        0 <= target_y(fr, q) < sh as int,
{
    reveal(target_cell);
    reveal(target_x);
    reveal(target_y);
    lemma_mod_bound(q, fr.rect.width as int);
    assert(0 <= q % (fr.rect.width as int) < fr.rect.width as int);
    assert(q / (fr.rect.width as int) < fr.rect.height as int) by {
        if q / (fr.rect.width as int) >= fr.rect.height as int {
            lemma_mul_inequality(fr.rect.height as int, q / (fr.rect.width as int), fr.rect.width as int);
            lemma_mul_is_commutative(fr.rect.height as int, fr.rect.width as int);
            lemma_div_mul_le(q, fr.rect.width as int);
            assert(q >= (fr.rect.width as int) * (fr.rect.height as int));
        }
    }
    if fr.interlaced {
        lemma_ir_bijective(fr.rect.height as int);
    }
}

/// Every cell inside an `sw × sh` screen has its four bytes inside a canvas of
/// `4·sw·sh` bytes (THEOREMS' `canvas_offset_bound`).
pub proof fn lemma_cell_in_canvas(y: int, x: int, sw: int, sh: int, len: nat)
    requires
        0 <= y < sh,
        0 <= x < sw,
        0 <= sw,
        0 <= sh,
        len == 4 * sw * sh,
    ensures
        4 * (y * sw + x) + 3 < len,
{
    lemma_cell_offset_in_bounds(y, x, sw, sh);
}

/// `draw_step`'s effect on the output clause: the transparent index writes
/// nothing, a clipped position writes nothing, and a written cell takes the
/// table's entry, whose table is in `ts`.
///
/// The geometry is the executable's: `frame_ok` plus the frames-inside-screen and
/// index-range facts that `draw_index` carries.
pub proof fn lemma_draw_step_pixels(
    cv: Seq<u8>,
    fr: FrameSpec,
    table: Seq<u8>,
    sw: u32,
    sh: u32,
    q: int,
    v: u8,
    ts: Set<Seq<u8>>,
)
    requires
        pixels_well_formed(cv, ts),
        ts.contains(table),
        index_ok(fr, v),
        3 * (v as int) + 2 < table.len(),
        0 <= q < (fr.rect.width as int) * (fr.rect.height as int),
        1 <= fr.rect.width,
        1 <= fr.rect.height,
        fr.rect.height as int <= max_dimension(),
        0 <= fr.rect.left,
        0 <= fr.rect.top,
        (fr.rect.left as int) + (fr.rect.width as int) <= sw as int,
        (fr.rect.top as int) + (fr.rect.height as int) <= sh as int,
        cv.len() == 4 * (sw as int) * (sh as int),
        1 <= sw,
        1 <= sh,
    ensures
        match draw_step(cv, fr, table, sw, sh, q, v) {
            Ok(cv2) => pixels_well_formed(cv2, ts),
            Err(_) => true,
        },
{
    reveal(draw_step);
    reveal(index_ok);
    if fr.transparent == Some(v) {
        assert(draw_step(cv, fr, table, sw, sh, q, v) == Ok(cv));
    } else {
        assert((v as int) < fr.c);
        match target_cell(fr, sw, sh, q) {
            None => {
                assert(draw_step(cv, fr, table, sw, sh, q, v) == Ok(cv));
            }
            Some(cell) => {
                lemma_target_cell_shape(fr, sw, sh, q, cell);
                assert(0 <= target_x(fr, q) < sw as int);
                assert(0 <= target_y(fr, q) < sh as int);
                assert(cell == target_y(fr, q) * (sw as int) + target_x(fr, q));
                lemma_cell_in_canvas(target_y(fr, q), target_x(fr, q), sw as int, sh as int, cv.len());
                assert(4 * cell + 3 < cv.len());
                lemma_write_pixel_pixels(cv, table, sw, sh, cell, v as int, fr, q, ts);
            }
        }
    }
}

/// `draw_run_at` at the end of a sequence adds nothing.
pub proof fn lemma_draw_run_at_done(
    cv: Seq<u8>,
    fr: FrameSpec,
    table: Seq<u8>,
    sw: u32,
    sh: u32,
    idx: Seq<u8>,
    q: int,
)
    requires
        q >= idx.len() as int,
    ensures
        draw_run_at(cv, fr, table, sw, sh, idx, q) == (cv, Option::<ComposeError>::None),
{
    reveal(draw_run_at);
}

/// `draw_run_at`'s step: one index, then the rest of the sequence.
pub proof fn lemma_draw_run_at_unfold(
    cv: Seq<u8>,
    fr: FrameSpec,
    table: Seq<u8>,
    sw: u32,
    sh: u32,
    idx: Seq<u8>,
    q: int,
)
    requires
        0 <= q < idx.len() as int,
    ensures
        match draw_step(cv, fr, table, sw, sh, q, idx[q]) {
            Err(e) => draw_run_at(cv, fr, table, sw, sh, idx, q) == (cv, Option::<ComposeError>::Some(e)),
            Ok(cv2) => draw_run_at(cv, fr, table, sw, sh, idx, q) == draw_run_at(
                cv2,
                fr,
                table,
                sw,
                sh,
                idx,
                q + 1,
            ),
        },
{
    reveal(draw_run_at);
}

/// The drawing from position `q` preserves the output clause: the
/// position-indexed form of T13's pixel clause, which the executable's drawing
/// loop can use directly and which gives the whole-frame statement at `q = 0`.
pub proof fn lemma_draw_pixels_at(
    cv: Seq<u8>,
    fr: FrameSpec,
    table: Seq<u8>,
    sw: u32,
    sh: u32,
    idx: Seq<u8>,
    q: int,
    ts: Set<Seq<u8>>,
)
    requires
        pixels_well_formed(cv, ts),
        ts.contains(table),
        forall|j: int| q <= j < idx.len() ==> #[trigger] index_ok(fr, idx[j]),
        forall|j: int| q <= j < idx.len() ==> 3 * (idx[j] as int) + 2 < table.len(),
        1 <= fr.rect.width,
        1 <= fr.rect.height,
        fr.rect.height as int <= max_dimension(),
        0 <= fr.rect.left,
        0 <= fr.rect.top,
        (fr.rect.left as int) + (fr.rect.width as int) <= sw as int,
        (fr.rect.top as int) + (fr.rect.height as int) <= sh as int,
        cv.len() == 4 * (sw as int) * (sh as int),
        1 <= sw,
        1 <= sh,
        idx.len() <= (fr.rect.width as int) * (fr.rect.height as int),
        0 <= q <= idx.len() as int,
    ensures
        draw_run_at(cv, fr, table, sw, sh, idx, q).1 is None
            ==> pixels_well_formed(draw_run_at(cv, fr, table, sw, sh, idx, q).0, ts),
    decreases (idx.len() as int - q) as nat,
{
    reveal(pixels_well_formed);
    reveal(draw_run_at);
    if q >= idx.len() as int {
        lemma_draw_run_at_done(cv, fr, table, sw, sh, idx, q);
    } else {
        lemma_draw_run_at_unfold(cv, fr, table, sw, sh, idx, q);
        match draw_step(cv, fr, table, sw, sh, q, idx[q]) {
            Err(_) => {}
            Ok(cv2) => {
                assert(q < (fr.rect.width as int) * (fr.rect.height as int)) by {
                    assert(idx.len() <= (fr.rect.width as int) * (fr.rect.height as int));
                }
                lemma_draw_step_pixels(cv, fr, table, sw, sh, q, idx[q], ts);
                lemma_draw_pixels_at(cv2, fr, table, sw, sh, idx, q + 1, ts);
            }
        }
    }
}

/// The whole-frame statement: `draw_run` from position 0 (T13's pixel clause).
pub proof fn lemma_draw_run_pixels(
    cv: Seq<u8>,
    fr: FrameSpec,
    table: Seq<u8>,
    sw: u32,
    sh: u32,
    idx: Seq<u8>,
    ts: Set<Seq<u8>>,
)
    requires
        pixels_well_formed(cv, ts),
        ts.contains(table),
        forall|j: int| 0 <= j < idx.len() ==> #[trigger] index_ok(fr, idx[j]),
        forall|j: int| 0 <= j < idx.len() ==> 3 * (idx[j] as int) + 2 < table.len(),
        1 <= fr.rect.width,
        1 <= fr.rect.height,
        fr.rect.height as int <= max_dimension(),
        0 <= fr.rect.left,
        0 <= fr.rect.top,
        (fr.rect.left as int) + (fr.rect.width as int) <= sw as int,
        (fr.rect.top as int) + (fr.rect.height as int) <= sh as int,
        cv.len() == 4 * (sw as int) * (sh as int),
        1 <= sw,
        1 <= sh,
        idx.len() <= (fr.rect.width as int) * (fr.rect.height as int),
    ensures
        draw_run(cv, fr, table, sw, sh, idx).1 is None
            ==> pixels_well_formed(draw_run(cv, fr, table, sw, sh, idx).0, ts),
{
    reveal(draw_run);
    reveal(draw_run_at);
    crate::mc::lemma_draw_run_is_at(cv, fr, table, sw, sh, idx);
    lemma_draw_pixels_at(cv, fr, table, sw, sh, idx, 0, ts);
}


// ---------------------------------------------------------------------------
// T8/T7's frame sequence: the driver the M5 iterator calls once per frame.
// ---------------------------------------------------------------------------

/// One `next_frame` step: apply the previous frame's disposal, save this frame's
/// rectangle when its disposal is 3, draw the frame's indices. Returns the first
/// `ColorIndexOutOfRange`; on an error the canvas and `Saved` are unchanged
/// (G12.4: no partial frame).
///
/// This is `composite` under another name, so the call sites of the M5 iterator
/// can read as the frame loop it is.
pub fn next_frame(
    canvas: &mut [u8],
    saved: &mut [u8],
    idx: &[u8],
    fr: Frame,
    table: &[u8],
    sw: u32,
    sh: u32,
    prev: Option<Frame>,
) -> (r: Result<(), ComposeError>)
    requires
        canvas_ok(canvas, sw, sh),
        canvas_ok(saved, sw, sh),
        frame_ok(fr),
        table_ok(table, fr),
        idx@.len() == (fr.rect.width as int) * (fr.rect.height as int),
        match prev {
            None => true,
            Some(p) => frame_ok(p),
        },
    ensures
        final(canvas)@.len() == old(canvas)@.len(),
        final(saved)@.len() == old(saved)@.len(),
        final(canvas)@
            == mc(
            old(canvas)@,
            old(saved)@,
            sw,
            sh,
            match prev {
                None => None,
                Some(p) => Some(p.spec_frame()),
            },
            fr.spec_frame(),
            table@,
            idx@,
        ).0,
        final(saved)@
            == mc(
            old(canvas)@,
            old(saved)@,
            sw,
            sh,
            match prev {
                None => None,
                Some(p) => Some(p.spec_frame()),
            },
            fr.spec_frame(),
            table@,
            idx@,
        ).1,
        match r {
            Ok(()) => mc(
                old(canvas)@,
                old(saved)@,
                sw,
                sh,
                match prev {
                    None => None,
                    Some(p) => Some(p.spec_frame()),
                },
                fr.spec_frame(),
                table@,
                idx@,
            ).2 is None,
            Err(e) => mc(
                old(canvas)@,
                old(saved)@,
                sw,
                sh,
                match prev {
                    None => None,
                    Some(p) => Some(p.spec_frame()),
                },
                fr.spec_frame(),
                table@,
                idx@,
            ).2 == Some(e),
        },
{
    composite(canvas, saved, idx, fr, table, sw, sh, prev)
}

// ---------------------------------------------------------------------------
// Under `Clip`: a position is written iff its unnarrowed coordinates are inside
// the screen (T5's clipped clause), and a rectangle beyond the screen writes
// nothing (Q18).
// ---------------------------------------------------------------------------

/// `frame_bounds = Clip`: the write of position `q` is skipped exactly when
/// `target_x` or `target_y` is at or beyond the screen edge, i.e. when
/// `target_cell` is `None`.
pub proof fn lemma_clip_skips_outside(fr: FrameSpec, sw: u32, sh: u32, q: int)
    ensures
        target_cell(fr, sw, sh, q) is None <==> (target_x(fr, q) >= sw as int || target_y(fr, q)
            >= sh as int),
        target_cell(fr, sw, sh, q) is Some <==> (target_x(fr, q) < sw as int && target_y(fr, q)
            < sh as int),
{
    reveal(target_cell);
}

/// One pass of the walk: `ir_at(r, h, k)` is non-negative for a non-negative
/// residual `r`, whatever `r`'s relation to the pass lengths. The four pass cases
/// are literal `k` values, so the model's `if k == …` dispatch reduces.
pub proof fn lemma_ir_at_nonneg_step(r: int, h: int, k: int)
    requires
        1 <= h <= max_dimension(),
        0 <= k <= 3,
        0 <= r,
    ensures
        0 <= crate::mc::ir_at(r, h, k),
    decreases (4 - k) as nat,
{
    reveal(crate::mc::ir_at);
    reveal(crate::mc::pass_len);
    reveal(crate::mc::pass_start);
    reveal(crate::mc::pass_step);
    assert(crate::mc::pass_start(0) == 0);
    assert(crate::mc::pass_start(1) == 4);
    assert(crate::mc::pass_start(2) == 2);
    assert(crate::mc::pass_step(0) == 8);
    assert(crate::mc::pass_step(1) == 8);
    assert(crate::mc::pass_step(2) == 4);
    if k == 0 {
        if r < crate::mc::pass_len(0, h) {
            assert(crate::mc::ir_at(r, h, 0) == r * 8);
            lemma_mul_inequality(0, r, 8);
        } else {
            lemma_ir_at_nonneg_step(r - crate::mc::pass_len(0, h), h, 1);
            assert(r - crate::mc::pass_len(0, h) >= 0);
        }
    } else if k == 1 {
        if r < crate::mc::pass_len(1, h) {
            assert(crate::mc::ir_at(r, h, 1) == 4 + r * 8);
            lemma_mul_inequality(0, r, 8);
        } else {
            lemma_ir_at_nonneg_step(r - crate::mc::pass_len(1, h), h, 2);
            assert(r - crate::mc::pass_len(1, h) >= 0);
        }
    } else if k == 2 {
        if r < crate::mc::pass_len(2, h) {
            assert(crate::mc::ir_at(r, h, 2) == 2 + r * 4);
            lemma_mul_inequality(0, r, 4);
        } else {
            lemma_ir_at_nonneg_step(r - crate::mc::pass_len(2, h), h, 3);
            assert(r - crate::mc::pass_len(2, h) >= 0);
        }
    } else {
        assert(k == 3);
        assert(crate::mc::pass_start(3) == 1);
        assert(crate::mc::pass_step(3) == 2);
        if r < crate::mc::pass_len(3, h) {
            assert(crate::mc::ir_at(r, h, 3) == 1 + r * 2);
            lemma_mul_inequality(0, r, 2);
        } else {
            assert(crate::mc::ir_at(r, h, 3) == crate::mc::ir_at(
                r - crate::mc::pass_len(3, h),
                h,
                4,
            ));
            assert(crate::mc::ir_at(r - crate::mc::pass_len(3, h), h, 4) == 0);
        }
    }
}

/// The drawing column of any reachable position is at least the rectangle's left
/// edge, so a rectangle starting at or beyond the screen's right edge clips
/// every position away.
pub proof fn lemma_target_x_ge(fr: FrameSpec, q: int, sw: u32)
    requires
        1 <= fr.rect.width,
        fr.rect.left as int >= sw as int,
        0 <= q,
    ensures
        target_x(fr, q) >= sw as int,
{
    reveal(target_x);
    assert(target_x(fr, q) == fr.rect.left as int + q % (fr.rect.width as int));
    lemma_mod_bound(q, fr.rect.width as int);
}

/// The drawing row of any reachable position is at least the rectangle's top
/// edge: the stream row is `0, 1, 2, …` and the interlace row is non-negative.
pub proof fn lemma_target_y_ge(fr: FrameSpec, q: int, sh: u32)
    requires
        1 <= fr.rect.width,
        1 <= fr.rect.height,
        fr.rect.height as int <= max_dimension(),
        0 <= fr.rect.top,
        fr.rect.top as int >= sh as int,
        0 <= q < (fr.rect.width as int) * (fr.rect.height as int),
    ensures
        target_y(fr, q) >= sh as int,
{
    reveal(target_y);
    lemma_target_y_nonneg(fr, q);
}

/// The drawing row of any position is `top + (a non-negative image row)`: the
/// stream row `⌊q/w⌋` is non-negative and the interlace row is `IR` of it, which
/// is non-negative on the drawing domain.
pub proof fn lemma_target_y_nonneg(fr: FrameSpec, q: int)
    requires
        1 <= fr.rect.width,
        1 <= fr.rect.height,
        fr.rect.height as int <= max_dimension(),
        0 <= fr.rect.top,
        0 <= q,
    ensures
        target_y(fr, q) >= fr.rect.top as int,
{
    reveal(target_y);
    lemma_div_pos_is_pos(q, fr.rect.width as int);
    if fr.interlaced {
        lemma_ir_at_nonneg_step(q / (fr.rect.width as int), fr.rect.height as int, 0);
    }
}

/// A frame's stream row is inside the frame's height, so its interlace row is
/// non-negative and below the height.
pub proof fn lemma_iv_nonneg(r: int, h: int)
    requires
        1 <= h <= max_dimension(),
        0 <= r < h,
    ensures
        0 <= crate::mc::ir(r, h) < h,
{
    crate::interlace::lemma_iv_nat(r, h);
    crate::interlace::lemma_ir_eq_iv(r, h);
}

/// A rectangle entirely beyond the screen clips to nothing, so disposal 2 and the
/// disposal-3 restore write nothing and the drawing of every reachable position
/// writes nothing (Q18: no panic, no write).
pub proof fn lemma_clip_all_outside(fr: FrameSpec, sw: u32, sh: u32)
    requires
        fr.rect.left as int >= sw as int || fr.rect.top as int >= sh as int,
        1 <= fr.rect.width,
        1 <= fr.rect.height,
        fr.rect.height as int <= max_dimension(),
        0 <= fr.rect.left,
        0 <= fr.rect.top,
    ensures
        crate::mc::clip_is_empty(sw, sh, fr.rect),
        forall|q: int|
            0 <= q < (fr.rect.width as int) * (fr.rect.height as int) ==> #[trigger] target_cell(
                fr,
                sw,
                sh,
                q,
            ) is None,
{
    reveal(crate::mc::clip_is_empty);
    reveal(crate::mc::clip);
    assert forall|q: int|
        0 <= q < (fr.rect.width as int) * (fr.rect.height as int) implies #[trigger] target_cell(
        fr,
        sw,
        sh,
        q,
    ) is None by {
        if fr.rect.left as int >= sw as int {
            lemma_target_x_ge(fr, q, sw);
        } else {
            lemma_target_y_ge(fr, q, sh);
        }
        reveal(target_cell);
    }
}


// ---------------------------------------------------------------------------
// T12's step accounting for the compositor's own loops (THEOREMS.md, "Work").
// ---------------------------------------------------------------------------

/// The drawing loop's step count: one iteration per index, so `|idx|` iterations
/// exactly, and `draw_index`'s own body adds none (it is straight-line, with one
/// division and one `interlace_row` call, which the fused-call convention charges
/// to the constant part of the frame's work).
///
/// The statement the M5 integration needs: the drawing loop of a frame runs
/// `w·h` iterations, inside `C_CALL·(Δ + A)`, since `Δ + A >= 13` and
/// `C_CALL = 32` (G18, D20).
pub proof fn lemma_draw_steps_bound(w: int, h: int, c_call: int, delta: int)
    requires
        1 <= w,
        1 <= h,
        0 <= delta,
        delta + w * h >= 13,
        32 <= c_call,
    ensures
        w * h <= c_call * (delta + w * h),
{
    // `w·h <= delta + w·h <= 32·(delta + w·h) <= c_call·(delta + w·h)`: the middle
    // step uses the bound itself as the multiplier, the last `c_call >= 32`.
    assert(w * h >= 1) by {
        lemma_mul_inequality(1, h, w);
    }
    assert(w * h <= delta + w * h) by {
        assert(delta + w * h - w * h == delta);
    }
    lemma_mul_inequality(1, 32, delta + w * h);
    lemma_mul_is_commutative(1, delta + w * h);
    assert(delta + w * h <= 32 * (delta + w * h));
    lemma_mul_inequality(32, c_call, delta + w * h);
    assert(32 * (delta + w * h) <= c_call * (delta + w * h));
}

/// Disposal and restore touch at most the frame's rectangle (G12.3, Q18): the
/// rows `[top, min(top+h, sh))` and columns `[left, min(left+w, sw))`, so at most
/// `w·h` cells and at most `4·w·h` bytes, charged to the *previous* frame's `w·h`
/// by G18 (D6).
///
/// `left <= sw` and `top <= sh` are the reachable domain's facts (a rectangle
/// starting inside the screen; one entirely beyond it is the `Q18` case, where
/// the spans are 0 and the statement below is trivial).
pub proof fn lemma_disposal_bytes_bound(fr: FrameSpec, sw: u32, sh: u32)
    requires
        1 <= fr.rect.width,
        1 <= fr.rect.height,
        0 <= fr.rect.left <= sw as int,
        0 <= fr.rect.top <= sh as int,
    ensures
        ({
            let (x0, x1, y0, y1) = crate::mc::clip(fr.rect, sw, sh);
            0 <= x1 - x0 <= (fr.rect.width as int) && 0 <= y1 - y0 <= (fr.rect.height as int)
                && (x1 - x0) * (y1 - y0) <= (fr.rect.width as int) * (fr.rect.height as int)
        }),
{
    reveal(crate::mc::clip);
    let (x0, x1, y0, y1) = crate::mc::clip(fr.rect, sw, sh);
    assert(x0 == fr.rect.left as int);
    assert(y0 == fr.rect.top as int);
    assert(x0 <= x1) by {
        if fr.rect.left as int + fr.rect.width as int <= sw as int {
            assert(x1 == fr.rect.left as int + fr.rect.width as int);
        } else {
            assert(x1 == sw as int);
        }
    }
    assert(y0 <= y1) by {
        if fr.rect.top as int + fr.rect.height as int <= sh as int {
            assert(y1 == fr.rect.top as int + fr.rect.height as int);
        } else {
            assert(y1 == sh as int);
        }
    }
    assert(x1 <= fr.rect.left as int + fr.rect.width as int) by {
        if fr.rect.left as int + fr.rect.width as int <= sw as int {
            assert(x1 == fr.rect.left as int + fr.rect.width as int);
        } else {
            assert(x1 == sw as int);
        }
    }
    assert(y1 <= fr.rect.top as int + fr.rect.height as int) by {
        if fr.rect.top as int + fr.rect.height as int <= sh as int {
            assert(y1 == fr.rect.top as int + fr.rect.height as int);
        } else {
            assert(y1 == sh as int);
        }
    }
    assert(x1 - x0 <= fr.rect.width as int) by {
        if x1 - x0 > fr.rect.width as int {
            assert(x1 > fr.rect.left as int + fr.rect.width as int);
        }
    }
    assert(y1 - y0 <= fr.rect.height as int) by {
        if y1 - y0 > fr.rect.height as int {
            assert(y1 > fr.rect.top as int + fr.rect.height as int);
        }
    }
    assert(0 <= x1 - x0);
    assert(0 <= y1 - y0);
    assert(0 <= fr.rect.width as int);
    assert(0 <= fr.rect.height as int);
    lemma_mul_inequality(x1 - x0, fr.rect.width as int, y1 - y0);
    assert((x1 - x0) * (y1 - y0) <= (fr.rect.width as int) * (y1 - y0));
    lemma_mul_inequality(y1 - y0, fr.rect.height as int, fr.rect.width as int);
    assert((fr.rect.width as int) * (y1 - y0) <= (fr.rect.width as int) * (fr.rect.height as int));
}

/// The `Saved` fill's cost: `4·N` bytes of the canvas reservation, charged once
/// per decoder to the call that reserves `Saved` (`Req_next_p`'s `(Saved, 4·N)`),
/// so `4·N` is the whole charge and no frame pays it again (G18, D6, T12).
pub proof fn lemma_saved_fill_cost(n: int)
    requires
        0 <= n,
    ensures
        4 * n >= 0,
{
    lemma_mul_inequality(0, n, 4);
}

// ---------------------------------------------------------------------------
// The `Sink` adapter (design memo step 2): the compositor as `lzw::Sink`.
//
// The proved per-call helper `draw_index` is the whole per-call drawing
// contract; this adapter only has to (a) build the temporary `DrawState { q }`
// the memo prescribes — a fresh state per call, so no persistent counter is
// reused — (b) name the validity predicate `ok(v) = transparent == Some(v) ||
// v < C`, and (c) keep the ghost call log `H`. It binds `v`'s cell from a slice
// argument rather than a persistent sink field, so no `Sink` implementation for
// `Drawer` is needed and the borrow `Drawer` takes of `polygon` stays out of the
// borrow the canvas needs.
// ---------------------------------------------------------------------------

/// `ok(v)` for a frame: the transparency test first, then the table reach (G12.2,
/// N12). `draw_step` rejects exactly the complement.
pub open spec fn drawer_ok(fr: Frame, v: u8) -> bool {
    fr.transparent == Some(v) || (v as int) < fr.c
}

/// `draw_step`'s `Ok`/`Err` split is `drawer_ok`: the transparent index leaves
/// the canvas and every other index at or above `C` is `ColorIndexOutOfRange`.
pub proof fn lemma_draw_step_ok_shape(
    cv: Seq<u8>,
    fr: Frame,
    table: Seq<u8>,
    sw: u32,
    sh: u32,
    q: int,
    v: u8,
)
    ensures
        draw_step(cv, fr.spec_frame(), table, sw, sh, q, v) is Ok <==> drawer_ok(fr, v),
{
    reveal(draw_step);
}

/// The compositor as one `decode` call's sink: the canvas, the active table, the
/// frame record and the screen, with the ghost call log.
pub struct DrawerSink<'c, 't> {
    pub canvas: &'c mut [u8],
    pub table: &'t [u8],
    pub fr: Frame,
    pub sw: u32,
    pub sh: u32,
    /// Ghost: the sink's call log `H` (G18's observation of `put`).
    pub log: Ghost<Seq<(u32, u8)>>,
}

impl<'c, 't> DrawerSink<'c, 't> {
    /// A sink for one frame, with the canvas's log empty.
    pub fn new(canvas: &'c mut [u8], table: &'t [u8], fr: Frame, sw: u32, sh: u32) -> (s: Self)
        ensures
            s.fr == fr,
            s.canvas@ == old(canvas)@,
            final(canvas)@ == final(s.canvas)@,
            s.table == table,
            s.sw == sw,
            s.sh == sh,
            s.log@ == Seq::<(u32, u8)>::empty(),
    {
        DrawerSink { canvas, table, fr, sw, sh, log: Ghost(Seq::empty()) }
    }
}

impl<'c, 't> crate::lzw::Sink for DrawerSink<'c, 't> {
    closed spec fn sink_area(&self) -> int {
        (self.fr.rect.width as int) * (self.fr.rect.height as int)
    }

    closed spec fn sink_inv(&self) -> bool {
        &&& frame_ok(self.fr)
        &&& canvas_ok(self.canvas, self.sw, self.sh)
        &&& table_ok(self.table, self.fr)
    }

    closed spec fn sink_ok(&self, v: u8) -> bool {
        drawer_ok(self.fr, v)
    }

    closed spec fn sink_log(&self) -> Seq<(u32, u8)> {
        self.log@
    }

    closed spec fn sink_view(&self) -> Seq<u8> { self.canvas@ }

    #[verifier::prophetic]
    closed spec fn sink_future(&self) -> Seq<u8> { final(self.canvas)@ }

    closed spec fn sink_effect(&self) -> spec_fn(Seq<u8>,u32,u8)->Seq<u8> {
        crate::emission_compose::draw_effect(self.fr.spec_frame(),self.table@,self.sw,self.sh)
    }

    /// One accepted or rejected index. The canvas effect is `draw_index`'s proved
    /// `draw_step` relation at the temporary state `{q}`; the log is this call's
    /// pair, appended.
    fn put(&mut self, q: u32, v: u8) -> (r: bool)
        ensures
            r == final(self).sink_ok(v),
            final(self).log@ == old(self).log@.push((q, v)),
            final(self).sink_inv(),
            // The canvas is `draw_step`'s: advanced on accept, unchanged on reject.
            match draw_step(
                old(self).canvas@,
                old(self).fr.spec_frame(),
                old(self).table@,
                old(self).sw,
                old(self).sh,
                q as int,
                v,
            ) {
                Ok(cv2) => final(self).canvas@ == cv2,
                Err(e) => final(self).canvas@ == old(self).canvas@,
            },
    {
        let mut st = DrawState { q };
        let res = draw_index(
            &mut *self.canvas,
            &mut st,
            self.fr,
            self.table,
            self.sw,
            self.sh,
            v,
        );
        proof {
            lemma_draw_step_ok_shape(
                old(self).canvas@,
                self.fr,
                self.table@,
                self.sw,
                self.sh,
                q as int,
                v,
            );
            self.log = Ghost(self.log@.push((q, v)));
            assert(self.sink_effect() =~= old(self).sink_effect());
        }
        res.is_ok()
    }
}

} // verus!

verus! {
#[cfg(verus_only)]
use crate::lzw::Sink;

/// Concrete-call drawing theorem over the Drawer's borrowed canvas. No canvas
/// equality is required when a colour error rejects the frame.
pub fn decode_drawer(b:&[u8],data:crate::parse::ImageData,area:u32,opts:crate::lzw::LzwOptions,
    kind:crate::policy::LzwMin,t:&mut [u8],sink:&mut DrawerSink)->(rep:crate::lzw::LzwReport)
    requires b.len()<=crate::limits::MAX_INPUT,1<=area<=crate::limits::MAX_TOTAL_PIXELS,
        old(sink).sink_inv(),old(sink).sink_area()==area as int,crate::lzw::table_len_ok(old(t)@),
        crate::parse::lzw_min_ok(kind,data.min_code_size as int,area as int),
        opts.checked_literals==(kind==crate::policy::LzwMin::Range1to11Checked),
        crate::model::sub_scan(b@,data.first as int) is Some,
        crate::lzw::read_d(b@,data.first as int).len()==data.data_len,data.data_len<=crate::limits::MAX_INPUT,
    ensures final(sink).sink_inv(),crate::lzw::table_len_ok(final(t)@),
        final(sink).sink_future()==old(sink).sink_future(),
        forall|ts:Set<Seq<u8>>| pixels_well_formed(old(sink).canvas@,ts) && ts.contains(old(sink).table@)
            ==> pixels_well_formed(final(sink).canvas@,ts),
        crate::lzw::projected_error(rep)==if exists|i:int| 0<=i<crate::lzw_model::ML(data.min_code_size as int,crate::lzw::read_d(b@,data.first as int),area as int,crate::lzw::lzw_ps(data.min_code_size,opts)).idx.len()
            && !drawer_ok(old(sink).fr,#[trigger] crate::lzw_model::ML(data.min_code_size as int,crate::lzw::read_d(b@,data.first as int),area as int,crate::lzw::lzw_ps(data.min_code_size,opts)).idx[i])
            {Some(crate::gif::Error::ColorIndexOutOfRange)} else {rep.error},
        final(sink).canvas@.len()==old(sink).canvas@.len(),
        crate::lzw::report_matches(rep,crate::lzw_model::ML(data.min_code_size as int,crate::lzw::read_d(b@,data.first as int),area as int,crate::lzw::lzw_ps(data.min_code_size,opts))),
        crate::lzw::colour_observation(rep,crate::lzw_model::ML(data.min_code_size as int,crate::lzw::read_d(b@,data.first as int),area as int,crate::lzw::lzw_ps(data.min_code_size,opts)).idx,|v:u8|drawer_ok(old(sink).fr,v)),
        rep.colour is None ==> crate::mc::draw_fold(old(sink).canvas@,old(sink).fr.spec_frame(),old(sink).table@,old(sink).sw,old(sink).sh,
            crate::lzw_model::ML(data.min_code_size as int,crate::lzw::read_d(b@,data.first as int),area as int,crate::lzw::lzw_ps(data.min_code_size,opts)).idx,0)==Ok(final(sink).canvas@),
        rep.work@ <= 5 * data.data_len as nat + 2 * area as nat,
{
    let ghost cv0=sink.canvas@;
    let ghost frame=sink.fr;
    let ghost colors=sink.table@;
    let ghost sw=sink.sw;
    let ghost sh=sink.sh;
    let ghost ok=|v:u8|sink.sink_ok(v);
    proof {assert(ok =~= |v:u8|drawer_ok(frame,v));}
    let rep=crate::lzw::decode_image_data(b,data,area,opts,kind,t,sink);
    proof {
        let mr=crate::lzw_model::ML(data.min_code_size as int,crate::lzw::read_d(b@,data.first as int),area as int,crate::lzw::lzw_ps(data.min_code_size,opts));
        let o=mr.idx;
        crate::lzw::lemma_public_projection(rep,mr,ok);
        assert(sink.canvas@==crate::lzw::effect_fold(cv0,rep.h@,crate::emission_compose::draw_effect(frame.spec_frame(),colors,sw,sh)));
        assert forall|ts:Set<Seq<u8>>| pixels_well_formed(cv0,ts) && ts.contains(colors) implies pixels_well_formed(sink.canvas@,ts) by {
            crate::emission_compose::lemma_effect_pixels(cv0,frame.spec_frame(),colors,sw,sh,rep.h@,ts);
        }
        if rep.colour is None {
            assert forall|p:int| 0<=p<o.len() implies frame.transparent==Some(#[trigger] o[p]) || (o[p] as int)<frame.c by {};
            crate::emission_compose::lemma_complete_log_canvas(cv0,frame.spec_frame(),colors,sw,sh,rep.h@,o);
        }
    }
    rep
}

/// Rejoin the Drawer's mutable borrow with the caller's canvas explicitly.
/// The constructor's future-reference equation carries the decoder's effect
/// back to this formal parameter when the local sink goes out of scope.
pub fn decode_canvas(b:&[u8],data:crate::parse::ImageData,area:u32,opts:crate::lzw::LzwOptions,
    kind:crate::policy::LzwMin,t:&mut [u8],canvas:&mut [u8],colors:&[u8],fr:Frame,sw:u32,sh:u32)->(rep:crate::lzw::LzwReport)
    requires b.len()<=crate::limits::MAX_INPUT,1<=area<=crate::limits::MAX_TOTAL_PIXELS,
        frame_ok(fr),canvas_ok(old(canvas),sw,sh),table_ok(colors,fr),
        (fr.rect.width as int)*(fr.rect.height as int)==area as int,
        crate::lzw::table_len_ok(old(t)@),
        crate::parse::lzw_min_ok(kind,data.min_code_size as int,area as int),
        opts.checked_literals==(kind==crate::policy::LzwMin::Range1to11Checked),
        crate::model::sub_scan(b@,data.first as int) is Some,
        crate::lzw::read_d(b@,data.first as int).len()==data.data_len,data.data_len<=crate::limits::MAX_INPUT,
    ensures crate::lzw::table_len_ok(final(t)@),final(canvas)@.len()==old(canvas)@.len(),
        crate::lzw::report_matches(rep,crate::lzw_model::ML(data.min_code_size as int,crate::lzw::read_d(b@,data.first as int),area as int,crate::lzw::lzw_ps(data.min_code_size,opts))),
        crate::lzw::colour_observation(rep,crate::lzw_model::ML(data.min_code_size as int,crate::lzw::read_d(b@,data.first as int),area as int,crate::lzw::lzw_ps(data.min_code_size,opts)).idx,|v:u8|drawer_ok(fr,v)),
        rep.colour is None ==> crate::mc::draw_fold(old(canvas)@,fr.spec_frame(),colors@,sw,sh,
            crate::lzw_model::ML(data.min_code_size as int,crate::lzw::read_d(b@,data.first as int),area as int,crate::lzw::lzw_ps(data.min_code_size,opts)).idx,0)==Ok(final(canvas)@),
        forall|ts:Set<Seq<u8>>| pixels_well_formed(old(canvas)@,ts) && ts.contains(colors@)
            ==> pixels_well_formed(final(canvas)@,ts),
        rep.work@ <= 5 * data.data_len as nat + 2 * area as nat,
{
    let rep={
        let mut sink=DrawerSink::new(canvas,colors,fr,sw,sh);
        decode_drawer(b,data,area,opts,kind,t,&mut sink)
    };
    rep
}
} // verus!
