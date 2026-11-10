/* libjxl (github.com/libjxl/libjxl, built from source) decode runner. Usage: libjxl MODE FILE [DUMP]
   Output: float RGBA, orientation applied (libjxl default; glycin's jxl-rs and jxl-oxide also apply it, output dims swap), alpha un-premultiplied,
   all displayed frames (coalesced); RGBA8 = round(clamp(v)*255). Single-threaded. */
#include "../../modern/runner_common.h"
#include <jxl/decode.h>
#include <math.h>
int main(int argc,char**argv){
 if(argc<3)return jerr("usage","libjxl MODE FILE [DUMP]");
 size_t len;uint8_t*d=read_file(argv[2],&len);if(!d)return jerr("io","cannot read");
 const char*dump=argc>3?argv[3]:NULL;
 JxlDecoder*dec=JxlDecoderCreate(NULL);
 if(JxlDecoderSubscribeEvents(dec,JXL_DEC_BASIC_INFO|JXL_DEC_FRAME|JXL_DEC_FULL_IMAGE)!=JXL_DEC_SUCCESS)return jerr("setup","subscribe");
 JxlDecoderSetUnpremultiplyAlpha(dec,JXL_TRUE);
 JxlDecoderSetInput(dec,d,len);JxlDecoderCloseInput(dec);
 JxlBasicInfo bi;int have=0;JxlPixelFormat pf={4,JXL_TYPE_FLOAT,JXL_NATIVE_ENDIAN,0};
 cimg_t fr[MAX_FRAMES];int n=0;float*buf=NULL;size_t bsz=0;const char*later=NULL;char lerr[64];
 for(;;){JxlDecoderStatus st=JxlDecoderProcessInput(dec);
  if(st==JXL_DEC_SUCCESS)break;
  if(st==JXL_DEC_ERROR||st==JXL_DEC_NEED_MORE_INPUT){
   const char*c=st==JXL_DEC_ERROR?"decoder-error":"truncated";
   if(n==0)return jerr(c,st==JXL_DEC_ERROR?"JXL_DEC_ERROR":"JXL_DEC_NEED_MORE_INPUT after close_input");
   snprintf(lerr,sizeof lerr,"%s",c);later=lerr;break;}
  if(st==JXL_DEC_BASIC_INFO){if(JxlDecoderGetBasicInfo(dec,&bi)!=JXL_DEC_SUCCESS)return jerr("info","GetBasicInfo");have=1;
   if(bi.xsize==0||bi.ysize==0||(unsigned long long)bi.xsize*bi.ysize>MAX_PIXELS)return jerr("harness","output dimensions outside harness limit");}
  else if(st==JXL_DEC_NEED_IMAGE_OUT_BUFFER){if(!have)return jerr("order","buffer before info");
   if(JxlDecoderImageOutBufferSize(dec,&pf,&bsz)!=JXL_DEC_SUCCESS)return jerr("buffer","size");
   free(buf);buf=malloc(bsz);if(!buf)return jerr("oom","buffer");
   if(JxlDecoderSetImageOutBuffer(dec,&pf,buf,bsz)!=JXL_DEC_SUCCESS)return jerr("buffer","set");}
  else if(st==JXL_DEC_FULL_IMAGE){
   if(n>=MAX_FRAMES)break;
   size_t px=(size_t)bi.xsize*bi.ysize;uint8_t*o=malloc(px*4);
   for(size_t i=0;i<px*4;i++){float v=buf[i];if(!(v>0))v=0;if(v>1)v=1;o[i]=(uint8_t)(v*255.0f+0.5f);}
   fr[n].w=bi.xsize;fr[n].h=bi.ysize;fr[n].rgba=o;n++;}}
 if(n==0)return jerr("decode","no full image event");
 char ex[200];snprintf(ex,sizeof ex,",\"later_frame_error\":%s%s%s,\"orientation\":%d,\"have_animation\":%s,\"bits\":%u,\"alpha_bits\":%u,\"channels\":%u",later?"\"":"null",later?later:"",later?"\"":"",(int)bi.orientation,bi.have_animation?"true":"false",bi.bits_per_sample,bi.alpha_bits,bi.num_color_channels);
 jok(fr,n,bi.num_color_channels==1?(bi.alpha_bits?"gray+alpha":"gray"):(bi.alpha_bits?"rgba":"rgb"),dump,ex);return 0;}
