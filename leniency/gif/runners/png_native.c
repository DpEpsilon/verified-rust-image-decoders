/* Native PNG runners (default image only: none of these builds implements APNG).
 * usage: png-native SELECTOR FILE [RGBA_DUMP]
 *   libpng  system libpng (png_read_row API; expand, gray->rgb, add alpha; 16-bit kept;
 *           no gamma/ICC transform; library-default CRC and benign-error policy;
 *           png_read_end() failures after the pixels decode are reported as tail_error)
 *   spng    libspng spng_decode_image(SPNG_FMT_RGBA16, SPNG_DECODE_TRNS), then spng_decode_chunks()
 *   stb     stb_image (stbi_load_16_from_memory for 16-bit sources)
 * 16-bit samples: high byte for rgba_sha256, exact big-endian RGBA16 for rgba16_sha256.
 */
#include "acc.h"
#include <png.h>
#include <setjmp.h>
#include <spng.h>
#define STB_IMAGE_IMPLEMENTATION
#define STBI_ONLY_PNG
#define STBI_NO_STDIO
#include "stb_image.h"

static char *hash16(const guint16 *v, gsize count) {
    GChecksum *c = g_checksum_new(G_CHECKSUM_SHA256);
    for (gsize i = 0; i < count; i++) { guchar be[2] = {v[i] >> 8, v[i] & 255}; g_checksum_update(c, be, 2); }
    char *s = g_strdup(g_checksum_get_string(c)); g_checksum_free(c); return s;
}
static void finish16(Acc *a, guint16 *v, int depth16, const char *orig, const char *sel, const char *dump, const char *tail) {
    gsize count = (gsize)a->w * a->h * 4; guchar *b8 = g_malloc(count);
    for (gsize i = 0; i < count; i++) b8[i] = v[i] >> 8;
    acc_frame(a, b8, -1, -1, 0, 0, a->w, a->h, -1);
    GString *extra = g_string_new("");
    if (depth16) { char *h = hash16(v, count); g_string_append_printf(extra, "\"rgba16_sha256\":\"%s\"", h); g_free(h); }
    if (tail) { if (extra->len) g_string_append_c(extra, ','); g_string_append(extra, "\"tail_error\":\""); json_str(extra, tail); g_string_append_c(extra, '"'); }
    acc_ok(a, orig, sel, dump, extra->len ? extra->str : NULL);
}

typedef struct { jmp_buf jb; char msg[300]; int warnings; const guchar *p; gsize n, off; } PngCtx;
static void png_err(png_structp p, png_const_charp m) { PngCtx *c = png_get_error_ptr(p); snprintf(c->msg, sizeof c->msg, "%s", m); longjmp(c->jb, 1); }
static void png_warn(png_structp p, png_const_charp m) { PngCtx *c = png_get_error_ptr(p); c->warnings++; (void)m; }
static void png_rd(png_structp p, png_bytep out, size_t n) {
    PngCtx *c = png_get_io_ptr(p);
    if (n > c->n - c->off) png_error(p, "Read Error: unexpected end of data");
    memcpy(out, c->p + c->off, n); c->off += n;
}

static void run_libpng(const guchar *b, gsize n, const char *dump) {
    static PngCtx c; memset(&c, 0, sizeof c); c.p = b; c.n = n;
    png_structp p = png_create_read_struct(PNG_LIBPNG_VER_STRING, &c, png_err, png_warn);
    png_infop info = png_create_info_struct(p);
    static guint16 *v16; static guchar *row; static Acc a; static int decoded; decoded = 0; v16 = NULL; row = NULL;
    static int depth; static char orig[64];
    if (setjmp(c.jb)) {
        if (decoded) { finish16(&a, v16, depth == 16, orig, "default-image", dump, c.msg); return; }
        fail("libpng", c.msg); return;
    }
    png_set_read_fn(p, &c, png_rd);
    png_read_info(p, info);
    png_uint_32 w = png_get_image_width(p, info), h = png_get_image_height(p, info);
    int ct = png_get_color_type(p, info); depth = png_get_bit_depth(p, info);
    snprintf(orig, sizeof orig, "ct%d/%d", ct, depth);
    if (!acc_init(&a, w, h)) { fail("Dimensions", "canvas outside harness limit"); return; }
    png_set_expand(p);
    png_set_gray_to_rgb(p);
    png_set_add_alpha(p, 0xffff, PNG_FILLER_AFTER);
    png_set_expand_16(p);   /* uniform 16-bit output; 8-bit sources become v*257 */
    int passes = png_set_interlace_handling(p);
    png_read_update_info(p, info);
    gsize rb = png_get_rowbytes(p, info);
    if (rb != (gsize)w * 8) png_error(p, "unexpected row layout");
    row = g_malloc0(rb * h); v16 = g_malloc0((gsize)w * h * 8);
    for (int pass = 0; pass < passes; pass++) for (png_uint_32 y = 0; y < h; y++) png_read_row(p, row + y * rb, NULL);
    for (gsize i = 0; i < (gsize)w * h * 4; i++) v16[i] = (row[2*i] << 8) | row[2*i+1];
    decoded = 1;
    png_read_end(p, NULL);
    char warn[64] = ""; if (c.warnings) snprintf(warn, sizeof warn, "%d libpng warning(s)", c.warnings);
    finish16(&a, v16, depth == 16, orig, "default-image", dump, NULL);
}

static void run_spng(const guchar *b, gsize n, const char *dump) {
    spng_ctx *c = spng_ctx_new(0);
    spng_set_image_limits(c, 1u << 24, 1u << 24);
    spng_set_chunk_limits(c, 64u << 20, 256u << 20);
    int r = spng_set_png_buffer(c, b, n);
    struct spng_ihdr ih; size_t size;
    if (!r) r = spng_get_ihdr(c, &ih);
    if (!r) r = spng_decoded_image_size(c, SPNG_FMT_RGBA16, &size);
    if (r) { fail("spng", spng_strerror(r)); return; }
    Acc a;
    if (!acc_init(&a, ih.width, ih.height) || size != (size_t)ih.width * ih.height * 8) { fail("Dimensions", "canvas outside harness limit"); return; }
    guint16 *v = g_malloc(size);
    r = spng_decode_image(c, v, size, SPNG_FMT_RGBA16, SPNG_DECODE_TRNS);
    if (r) { fail("spng", spng_strerror(r)); return; }
    int tail = spng_decode_chunks(c);
    char orig[64]; snprintf(orig, sizeof orig, "ct%d/%d", ih.color_type, ih.bit_depth);
    finish16(&a, v, ih.bit_depth == 16, orig, "default-image", dump, tail ? spng_strerror(tail) : NULL);
}

static void run_stb(const guchar *b, gsize n, const char *dump) {
    int w, h, comp;
    if (n > INT_MAX) { fail("size", "too large"); return; }
    int is16 = stbi_is_16_bit_from_memory(b, (int)n);
    Acc a; guint16 *v;
    if (is16) {
        stbi_us *px = stbi_load_16_from_memory(b, (int)n, &w, &h, &comp, 4);
        if (!px) { fail("stbi", stbi_failure_reason()); return; }
        if (!acc_init(&a, w, h)) { fail("Dimensions", "canvas outside harness limit"); return; }
        v = px;
    } else {
        stbi_uc *px = stbi_load_from_memory(b, (int)n, &w, &h, &comp, 4);
        if (!px) { fail("stbi", stbi_failure_reason()); return; }
        if (!acc_init(&a, w, h)) { fail("Dimensions", "canvas outside harness limit"); return; }
        v = g_malloc((gsize)w * h * 8);
        for (gsize i = 0; i < (gsize)w * h * 4; i++) v[i] = px[i] * 257;
    }
    char orig[32]; snprintf(orig, sizeof orig, "comp%d/%d", comp, is16 ? 16 : 8);
    finish16(&a, v, is16, orig, "default-image", dump, NULL);
}

int main(int argc, char **argv) {
    if (argc < 3) { fail("usage", "png-native SELECTOR FILE [DUMP]"); return 0; }
    gchar *b = NULL; gsize n = 0; GError *e = NULL;
    if (!g_file_get_contents(argv[2], &b, &n, &e)) { fail("io", e->message); return 0; }
    const char *dump = argc > 3 ? argv[3] : NULL;
    if (!strcmp(argv[1], "libpng")) run_libpng((guchar *)b, n, dump);
    else if (!strcmp(argv[1], "spng")) run_spng((guchar *)b, n, dump);
    else if (!strcmp(argv[1], "stb")) run_stb((guchar *)b, n, dump);
    else fail("usage", "unknown selector");
    return 0;
}
