/* In-process gdk-pixbuf decode benchmark, same CLI/output as the Rust `bench` binary.
   gdkbench FORMAT check|time LISTFILE [PASSES] [WARMUP]
   Decode = GdkPixbufLoader (forced type), one write of the whole in-memory file, close, unref.
   No file I/O in the timed region. */
#include <gdk-pixbuf/gdk-pixbuf.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <time.h>
#include <sys/resource.h>

typedef struct { char *path; guchar *data; gsize len; } File;
static GdkPixbuf *decode(const char *fmt, const File *f, char **err) {
    GError *e = NULL;
    GdkPixbufLoader *l = gdk_pixbuf_loader_new_with_type(fmt, &e);
    if (!l) { *err = g_strdup(e ? e->message : "loader"); if (e) g_error_free(e); return NULL; }
    GdkPixbuf *p = NULL;
    if (gdk_pixbuf_loader_write(l, f->data, f->len, &e) && gdk_pixbuf_loader_close(l, &e)) {
        p = gdk_pixbuf_loader_get_pixbuf(l);
        if (p) g_object_ref(p);
    }
    if (!p) *err = g_strdup(e ? e->message : "no pixbuf");
    if (e) g_error_free(e);
    if (p == NULL) gdk_pixbuf_loader_close(l, NULL);
    g_object_unref(l);
    return p;
}
static long status_kb(const char *key) { (void)key; struct rusage r; return getrusage(RUSAGE_SELF, &r) ? 0 : r.ru_maxrss; }
static unsigned long long cpu_ns(void) { struct timespec t; clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &t); return t.tv_sec * 1000000000ULL + t.tv_nsec; }
static unsigned long long now_ns(void) { struct timespec t; clock_gettime(CLOCK_MONOTONIC, &t); return t.tv_sec * 1000000000ULL + t.tv_nsec; }
int main(int argc, char **argv) {
    if (argc < 4) { fprintf(stderr, "usage: gdkbench FORMAT check|time LIST [PASSES] [WARMUP]\n"); return 2; }
    const char *fmt = argv[1];
    FILE *lf = fopen(argv[3], "r"); if (!lf) return 2;
    File *files = NULL; size_t n = 0; char line[4096];
    while (fgets(line, sizeof line, lf)) {
        line[strcspn(line, "\n")] = 0; if (!line[0]) continue;
        files = realloc(files, (n + 1) * sizeof *files);
        files[n].path = strdup(line); gchar *d; gsize len;
        if (!g_file_get_contents(line, &d, &len, NULL)) return 2;
        files[n].data = (guchar *)d; files[n].len = len; n++;
    }
    fclose(lf);
    if (!strcmp(argv[2], "check")) {
        for (size_t i = 0; i < n; i++) {
            char *err = NULL; GdkPixbuf *p = decode(fmt, &files[i], &err);
            if (!p) { for (char *c = err; *c; c++) if (*c == '\t' || *c == '\n') *c = ' '; printf("C\t%s\terr\t0\t0\t%s\n", files[i].path, err); g_free(err); continue; }
            int w = gdk_pixbuf_get_width(p), h = gdk_pixbuf_get_height(p), ch = gdk_pixbuf_get_n_channels(p), st = gdk_pixbuf_get_rowstride(p);
            gboolean alpha = gdk_pixbuf_get_has_alpha(p); guchar *src = gdk_pixbuf_get_pixels(p);
            if (ch < 3 || ch > 4) { printf("C\t%s\terr\t0\t0\tchannels\n", files[i].path); continue; }
            guchar *out = g_malloc((gsize)w * h * 4);
            for (int y = 0; y < h; y++) for (int x = 0; x < w; x++) {
                guchar *a = src + (gsize)y * st + (gsize)x * ch, *b = out + ((gsize)y * w + x) * 4;
                b[0] = a[0]; b[1] = a[1]; b[2] = a[2]; b[3] = alpha ? a[3] : 255;
            }
            char *hash = g_compute_checksum_for_data(G_CHECKSUM_SHA256, out, (gsize)w * h * 4);
            printf("C\t%s\tok\t%d\t%d\t%s\n", files[i].path, w, h, hash);
            g_free(hash); g_free(out); g_object_unref(p);
        }
        return 0;
    }
    int passes = argc > 4 ? atoi(argv[4]) : 7, warm = argc > 5 ? atoi(argv[5]) : 2;
    long baseline = status_kb("VmHWM:"); unsigned long long pixels = 0;
    for (int w = 0; w < warm; w++) for (size_t i = 0; i < n; i++) {
        char *err = NULL; GdkPixbuf *p = decode(fmt, &files[i], &err);
        if (!p) { fprintf(stderr, "decode failed: %s\n", err); return 1; }
        if (w == 0) pixels += (unsigned long long)gdk_pixbuf_get_width(p) * gdk_pixbuf_get_height(p);
        g_object_unref(p);
    }
    unsigned long long tcal = now_ns();
    for (size_t i = 0; i < n; i++) { char *err = NULL; GdkPixbuf *p = decode(fmt, &files[i], &err); if (p) g_object_unref(p); }
    tcal = now_ns() - tcal; if (!tcal) tcal = 1;
    unsigned long long reps = (100000000ULL + tcal - 1) / tcal; if (!reps) reps = 1;
    pixels *= reps;
    unsigned long long total_c0 = cpu_ns();
    unsigned long long *wall = malloc(passes * sizeof *wall), *cpu = malloc(passes * sizeof *cpu);
    for (int k = 0; k < passes; k++) {
        unsigned long long c0 = cpu_ns(), t0 = now_ns();
        for (unsigned long long r = 0; r < reps; r++) for (size_t i = 0; i < n; i++) { char *err = NULL; GdkPixbuf *p = decode(fmt, &files[i], &err); if (p) g_object_unref(p); }
        wall[k] = now_ns() - t0; cpu[k] = cpu_ns() - c0;
    }
    unsigned long long total_cpu = cpu_ns() - total_c0;
    printf("T\t");
    for (int k = 0; k < passes; k++) printf("%s%llu", k ? "," : "", wall[k]);
    printf("\t");
    for (int k = 0; k < passes; k++) printf("%s%llu", k ? "," : "", cpu[k]);
    printf("\t%llu\t%zu\t%ld\t%ld\t%llu\n", pixels, n, baseline, status_kb("VmHWM:"), total_cpu);
    return 0;
}
