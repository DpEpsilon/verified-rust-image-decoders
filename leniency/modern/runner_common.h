/* Shared by the native (C) runners: SHA-256, JSON-line output, RGBA dump. */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
typedef struct { uint32_t h[8]; uint64_t n; uint8_t buf[64]; size_t k; } sha_t;
static const uint32_t K256[64]={0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2};
#define ROR(x,n) (((x)>>(n))|((x)<<(32-(n))))
static void sha_blk(sha_t*s,const uint8_t*p){uint32_t w[64],a,b,c,d,e,f,g,h,t1,t2;int i;
 for(i=0;i<16;i++)w[i]=(uint32_t)p[4*i]<<24|p[4*i+1]<<16|p[4*i+2]<<8|p[4*i+3];
 for(i=16;i<64;i++){uint32_t s0=ROR(w[i-15],7)^ROR(w[i-15],18)^(w[i-15]>>3),s1=ROR(w[i-2],17)^ROR(w[i-2],19)^(w[i-2]>>10);w[i]=w[i-16]+s0+w[i-7]+s1;}
 a=s->h[0];b=s->h[1];c=s->h[2];d=s->h[3];e=s->h[4];f=s->h[5];g=s->h[6];h=s->h[7];
 for(i=0;i<64;i++){t1=h+(ROR(e,6)^ROR(e,11)^ROR(e,25))+((e&f)^(~e&g))+K256[i]+w[i];t2=(ROR(a,2)^ROR(a,13)^ROR(a,22))+((a&b)^(a&c)^(b&c));h=g;g=f;f=e;e=d+t1;d=c;c=b;b=a;a=t1+t2;}
 s->h[0]+=a;s->h[1]+=b;s->h[2]+=c;s->h[3]+=d;s->h[4]+=e;s->h[5]+=f;s->h[6]+=g;s->h[7]+=h;}
static void sha_init(sha_t*s){static const uint32_t i[8]={0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19};memcpy(s->h,i,32);s->n=0;s->k=0;}
static void sha_upd(sha_t*s,const void*d,size_t n){const uint8_t*p=d;s->n+=n;while(n){size_t c=64-s->k;if(c>n)c=n;memcpy(s->buf+s->k,p,c);s->k+=c;p+=c;n-=c;if(s->k==64){sha_blk(s,s->buf);s->k=0;}}}
static void sha_hex(sha_t*s,char out[65]){uint64_t bits=s->n*8;uint8_t pad=0x80;sha_upd(s,&pad,1);pad=0;while(s->k!=56)sha_upd(s,&pad,1);uint8_t l[8];for(int i=0;i<8;i++)l[i]=bits>>(56-8*i);sha_upd(s,l,8);for(int i=0;i<8;i++)sprintf(out+8*i,"%08x",s->h[i]);}
static void sha256_hex(const void*d,size_t n,char out[65]){sha_t s;sha_init(&s);sha_upd(&s,d,n);sha_hex(&s,out);}
static void json_str(const char*s){putchar('"');for(const unsigned char*p=(const unsigned char*)s;*p;p++){if(*p=='"'||*p=='\\')printf("\\%c",*p);else if(*p<32||*p>126)printf("\\u%04x",*p);else putchar(*p);}putchar('"');}
static int jerr(const char*cls,const char*msg){printf("{\"status\":\"error\",\"error_class\":");json_str(cls);printf(",\"message\":");json_str(msg?msg:"");printf("}\n");return 0;}
static uint8_t*read_file(const char*path,size_t*len){FILE*f=fopen(path,"rb");if(!f)return NULL;fseek(f,0,SEEK_END);long n=ftell(f);fseek(f,0,SEEK_SET);uint8_t*b=malloc(n?n:1);if(fread(b,1,n,f)!=(size_t)n){fclose(f);free(b);return NULL;}fclose(f);*len=n;return b;}
#define MAX_PIXELS 16000000ull
#define MAX_FRAMES 32
/* Accumulates up to MAX_FRAMES frames of straight RGBA8; prints the JSON line. */
typedef struct { int w,h; uint8_t*rgba; } cimg_t;
static int jok(cimg_t*fr,int n,const char*orig,const char*dump,const char*extra){
 if(n<1||fr[0].w<=0||fr[0].h<=0||(unsigned long long)fr[0].w*fr[0].h>MAX_PIXELS)return jerr("harness","output dimensions outside harness limit");
 char hex[65];size_t len=(size_t)fr[0].w*fr[0].h*4;sha256_hex(fr[0].rgba,len,hex);
 int alpha=0;for(size_t i=3;i<len;i+=4)if(fr[0].rgba[i]!=255){alpha=1;break;}
 if(dump){FILE*f=fopen(dump,"wb");if(f){for(int i=0;i<n;i++)fwrite(fr[i].rgba,1,(size_t)fr[i].w*fr[i].h*4,f);fclose(f);}}
 sha_t all;sha_init(&all);for(int i=0;i<n;i++){char h2[65],b[160];sha256_hex(fr[i].rgba,(size_t)fr[i].w*fr[i].h*4,h2);int m=snprintf(b,sizeof b,"%dx%d:%s%s",fr[i].w,fr[i].h,h2,i+1<n?",":"");sha_upd(&all,b,m);}
 char ah[65];sha_hex(&all,ah);
 printf("{\"status\":\"ok\",\"width\":%d,\"height\":%d,\"orig_color_type\":",fr[0].w,fr[0].h);json_str(orig);
 printf(",\"alpha_present\":%s,\"rgba_sha256\":\"%s\",\"frames\":%d,\"frames_sha256\":\"%s\"%s}\n",alpha?"true":"false",hex,n,ah,extra?extra:"");return 0;}
