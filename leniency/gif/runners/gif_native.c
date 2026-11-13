/* Native GIF runners. usage: gif-native SELECTOR FILE [RGBA_DUMP]
 *   giflib   giflib DGifSlurp (raw frames) + reference compositor from acc.h
 *   libnsgif libnsgif's own frame scheduler and compositor (nsgif_frame_prepare/decode)
 *   stb      stb_image stbi_load_gif_from_memory (stb's own compositor)
 */
#include "acc.h"
#include <gif_lib.h>
#include <nsgif.h>
#define STB_IMAGE_IMPLEMENTATION
#define STBI_ONLY_GIF
#define STBI_NO_STDIO
#include "stb_image.h"

typedef struct { const guchar *p; gsize n, off; } Mem;
static int mem_read(GifFileType *g, GifByteType *buf, int n) {
    Mem *m = g->UserData; gsize k = MIN((gsize)n, m->n - m->off);
    memcpy(buf, m->p + m->off, k); m->off += k; return (int)k;
}

static void run_giflib(const guchar *b, gsize n, const char *dump) {
    Mem m = {b, n, 0}; int e = 0;
    GifFileType *g = DGifOpen(&m, mem_read, &e);
    if (!g) { fail("DGifOpen", GifErrorString(e)); return; }
    if (DGifSlurp(g) != GIF_OK) {
        char msg[200]; snprintf(msg, sizeof msg, "%s (frames slurped: %d)", GifErrorString(g->Error), g->ImageCount);
        fail("DGifSlurp", msg); DGifCloseFile(g, &e); return;
    }
    Acc a; Comp c;
    if (!acc_init(&a, g->SWidth, g->SHeight)) { fail("Dimensions", "logical screen outside harness limit"); return; }
    comp_init(&c, &a);
    for (int i = 0; i < g->ImageCount; i++) {
        SavedImage *s = &g->SavedImages[i]; GifImageDesc *d = &s->ImageDesc;
        ColorMapObject *cm = d->ColorMap ? d->ColorMap : g->SColorMap;
        unsigned char pal[768] = {0}; int nc = 0;
        if (cm) { nc = MIN(cm->ColorCount, 256); for (int k = 0; k < nc; k++) { pal[3*k] = cm->Colors[k].Red; pal[3*k+1] = cm->Colors[k].Green; pal[3*k+2] = cm->Colors[k].Blue; } }
        GraphicsControlBlock gcb = {0, false, 0, NO_TRANSPARENT_COLOR};
        DGifSavedExtensionToGCB(g, i, &gcb);
        if (!s->RasterBits) { fail("NoRaster", "frame without raster"); return; }
        comp_draw(&c, &a, s->RasterBits, d->Left, d->Top, d->Width, d->Height, pal, nc, gcb.TransparentColor, gcb.DisposalMode);
        if (!acc_frame(&a, c.canvas, (long)gcb.DelayTime * 10, gcb.DisposalMode, d->Left, d->Top, d->Width, d->Height, -1)) break;
    }
    DGifCloseFile(g, &e);
    acc_ok(&a, "Indexed8", "harness-compositor", dump, NULL);
}

static nsgif_bitmap_t *bm_create(int w, int h) { return calloc((size_t)w * h, 4); }
static void bm_destroy(nsgif_bitmap_t *b) { free(b); }
static uint8_t *bm_buffer(nsgif_bitmap_t *b) { return b; }

static void run_nsgif(const guchar *b, gsize n, const char *dump) {
    nsgif_bitmap_cb_vt vt = {bm_create, bm_destroy, bm_buffer, NULL, NULL, NULL, NULL};
    nsgif_t *g; nsgif_error e = nsgif_create(&vt, NSGIF_BITMAP_FMT_R8G8B8A8, &g);
    if (e != NSGIF_OK) { fail("nsgif_create", nsgif_strerror(e)); return; }
    nsgif_error scan = nsgif_data_scan(g, n, b);
    nsgif_data_complete(g);
    const nsgif_info_t *info = nsgif_get_info(g);
    if (info->frame_count == 0) { fail(scan == NSGIF_OK ? "NoFrames" : "nsgif_data_scan", nsgif_strerror(scan)); nsgif_destroy(g); return; }
    Acc a;
    if (!acc_init(&a, info->width, info->height)) { fail("Dimensions", "canvas outside harness limit"); nsgif_destroy(g); return; }
    const char *tail = NULL;
    for (uint32_t k = 0; k < info->frame_count + 1; k++) {
        nsgif_rect_t area; uint32_t delay, fi;
        e = nsgif_frame_prepare(g, &area, &delay, &fi);
        if (e == NSGIF_ERR_ANIMATION_END || (k > 0 && e == NSGIF_ERR_FRAME_DISPLAY)) break;
        if (e != NSGIF_OK) { if (!a.n) { fail("nsgif_frame_prepare", nsgif_strerror(e)); return; } tail = nsgif_strerror(e); break; }
        if (k > 0 && fi == 0) break;  /* wrapped: one pass is enough */
        nsgif_bitmap_t *bm;
        e = nsgif_frame_decode(g, fi, &bm);
        if (e != NSGIF_OK) { if (!a.n) { fail("nsgif_frame_decode", nsgif_strerror(e)); return; } tail = nsgif_strerror(e); break; }
        if (info->width != (uint32_t)a.w || info->height != (uint32_t)a.h) { tail = "canvas-size-changed"; break; }
        const nsgif_frame_info_t *f = nsgif_get_frame_info(g, fi);
        if (!acc_frame(&a, bm, f ? (long)f->delay * 10 : -1, f ? f->disposal : -1, f ? f->rect.x0 : 0, f ? f->rect.y0 : 0,
                       f ? (long)f->rect.x1 - f->rect.x0 : 0, f ? (long)f->rect.y1 - f->rect.y0 : 0, -1)) break;
    }
    char extra[200] = "";
    if (scan != NSGIF_OK || tail) snprintf(extra, sizeof extra, "\"tail_error\":\"%s\"", tail ? tail : nsgif_strerror(scan));
    acc_ok(&a, "RGBA8", "nsgif-frames", dump, extra[0] ? extra : NULL);
    nsgif_destroy(g);
}

static void run_stb(const guchar *b, gsize n, const char *dump) {
    int *delays = NULL, w, h, z, comp;
    if (n > INT_MAX) { fail("size", "too large"); return; }
    stbi_uc *px = stbi_load_gif_from_memory(b, (int)n, &delays, &w, &h, &z, &comp, 4);
    if (!px) { fail("stbi", stbi_failure_reason()); return; }
    Acc a;
    if (!acc_init(&a, w, h)) { fail("Dimensions", "canvas outside harness limit"); return; }
    for (int i = 0; i < z; i++)
        if (!acc_frame(&a, px + (gsize)i * w * h * 4, delays ? delays[i] : -1, -1, 0, 0, w, h, -1)) break;
    acc_ok(&a, "RGBA8", "stb-frames", dump, NULL);
}

int main(int argc, char **argv) {
    if (argc < 3) { fail("usage", "gif-native SELECTOR FILE [DUMP]"); return 0; }
    gchar *b = NULL; gsize n = 0; GError *e = NULL;
    if (!g_file_get_contents(argv[2], &b, &n, &e)) { fail("io", e->message); return 0; }
    const char *dump = argc > 3 ? argv[3] : NULL;
    if (!strcmp(argv[1], "giflib")) run_giflib((guchar *)b, n, dump);
    else if (!strcmp(argv[1], "libnsgif")) run_nsgif((guchar *)b, n, dump);
    else if (!strcmp(argv[1], "stb")) run_stb((guchar *)b, n, dump);
    else fail("usage", "unknown selector");
    return 0;
}
