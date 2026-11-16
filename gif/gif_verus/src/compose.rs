//! G11 interlace and G12 compositing: drawing indices onto the canvas,
//! disposal 2 and 3, the restore buffer `Saved`, and clipping to the screen
//! (policy field `frame_bounds = Clip`).
//!
//! Canvas layout: `W·H` RGBA8 pixels, top row first; pixel `(x, y)` with
//! `x < W`, `y < H` is at byte offset `4·(y·W + x)`, below `4·W·H <=
//! MAX_OUTPUT`. All coordinate arithmetic is on `u32` values up to 131,070
//! (`left + w`, `top + h`) and is never narrowed; offsets are formed only after
//! both bounds are checked.
//!
//! `Saved` has the canvas's shape: the rectangle of a disposal-3 frame is copied
//! into the same offsets of `Saved` and copied back by the restore, so the
//! restore reads exactly the bytes the save wrote.

use crate::lzw::Sink;


/// A frame rectangle as the descriptor gives it (`w, h >= 1`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

/// G11: the image row holding stream row `r` of an interlaced image of height
/// `h` (`0 <= r < h`). Passes start at rows 0, 4, 2, 1 with steps 8, 8, 4, 2;
/// a pass is empty when its start is `>= h`.
pub fn interlace_row(r: u32, h: u32) -> u32 {
    let n0 = if h > 0 { (h - 1) / 8 + 1 } else { 0 };
    if r < n0 {
        return r * 8;
    }
    let r1 = r - n0;
    let n1 = if h > 4 { (h - 5) / 8 + 1 } else { 0 };
    if r1 < n1 {
        return 4 + r1 * 8;
    }
    let r2 = r1 - n1;
    let n2 = if h > 2 { (h - 3) / 4 + 1 } else { 0 };
    if r2 < n2 {
        return 2 + r2 * 4;
    }
    let r3 = r2 - n2;
    1 + r3 * 2
}

/// The part of `rect` inside a `sw × sh` screen, as half-open column and row
/// ranges `(x0, x1, y0, y1)`; empty ranges have `x0 >= x1` or `y0 >= y1`.
fn clip(rect: Rect, sw: u32, sh: u32) -> (u32, u32, u32, u32) {
    let x1 = core::cmp::min(rect.left + rect.width, sw);
    let y1 = core::cmp::min(rect.top + rect.height, sh);
    (rect.left, x1, rect.top, y1)
}

/// G12.3 disposal 2: set the frame's rectangle, clipped to the screen, to
/// `(0,0,0,0)`.
pub fn clear_rect(canvas: &mut [u8], sw: u32, sh: u32, rect: Rect) {
    let (x0, x1, y0, y1) = clip(rect, sw, sh);
    if x0 >= x1 {
        return;
    }
    let mut y = y0;
    while y < y1 {
        let a = 4 * (y as usize * sw as usize + x0 as usize);
        let z = 4 * (y as usize * sw as usize + x1 as usize);
        if z <= canvas.len() {
            let mut i = a;
            while i < z {
                canvas[i] = 0;
                i += 1;
            }
        }
        y += 1;
    }
}

/// Copy the rectangle (clipped) from `src` to the same offsets of `dst`. Used
/// for the disposal-3 save (canvas → `Saved`) and restore (`Saved` → canvas).
pub fn copy_rect(dst: &mut [u8], src: &[u8], sw: u32, sh: u32, rect: Rect) {
    let (x0, x1, y0, y1) = clip(rect, sw, sh);
    if x0 >= x1 {
        return;
    }
    let mut y = y0;
    while y < y1 {
        let a = 4 * (y as usize * sw as usize + x0 as usize);
        let z = 4 * (y as usize * sw as usize + x1 as usize);
        if z <= dst.len() && z <= src.len() {
            dst[a..z].copy_from_slice(&src[a..z]);
        }
        y += 1;
    }
}

/// The G12.2 drawing sink for one frame. `table` is the active colour table
/// (`3·C` bytes), `transparent` the frame's transparent index.
pub struct Drawer<'c, 't> {
    pub canvas: &'c mut [u8],
    pub table: &'t [u8],
    pub transparent: Option<u8>,
    pub rect: Rect,
    pub interlaced: bool,
    pub screen_w: u32,
    pub screen_h: u32,
    /// Cached stream row: positions `[row_start, row_start + w)` are stream row
    /// `row`, drawn at image row `row_y` (screen row `top + row_y`).
    row: u32,
    row_start: u32,
    row_valid: bool,
    row_y: u32,
}

impl<'c, 't> Drawer<'c, 't> {
    pub fn new(
        canvas: &'c mut [u8],
        table: &'t [u8],
        transparent: Option<u8>,
        rect: Rect,
        interlaced: bool,
        screen_w: u32,
        screen_h: u32,
    ) -> Drawer<'c, 't> {
        Drawer {
            canvas,
            table,
            transparent,
            rect,
            interlaced,
            screen_w,
            screen_h,
            row: 0,
            row_start: 0,
            row_valid: false,
            row_y: 0,
        }
    }
}

impl<'c, 't> Sink for Drawer<'c, 't> {
    fn put(&mut self, q: u32, v: u8) -> bool {
        // G12.2: the transparent index leaves the pixel, even if `v >= C`.
        if let Some(t) = self.transparent {
            if v == t {
                return true;
            }
        }
        let k = 3 * v as usize;
        if k + 3 > self.table.len() {
            return false;
        }
        let w = self.rect.width;
        if w == 0 {
            return true;
        }
        if !(self.row_valid && q >= self.row_start && q - self.row_start < w) {
            let r = q / w;
            self.row = r;
            self.row_start = r * w;
            self.row_valid = true;
            self.row_y = if self.interlaced { interlace_row(r, self.rect.height) } else { r };
        }
        let x = self.rect.left + (q - self.row_start);
        let y = self.rect.top + self.row_y;
        // Clip (a no-op under frame_bounds=Reject, where the rectangle is inside).
        if x >= self.screen_w || y >= self.screen_h {
            return true;
        }
        let o = 4 * (y as usize * self.screen_w as usize + x as usize);
        if o + 4 <= self.canvas.len() {
            self.canvas[o] = self.table[k];
            self.canvas[o + 1] = self.table[k + 1];
            self.canvas[o + 2] = self.table[k + 2];
            self.canvas[o + 3] = 255;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::interlace_row;

    fn reference(r: u32, h: u32) -> u32 {
        let mut r = r;
        for (start, step) in [(0u32, 8u32), (4, 8), (2, 4), (1, 2)] {
            let n = if h > start { (h - start + step - 1) / step } else { 0 };
            if r < n {
                return start + r * step;
            }
            r -= n;
        }
        panic!("row out of range")
    }

    #[test]
    fn interlace_matches_reference_and_is_a_bijection() {
        for h in 1..200u32 {
            let mut seen = [false; 200];
            for r in 0..h {
                let y = interlace_row(r, h);
                assert_eq!(y, reference(r, h));
                assert!(y < h && !seen[y as usize]);
                seen[y as usize] = true;
            }
        }
        for h in [65_535u32, 65_534, 65_533, 65_529] {
            assert_eq!(interlace_row(h - 1, h), reference(h - 1, h));
            assert_eq!(interlace_row(0, h), 0);
        }
    }
}

