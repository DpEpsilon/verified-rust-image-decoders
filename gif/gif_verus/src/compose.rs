//! G11 interlace and G12 compositing — the M1/M4 module's public surface.
//!
//! The plain M1 bodies that used to live here (an unverified `interlace_row`,
//! `clip`, `clear_rect`, `copy_rect`, the interval-cached `Drawer` sink and its
//! `Sink` impl) are gone. They were M1 stand-ins that no verifier had touched,
//! and their `Sink` impl cannot satisfy `lzw::Sink`'s log/validity contract
//! (they had no `sink_ok`/`sink_log`). The proved M4 module
//! (`crate::compose_new`, `compose_proof/src/compose.rs`) supplies `Rect`,
//! `Frame`, `DrawState`, `draw_index`, `clear_rect`, `copy_rect` and the
//! `DrawerSink` adapter; `crate::interlace` supplies the proved `interlace_row`.
//!
//! The re-exports below keep the two M1 import paths (`crate::compose::Rect`
//! in `parse.rs`, `crate::compose::{clear_rect, copy_rect, Drawer, Rect}` in
//! `gif.rs`) working without editing those files; `Drawer` now names
//! `DrawerSink`, whose `put` carries the G12.2 contract.
//!
//! `Drawer`'s `gif.rs` call site still uses `Drawer::new(canvas, colors,
//! transparent, rect, interlaced, sw, sh)` and is not wired to the proved
//! helper or to `lzw::decode`'s new report signature; that is M5b's wiring step.

pub use crate::compose_new::{
    clear_rect, copy_rect, draw_index, new_state, DrawState, Frame, Rect, DrawerSink as Drawer,
};
pub use crate::interlace::interlace_row;
