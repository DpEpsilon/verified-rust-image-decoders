/* libjpeg(-turbo) decoding column: default decompression parameters (islow IDCT,
 * fancy upsampling), output RGB/gray/CMYK normalised to straight RGBA8.
 * usage: libjpeg-runner FILE [RGBA_DUMP]
 * CMYK normalisation: with an Adobe APP14 marker the stored samples are inverted
 * (Photoshop convention): R = C*K/255; otherwise R = (255-C)*(255-K)/255.
 */
#include <setjmp.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <jpeglib.h>
#include "json.h"

struct err { struct jpeg_error_mgr pub; jmp_buf jb; char first_warning[JMSG_LENGTH_MAX]; };
static void on_error(j_common_ptr c) { longjmp(((struct err *)c->err)->jb, 1); }
static void on_message(j_common_ptr c, int level) {
    struct err *e = (struct err *)c->err;
    if (level < 0) {
        if (!e->first_warning[0]) (*c->err->format_message)(c, e->first_warning);
        e->pub.num_warnings++;
    }
}
static const char *csname(J_COLOR_SPACE s) {
    switch (s) { case JCS_GRAYSCALE: return "Gray"; case JCS_RGB: return "RGB"; case JCS_YCbCr: return "YCbCr";
                 case JCS_CMYK: return "CMYK"; case JCS_YCCK: return "YCCK"; default: return "Unknown"; }
}
int main(int argc, char **argv) {
    if (argc < 2) { json_error("usage", "usage: libjpeg-runner FILE [DUMP]"); return 0; }
    FILE *f = fopen(argv[1], "rb");
    if (!f) { json_error("io", "open failed"); return 0; }
    fseek(f, 0, SEEK_END); long len = ftell(f); fseek(f, 0, SEEK_SET);
    unsigned char *buf = malloc(len ? len : 1);
    if (fread(buf, 1, len, f) != (size_t)len) { json_error("io", "read failed"); return 0; }
    fclose(f);
    struct jpeg_decompress_struct d; struct err e; memset(&e, 0, sizeof e);
    d.err = jpeg_std_error(&e.pub); e.pub.error_exit = on_error; e.pub.emit_message = on_message;
    unsigned char *out = NULL, *rgba = NULL;
    if (setjmp(e.jb)) {
        char msg[JMSG_LENGTH_MAX]; (*d.err->format_message)((j_common_ptr)&d, msg);
        char cls[64]; snprintf(cls, sizeof cls, "JERR_%d", e.pub.msg_code);
        json_error(cls, msg); jpeg_destroy_decompress(&d); return 0;
    }
    jpeg_create_decompress(&d);
    jpeg_mem_src(&d, buf, len);
    jpeg_read_header(&d, TRUE);
    J_COLOR_SPACE in = d.jpeg_color_space;
    int cmyk = in == JCS_CMYK || in == JCS_YCCK;
    d.out_color_space = cmyk ? JCS_CMYK : in == JCS_GRAYSCALE ? JCS_GRAYSCALE : JCS_RGB;
    if ((unsigned long long)d.image_width * d.image_height > 16000000ULL) { json_error("limit", "output dimensions outside harness limit"); return 0; }
    jpeg_start_decompress(&d);
    int w = d.output_width, h = d.output_height, n = d.output_components;
    out = malloc((size_t)w * n); rgba = malloc((size_t)w * h * 4);
    while (d.output_scanline < d.output_height) {
        int y = d.output_scanline; JSAMPROW row = out;
        jpeg_read_scanlines(&d, &row, 1);
        for (int x = 0; x < w; x++) {
            unsigned char *p = out + x * n, *q = rgba + ((size_t)y * w + x) * 4;
            if (n == 1) { q[0] = q[1] = q[2] = p[0]; }
            else if (n == 3) { q[0] = p[0]; q[1] = p[1]; q[2] = p[2]; }
            else {
                for (int k = 0; k < 3; k++) q[k] = d.saw_Adobe_marker ? (p[k] * p[3] + 127) / 255
                                                                      : ((255 - p[k]) * (255 - p[3]) + 127) / 255;
            }
            q[3] = 255;
        }
    }
    jpeg_finish_decompress(&d);
    char orig[160];
    snprintf(orig, sizeof orig, "%s/%dc/%dbit%s%s%s", csname(in), d.num_components, d.data_precision,
             d.progressive_mode ? "/progressive" : "", d.arith_code ? "/arithmetic" : "",
             d.saw_Adobe_marker ? "/adobe" : "");
    char extra[JMSG_LENGTH_MAX + 200]; char wmsg[JMSG_LENGTH_MAX * 2];
    json_escape(e.first_warning, wmsg, sizeof wmsg);
    snprintf(extra, sizeof extra, "\"components\":%d,\"bit_depth\":%d,\"orientation\":\"not-parsed\",\"warnings\":%ld,\"first_warning\":\"%s\"",
             d.num_components, d.data_precision, e.pub.num_warnings, wmsg);
    json_ok(w, h, orig, rgba, argc > 2 ? argv[2] : NULL, extra);
    jpeg_destroy_decompress(&d);
    return 0;
}
