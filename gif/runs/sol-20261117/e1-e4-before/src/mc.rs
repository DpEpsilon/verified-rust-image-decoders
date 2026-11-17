//! Total mathematical models for G11 (interlace) and G12 (compositing):
//! `IR` and `MC` of `../THEOREMS.md`, transcribed from `../spec-draft/SPEC.md`
//! independently of the executable code.
//!
//! Arithmetic is on mathematical integers (no narrowing, no overflow). The
//! canvas is a `Seq<u8>` of `4·W·H` bytes, pixel `(x, y)` at byte offset
//! `4·(y·W + x)`, as `gif_verus/src/compose.rs` lays it out. The models are total:
//! `int` division is total in Verus, and every theorem below states the domain
//! preconditions it needs.

use vstd::prelude::*;
#[cfg(verus_only)]
use crate::compose_new::ComposeError;

verus! {

// ---------------------------------------------------------------------------
// G1 limits, as the named constants of `limits.rs` (never a numeral).
// ---------------------------------------------------------------------------

pub open spec fn max_dimension() -> int {
    crate::limits::MAX_DIMENSION as int
}

pub open spec fn max_output() -> int {
    crate::limits::MAX_OUTPUT as int
}

// ---------------------------------------------------------------------------
// The frame rectangle and the frame's G11/G12 fields.
// ---------------------------------------------------------------------------

/// A frame rectangle exactly as G8's image descriptor gives it. `w, h >= 1` on
/// every reachable state (G8 rejects zero). Widths and heights are LE16, so
/// `left + width <= 2·MAX_DIMENSION` holds without narrowing (THEOREMS'
/// `rect_sum_unnarrowed`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

/// The parts of a frame that G11 and G12 consult: its rectangle, its interlace
/// flag, the entry count `C` of the active colour table (the table holds `3·C`
/// bytes), its transparent index (G6.1, absent when the flag is clear), and its
/// disposal (G6.1, 0..3 on every reachable state).
pub struct FrameSpec {
    pub rect: Rect,
    pub interlaced: bool,
    pub c: int,
    pub transparent: Option<u8>,
    pub disposal: int,
}

// ---------------------------------------------------------------------------
// G11. Interlace (`IR`)
// ---------------------------------------------------------------------------

/// G11's four passes, as `(start, step)` pairs: `0, 8, 16, …`, then `4, 12, …`,
/// then `2, 6, 10, …`, then `1, 3, 5, …`.
pub open spec fn pass_start(k: int) -> int {
    if k == 0 {
        0
    } else if k == 1 {
        4
    } else if k == 2 {
        2
    } else {
        1
    }
}

pub open spec fn pass_step(k: int) -> int {
    if k == 2 {
        4
    } else if k == 3 {
        2
    } else {
        8
    }
}

/// G11: the number of image rows pass `k` takes, `⌈(h − start)/step⌉`, and 0 when
/// the pass is empty because its starting row is `>= h` (THEOREMS'
/// `interlace_pass_sizes`).
pub open spec fn pass_len(k: int, h: int) -> int {
    let s = pass_start(k);
    let t = pass_step(k);
    if h > s {
        (h - s + (t - 1)) / t
    } else {
        0
    }
}

/// G11: the image row of stream row `r`, walking the passes in order.
pub open spec fn ir_at(r: int, h: int, k: int) -> int
    decreases (4 - k) as nat,
{
    if k >= 4 {
        // Unreachable on `[0, h)`: the four passes cover exactly `h` rows.
        0
    } else if r < pass_len(k, h) {
        pass_start(k) + r * pass_step(k)
    } else {
        ir_at(r - pass_len(k, h), h, k + 1)
    }
}

/// `IR(r, h)`: G11's image row for stream row `r` (`0 <= r < h`).
pub open spec fn ir(r: int, h: int) -> int {
    ir_at(r, h, 0)
}

/// THEOREMS' `interlace_bijection`, as a predicate over `h`.
pub open spec fn ir_bijective(h: int) -> bool {
    // `IR` is a function, so one value per argument settles the `surjective` leg
    // and the `injective` leg together with the two `in_range` legs of
    // `interlace_bijection`; stated separately, as THEOREMS does.
    forall|r: int| 0 <= r < h ==> 0 <= #[trigger] ir(r, h) < h
}

/// The whole of G11's bijection claim, `1 <= h <= MAX_DIMENSION`.
pub open spec fn ir_bijective_on_cap() -> bool {
    forall|h: int| 1 <= h <= max_dimension() ==> #[trigger] ir_bijective(h)
}


// ---------------------------------------------------------------------------
// G12. Compositing (`MC`)
// ---------------------------------------------------------------------------

/// The clipped rectangle of `r` inside a `sw × sh` screen, as half-open column
/// and row ranges `(x0, x1, y0, y1)`. `x0 <= x1` and `y0 <= y1` hold always; an
/// empty range has `x0 == x1` or `y0 == y1`. `x0` and `y0` are never clipped
/// upwards: the descriptor gives unsigned coordinates, so a rectangle starting at
/// or beyond the right or bottom edge has `x1 == sw <= x0` or `y1 == sh <= y0`
/// (Q18).
pub open spec fn clip(r: Rect, sw: u32, sh: u32) -> (int, int, int, int) {
    let x1 = if r.left as int + r.width as int <= sw as int {
        r.left as int + r.width as int
    } else {
        sw as int
    };
    let y1 = if r.top as int + r.height as int <= sh as int {
        r.top as int + r.height as int
    } else {
        sh as int
    };
    (r.left as int, x1, r.top as int, y1)
}

/// Whether a frame rectangle lies inside a `sw × sh` screen: G8's check under
/// `frame_bounds = Reject`.
pub open spec fn inside_screen(r: Rect, sw: u32, sh: u32) -> bool {
    r.left as int + r.width as int <= sw as int && r.top as int + r.height as int <= sh as int
}

/// G12.2: the pixel column of stream position `q` of frame `fr`.
pub open spec fn target_x(fr: FrameSpec, q: int) -> int {
    fr.rect.left as int + q % (fr.rect.width as int)
}

/// G12.2: the pixel row of stream position `q`; the interlace row is
/// `IR(q / w, h)` for an interlaced frame and `q / w` otherwise.
pub open spec fn target_y(fr: FrameSpec, q: int) -> int {
    let r = q / (fr.rect.width as int);
    fr.rect.top as int + if fr.interlaced {
        ir(r, fr.rect.height as int)
    } else {
        r
    }
}

/// G12.2: the canvas cell (pixel index, `x` runnable) written for stream position
/// `q`, or `None` when the position is clipped away (`x >= W` or `y >= H`; never
/// under `frame_bounds = Reject`, where the rectangle is inside the screen).
pub open spec fn target_cell(fr: FrameSpec, sw: u32, sh: u32, q: int) -> Option<int> {
    let x = target_x(fr, q);
    let y = target_y(fr, q);
    if x < sw as int && y < sh as int {
        Some(y * (sw as int) + x)
    } else {
        None
    }
}

/// The four bytes of the colour-table entry `v`, opaque (G13).
pub open spec fn pixel_word(table: Seq<u8>, v: int) -> Seq<u8> {
    seq![table[3 * v], table[3 * v + 1], table[3 * v + 2], 255u8]
}

/// G12.2's write of one pixel: the four bytes at offset `o`, in place.
pub open spec fn write_pixel(cv: Seq<u8>, o: int, table: Seq<u8>, v: int) -> Seq<u8> {
    let pw = pixel_word(table, v);
    cv.update(o, pw[0]).update(o + 1, pw[1]).update(o + 2, pw[2]).update(o + 3, pw[3])
}

/// G12.2 for one emitted index `v` at stream position `q`: the transparent index
/// leaves the pixel unchanged (even when it is `>= C`, N12); otherwise `v < C` is
/// required, else `ColorIndexOutOfRange`; then the cell is set to entry `v`.
pub open spec fn draw_step(cv: Seq<u8>, fr: FrameSpec, table: Seq<u8>, sw: u32, sh: u32, q: int, v: u8) -> Result<
    Seq<u8>,
    ComposeError,
> {
    if fr.transparent == Some(v) {
        Ok(cv)
    } else if (v as int) >= fr.c {
        Err(ComposeError::ColorIndexOutOfRange)
    } else {
        match target_cell(fr, sw, sh, q) {
            None => Ok(cv),
            Some(cell) => Ok(write_pixel(cv, 4 * cell, table, v as int)),
        }
    }
}

/// G12.2 over a whole index sequence: the fold of `draw_step` from position `q`,
/// stopping at the first error (L-INTERLEAVE).
pub open spec fn draw_fold(
    cv: Seq<u8>,
    fr: FrameSpec,
    table: Seq<u8>,
    sw: u32,
    sh: u32,
    idx: Seq<u8>,
    q: int,
) -> Result<Seq<u8>, ComposeError>
    decreases (idx.len() - q) as nat,
{
    if q >= idx.len() {
        Ok(cv)
    } else {
        match draw_step(cv, fr, table, sw, sh, q, idx[q]) {
            Err(e) => Err(e),
            Ok(cv2) => draw_fold(cv2, fr, table, sw, sh, idx, q + 1),
        }
    }
}

/// G12.2's drawing of the index sequence from position `q`, carrying the canvas
/// through the first error: `(canvas at the error or at the end, first error)`.
/// This is the shape the executable's loop proves, and `draw_fold` is its
/// error-only projection.
pub open spec fn draw_run_at(
    cv: Seq<u8>,
    fr: FrameSpec,
    table: Seq<u8>,
    sw: u32,
    sh: u32,
    idx: Seq<u8>,
    q: int,
) -> (Seq<u8>, Option<ComposeError>)
    decreases (idx.len() - q) as nat,
{
    if q >= idx.len() {
        (cv, None)
    } else {
        match draw_step(cv, fr, table, sw, sh, q, idx[q]) {
            Err(e) => (cv, Some(e)),
            Ok(cv2) => draw_run_at(cv2, fr, table, sw, sh, idx, q + 1),
        }
    }
}

/// The unfolding of `draw_run_at` at a position inside the sequence: the canvas
/// goes through `draw_step`, and an error there stops the run with the canvas
/// unchanged. This is the equation the executable's loop body needs.
pub proof fn lemma_draw_run_at_step(
    cv: Seq<u8>,
    fr: FrameSpec,
    table: Seq<u8>,
    sw: u32,
    sh: u32,
    idx: Seq<u8>,
    q: int,
)
    requires
        0 <= q < idx.len(),
    ensures
        match draw_step(cv, fr, table, sw, sh, q, idx[q]) {
            Err(e) => draw_run_at(cv, fr, table, sw, sh, idx, q) == (cv, Some(e)),
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

/// The unfolding at the end of the sequence: the run is over, with no error.
pub proof fn lemma_draw_run_at_end(
    cv: Seq<u8>,
    fr: FrameSpec,
    table: Seq<u8>,
    sw: u32,
    sh: u32,
    idx: Seq<u8>,
    q: int,
)
    requires
        q >= idx.len(),
    ensures
        draw_run_at(cv, fr, table, sw, sh, idx, q) == (cv, Option::<ComposeError>::None),
{
    reveal(draw_run_at);
}

/// G12.2's drawing of a whole frame, carrying the canvas through an error.
pub open spec fn draw_run(cv: Seq<u8>, fr: FrameSpec, table: Seq<u8>, sw: u32, sh: u32, idx: Seq<u8>) -> (
    Seq<u8>,
    Option<ComposeError>,
) {
    draw_run_at(cv, fr, table, sw, sh, idx, 0)
}



/// G12.2's drawing of a whole frame: every index drawn from stream position 0.
pub open spec fn draw_all(cv: Seq<u8>, fr: FrameSpec, table: Seq<u8>, sw: u32, sh: u32, idx: Seq<u8>) -> Result<
    Seq<u8>,
    ComposeError,
> {
    draw_fold(cv, fr, table, sw, sh, idx, 0)
}

// ---------------------------------------------------------------------------
// The output pixel clause (G12.1, G12.3, G13; THEOREMS' T13 second paragraph):
// a pixel with alpha 255 carries the RGB of the colour-table entry that drew it,
// and every other pixel is exactly `(0,0,0,0)`.
// ---------------------------------------------------------------------------

/// Pixel `i` of the canvas (bytes `4·i … 4·i+3`) is the opaque word of the entry
/// `v` of `table`: the RGB of that entry, copied as stored, with alpha 255.
pub open spec fn pixel_is_entry(cv: Seq<u8>, table: Seq<u8>, i: int, v: int) -> bool {
    &&& cv[4 * i] == table[3 * v]
    &&& cv[4 * i + 1] == table[3 * v + 1]
    &&& cv[4 * i + 2] == table[3 * v + 2]
    &&& cv[4 * i + 3] == 255u8
}

/// Pixel `i` is exactly `(0,0,0,0)`: the initial canvas (G12.1), a disposal-2
/// fill (G12.3) or an uncovered cell.
pub open spec fn pixel_is_clear(cv: Seq<u8>, i: int) -> bool {
    &&& cv[4 * i] == 0u8
    &&& cv[4 * i + 1] == 0u8
    &&& cv[4 * i + 2] == 0u8
    &&& cv[4 * i + 3] == 0u8
}

/// The whole-canvas output clause, relative to a set `ts` of colour tables (the
/// tables active for the frames that drew): every alpha-255 pixel's RGB is some
/// entry of some table in `ts`, and every other pixel is `(0,0,0,0)`.
pub open spec fn pixels_well_formed(cv: Seq<u8>, ts: Set<Seq<u8>>) -> bool {
    forall|i: int|
        0 <= i < cv.len() / 4 ==> {
            ||| (exists|table: Seq<u8>, v: int|
                ts.contains(table) && 0 <= v && 3 * v + 2 < table.len() && #[trigger] pixel_is_entry(
                    cv,
                    table,
                    i,
                    v,
                ))
            ||| #[trigger] pixel_is_clear(cv, i)
        }
}

// ---------------------------------------------------------------------------
// G12.3. Disposal and `Saved`. `Saved` has the canvas's shape, so a rectangle's
// pixels sit at the same byte offsets of both.
//
// These operations are defined *per byte index*: THEOREMS states their effect per
// position ("inside the rectangle and drawn, or unchanged"), and this definition
// is that statement. Byte `i` holds pixel `i / 4` at row `i / 4 / W` and column
// `i / 4 % W`, so the rectangle is the byte set
// `{i : x0 <= i/4 % W < x1, y0 <= i/4 / W < y1}`.
// ---------------------------------------------------------------------------

/// The canvas cell (column) holding byte `i`, on a `sw`-wide screen.
pub open spec fn byte_col(i: int, sw: int) -> int {
    i / 4 % sw
}

/// The canvas row holding byte `i`, on a `sw`-wide screen.
pub open spec fn byte_row(i: int, sw: int) -> int {
    i / 4 / sw
}

/// Whether byte `i` lies in the clipped rectangle of `r` (Q18: an empty clipped
/// rectangle has no such byte).
pub open spec fn in_clipped(i: int, sw: u32, sh: u32, r: Rect) -> bool {
    let (x0, x1, y0, y1) = clip(r, sw, sh);
    x0 <= byte_col(i, sw as int) < x1 && y0 <= byte_row(i, sw as int) < y1
}

/// Disposal 2 (G12.3): every byte of the clipped rectangle becomes 0 and every
/// other byte is unchanged, so the rectangle's pixels are `(0,0,0,0)`.
pub open spec fn clear_rect(cv: Seq<u8>, sw: u32, sh: u32, r: Rect) -> Seq<u8> {
    Seq::new(cv.len() as nat, |i: int| if in_clipped(i, sw, sh, r) { 0u8 } else { cv[i] })
}

/// The disposal-3 save (`canvas → Saved`) and restore (`Saved → canvas`): the
/// clipped rectangle's bytes come from `src` and every other byte from `dst`.
/// Both act on the clipped rectangle, so they write nothing when it is empty
/// (Q18), and the restore reads exactly the bytes the save wrote.
pub open spec fn copy_rect(dst: Seq<u8>, src: Seq<u8>, sw: u32, sh: u32, r: Rect) -> Seq<u8> {
    Seq::new(dst.len() as nat, |i: int| if in_clipped(i, sw, sh, r) { src[i] } else { dst[i] })
}

/// An empty clipped rectangle: `x1 <= x0` or `y1 <= y0`.
pub open spec fn clip_is_empty(sw: u32, sh: u32, r: Rect) -> bool {
    let (x0, x1, y0, y1) = clip(r, sw, sh);
    x1 <= x0 || y1 <= y0
}

// ---------------------------------------------------------------------------
// `MC`: one frame transition.
// ---------------------------------------------------------------------------

/// `MC`'s disposal step: the disposal of the *previous* frame applied to the
/// canvas. Disposals 0 and 1 leave the canvas; 2 clears the (clipped) rectangle;
/// 3 restores the `Saved` rows. A previous frame with no Graphic Control has
/// disposal 0.
pub open spec fn dispose_prev(cv: Seq<u8>, saved: Seq<u8>, sw: u32, sh: u32, prev: Option<FrameSpec>) -> Seq<u8> {
    match prev {
        None => cv,
        Some(p) => {
            if p.disposal == 2 {
                clear_rect(cv, sw, sh, p.rect)
            } else if p.disposal == 3 {
                copy_rect(cv, saved, sw, sh, p.rect)
            } else {
                cv
            }
        },
    }
}

/// The frame sequence of T7/T8: `canvas_k = MC(canvas_{k-1}, ...)` for every
/// frame, with `Saved` carried along. The frames are a `Seq` of `(frame, table,
/// indices, previous disposal and rectangle)`-shaped steps; `MCs` folds them.
pub open spec fn mc_step(
    cv: Seq<u8>,
    saved: Seq<u8>,
    sw: u32,
    sh: u32,
    step: (Option<FrameSpec>, FrameSpec, Seq<u8>, Seq<u8>),
) -> (Seq<u8>, Seq<u8>, Option<ComposeError>) {
    mc(cv, saved, sw, sh, step.0, step.1, step.3, step.2)
}

/// The canvas and `Saved` after the first `k` frames of `steps`.
pub open spec fn mc_frames_from(
    cv: Seq<u8>,
    saved: Seq<u8>,
    sw: u32,
    sh: u32,
    steps: Seq<(Option<FrameSpec>, FrameSpec, Seq<u8>, Seq<u8>)>,
    k: int,
) -> (Seq<u8>, Seq<u8>, Option<ComposeError>)
    decreases k,
{
    if k <= 0 {
        (cv, saved, Option::<ComposeError>::None)
    } else {
        let prev = mc_frames_from(cv, saved, sw, sh, steps, k - 1);
        if prev.2 is Some {
            prev
        } else {
            mc_step(prev.0, prev.1, sw, sh, steps[k - 1])
        }
    }
}

/// `canvas_k = MC(canvas_{k-1}, ...)`: one step of the fold, in the form T8
/// states.
pub proof fn lemma_mc_frames_step(
    cv: Seq<u8>,
    saved: Seq<u8>,
    sw: u32,
    sh: u32,
    steps: Seq<(Option<FrameSpec>, FrameSpec, Seq<u8>, Seq<u8>)>,
    k: int,
)
    requires
        0 < k <= steps.len(),
    ensures
        mc_frames_from(cv, saved, sw, sh, steps, k - 1).2 is None ==> mc_frames_from(
            cv,
            saved,
            sw,
            sh,
            steps,
            k,
        ) == mc_step(
            mc_frames_from(cv, saved, sw, sh, steps, k - 1).0,
            mc_frames_from(cv, saved, sw, sh, steps, k - 1).1,
            sw,
            sh,
            steps[k - 1],
        ),
        mc_frames_from(cv, saved, sw, sh, steps, k - 1).2 is Some ==> mc_frames_from(
            cv,
            saved,
            sw,
            sh,
            steps,
            k,
        ) == mc_frames_from(cv, saved, sw, sh, steps, k - 1),
{
    reveal(mc_frames_from);
}

/// `MC` after its disposal and save steps: draw the index sequence onto the
/// disposed canvas and return it with the `Saved` buffer. Factored out so the
/// executable can take the three steps as three calls, with `draw_run`'s
/// canvas-through-an-error shape.
pub open spec fn mc_from(
    disposed: Seq<u8>,
    saved2: Seq<u8>,
    fr: FrameSpec,
    table: Seq<u8>,
    sw: u32,
    sh: u32,
    idx: Seq<u8>,
) -> (Seq<u8>, Seq<u8>, Option<ComposeError>) {
    let run = draw_run(disposed, fr, table, sw, sh, idx);
    (run.0, saved2, run.1)
}

/// The canvas `MC` works on after applying the previous frame's disposal.
pub open spec fn mc_disposed(
    cv: Seq<u8>,
    saved: Seq<u8>,
    sw: u32,
    sh: u32,
    prev: Option<FrameSpec>,
) -> Seq<u8> {
    dispose_prev(cv, saved, sw, sh, prev)
}

/// The `Saved` buffer `MC` leaves behind: the rectangle's pixels saved when this
/// frame's disposal is 3, and the buffer unchanged otherwise.
pub open spec fn mc_saved(
    cv: Seq<u8>,
    saved: Seq<u8>,
    sw: u32,
    sh: u32,
    prev: Option<FrameSpec>,
    fr: FrameSpec,
) -> Seq<u8> {
    if fr.disposal == 3 {
        copy_rect(saved, dispose_prev(cv, saved, sw, sh, prev), sw, sh, fr.rect)
    } else {
        saved
    }
}

/// The unfolded `MC`, so a caller can move between its three steps one at a time.
pub proof fn lemma_mc_unfold(
    cv: Seq<u8>,
    saved: Seq<u8>,
    sw: u32,
    sh: u32,
    prev: Option<FrameSpec>,
    fr: FrameSpec,
    table: Seq<u8>,
    idx: Seq<u8>,
)
    ensures
        mc(cv, saved, sw, sh, prev, fr, table, idx) == mc_from(
            mc_disposed(cv, saved, sw, sh, prev),
            mc_saved(cv, saved, sw, sh, prev, fr),
            fr,
            table,
            sw,
            sh,
            idx,
        ),
{
    reveal(mc);
    reveal(mc_from);
    reveal(mc_disposed);
    reveal(mc_saved);
}

/// `MC`: apply the previous frame's disposal (G12.3), save this frame's rectangle
/// into `Saved` when its disposal is 3 (G12.2, before its first write), then draw
/// `ML`'s index sequence through `IR` and the active table (G12.2). `Ok` carries
/// the canvas and the `Saved` buffer; `Err` the first `ColorIndexOutOfRange`.
pub open spec fn mc(
    cv: Seq<u8>,
    saved: Seq<u8>,
    sw: u32,
    sh: u32,
    prev: Option<FrameSpec>,
    fr: FrameSpec,
    table: Seq<u8>,
    idx: Seq<u8>,
) -> (Seq<u8>, Seq<u8>, Option<ComposeError>) {
    mc_from(
        mc_disposed(cv, saved, sw, sh, prev),
        mc_saved(cv, saved, sw, sh, prev, fr),
        fr,
        table,
        sw,
        sh,
        idx,
    )
}


/// `draw_run` is `draw_run_at` from position 0.
pub proof fn lemma_draw_run_is_at(
    cv: Seq<u8>,
    fr: FrameSpec,
    table: Seq<u8>,
    sw: u32,
    sh: u32,
    idx: Seq<u8>,
)
    ensures
        draw_run(cv, fr, table, sw, sh, idx) == draw_run_at(cv, fr, table, sw, sh, idx, 0),
{
    reveal(draw_run);
}
} // verus!
