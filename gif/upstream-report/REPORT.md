# Out-of-bounds memset in the animated-GIF disposal-2 path

## What and where

`gdk-pixbuf/io-gif-animation.c`, in `gdk_pixbuf_gif_anim_iter_get_pixbuf()`, disposal-method-2 branch (`GDK_PIXBUF_FRAME_DISPOSE`); 2.42.12 lines 447-459, faulting statement the `memset` at line 456. The branch clears the previous frame's rectangle with `memset (pixels + offset, 0, (x_end - x_offset) * 4)`, where `x_end = MIN (x_offset + width, anim->width)`. The GIF image-descriptor left position is not validated against the logical screen width, so a frame whose left edge lies beyond the screen gives `x_end < x_offset` and a negative width of `(x_end - x_offset) * 4`. The `memset` prototype converts that to `size_t`; with the recorded input it is `-8`, so the call receives `18446744073709551608`. The two `g_size_checked_*` guards above check that the byte offset does not overflow; they do not check that the length fits the allocation, so they miss this.

The same code is still present on the development branch, `master` (`main` does not exist): `https://gitlab.gnome.org/GNOME/gdk-pixbuf/-/raw/master/gdk-pixbuf/io-gif-animation.c`, mirrored at `https://raw.githubusercontent.com/GNOME/gdk-pixbuf/master/gdk-pixbuf/io-gif-animation.c` (fetched 2026-11-14), still has the unclipped expression, now at line 458. The last commit changing this branch is `6976bdc8ee9dd2c2954f91066f7b0f643769a379` (authored 2021-06-02, "gif: Check for overflow when compositing or clearing frames"), which added the two overflow guards but no lower-bound check; the only later commit touching the file, `c68ba74e93ff8d0a4e63b5d90c3f1a3f6cc6e3a7` (2025-08-26), marks the animation APIs deprecated and leaves the branch as shown.

## Minimal trigger

A GIF whose disposal-2 frame has its left offset beyond the logical screen width (image descriptor left 6, screen width 4), followed by one more frame; the frame's pixel data is irrelevant. The bug is reached by playing the animation, not by loading alone: the second `gdk_pixbuf_animation_iter_get_pixbuf()` call, after `gdk_pixbuf_animation_iter_advance()`, runs the disposal for the bad frame while rendering the frame after it. `gdk_pixbuf_animation_new_from_file()` alone returns successfully; so does `gdk_pixbuf_new_from_file()`, which takes the static path and never composites later frames; and so does `gdk_pixbuf_loader_write()`/`close()`. The iterator is required.

## Observed effect

AddressSanitizer reports `negative-size-param (size=-8)` in `memset`, destination 24 bytes inside a 32-byte heap allocation (the 4x2 RGBA canvas, rowstride 16). Without a sanitizer the outcome depends on the libc: on the glibc tested here (2.41-12+deb13u4) the `memset` call returned after writing zeros outside the allocation, rather than faulting. Exploitability was not evaluated beyond that observation; this report claims only the out-of-bounds write.

## Versions tested

Runtime `libgdk-pixbuf-2.0-0` `2.42.12+dfsg-4+deb13u1` (Debian 13), source 2.42.12. Found while differential-testing GIF decoders against a corpus of real files, not while targeting any application. Reproduced against a locally built pristine 2.42.12 tree with AddressSanitizer, where the stack names the source line.

## Proposed fix

Compute the clipped width once (`x_end - x_offset`), skip the clear when it is not positive, and pass the width to `memset` as a `gsize`. The row range is already clipped (`y < y_end`); the missing clip is horizontal. The adjacent `GDK_PIXBUF_FRAME_REVERT` branch hands the same unclipped rectangle to `gdk_pixbuf_copy_area()`, whose preconditions it does not satisfy, so the patch guards that call against the screen bounds too and skips the copy instead of returning early with a `g_return_if_fail` warning. `fix.patch` here applies with `patch -p1` and changes nothing for a frame inside the screen.

## Where to send it

Please route this through the project's private security reporting channel rather than the public issue tracker. `CONTRIBUTING.md` on the development branch says not to open an issue for security reports and points at a GNOME security mailing list: `https://gitlab.gnome.org/GNOME/gdk-pixbuf/-/blob/master/CONTRIBUTING.md` (the address appears there as a `mailto:` link). Pointer only, for the sender to verify: GNOME's security pages were not reachable from here and the repository has no `SECURITY.md`.
