#!/usr/bin/env python3
"""XPM (image/x-xpixmap) leniency adapter. Run: ./run.sh --inputs 10000 --seed 731 --jobs 6"""
import sys
from pathlib import Path
here = Path(__file__).resolve().parent
sys.dont_write_bytecode = True
sys.path.insert(0, str(here)); sys.path.append(str(here.parent / 'xbm'))
import lenient_text, xpm_gen

BIN = Path('/tmp/claude-0/leniency-xpm-bin')
RUNNERS = {
    'glycin': (BIN / 'rust-runner', ['glycin']),             # glycin-image-rs ImgLoader -> image-extras xpm
    'image-extras': (BIN / 'rust-runner', ['image-extras']),  # image_extras::xpm::XpmDecoder directly
    'ez-pixmap': (BIN / 'rust-runner', ['ez']),              # ez-pixmap 0.2.2 (naive subset: cpp 1, c key only), fed the quoted strings
    'gdk-file': (BIN / 'gdk', ['xpm', 'file']),
    'gdk-auto-whole': (BIN / 'gdk', ['xpm', 'auto-whole']),   # content-sniffed ("/* XPM */" / "! XPM2" magic)
    'gdk-forced-whole': (BIN / 'gdk', ['xpm', 'forced-whole']),
    'gdk-forced-byte': (BIN / 'gdk', ['xpm', 'forced-byte']),
}
CFG = dict(name='xpm', ext='xpm', runners=RUNNERS, generate=xpm_gen.generate, notes=str(here / 'notes.md'),
           independent={'glycin': 'image-extras', 'image-extras': 'image-extras', 'ez-pixmap': 'ez-pixmap', 'gdk-file': 'gdk-pixbuf',
                        'gdk-auto-whole': 'gdk-pixbuf', 'gdk-forced-whole': 'gdk-pixbuf', 'gdk-forced-byte': 'gdk-pixbuf'},
           unavailable={'libXpm': 'headers/library not installed (no X11/xpm.h, no libXpm.so) and its source host (freedesktop.org) is outside the allowed network; github.com mirror was not accessible',
                        'imagemagick/netpbm': 'no convert/xpmtoppm CLI installed',
                        'sdl-image/imlib2': 'not installed'})
if __name__ == '__main__':
    lenient_text.main(CFG)
