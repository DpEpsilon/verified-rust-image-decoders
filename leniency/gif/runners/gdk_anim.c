/* gdk-pixbuf runner for GIF and PNG/APNG.
 * usage: gdk-anim TYPE MODE FILE [RGBA_DUMP]
 *   TYPE  gif | png (passed to gdk_pixbuf_loader_new_with_type for forced-* modes)
 *   MODE  anim         gdk_pixbuf_animation_new_from_file, composited frames from the
 *                      animation iterator, advancing synthetic time by each frame delay
 *         forced-whole loader forced to TYPE, whole-buffer write, static pixbuf (first frame)
 *         forced-byte  same with one-byte writes (incremental parser path)
 *         forced-anim  forced loader, whole buffer, then iterate gdk_pixbuf_loader_get_animation
 * Frame count for "anim": the iterator is advanced until get_delay_time() is -1
 * (GDK stops after the last loop) or, for an infinitely looping GIF, until the
 * (hash, delay) sequence returns to frame 0 with the observed period.
 */
#include <gdk-pixbuf/gdk-pixbuf.h>
#include <gio/gio.h>
#include "acc.h"

static int to_rgba(GdkPixbuf *p, guchar *out, int w, int h) {
    int n = gdk_pixbuf_get_n_channels(p), stride = gdk_pixbuf_get_rowstride(p);
    gboolean alpha = gdk_pixbuf_get_has_alpha(p);
    if (gdk_pixbuf_get_width(p) != w || gdk_pixbuf_get_height(p) != h || n < 3 || n > 4) return 0;
    const guchar *src = gdk_pixbuf_read_pixels(p);
    for (int y = 0; y < h; y++) for (int x = 0; x < w; x++) {
        const guchar *s = src + (gsize)y * stride + (gsize)x * n; guchar *d = out + ((gsize)y * w + x) * 4;
        d[0] = s[0]; d[1] = s[1]; d[2] = s[2]; d[3] = alpha ? s[3] : 255;
    }
    return 1;
}

static void run_anim(GdkPixbufAnimation *anim, const char *dump, const char *sel) {
    int w = gdk_pixbuf_animation_get_width(anim), h = gdk_pixbuf_animation_get_height(anim);
    Acc a;
    if (!acc_init(&a, w, h)) { fail("Dimensions", "animation size outside harness limit"); return; }
    GTimeVal t = {1000000, 0};
    G_GNUC_BEGIN_IGNORE_DEPRECATIONS
    GdkPixbufAnimationIter *it = gdk_pixbuf_animation_get_iter(anim, &t);
    G_GNUC_END_IGNORE_DEPRECATIONS
    enum { STEPS = ACC_MAX_FRAMES * 2 };
    gsize len = (gsize)w * h * 4;
    int maxsteps = (int)MIN((guint64)STEPS, MAX(1ULL, 32000000ULL / ((guint64)w * h)));
    guchar *frames = g_try_malloc(len * maxsteps);
    if (!frames) { fail("OutOfMemory", "frame buffer"); return; }
    char *hashes[STEPS]; int delays[STEPS]; int n = 0, terminated = 0;
    const char *orig = "RGB8";
    while (n < maxsteps) {
        GdkPixbuf *p = gdk_pixbuf_animation_iter_get_pixbuf(it);
        if (!p || !to_rgba(p, frames + len * n, w, h)) { fail("FrameSize", "iterator pixbuf missing or size differs from animation"); return; }
        if (gdk_pixbuf_get_has_alpha(p)) orig = "RGBA8";
        int d = gdk_pixbuf_animation_iter_get_delay_time(it);
        /* After the last loop GDK keeps showing the final frame with delay -1:
         * that state is not a new frame unless it is the very first (static) one. */
        if (d < 0 && n > 0) { terminated = 1; break; }
        hashes[n] = g_compute_checksum_for_data(G_CHECKSUM_SHA256, frames + len * n, len); delays[n] = d; n++;
        if (d < 0 || gdk_pixbuf_animation_is_static_image(anim)) { terminated = 1; break; }
        long us = (long)t.tv_usec + (long)d * 1000;
        t.tv_sec += us / 1000000; t.tv_usec = us % 1000000;
        G_GNUC_BEGIN_IGNORE_DEPRECATIONS
        gdk_pixbuf_animation_iter_advance(it, &t);
        G_GNUC_END_IGNORE_DEPRECATIONS
    }
    /* GDK exposes no frame count: take the smallest period of the (pixels, delay)
     * sequence (loops repeat it); identical consecutive frames can collapse. */
    int period = n;
    for (int p = 1; p < n; p++) {
        if (terminated ? n % p : n < 2 * p) continue;
        int ok = 1;
        for (int i = 0; i + p < n && ok; i++) ok = delays[i] == delays[i + p] && !strcmp(hashes[i], hashes[i + p]);
        if (ok) { period = p; break; }
    }
    for (int i = 0; i < period; i++) if (!acc_frame(&a, frames + len * i, delays[i], -1, 0, 0, w, h, -1)) break;
    if (!terminated && period == n) a.capped = 1;
    char extra[64]; snprintf(extra, sizeof extra, "\"loop_period\":%d,\"gdk_steps\":%d", period, n);
    acc_ok(&a, orig, sel, dump, extra);
}

int main(int argc, char **argv) {
    if (argc < 4) { fail("usage", "gdk-anim TYPE MODE FILE [DUMP]"); return 0; }
    const char *type = argv[1], *mode = argv[2], *path = argv[3], *dump = argc > 4 ? argv[4] : NULL;
    GError *e = NULL;
    if (!strcmp(mode, "anim")) {
        GdkPixbufAnimation *anim = gdk_pixbuf_animation_new_from_file(path, &e);
        if (!anim) { fail("gdk", e ? e->message : "no animation"); return 0; }
        run_anim(anim, dump, gdk_pixbuf_animation_is_static_image(anim) ? "static" : "animation");
        return 0;
    }
    gchar *bytes = NULL; gsize len = 0;
    if (!g_file_get_contents(path, &bytes, &len, &e)) { fail("io", e->message); return 0; }
    GdkPixbufLoader *loader = gdk_pixbuf_loader_new_with_type(type, &e);
    if (!loader) { fail("gdk", e ? e->message : "no loader"); return 0; }
    gsize step = !strcmp(mode, "forced-byte") ? 1 : (len ? len : 1);
    for (gsize i = 0; i < len; i += step)
        if (!gdk_pixbuf_loader_write(loader, (guchar *)bytes + i, MIN(step, len - i), &e)) break;
    gboolean closed = FALSE;
    if (!e) closed = gdk_pixbuf_loader_close(loader, &e);
    else { GError *e2 = NULL; gdk_pixbuf_loader_close(loader, &e2); if (e2) g_error_free(e2); }
    if (!closed) { fail("gdk", e ? e->message : "close failed"); return 0; }
    if (!strcmp(mode, "forced-anim")) {
        GdkPixbufAnimation *anim = gdk_pixbuf_loader_get_animation(loader);
        if (!anim) { fail("gdk", "no animation"); return 0; }
        run_anim(anim, dump, gdk_pixbuf_animation_is_static_image(anim) ? "static" : "animation");
        return 0;
    }
    GdkPixbuf *p = gdk_pixbuf_loader_get_pixbuf(loader);
    if (!p) { fail("gdk", "no pixbuf"); return 0; }
    int w = gdk_pixbuf_get_width(p), h = gdk_pixbuf_get_height(p);
    Acc a;
    if (!acc_init(&a, w, h)) { fail("Dimensions", "pixbuf size outside harness limit"); return 0; }
    guchar *buf = g_malloc((gsize)w * h * 4);
    if (!to_rgba(p, buf, w, h)) { fail("Channels", "unsupported pixbuf layout"); return 0; }
    acc_frame(&a, buf, -1, -1, 0, 0, w, h, -1);
    acc_ok(&a, gdk_pixbuf_get_has_alpha(p) ? "RGBA8" : "RGB8", "static", dump, NULL);
    return 0;
}
