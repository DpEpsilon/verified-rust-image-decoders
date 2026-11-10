/* Reference XBM reader: libX11's XReadBitmapFileData (accepts X10 and X11 bitmaps).
   Output: set bit = black, clear bit = white, opaque; top row first. */
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct { unsigned h[8]; } S;
static const unsigned K[64] = {
0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2};
#define R(x,n) (((x)>>(n))|((x)<<(32-(n))))
static void blk(S *s, const unsigned char *p) {
  unsigned w[64], a, b, c, d, e, f, g, h, i, t1, t2;
  for (i = 0; i < 16; i++) w[i] = (unsigned)p[4*i] << 24 | p[4*i+1] << 16 | p[4*i+2] << 8 | p[4*i+3];
  for (; i < 64; i++) w[i] = (R(w[i-2],17)^R(w[i-2],19)^(w[i-2]>>10)) + w[i-7] + (R(w[i-15],7)^R(w[i-15],18)^(w[i-15]>>3)) + w[i-16];
  a=s->h[0];b=s->h[1];c=s->h[2];d=s->h[3];e=s->h[4];f=s->h[5];g=s->h[6];h=s->h[7];
  for (i = 0; i < 64; i++) {
    t1 = h + (R(e,6)^R(e,11)^R(e,25)) + ((e&f)^(~e&g)) + K[i] + w[i];
    t2 = (R(a,2)^R(a,13)^R(a,22)) + ((a&b)^(a&c)^(b&c));
    h=g;g=f;f=e;e=d+t1;d=c;c=b;b=a;a=t1+t2;
  }
  s->h[0]+=a;s->h[1]+=b;s->h[2]+=c;s->h[3]+=d;s->h[4]+=e;s->h[5]+=f;s->h[6]+=g;s->h[7]+=h;
}
static void sha256_hex(const unsigned char *m, size_t n, char *out) {
  S s = {{0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19}};
  size_t i; for (i = 0; i + 64 <= n; i += 64) blk(&s, m + i);
  unsigned char t[128] = {0}; size_t r = n - i; memcpy(t, m + i, r); t[r] = 0x80;
  size_t tl = (r < 56) ? 64 : 128; unsigned long long bits = (unsigned long long)n * 8;
  for (int k = 0; k < 8; k++) t[tl-1-k] = bits >> (8*k);
  blk(&s, t); if (tl == 128) blk(&s, t + 64);
  for (int k = 0; k < 8; k++) sprintf(out + 8*k, "%08x", s.h[k]);
}
static void fail(const char *cls) {
  printf("{\"status\":\"error\",\"error_class\":\"%s\",\"message\":\"XReadBitmapFileData returned %s\"}\n", cls, cls);
}
int main(int argc, char **argv) {
  if (argc < 3) { fail("usage"); return 0; }
  unsigned w = 0, h = 0; int xh = -1, yh = -1; unsigned char *data = NULL;
  int r = XReadBitmapFileData(argv[2], &w, &h, &data, &xh, &yh);
  if (r != BitmapSuccess) { fail(r == BitmapOpenFailed ? "BitmapOpenFailed" : r == BitmapFileInvalid ? "BitmapFileInvalid" : "BitmapNoMemory"); return 0; }
  if (w == 0 || h == 0 || (unsigned long long)w * h > 16000000ULL) { fail("DimensionsOutsideHarnessLimit"); return 0; }
  size_t stride = (w + 7) / 8; unsigned char *rgba = malloc((size_t)w * h * 4);
  for (unsigned y = 0; y < h; y++) for (unsigned x = 0; x < w; x++) {
    int bit = (data[y*stride + x/8] >> (x%8)) & 1; unsigned char g = bit ? 0 : 255;
    unsigned char *p = rgba + ((size_t)y*w + x)*4; p[0]=p[1]=p[2]=g; p[3]=255;
  }
  char hex[65]; sha256_hex(rgba, (size_t)w * h * 4, hex);
  if (argc >= 4) { FILE *f = fopen(argv[3], "wb"); if (f) { fwrite(rgba, 1, (size_t)w*h*4, f); fclose(f); } }
  if (xh >= 0 || yh >= 0)
    printf("{\"status\":\"ok\",\"width\":%u,\"height\":%u,\"orig_color_type\":\"L1\",\"alpha_present\":false,\"rgba_sha256\":\"%s\",\"hotspot\":[%d,%d]}\n", w, h, hex, xh, yh);
  else
    printf("{\"status\":\"ok\",\"width\":%u,\"height\":%u,\"orig_color_type\":\"L1\",\"alpha_present\":false,\"rgba_sha256\":\"%s\",\"hotspot\":null}\n", w, h, hex);
  return 0;
}
