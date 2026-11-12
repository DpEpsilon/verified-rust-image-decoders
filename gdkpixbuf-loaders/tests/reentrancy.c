/* Reentrancy driver for tests/reentrancy.rs (our own code; public gdk-pixbuf
 * API only).
 *
 *   reentrancy ACTION SIGNAL
 *
 * ACTION is close | write | closewrite | none; SIGNAL is size | prepared |
 * updated.  The driver builds a tiny valid 24-bit BMP in memory (no corpus
 * bytes are read or embedded), creates a real GdkPixbufLoader, connects
 * size-prepared, area-prepared and area-updated handlers, writes the whole file
 * and closes.  The handler for SIGNAL, on its first invocation only, re-enters
 * the loader from inside the module's stop_load callback:
 *
 *   close      gdk_pixbuf_loader_close(loader, &err)
 *   write      gdk_pixbuf_loader_write(loader, extra, n, &err)
 *   closewrite both, in that order
 *   none       nothing (control run)
 *
 * It records the return value and the GError (domain, code) of that re-entrant
 * call and of the outer close, and prints one JSON line.  Build and run under
 * AddressSanitizer (see tests/reentrancy.rs): a stop_load that frees its context
 * twice aborts with a double-free report before any line is printed.
 */
#include <gdk-pixbuf/gdk-pixbuf.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static const char *g_action;
static const char *g_signal;
static int g_acted;            /* the trigger handler has already re-entered */

static int n_size, n_prep, n_upd;
static int re_close_rc = -1, re_close_domain = 0, re_close_code = 0;
static int re_write_rc = -1, re_write_domain = 0, re_write_code = 0;
static int outer_close_rc = -1, outer_close_domain = 0, outer_close_code = 0;
static int n_critical, n_warning;

/* Count GLib log messages instead of letting them litter stderr. */
static void log_handler(const gchar *domain, GLogLevelFlags level, const gchar *msg, gpointer d) {
    (void)domain; (void)msg; (void)d;
    if (level & (G_LOG_LEVEL_CRITICAL | G_LOG_LEVEL_ERROR)) n_critical++;
    else if (level & G_LOG_LEVEL_WARNING) n_warning++;
}

static gboolean is_trigger(const char *signal) { return strcmp(g_signal, signal) == 0; }

/* The bytes handed to a re-entrant write; content is irrelevant, only that a
 * write is attempted from inside a callback. */
static const guchar extra[16] = { 'B','M',0,0,0,0,0,0, 54,0,0,0, 40,0,0,0 };

static void reenter(GdkPixbufLoader *loader) {
    GError *err = NULL;
    if (strcmp(g_action, "close") == 0 || strcmp(g_action, "closewrite") == 0) {
        re_close_rc = gdk_pixbuf_loader_close(loader, &err) ? 1 : 0;
        if (err) { re_close_domain = err->domain; re_close_code = err->code; g_error_free(err); err = NULL; }
    }
    if (strcmp(g_action, "write") == 0 || strcmp(g_action, "closewrite") == 0) {
        re_write_rc = gdk_pixbuf_loader_write(loader, extra, sizeof extra, &err) ? 1 : 0;
        if (err) { re_write_domain = err->domain; re_write_code = err->code; g_error_free(err); err = NULL; }
    }
}

static void on_size(GdkPixbufLoader *l, int w, int h, gpointer d) {
    (void)w; (void)h; (void)d;
    n_size++;
    if (is_trigger("size") && !g_acted) { g_acted = 1; reenter(l); }
}
static void on_prep(GdkPixbufLoader *l, gpointer d) {
    (void)d;
    n_prep++;
    if (is_trigger("prepared") && !g_acted) { g_acted = 1; reenter(l); }
}
static void on_upd(GdkPixbufLoader *l, int x, int y, int w, int h, gpointer d) {
    (void)x; (void)y; (void)w; (void)h; (void)d;
    n_upd++;
    if (is_trigger("updated") && !g_acted) { g_acted = 1; reenter(l); }
}

static void le16(guchar *p, unsigned v) { p[0] = v & 0xff; p[1] = (v >> 8) & 0xff; }
static void le32(guchar *p, unsigned v) { p[0] = v & 0xff; p[1] = (v >> 8) & 0xff; p[2] = (v >> 16) & 0xff; p[3] = (v >> 24) & 0xff; }

/* 2x2 24-bit bottom-up BMP: 54-byte header + two 6-byte rows + 2 pad bytes. */
static guchar *make_bmp(gsize *len) {
    guchar *b = g_malloc(70);
    memcpy(b, "BM", 2);
    le32(b + 2, 70); le32(b + 6, 0); le32(b + 10, 54);
    le32(b + 14, 40); le32(b + 18, 2); le32(b + 22, 2);
    le16(b + 26, 1); le16(b + 28, 24);
    for (int i = 30; i < 54; i += 4) le32(b + i, 0);
    memcpy(b + 54, "\0\0\xff\0\xff\0\0\0", 8);      /* bottom row: red, green */
    memcpy(b + 62, "\xff\0\0\xff\xff\xff\0\0", 8);  /* top row: blue, white */
    *len = 70;
    return b;
}

int main(int argc, char **argv) {
    if (argc < 3) { fprintf(stderr, "usage: reentrancy ACTION SIGNAL\n"); return 2; }
    g_action = argv[1];
    g_signal = argv[2];
    /* g_log_set_handler(NULL, ...) only catches the NULL log domain, while
     * g_return_val_if_fail logs under "GdkPixbuf"; the default handler catches
     * every domain, so it sees gdk-pixbuf's own criticals too. */
    g_log_set_default_handler(log_handler, NULL);

    gsize len = 0;
    guchar *bmp = make_bmp(&len);
    GdkPixbufLoader *loader = gdk_pixbuf_loader_new();
    g_signal_connect(loader, "size-prepared", G_CALLBACK(on_size), NULL);
    g_signal_connect(loader, "area-prepared", G_CALLBACK(on_prep), NULL);
    g_signal_connect(loader, "area-updated", G_CALLBACK(on_upd), NULL);

    GError *werr = NULL;
    gboolean wrote = gdk_pixbuf_loader_write(loader, bmp, len, &werr);
    GError *cerr = NULL;
    outer_close_rc = gdk_pixbuf_loader_close(loader, &cerr) ? 1 : 0;
    if (cerr) { outer_close_domain = cerr->domain; outer_close_code = cerr->code; g_error_free(cerr); }

    GdkPixbuf *p = gdk_pixbuf_loader_get_pixbuf(loader);
    printf("{\"action\":\"%s\",\"signal\":\"%s\",\"status\":\"ok\","
           "\"wrote\":%d,\"n_size\":%d,\"n_prepared\":%d,\"n_updated\":%d,"
           "\"re_close_rc\":%d,\"re_close_domain\":%u,\"re_close_code\":%d,"
           "\"re_write_rc\":%d,\"re_write_domain\":%u,\"re_write_code\":%d,"
           "\"outer_close_rc\":%d,\"outer_close_domain\":%u,\"outer_close_code\":%d,"
           "\"pixbuf\":%s,\"critical\":%d,\"warning\":%d}\n",
           g_action, g_signal, wrote ? 1 : 0, n_size, n_prep, n_upd,
           re_close_rc, (unsigned)re_close_domain, re_close_code,
           re_write_rc, (unsigned)re_write_domain, re_write_code,
           outer_close_rc, (unsigned)outer_close_domain, outer_close_code,
           p ? "true" : "false", n_critical, n_warning);

    if (werr) g_error_free(werr);
    g_object_unref(loader);
    g_free(bmp);
    return 0;
}
