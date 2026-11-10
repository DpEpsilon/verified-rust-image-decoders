/* Deterministic synthetic JPEG XL seed generator on libjxl's encoder API (github.com/libjxl/libjxl, built with cmake).
   Usage: mkseeds OUTDIR. Pixel data are closed-form gradients; no external image. */
#include <jxl/encode.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
static const char*OUT;
typedef struct{const char*name;int w,h,nc,alpha,bits;int lossless;float dist;int effort;int frames;int container;int modular;int prog;int squeeze;int palette;int orient;int cs;int exif;int resamp;int noise;int dec_speed;}S;
/* cs: 0 sRGB/gray-sRGB, 1 linear, 2 P3 sRGB-TRC, 3 PQ rec2020 */
static void gen(const S*s){
 JxlEncoder*enc=JxlEncoderCreate(NULL);
 JxlBasicInfo bi;JxlEncoderInitBasicInfo(&bi);bi.xsize=s->w;bi.ysize=s->h;bi.bits_per_sample=s->bits;bi.exponent_bits_per_sample=0;
 bi.uses_original_profile=s->lossless?JXL_TRUE:JXL_FALSE;bi.num_color_channels=s->nc;if(s->alpha){bi.alpha_bits=s->bits;bi.num_extra_channels=1;}
 if(s->frames>1){bi.have_animation=JXL_TRUE;bi.animation.tps_numerator=1000;bi.animation.tps_denominator=1;bi.animation.num_loops=2;}
 if(s->orient)bi.orientation=(JxlOrientation)s->orient;
 JxlEncoderUseContainer(enc,s->container||s->exif?JXL_TRUE:JXL_FALSE);
 if(s->exif)JxlEncoderUseBoxes(enc);
 if(JxlEncoderSetBasicInfo(enc,&bi)!=JXL_ENC_SUCCESS){fprintf(stderr,"basic info failed %s\n",s->name);return;}
 JxlColorEncoding ce;JxlColorEncodingSetToSRGB(&ce,s->nc==1);
 if(s->cs==1)JxlColorEncodingSetToLinearSRGB(&ce,s->nc==1);
 if(s->cs==2&&s->nc==3){ce.primaries=JXL_PRIMARIES_P3;ce.white_point=JXL_WHITE_POINT_D65;}
 if(s->cs==3&&s->nc==3){ce.primaries=JXL_PRIMARIES_2100;ce.transfer_function=JXL_TRANSFER_FUNCTION_PQ;}
 if(JxlEncoderSetColorEncoding(enc,&ce)!=JXL_ENC_SUCCESS){fprintf(stderr,"color enc failed %s\n",s->name);return;}
 int ch=s->nc+(s->alpha?1:0);int bytes=s->bits>8?2:1;
 for(int f=0;f<s->frames;f++){
  JxlEncoderFrameSettings*fs=JxlEncoderFrameSettingsCreate(enc,NULL);
  if(s->lossless)JxlEncoderSetFrameLossless(fs,JXL_TRUE);else JxlEncoderSetFrameDistance(fs,s->dist);
  JxlEncoderFrameSettingsSetOption(fs,JXL_ENC_FRAME_SETTING_EFFORT,s->effort?s->effort:3);
  if(s->modular)JxlEncoderFrameSettingsSetOption(fs,JXL_ENC_FRAME_SETTING_MODULAR,1);
  if(s->prog){JxlEncoderFrameSettingsSetOption(fs,JXL_ENC_FRAME_SETTING_PROGRESSIVE_AC,1);JxlEncoderFrameSettingsSetOption(fs,JXL_ENC_FRAME_SETTING_QPROGRESSIVE_AC,1);}
  if(s->squeeze)JxlEncoderFrameSettingsSetOption(fs,JXL_ENC_FRAME_SETTING_RESPONSIVE,1);
  if(s->palette)JxlEncoderFrameSettingsSetOption(fs,JXL_ENC_FRAME_SETTING_PALETTE_COLORS,s->palette);
  if(s->resamp)JxlEncoderFrameSettingsSetOption(fs,JXL_ENC_FRAME_SETTING_RESAMPLING,s->resamp);
  if(s->noise)JxlEncoderFrameSettingsSetFloatOption(fs,JXL_ENC_FRAME_SETTING_PHOTON_NOISE,(float)s->noise);
  if(s->dec_speed)JxlEncoderFrameSettingsSetOption(fs,JXL_ENC_FRAME_SETTING_DECODING_SPEED,s->dec_speed);
  if(s->frames>1){JxlFrameHeader fh;JxlEncoderInitFrameHeader(&fh);fh.duration=100+f*50;JxlEncoderSetFrameHeader(fs,&fh);}
  size_t n=(size_t)s->w*s->h*ch*bytes;uint8_t*buf=calloc(1,n);int maxv=(1<<s->bits)-1;
  for(int y=0;y<s->h;y++)for(int x=0;x<s->w;x++)for(int c=0;c<ch;c++){
   long v;int isalpha=s->alpha&&c==ch-1;
   if(isalpha)v=((x*13+y*7+f)%5==0)?0:maxv-((x+y)%3)*(maxv/16);
   else if(c==0)v=(long)(((x+f*3)%s->w)*maxv)/(s->w>1?s->w-1:1);else if(c==1)v=(long)(y*maxv)/(s->h>1?s->h-1:1);else v=(((x^y)*37+c*61+f*29)&255)*(long)maxv/255;
   size_t i=((size_t)y*s->w+x)*ch+c;if(bytes==1)buf[i]=(uint8_t)v;else{buf[2*i]=v&255;buf[2*i+1]=v>>8;}}
  JxlPixelFormat pf={ch,bytes==1?JXL_TYPE_UINT8:JXL_TYPE_UINT16,JXL_LITTLE_ENDIAN,0};
  if(JxlEncoderAddImageFrame(fs,&pf,buf,n)!=JXL_ENC_SUCCESS){fprintf(stderr,"add frame failed %s\n",s->name);return;}
  free(buf);}
 if(s->exif){static const uint8_t ex[]={0,0,0,0,'M','M',0,42,0,0,0,8,0,0,0,0,0,0};JxlEncoderAddBox(enc,"Exif",ex,sizeof ex,s->exif==2?JXL_TRUE:JXL_FALSE);
  static const uint8_t xm[]="<x:xmpmeta xmlns:x=\"adobe:ns:meta/\"/>";JxlEncoderAddBox(enc,"xml ",xm,sizeof xm-1,JXL_FALSE);JxlEncoderCloseBoxes(enc);}
 JxlEncoderCloseInput(enc);
 size_t cap=1<<16,len=0;uint8_t*out=malloc(cap);JxlEncoderStatus st;
 do{uint8_t*nx=out+len;size_t avail=cap-len;st=JxlEncoderProcessOutput(enc,&nx,&avail);len=nx-out;if(st==JXL_ENC_NEED_MORE_OUTPUT){cap*=2;out=realloc(out,cap);}}while(st==JXL_ENC_NEED_MORE_OUTPUT);
 if(st!=JXL_ENC_SUCCESS){fprintf(stderr,"encode failed %s\n",s->name);return;}
 char p[512];snprintf(p,sizeof p,"%s/%s.jxl",OUT,s->name);FILE*fp=fopen(p,"wb");fwrite(out,1,len,fp);fclose(fp);free(out);JxlEncoderDestroy(enc);}
int main(int argc,char**argv){OUT=argc>1?argv[1]:".";
 /* name w h nc alpha bits lossless dist effort frames container modular prog squeeze palette orient cs exif resamp noise decspeed */
 S L[]={
 {"ml-gray8-16x16",16,16,1,0,8,1,0,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"ml-rgb8-16x16",16,16,3,0,8,1,0,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"ml-rgba8-16x16",16,16,3,1,8,1,0,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"ml-ga8-16x16",16,16,1,1,8,1,0,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"ml-rgb16-16x16",16,16,3,0,16,1,0,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"ml-gray16-16x16",16,16,1,0,16,1,0,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"ml-rgba16-16x16",16,16,3,1,16,1,0,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"ml-rgb8-1x1",1,1,3,0,8,1,0,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"ml-rgb8-17x13",17,13,3,0,8,1,0,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"ml-rgb8-container",16,16,3,0,8,1,0,3,1,1,0,0,0,0,0,0,0,0,0,0},
 {"ml-rgb8-effort1",16,16,3,0,8,1,0,1,1,0,0,0,0,0,0,0,0,0,0,0},
 {"ml-rgb8-effort7",16,16,3,0,8,1,0,7,1,0,0,0,0,0,0,0,0,0,0,0},
 {"ml-rgb8-squeeze",24,24,3,0,8,1,0,3,1,0,1,0,1,0,0,0,0,0,0,0},
 {"ml-rgb8-palette",16,16,3,0,8,1,0,3,1,0,1,0,0,64,0,0,0,0,0,0},
 {"ml-rgb8-orient6",16,12,3,0,8,1,0,3,1,0,0,0,0,0,6,0,0,0,0,0},
 {"ml-rgb8-linear",16,16,3,0,8,1,0,3,1,0,0,0,0,0,0,1,0,0,0,0},
 {"ml-rgb8-p3",16,16,3,0,8,1,0,3,1,0,0,0,0,0,0,2,0,0,0,0},
 {"ml-rgb16-pq",16,16,3,0,16,1,0,3,1,0,0,0,0,0,0,3,0,0,0,0},
 {"ml-rgb8-exif-xmp",16,16,3,0,8,1,0,3,1,1,0,0,0,0,0,0,1,0,0,0},
 {"ml-rgb8-exif-brob",16,16,3,0,8,1,0,3,1,1,0,0,0,0,0,0,2,0,0,0},
 {"mlossy-rgb8-d2",24,24,3,0,8,0,2.0f,3,1,0,1,0,0,0,0,0,0,0,0,0},
 {"vd-rgb8-d1",32,32,3,0,8,0,1.0f,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"vd-rgb8-d4",32,32,3,0,8,0,4.0f,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"vd-rgb8-d0p3-e7",32,32,3,0,8,0,0.3f,7,1,0,0,0,0,0,0,0,0,0,0,0},
 {"vd-gray8-d1",32,32,1,0,8,0,1.0f,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"vd-rgba8-d1",32,32,3,1,8,0,1.0f,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"vd-rgb8-d1-17x13",17,13,3,0,8,0,1.0f,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"vd-rgb8-d1-prog",32,32,3,0,8,0,1.0f,3,1,0,0,1,0,0,0,0,0,0,0,0},
 {"vd-rgb8-d1-effort1",32,32,3,0,8,0,1.0f,1,1,0,0,0,0,0,0,0,0,0,0,0},
 {"vd-rgb8-d1-effort9",32,32,3,0,8,0,1.0f,9,1,0,0,0,0,0,0,0,0,0,0,0},
 {"vd-rgb8-d1-resamp2",32,32,3,0,8,0,1.0f,3,1,0,0,0,0,0,0,0,2,0,0,0},
 {"vd-rgb8-d1-noise",32,32,3,0,8,0,1.0f,3,1,0,0,0,0,0,0,0,0,0,3200,0},
 {"vd-rgb8-d1-decspeed4",32,32,3,0,8,0,1.0f,3,1,0,0,0,0,0,0,0,0,0,0,4},
 {"vd-rgb16-d1",24,24,3,0,16,0,1.0f,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"vd-rgb8-d1-exif",32,32,3,0,8,0,1.0f,3,1,1,0,0,0,0,0,0,1,0,0,0},
 {"vd-rgb8-d1-80x72",80,72,3,0,8,0,1.0f,3,1,0,0,0,0,0,0,0,0,0,0,0},
 {"anim-ml-rgba8-3f",16,16,3,1,8,1,0,3,3,0,0,0,0,0,0,0,0,0,0,0},
 {"anim-ml-rgb8-2f",12,12,3,0,8,1,0,3,2,0,0,0,0,0,0,0,0,0,0,0},
 {"anim-vd-rgb8-3f",24,24,3,0,8,0,1.0f,3,3,0,0,0,0,0,0,0,0,0,0,0},
 {"anim-ml-rgb8-3f-container",16,16,3,0,8,1,0,3,3,1,0,0,0,0,0,0,0,0,0,0},
 };
 for(size_t i=0;i<sizeof L/sizeof*L;i++)gen(&L[i]);return 0;}
