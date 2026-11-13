/* Shared result accumulator for the GIF/PNG native runners.
 *
 * Every native runner feeds fully composited, top-first, straight RGBA8 canvases
 * (one per displayed frame) into acc_frame() and finishes with acc_ok() or fail().
 * The JSON contract matches gif/src/main.rs:
 *   rgba_sha256  SHA-256 of the first composited frame
 *   seq_sha256   SHA-256 over the concatenation of all composited frames (capped)
 *   frames       number of composited frames fed (capped at ACC_MAX_FRAMES)
 *   frame_info   [[delay_ms, disposal, x, y, w, h, blend], ...] for the first 16 frames
 */
#include <glib.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>

#define ACC_MAX_FRAMES 64
#define ACC_MAX_PIXELS 16000000ULL
#define ACC_MAX_TOTAL 64000000ULL

typedef struct {
    int w, h, n, capped, alpha;
    GChecksum *seq;
    char *first_hash;
    guchar *first;
    GString *info;
} Acc;

static void json_str(GString *j, const char *s) {
    char *v = g_utf8_make_valid(s ? s : "", -1);
    for (const unsigned char *p = (const unsigned char *)v; *p; p++) {
        if (*p == '"' || *p == '\\') { g_string_append_c(j, '\\'); g_string_append_c(j, *p); }
        else if (*p < 32) g_string_append_printf(j, "\\u%04x", *p);
        else g_string_append_c(j, *p);
    }
    g_free(v);
}

static void fail(const char *cls, const char *msg) {
    GString *j = g_string_new("{\"status\":\"error\",\"error_class\":\"");
    json_str(j, cls);
    g_string_append(j, "\",\"message\":\"");
    char buf[401]; snprintf(buf, sizeof buf, "%s", msg ? msg : "");
    json_str(j, buf);
    g_string_append(j, "\"}");
    puts(j->str); fflush(stdout);
    g_string_free(j, TRUE);
}

static int acc_init(Acc *a, int w, int h) {
    memset(a, 0, sizeof *a);
    if (w <= 0 || h <= 0 || (guint64)w * h > ACC_MAX_PIXELS) return 0;
    a->w = w; a->h = h;
    a->seq = g_checksum_new(G_CHECKSUM_SHA256);
    a->info = g_string_new("[");
    return 1;
}

/* Returns 0 when the frame or pixel budget is exhausted (caller should stop). */
static int acc_frame(Acc *a, const guchar *rgba, long delay_ms, int disposal,
                     long x, long y, long fw, long fh, int blend) {
    gsize len = (gsize)a->w * a->h * 4;
    if (a->n >= ACC_MAX_FRAMES || (guint64)(a->n + 1) * a->w * a->h > ACC_MAX_TOTAL) { a->capped = 1; return 0; }
    g_checksum_update(a->seq, rgba, len);
    if (a->n == 0) {
        a->first = g_memdup2(rgba, len);
        a->first_hash = g_compute_checksum_for_data(G_CHECKSUM_SHA256, rgba, len);
        for (gsize i = 3; i < len; i += 4) if (rgba[i] != 255) { a->alpha = 1; break; }
    }
    if (a->n < 16)
        g_string_append_printf(a->info, "%s[%ld,%d,%ld,%ld,%ld,%ld,%d]", a->n ? "," : "",
                               delay_ms, disposal, x, y, fw, fh, blend);
    a->n++;
    return a->n < ACC_MAX_FRAMES;
}

static void acc_ok(Acc *a, const char *orig, const char *selection, const char *dump, const char *extra) {
    if (!a->n) { fail("NoFrames", "decoder produced no frame"); return; }
    if (dump) g_file_set_contents(dump, (const char *)a->first, (gssize)a->w * a->h * 4, NULL);
    printf("{\"status\":\"ok\",\"width\":%d,\"height\":%d,\"frames\":%d,\"frames_capped\":%s,"
           "\"orig_color_type\":\"%s\",\"alpha_present\":%s,\"rgba_sha256\":\"%s\",\"seq_sha256\":\"%s\","
           "\"frame_info\":%s],\"selection\":\"%s\"%s%s}\n",
           a->w, a->h, a->n, a->capped ? "true" : "false", orig, a->alpha ? "true" : "false",
           a->first_hash, g_checksum_get_string(a->seq), a->info->str, selection,
           extra ? "," : "", extra ? extra : "");
    fflush(stdout);
}

/* Reference compositor used for raw-frame libraries (giflib): GIF89a semantics,
 * canvas starts transparent black, disposal 2 clears the frame rectangle to
 * transparent (not the background colour), disposal 3 restores the canvas saved
 * before the frame, disposal 0/1 and the undefined 4..7 keep the frame. */
typedef struct { guchar *canvas, *saved; int pdisp; long px, py, pw, ph; } Comp;

static void comp_init(Comp *c, Acc *a) {
    c->canvas = g_malloc0((gsize)a->w * a->h * 4);
    c->saved = g_malloc0((gsize)a->w * a->h * 4);
    c->pdisp = 0;
}

static void comp_dispose_prev(Comp *c, Acc *a) {
    if (c->pdisp == 2) {
        for (long y = c->py; y < c->py + c->ph && y < a->h; y++)
            for (long x = c->px; x < c->px + c->pw && x < a->w; x++)
                memset(c->canvas + ((gsize)y * a->w + x) * 4, 0, 4);
    } else if (c->pdisp == 3) {
        memcpy(c->canvas, c->saved, (gsize)a->w * a->h * 4);
    }
}

/* index: fw*fh palette indexes; pal: 256*3; transparent: -1 or index */
static void comp_draw(Comp *c, Acc *a, const unsigned char *index, long fx, long fy, long fw, long fh,
                      const unsigned char *pal, int ncolors, int transparent, int disposal) {
    comp_dispose_prev(c, a);
    if (disposal == 3) memcpy(c->saved, c->canvas, (gsize)a->w * a->h * 4);
    for (long y = 0; y < fh; y++) {
        long cy = fy + y; if (cy < 0 || cy >= a->h) continue;
        for (long x = 0; x < fw; x++) {
            long cx = fx + x; if (cx < 0 || cx >= a->w) continue;
            int i = index[(gsize)y * fw + x];
            if (i == transparent) continue;
            guchar *d = c->canvas + ((gsize)cy * a->w + cx) * 4;
            if (i < ncolors) { d[0] = pal[i*3]; d[1] = pal[i*3+1]; d[2] = pal[i*3+2]; d[3] = 255; }
            else { d[0] = d[1] = d[2] = 0; d[3] = 255; }
        }
    }
    c->pdisp = disposal; c->px = fx; c->py = fy; c->pw = fw; c->ph = fh;
}
