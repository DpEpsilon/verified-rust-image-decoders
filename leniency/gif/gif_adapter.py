#!/usr/bin/env python3
"""GIF (image/gif) leniency adapter. See anim_harness.py for the shared pipeline."""
from pathlib import Path
import sys
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import anim_harness as H  # noqa: E402
import gifgen  # noqa: E402

B = H.BIN
RUNNERS = {
    'glycin': (B / 'rust', ['gif:glycin']),
    'image': (B / 'rust', ['gif:image']),
    'gifcrate': (B / 'rust', ['gif:gifcrate']),
    'gdk-anim': (B / 'gdk-anim', ['gif', 'anim']),
    'gdk-forced-whole': (B / 'gdk-anim', ['gif', 'forced-whole']),
    'gdk-forced-byte': (B / 'gdk-anim', ['gif', 'forced-byte']),
    'giflib': (B / 'gif-native', ['giflib']),
    'libnsgif': (B / 'gif-native', ['libnsgif']),
    'stb': (B / 'gif-native', ['stb']),
    'gift': (B / 'rust', ['gif:gift']),
    'tinygif': (B / 'rust', ['gif:tinygif']),
}
FAMILIES = {'glycin': 'image-rs/gif-crate', 'image': 'image-rs/gif-crate', 'gifcrate': 'image-rs/gif-crate',
            'gdk-anim': 'gdk-pixbuf', 'gdk-forced-whole': 'gdk-pixbuf', 'gdk-forced-byte': 'gdk-pixbuf',
            'giflib': 'giflib', 'libnsgif': 'libnsgif', 'stb': 'stb_image', 'gift': 'gift', 'tinygif': 'tinygif'}
ANIMATING = ['glycin', 'image', 'gifcrate', 'gdk-anim', 'giflib', 'libnsgif', 'stb', 'gift', 'tinygif']
NOTES = ('`glycin` calls the in-tree `glycin_image_rs::ImgLoader` (mime `image/gif`) and requests frames with '
         '`loop_animation=false` until `NoMoreFrames`; it wraps `image::GifDecoder` =0.25.10, which wraps the `gif` crate 0.14.2, '
         'so `glycin`, `image` and `gifcrate` are one vote. `gifcrate` uses the gif crate\'s indexed frames with the harness reference '
         'compositor (transparent-black canvas; disposal 2 clears the rectangle to transparent; 3 restores; 0, 1 and undefined 4–7 keep; '
         'out-of-palette indexes paint opaque black); `giflib` (5.1.4 via the github.com/mirrorer/giflib mirror, the only allowed source) uses '
         '`DGifSlurp` with the same compositor. `libnsgif` (NetSurf, c26c7aa), `stb` (stb_image 2.30, 4e9b368), `gift` 0.13 and the GDK animation '
         'iterator composite with their own policies. `tinygif` 0.0.4 is an embedded decoder: it only yields frames introduced by a Graphic '
         'Control Extension, draws without disposal and without alpha. `gdk-anim` = `gdk_pixbuf_animation_new_from_file` with synthetic time '
         'advanced by each frame delay (GDK clamps delays: 0 → 100 ms, < 20 ms → 20 ms); `gdk-forced-*` = `GdkPixbufLoader` forced to `gif`, '
         'whole buffer or one byte per write, static pixbuf. Every hash is over straight RGBA8; GIF has no gamma/ICC handling in any runner. '
         'The family vote of a family is its first listed runner.')
UNAVAILABLE = ['ImageMagick, ffmpeg, gifsicle: not installed; apt is not usable at runtime and they were not built from source.',
               'giflib 5.2.x: upstream lives on SourceForge (not an allowed host); the GitHub mirror provides 5.1.4.',
               'wuffs (github.com/google/wuffs): buildable, not integrated in this pass.',
               'image-rs `gif` crate as an independent vote: not independent (image-rs uses it); shown as `gifcrate` with the reference compositor.']
FINDINGS = [
    'Unknown extension labels (0x21 followed by an unregistered label): the image-rs family (glycin, image, gif crate) rejects them because the gif crate defaults to `allow_unknown_blocks=false`; giflib, libnsgif, GDK, stb and gift skip them (struct:insert-unknown-ext 0 of 51 accepted by glycin, 51 of 51 by the others).',
    'Missing trailer (no 0x3B): glycin, libnsgif, stb and gift accept; giflib and every GDK path reject (struct:no-trailer, 51 inputs).',
    'GDK is the most lenient on LZW streams: it accepts every invalid code, TIFF-style early code-width change and mismatched LZW minimum-code-size byte that the others mostly reject (lzw:invalid-code 51/51 vs giflib 11, glycin 3; lzw:early-change 51/51 vs glycin 10; lzw:min-code-byte 428/428 vs glycin 250).',
    'Short pixel data (the LZW stream ends before width×height pixels): glycin and giflib reject; GDK, stb and libnsgif decode a partial frame (lzw:pixels-short, -one-short, -none).',
    'Frames that extend past the logical screen: glycin, giflib, libnsgif and GDK clip (or grow the canvas, libnsgif) and accept; stb and gift reject (field:left 5 of 384 accepted by stb and gift, 378–384 by the rest).',
    'Header version: giflib and libnsgif accept any three bytes after `GIF` (88a, 89b, spaces); glycin, GDK, stb and gift accept only 87a/89a (header:version).',
    'No colour table at all: giflib and libnsgif accept, glycin, GDK, stb and gift reject (struct:no-palette 37 inputs). A zero-sized logical screen is accepted only by libnsgif, which grows the canvas to the frame (struct:canvas-zero).',
    'Interlace: gift ignores the interlace flag and renders rows in stream order (every interlaced seed: rgb-only pixel difference against all other decoders); stb fills uncovered canvas with the background colour instead of transparency (frame-inset seed, field:frame_width pixel row).',
    'Incremental vs whole-buffer GDK: with one-byte writes the GDK GIF loader rejects an XMP application extension that whole-buffer loading accepts (struct:insert-xmp 0 of 51 vs 51 of 51), and is stricter on other fields (gdk-forced-byte 611 strict vs 423 for whole-buffer).',
    'Frame timing policy (not pixels): GDK maps delay 0 to 100 ms and anything below 20 ms to 20 ms; glycin maps delay 0 to 100 ms but keeps 10 ms; libnsgif reports 100 ms for frames without a GCE; image/gif crate/giflib/stb report raw centiseconds×10; gift exposes no delay. GDK frame counts come from its iterator, so identical consecutive frames can collapse.',
]

FORMAT = H.Format('gif', 'gif', RUNNERS, FAMILIES, ANIMATING, gifgen.generate, FINDINGS, NOTES, UNAVAILABLE, 'gifcrate', observe_only=['tinygif'])

if __name__ == '__main__':
    H.main(FORMAT)
