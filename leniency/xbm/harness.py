#!/usr/bin/env python3
"""XBM (image/x-xbitmap) leniency adapter. Run: ./run.sh --inputs 10000 --seed 731 --jobs 6"""
import sys
from pathlib import Path
here = Path(__file__).resolve().parent
sys.dont_write_bytecode = True
sys.path.insert(0, str(here))
import lenient_text, xbm_gen

BIN = Path('/tmp/claude-0/leniency-xbm-bin')
RUNNERS = {
    'glycin': (BIN / 'rust-runner', ['glycin']),            # glycin-image-rs ImgLoader -> image-extras xbm
    'image-extras': (BIN / 'rust-runner', ['image-extras']),  # image_extras::xbm::XbmDecoder directly
    'xbm-crate': (BIN / 'rust-runner', ['crate']),          # sorairolake/xbm-rs 0.3.0, independent
    'libx11': (BIN / 'x11', ['ref']),                       # libX11 XReadBitmapFileData (reference C implementation)
    'gdk-file': (BIN / 'gdk', ['xbm', 'file']),             # gdk_pixbuf_new_from_file
    'gdk-forced-whole': (BIN / 'gdk', ['xbm', 'forced-whole']),
    'gdk-forced-byte': (BIN / 'gdk', ['xbm', 'forced-byte']),
}
CFG = dict(name='xbm', ext='xbm', runners=RUNNERS, generate=xbm_gen.generate, notes=str(here / 'notes.md'),
           independent={'glycin': 'image-extras', 'image-extras': 'image-extras', 'xbm-crate': 'xbm-crate', 'libx11': 'libX11',
                        'gdk-file': 'gdk-pixbuf', 'gdk-forced-whole': 'gdk-pixbuf', 'gdk-forced-byte': 'gdk-pixbuf'},
           unavailable={'imagemagick/netpbm': 'no convert/xbmtopbm CLI installed; PyPI (Pillow) and distro packages are outside the allowed network',
                        'gdk-auto-detect': 'GDK has no content magic for XBM: loader auto-detection rejects even valid XBM; only filename (gdk-file) or forced type decodes. Excluded to avoid a constant outlier.'})
if __name__ == '__main__':
    lenient_text.main(CFG)
