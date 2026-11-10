#!/usr/bin/env python3
"""PNG / APNG (image/png, image/apng) leniency adapter. Shared pipeline: ../gif/anim_harness.py."""
from pathlib import Path
import sys
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE.parent / 'gif'))
import anim_harness as H  # noqa: E402
import pnggen  # noqa: E402

B = H.BIN
RUNNERS = {
    'glycin': (B / 'rust', ['png:glycin']),
    'glycin-apng': (B / 'rust', ['png:glycin-apng']),
    'image': (B / 'rust', ['png:image']),
    'pngcrate': (B / 'rust', ['png:pngcrate']),
    'pngcrate-apng': (B / 'rust', ['png:pngcrate-apng']),
    'libpng': (B / 'png-native', ['libpng']),
    'gdk-anim': (B / 'gdk-anim', ['png', 'anim']),
    'gdk-forced-whole': (B / 'gdk-anim', ['png', 'forced-whole']),
    'gdk-forced-byte': (B / 'gdk-anim', ['png', 'forced-byte']),
    'spng': (B / 'png-native', ['spng']),
    'lodepng': (B / 'rust', ['png:lodepng']),
    'stb': (B / 'png-native', ['stb']),
    'zune': (B / 'rust', ['png:zune']),
    'minipng': (B / 'rust', ['png:minipng']),
}
FAMILIES = {'glycin': 'image-rs/png-crate', 'glycin-apng': 'image-rs/png-crate', 'image': 'image-rs/png-crate',
            'pngcrate': 'image-rs/png-crate', 'pngcrate-apng': 'image-rs/png-crate',
            'libpng': 'libpng', 'gdk-anim': 'libpng', 'gdk-forced-whole': 'libpng', 'gdk-forced-byte': 'libpng',
            'spng': 'spng', 'lodepng': 'lodepng', 'stb': 'stb_image', 'zune': 'zune-png', 'minipng': 'minipng'}
ANIMATING = ['glycin', 'glycin-apng', 'image', 'pngcrate-apng']
NOTES = ('`glycin` / `glycin-apng` call the in-tree `glycin_image_rs::ImgLoader` with mime `image/png` / `image/apng` and request frames with '
         '`loop_animation=false` until `NoMoreFrames` (glycin animates any PNG whose `is_apng()` is true, under either mime). glycin, `image` '
         '(=0.25.10 `PngDecoder`, APNG via `apng().into_frames()`) and `pngcrate` share the `png` crate built from the same image-png git '
         'revision glycin\'s workspace patches in (e917d6a): **one vote**. `pngcrate` = default image with `Transformations::EXPAND`; '
         '`pngcrate-apng` = the png crate\'s fcTL frames through the harness APNG compositor (transparent canvas; dispose 0/1/2, first-frame '
         'PREVIOUS treated as BACKGROUND; blend SOURCE, or OVER on straight 8-bit samples in f32 with truncation, the arithmetic image-rs uses). `libpng` = system libpng 1.6.48 (png_read_row, expand, '
         'gray→RGB, add alpha, expand_16, default CRC and benign-error policy); gdk-pixbuf 2.42.12\'s built-in PNG loader is also libpng, so '
         'GDK and libpng are one family. `spng` (libspng 0.7.x, 46c81dc), `lodepng` crate 3.12 (Rust LodePNG), `stb` (stb_image 2.30), '
         '`zune` (zune-png 0.5.2) and `minipng` 1.0 are independent; none of the non-image-rs decoders implement APNG, so they return the '
         'default (IDAT) image. **16-bit normalisation:** every runner that sees 16-bit samples reports `rgba16_sha256` over exact big-endian '
         'RGBA16 and reduces to 8 bits by the high byte (`v >> 8`, libpng `png_set_strip_16`, which GDK uses); minipng and GDK only expose '
         '8 bits. **Colour management:** no runner applies gAMA, cHRM, sRGB or iCCP; hashes compare raw decoded samples. A `tail_error` '
         'means the decoder returned pixels but reported an error after them (libpng `png_read_end`, spng `spng_decode_chunks`). lodepng: 16-bit '
         'sources use its RGBA16 output (byte-swapped back from the crate\'s big-endian pairs), sources up to 8 bits use `decode32`.')
UNAVAILABLE = ['ImageMagick, ffmpeg, pngcheck: not installed; not built from source.',
               'wuffs PNG (github.com/google/wuffs): buildable, not integrated in this pass.',
               'APNG in libpng/spng/stb/lodepng/zune/minipng: not supported by these builds (system libpng has no APNG patch); they show the default image.',
               'The `png` crate as an independent vote: not independent (image-rs and glycin use it).']
FINDINGS = [
    'GDK returns a pixbuf when IDAT decompression fails: with a bad zlib header, raw deflate without zlib wrapper, too little image data or a bad compression-method byte, `GdkPixbufLoader` (whole and byte writes) closes without error and yields an image filled with a constant grey, while libpng itself, glycin, spng and stb reject (zlib:bad-zlib-header, zlib:raw-deflate, zlib:too-little-data, zlib:bad-cm: 70 of 70 accepted by GDK).',
    'Adler-32: glycin/image-rs, GDK, stb, zune and minipng ignore a wrong zlib Adler-32; libpng, spng and lodepng reject (zlib:bad-adler32). A missing Adler-32 is accepted by glycin (64/70), spng, stb, zune and minipng.',
    'CRC on critical chunks: stb and zune never check CRCs (229 of 229 accepted), minipng accepts 177 and spng 135; libpng (69), glycin (66) and lodepng (40) accept mainly cases where the corrupted CRC belongs to a chunk they do not need (crc:critical).',
    'IHDR compression and filter-method bytes must be 0: lodepng accepts any value (348/348 and 347/347) and zune accepts nonzero filter methods (268/347); every other decoder rejects.',
    'Missing IEND: libpng and GDK accept all 70; glycin accepts 13, lodepng, stb, zune and minipng reject (order:missing-iend). Conversely a wrong IEND length is accepted by glycin, libpng, spng and stb but rejected by lodepng and zune (length:IEND).',
    'Palette images without PLTE (or with PLTE after IDAT, or empty): lodepng, zune and minipng accept and invent colours; glycin, libpng, GDK, spng and stb reject (order:missing-plte). zune also applies a PLTE that appears in a greyscale image to the grey samples (order:plte, rgb-only difference).',
    'APNG: glycin rejects an acTL without any fcTL (58 of 58) and every fcTL sequence-number, size or offset error, whereas non-APNG decoders show the default image; image-rs/glycin reject 16-bit APNG (“does not support the color type Rgba16”); GDK’s file/animation path rejects every APNG (content sniffing yields image/apng, which no GDK loader claims) while the forced PNG loader shows the default image.',
    'tRNS: minipng ignores a greyscale tRNS key (alpha-only difference on gray2/gray8 seeds); lodepng, stb and zune reject a tRNS of wrong length, tRNS in a colour type with alpha or tRNS before PLTE, which libpng, GDK, spng and image-rs accept (order:trns-*). The lodepng crate’s RGBA16 conversion also drops a grey key below 8 bits; the harness uses its RGBA8 path for such sources.',
    'Chunk-type bits: a lower-case third letter (reserved bit) is accepted by glycin, spng and libpng for 52–94 inputs but rejected by GDK (2); turning IDAT/PLTE ancillary by case makes stb, lodepng, zune and minipng reject where libpng-based and image-rs decoders accept (type-bit:*).',
    'Declared IHDR height larger than the data: GDK accepts 225 of 343 (partially filled image) vs 114 for libpng/spng/stb and 91 for glycin; lodepng accepts none (ihdr:height). 16-bit samples agree bit-exactly across every decoder that exposes them; with the high-byte rule the 8-bit hashes also agree, so no 16-bit normalisation disagreement was observed.',
]

FORMAT = H.Format('png', 'png', RUNNERS, FAMILIES, ANIMATING, pnggen.generate, FINDINGS, NOTES, UNAVAILABLE, 'pngcrate-apng')

if __name__ == '__main__':
    H.main(FORMAT)
