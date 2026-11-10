/* GDK-Pixbuf column for JPEG (built-in loader) and TIFF (libpixbufloader-tiff.so).
 * usage: gdk-jt FORMAT MODE FILE [RGBA_DUMP]
 * MODE file: gdk_pixbuf_new_from_file; forced-whole / forced-chunk: GdkPixbufLoader
 * created for FORMAT, fed the whole buffer or 4096-byte writes.
 * The RGBA hash is the pixbuf as decoded (no rotation); "orientation" is the
 * pixbuf option GDK exposes and oriented_* describe gdk_pixbuf_apply_embedded_orientation.
 */
#include <gdk-pixbuf/gdk-pixbuf.h>
#include <stdlib.h>
#include "json.h"

static unsigned char *to_rgba(GdkPixbuf *p) {
    int w = gdk_pixbuf_get_width(p), h = gdk_pixbuf_get_height(p), n = gdk_pixbuf_get_n_channels(p);
    int stride = gdk_pixbuf_get_rowstride(p); gboolean alpha = gdk_pixbuf_get_has_alpha(p);
    const guchar *src = gdk_pixbuf_read_pixels(p);
    unsigned char *out = g_malloc((gsize)w * h * 4);
    for (int y = 0; y < h; y++) for (int x = 0; x < w; x++) {
        const guchar *a = src + (gsize)y * stride + (gsize)x * n; unsigned char *b = out + ((gsize)y * w + x) * 4;
        b[0] = a[0]; b[1] = a[1]; b[2] = a[2]; b[3] = alpha ? a[3] : 255;
    }
    return out;
}
int main(int argc, char **argv) {
    if (argc < 4) { json_error("usage", "usage: gdk-jt FORMAT MODE FILE [DUMP]"); return 0; }
    const char *fmt = argv[1], *mode = argv[2];
    GError *e = NULL; GdkPixbuf *p = NULL;
    if (!strcmp(mode, "file")) p = gdk_pixbuf_new_from_file(argv[3], &e);
    else {
        gchar *bytes = NULL; gsize len = 0;
        if (g_file_get_contents(argv[3], &bytes, &len, &e)) {
            GdkPixbufLoader *l = gdk_pixbuf_loader_new_with_type(fmt, &e);
            if (l) {
                gsize step = strstr(mode, "chunk") ? 4096 : (len ? len : 1);
                for (gsize i = 0; i < len; i += step) if (!gdk_pixbuf_loader_write(l, (guchar *)bytes + i, MIN(step, len - i), &e)) break;
                if (!e) gdk_pixbuf_loader_close(l, &e); else gdk_pixbuf_loader_close(l, NULL);
                if (!e && gdk_pixbuf_loader_get_pixbuf(l)) p = g_object_ref(gdk_pixbuf_loader_get_pixbuf(l));
                g_object_unref(l);
            }
            g_free(bytes);
        }
    }
    if (!p) {
        char cls[64] = "gdk";
        if (e) snprintf(cls, sizeof cls, "gdk-%s-%d", g_quark_to_string(e->domain), e->code);
        /* GDK file-path messages embed the input path; strip it from the class, keep the message. */
        json_error(cls, e ? e->message : "no pixbuf");
        if (e) g_error_free(e);
        return 0;
    }
    int w = gdk_pixbuf_get_width(p), h = gdk_pixbuf_get_height(p), n = gdk_pixbuf_get_n_channels(p);
    if (w <= 0 || h <= 0 || (guint64)w * h > 16000000 || n < 3 || n > 4) { json_error("limit", "output size or channels outside harness limit"); return 0; }
    unsigned char *rgba = to_rgba(p);
    const char *orient = gdk_pixbuf_get_option(p, "orientation");
    GdkPixbuf *o = gdk_pixbuf_apply_embedded_orientation(p);
    unsigned char *orgba = to_rgba(o);
    int ow = gdk_pixbuf_get_width(o), oh = gdk_pixbuf_get_height(o);
    char *ohash = g_compute_checksum_for_data(G_CHECKSUM_SHA256, orgba, (gsize)ow * oh * 4);
    char extra[400], os[64];
    json_escape(orient ? orient : "none", os, sizeof os);
    snprintf(extra, sizeof extra, "\"components\":%d,\"bit_depth\":%d,\"orientation\":\"%s\",\"orientation_applied\":false,\"oriented_width\":%d,\"oriented_height\":%d,\"oriented_sha256\":\"%s\"%s",
             n, gdk_pixbuf_get_bits_per_sample(p), os, ow, oh, ohash, e ? ",\"warnings\":1" : "");
    json_ok(w, h, gdk_pixbuf_get_has_alpha(p) ? "RGBA8" : "RGB8", rgba, argc > 4 ? argv[4] : NULL, extra);
    g_free(ohash); g_free(rgba); g_free(orgba); g_object_unref(o); g_object_unref(p);
    if (e) g_error_free(e);
    return 0;
}
