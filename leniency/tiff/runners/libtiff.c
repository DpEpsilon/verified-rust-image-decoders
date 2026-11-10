/* libtiff decoding column: first IFD via TIFFReadRGBAImageOriented(ORIENTATION_TOPLEFT,
 * stop_on_error=1), the same RGBA interface GDK's TIFF module uses. libtiff's RGBA
 * interface returns associated (premultiplied) alpha; this runner un-premultiplies
 * so the output is straight RGBA8 like every other column.
 * usage: libtiff-runner FILE [RGBA_DUMP]
 */
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <tiffio.h>
#include "../../jpeg/runners/json.h"

static char last_error[512]; static int warnings; static char first_warning[512];
static void on_error(const char *m, const char *fmt, va_list ap) {
    (void)m; vsnprintf(last_error, sizeof last_error, fmt, ap);
}
static void on_warning(const char *m, const char *fmt, va_list ap) {
    (void)m; if (!warnings) vsnprintf(first_warning, sizeof first_warning, fmt, ap); warnings++;
}
int main(int argc, char **argv) {
    if (argc < 2) { json_error("usage", "usage: libtiff-runner FILE [DUMP]"); return 0; }
    TIFFSetErrorHandler(on_error); TIFFSetWarningHandler(on_warning);
    TIFF *t = TIFFOpen(argv[1], "r");
    if (!t) { json_error("TIFFOpen", last_error); return 0; }
    uint32_t w = 0, h = 0; uint16_t bps = 0, spp = 0, comp = 0, photo = 0, planar = 0, fmt = 1, orient = 1;
    TIFFGetField(t, TIFFTAG_IMAGEWIDTH, &w); TIFFGetField(t, TIFFTAG_IMAGELENGTH, &h);
    TIFFGetFieldDefaulted(t, TIFFTAG_BITSPERSAMPLE, &bps); TIFFGetFieldDefaulted(t, TIFFTAG_SAMPLESPERPIXEL, &spp);
    TIFFGetFieldDefaulted(t, TIFFTAG_COMPRESSION, &comp); TIFFGetField(t, TIFFTAG_PHOTOMETRIC, &photo);
    TIFFGetFieldDefaulted(t, TIFFTAG_PLANARCONFIG, &planar); TIFFGetFieldDefaulted(t, TIFFTAG_SAMPLEFORMAT, &fmt);
    TIFFGetFieldDefaulted(t, TIFFTAG_ORIENTATION, &orient);
    int tiled = TIFFIsTiled(t), big = TIFFIsBigEndian(t);
    if ((unsigned long long)w * h > 16000000ULL || !w || !h) { json_error("limit", "output dimensions outside harness limit"); TIFFClose(t); return 0; }
    char emsg[1024] = "";
    if (!TIFFRGBAImageOK(t, emsg)) { json_error("TIFFRGBAImageOK", emsg); TIFFClose(t); return 0; }
    uint32_t *raster = _TIFFmalloc((tmsize_t)w * h * 4);
    if (!raster) { json_error("alloc", "raster"); TIFFClose(t); return 0; }
    if (!TIFFReadRGBAImageOriented(t, w, h, raster, ORIENTATION_TOPLEFT, 1)) {
        json_error("TIFFReadRGBAImage", last_error[0] ? last_error : "read failed"); TIFFClose(t); return 0;
    }
    unsigned char *rgba = malloc((size_t)w * h * 4);
    for (size_t i = 0; i < (size_t)w * h; i++) {
        uint32_t p = raster[i]; unsigned a = TIFFGetA(p), c[3] = {TIFFGetR(p), TIFFGetG(p), TIFFGetB(p)};
        for (int k = 0; k < 3; k++) {
            unsigned v = a == 255 ? c[k] : a ? (c[k] * 255 + a / 2) / a : 0;
            rgba[i * 4 + k] = v > 255 ? 255 : v;
        }
        rgba[i * 4 + 3] = a;
    }
    int pages = TIFFNumberOfDirectories(t);
    char orig[160], extra[900], wm[600];
    snprintf(orig, sizeof orig, "photometric%u/%ubit/%uspp/fmt%u/comp%u/planar%u%s%s", photo, bps, spp, fmt, comp, planar,
             tiled ? "/tiled" : "/strips", big ? "/MM" : "/II");
    json_escape(first_warning, wm, sizeof wm);
    snprintf(extra, sizeof extra, "\"components\":%u,\"bit_depth\":%u,\"orientation\":%u,\"orientation_applied\":true,\"pages\":%d,\"warnings\":%d,\"first_warning\":\"%s\"",
             spp, bps, orient, pages, warnings, wm);
    json_ok(w, h, orig, rgba, argc > 2 ? argv[2] : NULL, extra);
    _TIFFfree(raster); free(rgba); TIFFClose(t);
    return 0;
}
