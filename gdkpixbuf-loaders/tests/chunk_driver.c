/* Test driver for tests/compare.py (our own code; uses only public
 * gdk-pixbuf API). Feeds FILE to a GdkPixbufLoader in CHUNK-byte writes
 * (0 = whole file in one write) and prints one JSON line: status, size,
 * has_alpha, the SHA-256 of the RGBA8 image (alpha forced to 255 when the
 * pixbuf has none, matching leniency/bmp/runners/gdk.c), the module that
 * handled it, and when each loader signal first fired (bytes written so far).
 *
 *   chunk_driver CHUNK FILE
 *   chunk_driver info FILE      gdk_pixbuf_get_file_info() result
 *   chunk_driver type FILE      whole write via gdk_pixbuf_loader_new_with_type("bmp")
 *   chunk_driver save -         try gdk_pixbuf_save_to_buffer(..., "bmp")
 */
#include <gdk-pixbuf/gdk-pixbuf.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static gsize written;
static long size_at = -1, prep_at = -1, upd_at = -1;
static int n_size, n_prep, n_upd;
static GString *order;

static void on_size(GdkPixbufLoader *l, int w, int h, gpointer d) {
    if (size_at < 0) size_at = (long)written;
    n_size++; g_string_append_c(order, 'S');
}
static void on_prep(GdkPixbufLoader *l, gpointer d) {
    if (prep_at < 0) prep_at = (long)written;
    n_prep++; g_string_append_c(order, 'P');
}
static void on_upd(GdkPixbufLoader *l, int x, int y, int w, int h, gpointer d) {
    if (upd_at < 0) upd_at = (long)written;
    n_upd++; if (order->len < 8) g_string_append_c(order, 'U');
}

static void print_err(const char *m) {
    GString *s = g_string_new(NULL);
    for (const char *p = m; *p; p++) {
        if (*p == '"' || *p == '\\') g_string_append_c(s, '\\');
        if ((unsigned char)*p >= 32) g_string_append_c(s, *p);
    }
    printf("{\"status\":\"error\",\"message\":\"%s\"}\n", s->str);
    g_string_free(s, TRUE);
}

int main(int argc, char **argv) {
    if (argc < 3) { fprintf(stderr, "usage: chunk_driver CHUNK|info FILE\n"); return 2; }
    if (!strcmp(argv[1], "info")) {
        int w = 0, h = 0;
        GdkPixbufFormat *f = gdk_pixbuf_get_file_info(argv[2], &w, &h);
        printf("{\"status\":\"%s\",\"width\":%d,\"height\":%d,\"format\":\"%s\"}\n",
               f ? "ok" : "error", w, h, f ? gdk_pixbuf_format_get_description(f) : "");
        return 0;
    }
    if (!strcmp(argv[1], "save")) { /* can this cache still write BMP? */
        GdkPixbuf *p = gdk_pixbuf_new(GDK_COLORSPACE_RGB, FALSE, 8, 1, 1);
        GError *e = NULL; gchar *buf = NULL; gsize n = 0;
        gboolean ok = gdk_pixbuf_save_to_buffer(p, &buf, &n, "bmp", &e, NULL);
        if (ok) printf("{\"status\":\"ok\",\"bytes\":%lu}\n", (unsigned long)n);
        else print_err(e ? e->message : "save failed");
        return 0;
    }
    order = g_string_new(NULL);
    gboolean by_type = !strcmp(argv[1], "type"); /* gdk_pixbuf_loader_new_with_type("bmp") */
    gsize chunk = by_type ? 0 : (gsize)strtoul(argv[1], NULL, 10);
    gchar *bytes = NULL; gsize len = 0; GError *e = NULL;
    if (!g_file_get_contents(argv[2], &bytes, &len, &e)) { print_err(e->message); return 0; }
    if (chunk == 0) chunk = len ? len : 1;
    GdkPixbufLoader *loader = by_type ? gdk_pixbuf_loader_new_with_type("bmp", &e) : gdk_pixbuf_loader_new();
    if (!loader) { print_err(e->message); return 0; }
    g_signal_connect(loader, "size-prepared", G_CALLBACK(on_size), NULL);
    g_signal_connect(loader, "area-prepared", G_CALLBACK(on_prep), NULL);
    g_signal_connect(loader, "area-updated", G_CALLBACK(on_upd), NULL);
    gboolean ok = TRUE;
    for (gsize i = 0; i < len && ok; i += chunk) {
        gsize n = MIN(chunk, len - i);
        written = i + n; /* signals fired inside this write see the new total */
        ok = gdk_pixbuf_loader_write(loader, (guchar *)bytes + i, n, &e);
    }
    gboolean closed = gdk_pixbuf_loader_close(loader, ok ? &e : NULL);
    GdkPixbufFormat *fmt = gdk_pixbuf_loader_get_format(loader);
    const char *desc = fmt ? gdk_pixbuf_format_get_description(fmt) : "";
    GdkPixbuf *p = gdk_pixbuf_loader_get_pixbuf(loader);
    if (!ok || !closed || !p) {
        print_err(e ? e->message : "no pixbuf");
        g_object_unref(loader); g_free(bytes); return 0;
    }
    int w = gdk_pixbuf_get_width(p), h = gdk_pixbuf_get_height(p), nc = gdk_pixbuf_get_n_channels(p);
    int stride = gdk_pixbuf_get_rowstride(p);
    gboolean alpha = gdk_pixbuf_get_has_alpha(p);
    const guchar *src = gdk_pixbuf_read_pixels(p);
    guchar *out = g_malloc((gsize)w * h * 4);
    for (int y = 0; y < h; y++)
        for (int x = 0; x < w; x++) {
            const guchar *v = src + (gsize)y * stride + (gsize)x * nc;
            guchar *q = out + ((gsize)y * w + x) * 4;
            q[0] = v[0]; q[1] = v[1]; q[2] = v[2]; q[3] = alpha ? v[3] : 255;
        }
    gchar *hash = g_compute_checksum_for_data(G_CHECKSUM_SHA256, out, (gsize)w * h * 4);
    printf("{\"status\":\"ok\",\"width\":%d,\"height\":%d,\"has_alpha\":%s,\"rgba_sha256\":\"%s\","
           "\"format\":\"%s\",\"len\":%lu,\"size_at\":%ld,\"prepared_at\":%ld,\"updated_at\":%ld,"
           "\"n_size\":%d,\"n_prepared\":%d,\"n_updated\":%d,\"order\":\"%s\"}\n",
           w, h, alpha ? "true" : "false", hash, desc, (unsigned long)len, size_at, prep_at, upd_at,
           n_size, n_prep, n_upd, order->str);
    g_free(hash); g_free(out); g_object_unref(loader); g_free(bytes);
    return 0;
}
