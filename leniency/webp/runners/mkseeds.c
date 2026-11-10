/* Deterministic synthetic WebP seed generator using libwebp's encoder, mux and anim encoder.
   Usage: mkseeds OUTDIR. Pixel content is a closed-form gradient/pattern; no external images. */
#include <webp/encode.h>
#include <webp/mux.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
static const char*out;
static void put(const char*name,const uint8_t*d,size_t n){char p[512];snprintf(p,sizeof p,"%s/%s.webp",out,name);FILE*f=fopen(p,"wb");fwrite(d,1,n,f);fclose(f);}
static uint8_t*pattern(int w,int h,int alpha,int variant){uint8_t*p=malloc((size_t)w*h*4);
 for(int y=0;y<h;y++)for(int x=0;x<w;x++){uint8_t*q=p+((size_t)y*w+x)*4;
  q[0]=(x*255)/(w>1?w-1:1);q[1]=(y*255)/(h>1?h-1:1);q[2]=((x^y)*37+variant*53)&255;
  if(variant&1&&((x/4+y/4)&1)){q[0]=255-q[0];}
  q[3]=alpha?(uint8_t)((x*17+y*29+variant*11)&255):255;
  if(alpha&&((x+y)%7==0))q[3]=0; if(alpha&&((x+y)%11==0))q[3]=255;}
 return p;}
typedef struct{const char*name;int w,h,alpha,lossless;float q;int variant;void(*tweak)(WebPConfig*);}spec_t;
static void t_none(WebPConfig*c){(void)c;}
static void t_nl60(WebPConfig*c){c->near_lossless=60;}
static void t_exact(WebPConfig*c){c->exact=1;}
static void t_sharp(WebPConfig*c){c->use_sharp_yuv=1;}
static void t_parts(WebPConfig*c){c->partitions=3;c->segments=4;}
static void t_alpha_raw(WebPConfig*c){c->alpha_compression=0;}
static void t_alpha_filter(WebPConfig*c){c->alpha_filtering=2;}
static void t_alpha_q(WebPConfig*c){c->alpha_quality=20;}
static void t_ss(WebPConfig*c){c->sns_strength=100;c->filter_strength=100;c->filter_sharpness=7;}
static void t_nofilter(WebPConfig*c){c->filter_strength=0;c->segments=1;}
static void t_m0(WebPConfig*c){c->method=0;}
static void t_lz6(WebPConfig*c){c->method=6;c->quality=100;}
static int enc(const spec_t*s,WebPMemoryWriter*wr){WebPConfig cfg;WebPConfigInit(&cfg);cfg.lossless=s->lossless;cfg.quality=s->q;cfg.thread_level=0;if(s->tweak)s->tweak(&cfg);
 if(!WebPValidateConfig(&cfg))return 0;WebPPicture pic;WebPPictureInit(&pic);pic.width=s->w;pic.height=s->h;pic.use_argb=s->lossless;
 uint8_t*px=pattern(s->w,s->h,s->alpha,s->variant);int ok=WebPPictureImportRGBA(&pic,px,s->w*4);free(px);if(!ok)return 0;
 WebPMemoryWriterInit(wr);pic.writer=WebPMemoryWrite;pic.custom_ptr=wr;ok=WebPEncode(&cfg,&pic);WebPPictureFree(&pic);return ok;}
static void anim(const char*name,int lossless,int alpha,int frames,int mixed,int minimize,int w,int h){
 WebPAnimEncoderOptions ao;WebPAnimEncoderOptionsInit(&ao);ao.anim_params.loop_count=2;ao.anim_params.bgcolor=0xffe0c0a0u;ao.allow_mixed=mixed;ao.minimize_size=minimize;ao.kmax=frames>2?2:1;
 WebPAnimEncoder*e=WebPAnimEncoderNew(w,h,&ao);WebPConfig cfg;WebPConfigInit(&cfg);cfg.lossless=lossless;cfg.quality=60;
 for(int i=0;i<frames;i++){WebPPicture pic;WebPPictureInit(&pic);pic.width=w;pic.height=h;pic.use_argb=1;
  uint8_t*px=calloc((size_t)w*h,4);
  for(int y=0;y<h;y++)for(int x=0;x<w;x++){uint8_t*q=px+((size_t)y*w+x)*4;int in=(x>=i*3&&x<i*3+w/2&&y>=i*2&&y<i*2+h/2);
    if(in){q[0]=40+i*70;q[1]=200-i*50;q[2]=(x*9+y*5)&255;q[3]=alpha?(uint8_t)(255-(i*40)):255;}else if(!alpha){q[0]=q[1]=q[2]=30;q[3]=255;} else {q[3]=0;}}
  WebPPictureImportRGBA(&pic,px,w*4);free(px);WebPAnimEncoderAdd(e,&pic,i*100,&cfg);WebPPictureFree(&pic);}
 WebPAnimEncoderAdd(e,NULL,frames*100,NULL);WebPData d;WebPDataInit(&d);WebPAnimEncoderAssemble(e,&d);put(name,d.bytes,d.size);WebPDataClear(&d);WebPAnimEncoderDelete(e);}
static void add_meta(const char*src,const char*dst,int icc,int exif,int xmp){
 char p[512];snprintf(p,sizeof p,"%s/%s.webp",out,src);FILE*f=fopen(p,"rb");fseek(f,0,SEEK_END);long n=ftell(f);fseek(f,0,SEEK_SET);uint8_t*b=malloc(n);fread(b,1,n,f);fclose(f);
 WebPData in={b,(size_t)n};WebPMux*m=WebPMuxCreate(&in,1);
 /* A tiny but structurally valid sRGB-like ICC payload (128-byte header + 0 tags); decoders that parse ICC may reject it. */
 static uint8_t icc_b[132];memset(icc_b,0,sizeof icc_b);icc_b[3]=132;memcpy(icc_b+12,"mntr",4);memcpy(icc_b+16,"RGB ",4);memcpy(icc_b+20,"XYZ ",4);memcpy(icc_b+36,"acsp",4);
 static const uint8_t exif_b[]={'M','M',0,42,0,0,0,8,0,0,0,0,0,0};
 static const uint8_t xmp_b[]="<x:xmpmeta xmlns:x=\"adobe:ns:meta/\"/>";
 WebPData d;if(icc){d.bytes=icc_b;d.size=sizeof icc_b;WebPMuxSetChunk(m,"ICCP",&d,1);}
 if(exif){d.bytes=exif_b;d.size=sizeof exif_b;WebPMuxSetChunk(m,"EXIF",&d,1);}
 if(xmp){d.bytes=xmp_b;d.size=sizeof xmp_b-1;WebPMuxSetChunk(m,"XMP ",&d,1);}
 WebPData o;WebPDataInit(&o);WebPMuxAssemble(m,&o);put(dst,o.bytes,o.size);WebPDataClear(&o);WebPMuxDelete(m);free(b);}
int main(int argc,char**argv){out=argc>1?argv[1]:".";
 spec_t S[]={
  {"ll-1x1",1,1,0,1,100,0,t_none},{"ll-2x2-rgb",2,2,0,1,100,0,t_none},{"ll-16x16-rgb",16,16,0,1,100,1,t_none},{"ll-16x16-rgba",16,16,1,1,100,2,t_none},
  {"ll-17x13-rgba",17,13,1,1,100,3,t_none},{"ll-32x32-m6",32,32,0,1,100,4,t_lz6},{"ll-16x16-nearlossless",16,16,0,1,100,5,t_nl60},{"ll-16x16-exact-alpha",16,16,1,1,100,6,t_exact},
  {"ll-24x8-m0",24,8,1,1,100,7,t_m0},
  {"lossy-16x16-q75",16,16,0,0,75,8,t_none},{"lossy-17x13-q50",17,13,0,0,50,9,t_none},{"lossy-1x1",1,1,0,0,75,0,t_none},{"lossy-33x33-q10",33,33,0,0,10,10,t_none},
  {"lossy-64x48-parts",64,48,0,0,70,11,t_parts},{"lossy-16x16-sharp",16,16,0,0,75,12,t_sharp},{"lossy-16x16-strongfilter",16,16,0,0,60,13,t_ss},{"lossy-16x16-nofilter",16,16,0,0,60,14,t_nofilter},
  {"lossy-alpha-16x16",16,16,1,0,75,15,t_none},{"lossy-alpha-raw-17x13",17,13,1,0,75,16,t_alpha_raw},{"lossy-alpha-filter-16x16",16,16,1,0,75,17,t_alpha_filter},{"lossy-alpha-q20-24x24",24,24,1,0,75,18,t_alpha_q},
 };
 for(size_t i=0;i<sizeof S/sizeof*S;i++){WebPMemoryWriter wr;if(enc(&S[i],&wr)){put(S[i].name,wr.mem,wr.size);WebPMemoryWriterClear(&wr);}else fprintf(stderr,"encode failed: %s\n",S[i].name);}
 anim("anim-ll-3f-16x16",1,0,3,0,0,16,16);anim("anim-ll-alpha-3f-20x12",1,1,3,0,1,20,12);anim("anim-lossy-3f-16x16",0,0,3,0,0,16,16);
 anim("anim-mixed-alpha-4f-24x16",0,1,4,1,0,24,16);anim("anim-ll-2f-8x8",1,0,2,0,0,8,8);
 add_meta("lossy-16x16-q75","meta-lossy-icc-exif-xmp",1,1,1);add_meta("ll-16x16-rgba","meta-ll-icc",1,0,0);add_meta("lossy-alpha-16x16","meta-lossy-alpha-exif",0,1,0);add_meta("anim-ll-3f-16x16","meta-anim-xmp",0,0,1);
 return 0;}
