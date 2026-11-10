/* Deterministic synthetic JPEG encoder built on the system libjpeg(-turbo).
 *
 * usage: jpeggen OUT RAW key=value...
 *   RAW is width*height*in_components bytes of samples produced by the Python
 *   generator (gray, RGB or CMYK order), so pixels have one documented source.
 * keys: w h in=gray|rgb|cmyk  cs=gray|ycc|rgb|cmyk|ycck  samp=11,11,11 (h,v per component)
 *       q=1..100 baseline=0|1 prog=0|1 arith=0|1 opt=0|1 rst=N (MCUs) rstrows=N
 *       jfif=0|1 adobe=0|1 scans=interleaved|separate dct=islow|ifast|float
 *       exif=N (APP1 Exif with orientation N, 0 = none) icc=N (ICC APP2 blob of N bytes)
 *       com=N (COM marker of N bytes) smooth=0..100
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <jpeglib.h>

static int geti(int argc, char **argv, const char *k, int def) {
    size_t n = strlen(k);
    for (int i = 3; i < argc; i++) if (!strncmp(argv[i], k, n) && argv[i][n] == '=') return atoi(argv[i] + n + 1);
    return def;
}
static const char *gets_(int argc, char **argv, const char *k, const char *def) {
    size_t n = strlen(k);
    for (int i = 3; i < argc; i++) if (!strncmp(argv[i], k, n) && argv[i][n] == '=') return argv[i] + n + 1;
    return def;
}
static J_COLOR_SPACE cs_of(const char *s) {
    if (!strcmp(s, "gray")) return JCS_GRAYSCALE;
    if (!strcmp(s, "rgb")) return JCS_RGB;
    if (!strcmp(s, "cmyk")) return JCS_CMYK;
    if (!strcmp(s, "ycck")) return JCS_YCCK;
    return JCS_YCbCr;
}
int main(int argc, char **argv) {
    if (argc < 3) { fprintf(stderr, "usage\n"); return 2; }
    int w = geti(argc, argv, "w", 16), h = geti(argc, argv, "h", 16);
    const char *in = gets_(argc, argv, "in", "rgb");
    int inc = !strcmp(in, "gray") ? 1 : !strcmp(in, "cmyk") ? 4 : 3;
    FILE *rf = fopen(argv[2], "rb");
    if (!rf) return 3;
    size_t need = (size_t)w * h * inc;
    unsigned char *raw = malloc(need);
    if (fread(raw, 1, need, rf) != need) return 4;
    fclose(rf);
    struct jpeg_compress_struct c; struct jpeg_error_mgr e;
    c.err = jpeg_std_error(&e);
    jpeg_create_compress(&c);
    FILE *of = fopen(argv[1], "wb");
    if (!of) return 5;
    jpeg_stdio_dest(&c, of);
    c.image_width = w; c.image_height = h; c.input_components = inc;
    c.in_color_space = inc == 1 ? JCS_GRAYSCALE : inc == 4 ? JCS_CMYK : JCS_RGB;
    jpeg_set_defaults(&c);
    jpeg_set_colorspace(&c, cs_of(gets_(argc, argv, "cs", "ycc")));
    const char *samp = gets_(argc, argv, "samp", "");
    for (int i = 0; *samp && i < c.num_components; i++) {
        c.comp_info[i].h_samp_factor = samp[0] - '0';
        c.comp_info[i].v_samp_factor = samp[1] - '0';
        samp += 2; if (*samp == ',') samp++;
    }
    jpeg_set_quality(&c, geti(argc, argv, "q", 85), geti(argc, argv, "baseline", 1));
    c.arith_code = geti(argc, argv, "arith", 0);
    c.optimize_coding = geti(argc, argv, "opt", 0);
    c.restart_interval = geti(argc, argv, "rst", 0);
    c.restart_in_rows = geti(argc, argv, "rstrows", 0);
    c.write_JFIF_header = geti(argc, argv, "jfif", c.write_JFIF_header);
    c.write_Adobe_marker = geti(argc, argv, "adobe", c.write_Adobe_marker);
    c.smoothing_factor = geti(argc, argv, "smooth", 0);
    const char *dct = gets_(argc, argv, "dct", "islow");
    c.dct_method = !strcmp(dct, "ifast") ? JDCT_IFAST : !strcmp(dct, "float") ? JDCT_FLOAT : JDCT_ISLOW;
    if (geti(argc, argv, "prog", 0)) jpeg_simple_progression(&c);
    static jpeg_scan_info scans[4];
    if (!strcmp(gets_(argc, argv, "scans", "interleaved"), "separate") && c.num_components > 1) {
        for (int i = 0; i < c.num_components; i++) {
            scans[i].comps_in_scan = 1; scans[i].component_index[0] = i;
            scans[i].Ss = 0; scans[i].Se = 63; scans[i].Ah = 0; scans[i].Al = 0;
        }
        c.scan_info = scans; c.num_scans = c.num_components;
    }
    jpeg_start_compress(&c, TRUE);
    int orient = geti(argc, argv, "exif", 0);
    if (orient) {
        /* Minimal big-endian TIFF/Exif: IFD0 with Orientation only. */
        unsigned char x[] = {'E','x','i','f',0,0,'M','M',0,42,0,0,0,8,0,1,
                             0x01,0x12,0,3,0,0,0,1,0,(unsigned char)orient,0,0,0,0,0,0};
        jpeg_write_marker(&c, JPEG_APP0 + 1, x, sizeof x);
    }
    int icc = geti(argc, argv, "icc", 0);
    if (icc) {
        unsigned char *p = calloc(icc, 1);
        /* Size, 'acsp' signature and a deterministic body; not a usable profile. */
        p[0] = icc >> 24; p[1] = icc >> 16; p[2] = icc >> 8; p[3] = icc;
        if (icc > 40) memcpy(p + 36, "acsp", 4);
        for (int i = 128; i < icc; i++) p[i] = (unsigned char)(i * 7);
        jpeg_write_icc_profile(&c, p, icc);
        free(p);
    }
    int com = geti(argc, argv, "com", 0);
    if (com) {
        unsigned char *p = malloc(com);
        for (int i = 0; i < com; i++) p[i] = 'a' + i % 26;
        jpeg_write_marker(&c, JPEG_COM, p, com);
        free(p);
    }
    while (c.next_scanline < c.image_height) {
        JSAMPROW row = raw + (size_t)c.next_scanline * w * inc;
        jpeg_write_scanlines(&c, &row, 1);
    }
    jpeg_finish_compress(&c);
    jpeg_destroy_compress(&c);
    fclose(of);
    free(raw);
    return 0;
}
