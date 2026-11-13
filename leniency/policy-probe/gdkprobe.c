/* gdkprobe FORMAT MODE FILE...  (MODE: forced-whole | forced-byte | auto-whole | file)
   One JSON line per file: status, w, h, has_alpha, sha (first 16 hex), rgba hex if <= 64 px.
   Output is straight RGBA8, top row first; RGB pixbufs get alpha 255. */
#include <gdk-pixbuf/gdk-pixbuf.h>
#include <stdio.h>
#include <string.h>

static void jstr(const char *s) {
    for (const unsigned char *p = (const unsigned char *)s; *p; p++) {
        if (*p == '"' || *p == '\\') putchar('\\');
        putchar(*p < 32 ? ' ' : *p);
    }
}
int main(int argc, char **argv) {
    if (argc < 4) return 2;
    const char *fmt = argv[1], *mode = argv[2];
    for (int k = 3; k < argc; k++) {
        GError *e = NULL; GdkPixbuf *p = NULL; gchar *bytes = NULL; gsize len = 0;
        if (!strcmp(mode, "file")) p = gdk_pixbuf_new_from_file(argv[k], &e);
        else if (g_file_get_contents(argv[k], &bytes, &len, &e)) {
            GdkPixbufLoader *l = !strncmp(mode, "forced", 6) ? gdk_pixbuf_loader_new_with_type(fmt, &e) : gdk_pixbuf_loader_new();
            if (l) {
                gsize step = strstr(mode, "byte") ? 1 : (len ? len : 1);
                gboolean wok = TRUE;
                for (gsize i = 0; i < len; i += step)
                    if (!gdk_pixbuf_loader_write(l, (guchar *)bytes + i, MIN(step, len - i), &e)) { wok = FALSE; break; }
                if (wok) gdk_pixbuf_loader_close(l, &e); else gdk_pixbuf_loader_close(l, NULL);
                if (!e && gdk_pixbuf_loader_get_pixbuf(l)) p = g_object_ref(gdk_pixbuf_loader_get_pixbuf(l));
                g_object_unref(l);
            }
            g_free(bytes);
        }
        printf("{\"file\":\""); jstr(argv[k]); printf("\",\"r\":");
        if (!p) { printf("{\"status\":\"error\",\"error\":\""); jstr(e ? e->message : "no pixbuf"); printf("\"}}\n"); if (e) g_error_free(e); fflush(stdout); continue; }
        int w = gdk_pixbuf_get_width(p), h = gdk_pixbuf_get_height(p), n = gdk_pixbuf_get_n_channels(p), st = gdk_pixbuf_get_rowstride(p);
        gboolean a = gdk_pixbuf_get_has_alpha(p);
        guchar *src = gdk_pixbuf_get_pixels(p), *out = g_malloc((gsize)w * h * 4);
        for (int y = 0; y < h; y++) for (int x = 0; x < w; x++) {
            guchar *s = src + (gsize)y * st + (gsize)x * n, *d = out + ((gsize)y * w + x) * 4;
            d[0] = s[0]; d[1] = s[1]; d[2] = s[2]; d[3] = a ? s[3] : 255;
        }
        char *hash = g_compute_checksum_for_data(G_CHECKSUM_SHA256, out, (gsize)w * h * 4);
        printf("{\"status\":\"ok\",\"w\":%d,\"h\":%d,\"has_alpha\":%s,\"sha\":\"%.16s\",\"rgba\":\"", w, h, a ? "true" : "false", hash);
        if ((gsize)w * h <= 64) for (gsize i = 0; i < (gsize)w * h * 4; i++) printf("%02x", out[i]);
        printf("\"}}\n"); fflush(stdout);
        g_free(hash); g_free(out); g_object_unref(p); if (e) g_error_free(e);
    }
    return 0;
}
