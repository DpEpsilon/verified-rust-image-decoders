/* libwebp (system 1.5.0) runner. Modes: simple = WebPDecodeRGBA (still images only);
   anim = WebPAnimDecoder (composited canvas, all frames). Usage: libwebp MODE FILE [DUMP] */
#include "../../modern/runner_common.h"
#include <webp/decode.h>
#include <webp/demux.h>
static const char*vp8(VP8StatusCode c){switch(c){case VP8_STATUS_OUT_OF_MEMORY:return"OUT_OF_MEMORY";case VP8_STATUS_INVALID_PARAM:return"INVALID_PARAM";case VP8_STATUS_BITSTREAM_ERROR:return"BITSTREAM_ERROR";case VP8_STATUS_UNSUPPORTED_FEATURE:return"UNSUPPORTED_FEATURE";case VP8_STATUS_SUSPENDED:return"SUSPENDED";case VP8_STATUS_USER_ABORT:return"USER_ABORT";case VP8_STATUS_NOT_ENOUGH_DATA:return"NOT_ENOUGH_DATA";default:return"OK";}}
int main(int argc,char**argv){
 if(argc<3)return jerr("usage","libwebp MODE FILE [DUMP]");
 size_t len;uint8_t*d=read_file(argv[2],&len);if(!d)return jerr("io","cannot read");
 const char*dump=argc>3?argv[3]:NULL;
 if(!strcmp(argv[1],"simple")){
  WebPBitstreamFeatures ft;VP8StatusCode c=WebPGetFeatures(d,len,&ft);
  if(c!=VP8_STATUS_OK)return jerr(vp8(c),"WebPGetFeatures");
  if(ft.has_animation){printf("{\"status\":\"skip\",\"error_class\":\"not-applicable\",\"message\":\"still-image API; file declares animation\"}\n");return 0;}
  if((unsigned long long)ft.width*ft.height>MAX_PIXELS)return jerr("harness","output dimensions outside harness limit");
  int w,h;uint8_t*px=WebPDecodeRGBA(d,len,&w,&h);
  if(!px){WebPDecoderConfig cfg;WebPInitDecoderConfig(&cfg);cfg.output.colorspace=MODE_RGBA;c=WebPDecode(d,len,&cfg);WebPFreeDecBuffer(&cfg.output);return jerr(c==VP8_STATUS_OK?"decode":vp8(c),"WebPDecodeRGBA failed");}
  cimg_t fr={w,h,px};char ex[128];snprintf(ex,sizeof ex,",\"has_animation\":%s,\"features_alpha\":%d,\"format\":%d",ft.has_animation?"true":"false",ft.has_alpha,ft.format);
  const char*o=ft.format==1?"lossy":ft.format==2?"lossless":"mixed/undefined";
  jok(&fr,1,o,dump,ex);return 0;}
 if(!strcmp(argv[1],"anim")){
  WebPData wd={d,len};WebPAnimDecoderOptions o;WebPAnimDecoderOptionsInit(&o);o.color_mode=MODE_RGBA;o.use_threads=0;
  WebPAnimDecoder*dec=WebPAnimDecoderNew(&wd,&o);if(!dec)return jerr("AnimDecoderNew","WebPAnimDecoderNew failed (demux/validation)");
  WebPAnimInfo info;WebPAnimDecoderGetInfo(dec,&info);
  if((unsigned long long)info.canvas_width*info.canvas_height>MAX_PIXELS)return jerr("harness","output dimensions outside harness limit");
  cimg_t fr[MAX_FRAMES];int n=0;int later=0;
  while(WebPAnimDecoderHasMoreFrames(dec)&&n<MAX_FRAMES){uint8_t*buf;int ts;if(!WebPAnimDecoderGetNext(dec,&buf,&ts)){later=1;break;}
   size_t sz=(size_t)info.canvas_width*info.canvas_height*4;fr[n].w=info.canvas_width;fr[n].h=info.canvas_height;fr[n].rgba=malloc(sz);memcpy(fr[n].rgba,buf,sz);n++;}
  if(n==0)return jerr("decode","WebPAnimDecoderGetNext failed on first frame");
  char ex[160];snprintf(ex,sizeof ex,",\"declared_frame_count\":%u,\"later_frame_error\":%s,\"loop_count\":%u",info.frame_count,later?"true":"null",info.loop_count);
  jok(fr,n,"RGBA8 canvas",dump,ex);return 0;}
 return jerr("usage","unknown mode");}
