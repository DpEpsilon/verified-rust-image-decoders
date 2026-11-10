/* One-line JSON result helpers shared by the native JPEG/TIFF runners. */
#ifndef LENIENCY_JSON_H
#define LENIENCY_JSON_H
#include <glib.h>
#include <stdio.h>
#include <string.h>

static void json_escape(const char *s, char *out, size_t cap) {
    size_t o = 0;
    for (const unsigned char *p = (const unsigned char *)(s ? s : ""); *p && o + 7 < cap; p++) {
        if (*p == '"' || *p == '\\') { out[o++] = '\\'; out[o++] = *p; }
        else if (*p < 32 || *p > 126) o += snprintf(out + o, cap - o, "\\u%04x", *p);
        else out[o++] = *p;
    }
    out[o] = 0;
}
static void json_error(const char *cls, const char *msg) {
    char a[256], b[1024];
    json_escape(cls, a, sizeof a); json_escape(msg, b, sizeof b);
    if (strlen(b) > 400) b[400] = 0;
    printf("{\"status\":\"error\",\"error_class\":\"%s\",\"message\":\"%s\"}\n", a, b);
    fflush(stdout);
}
/* rgba: w*h*4 straight top-first RGBA8; extra: already-formatted JSON members or NULL. */
static void json_ok(int w, int h, const char *orig, const unsigned char *rgba, const char *dump, const char *extra) {
    gsize n = (gsize)w * h * 4; int alpha = 0;
    for (gsize i = 3; i < n; i += 4) if (rgba[i] != 255) { alpha = 1; break; }
    char *hash = g_compute_checksum_for_data(G_CHECKSUM_SHA256, rgba, n);
    if (dump) g_file_set_contents(dump, (const char *)rgba, (gssize)n, NULL);
    char o[256]; json_escape(orig, o, sizeof o);
    printf("{\"status\":\"ok\",\"width\":%d,\"height\":%d,\"orig_color_type\":\"%s\",\"alpha_present\":%s,\"rgba_sha256\":\"%s\"%s%s}\n",
           w, h, o, alpha ? "true" : "false", hash, extra && *extra ? "," : "", extra ? extra : "");
    fflush(stdout);
    g_free(hash);
}
#endif
