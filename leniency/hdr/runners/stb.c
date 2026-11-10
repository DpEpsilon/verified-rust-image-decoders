/* stb_image v2.30 (public domain, fetched from github.com/nothings/stb) HDR path: stbi_loadf_from_memory, 3 components.
   usage: stb-hdr FILE [F32_DUMP]; same JSON contract as the Rust runner. */
#define STB_IMAGE_IMPLEMENTATION
#define STBI_ONLY_HDR
#define STBI_NO_STDIO
#include "stb_image.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

typedef struct { uint32_t h[8]; uint64_t len; uint8_t buf[64]; size_t n; } sha;
static const uint32_t K[64]={0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2};
#define R(x,n) (((x)>>(n))|((x)<<(32-(n))))
static void blk(sha*s,const uint8_t*p){uint32_t w[64],a,b,c,d,e,f,g,h,t1,t2;int i;for(i=0;i<16;i++)w[i]=(uint32_t)p[4*i]<<24|p[4*i+1]<<16|p[4*i+2]<<8|p[4*i+3];for(;i<64;i++){uint32_t s0=R(w[i-15],7)^R(w[i-15],18)^(w[i-15]>>3),s1=R(w[i-2],17)^R(w[i-2],19)^(w[i-2]>>10);w[i]=w[i-16]+s0+w[i-7]+s1;}
a=s->h[0];b=s->h[1];c=s->h[2];d=s->h[3];e=s->h[4];f=s->h[5];g=s->h[6];h=s->h[7];
for(i=0;i<64;i++){t1=h+(R(e,6)^R(e,11)^R(e,25))+((e&f)^(~e&g))+K[i]+w[i];t2=(R(a,2)^R(a,13)^R(a,22))+((a&b)^(a&c)^(b&c));h=g;g=f;f=e;e=d+t1;d=c;c=b;b=a;a=t1+t2;}
s->h[0]+=a;s->h[1]+=b;s->h[2]+=c;s->h[3]+=d;s->h[4]+=e;s->h[5]+=f;s->h[6]+=g;s->h[7]+=h;}
static void sinit(sha*s){static const uint32_t i[8]={0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19};memcpy(s->h,i,32);s->len=0;s->n=0;}
static void supd(sha*s,const uint8_t*p,size_t n){s->len+=n;while(n){size_t k=64-s->n;if(k>n)k=n;memcpy(s->buf+s->n,p,k);s->n+=k;p+=k;n-=k;if(s->n==64){blk(s,s->buf);s->n=0;}}}
static void sfin(sha*s,char*hex){uint64_t bits=s->len*8;uint8_t pad=0x80;supd(s,&pad,1);pad=0;while(s->n!=56)supd(s,&pad,1);uint8_t l[8];for(int i=0;i<8;i++)l[i]=bits>>(56-8*i);supd(s,l,8);for(int i=0;i<8;i++)sprintf(hex+8*i,"%08x",s->h[i]);}

static void jerr(const char*m){char cls[96];size_t i=0;for(;m[i]&&i<95&&m[i]!=':';i++)cls[i]=m[i];cls[i]=0;printf("{\"status\":\"error\",\"error_class\":\"%s\",\"message\":\"%s\"}\n",cls,m);}
int main(int argc,char**argv){
  if(argc<2){jerr("usage");return 0;}
  FILE*f=fopen(argv[1],"rb");if(!f){jerr("open failed");return 0;}
  fseek(f,0,SEEK_END);long n=ftell(f);fseek(f,0,SEEK_SET);
  unsigned char*b=malloc(n?n:1);if(fread(b,1,n,f)!=(size_t)n){jerr("read failed");return 0;}fclose(f);
  int w=0,h=0,c=0;
  float*d=stbi_loadf_from_memory(b,(int)n,&w,&h,&c,3);
  if(!d){jerr(stbi_failure_reason()?stbi_failure_reason():"unknown");return 0;}
  if(w<=0||h<=0||(uint64_t)w*h>16000000ULL){printf("{\"status\":\"error\",\"error_class\":\"HarnessLimit\",\"message\":\"dimensions over harness limit\"}\n");return 0;}
  size_t cnt=(size_t)w*h*3,nonfin=0;uint8_t*bytes=malloc(cnt*4);
  for(size_t i=0;i<cnt;i++){uint32_t u;memcpy(&u,&d[i],4);bytes[4*i]=u;bytes[4*i+1]=u>>8;bytes[4*i+2]=u>>16;bytes[4*i+3]=u>>24;if(!isfinite(d[i]))nonfin++;}
  if(argc>2){FILE*o=fopen(argv[2],"wb");if(o){fwrite(bytes,1,cnt*4,o);fclose(o);}}
  sha s;char hex[65];sinit(&s);supd(&s,bytes,cnt*4);sfin(&s,hex);
  printf("{\"status\":\"ok\",\"width\":%d,\"height\":%d,\"orig_color_type\":\"stb float x3 (file comps %d)\",\"channels\":3,\"alpha_present\":false,\"nan_inf\":%zu,\"rgba_sha256\":\"%s\"}\n",w,h,c,nonfin,hex);
  return 0;}
