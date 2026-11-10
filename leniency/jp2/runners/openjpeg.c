/* OpenJPEG 2.5.x (libopenjp2 built from github.com/uclouvain/openjpeg) decode runner.
   Usage: openjpeg MODE FILE [DUMP]; MODE = default (strict) | nonstrict (opj_decoder_set_strict_mode(false)).
   Normalisation policy (runner-side, not OpenJPEG's): colour channels by colour space (gray 1, sRGB 3, unspecified by
   component count), first extra component is alpha, unsigned = v*255/max rounded, signed components are shifted by +2^(prec-1),
   subsampled components are nearest-neighbour replicated; sYCC/eYCC/CMYK/ICC-only and >4 components are reported as skip (no conversion attempted). */
#include "../../modern/runner_common.h"
#include <openjpeg.h>
static int quiet(const char*m,void*d){(void)m;(void)d;return 0;}
static void qmsg(const char*m,void*d){(void)m;(void)d;}
int main(int argc,char**argv){
 if(argc<3)return jerr("usage","openjpeg MODE FILE [DUMP]");
 size_t len;uint8_t*d=read_file(argv[2],&len);if(!d)return jerr("io","cannot read");
 const char*dump=argc>3?argv[3]:NULL;int strict=strcmp(argv[1],"nonstrict")!=0;(void)quiet;
 OPJ_CODEC_FORMAT fmt=OPJ_CODEC_JP2;const char*kind="jp2";
 if(len>=2&&d[0]==0xff&&d[1]==0x4f){fmt=OPJ_CODEC_J2K;kind="j2k";}
 opj_codec_t*cd=opj_create_decompress(fmt);
 opj_set_info_handler(cd,qmsg,NULL);opj_set_warning_handler(cd,qmsg,NULL);opj_set_error_handler(cd,qmsg,NULL);
 opj_dparameters_t p;opj_set_default_decoder_parameters(&p);
 if(!opj_setup_decoder(cd,&p))return jerr("setup","opj_setup_decoder");
 opj_decoder_set_strict_mode(cd,strict?OPJ_TRUE:OPJ_FALSE);
 opj_stream_t*st=opj_stream_create_default_file_stream(argv[2],1);
 opj_image_t*im=NULL;
 if(!opj_read_header(st,cd,&im))return jerr("read_header","opj_read_header failed");
 if(im->x1<=im->x0||im->y1<=im->y0)return jerr("header","empty image area");
 unsigned long long W=im->x1-im->x0,H=im->y1-im->y0;
 if(W*H>MAX_PIXELS)return jerr("harness","output dimensions outside harness limit");
 if(!opj_decode(cd,st,im))return jerr("decode","opj_decode failed");
 if(!opj_end_decompress(cd,st))return jerr("decode","opj_end_decompress failed");
 int nc=im->numcomps;
 for(int i=0;i<nc;i++)if(!im->comps[i].data)return jerr("decode","component without data");
 int colour;const char*cs;
 switch(im->color_space){
  case OPJ_CLRSPC_GRAY:colour=1;cs="gray";break;
  case OPJ_CLRSPC_SRGB:colour=3;cs="srgb";break;
  case OPJ_CLRSPC_UNSPECIFIED:case OPJ_CLRSPC_UNKNOWN:colour=nc<=2?1:3;cs="unspecified";break;
  default:printf("{\"status\":\"skip\",\"error_class\":\"not-applicable\",\"message\":\"colour space %d needs conversion the runner does not implement\"}\n",(int)im->color_space);return 0;}
 if(nc<colour||nc>colour+1){printf("{\"status\":\"skip\",\"error_class\":\"not-applicable\",\"message\":\"%d components for %s\"}\n",nc,cs);return 0;}
 int alpha=nc==colour+1;
 uint8_t*px=malloc((size_t)W*H*4);
 for(unsigned y=0;y<H;y++)for(unsigned x=0;x<W;x++){
  uint8_t v[4]={0,0,0,255};
  for(int c=0;c<nc;c++){opj_image_comp_t*cc=&im->comps[c];
   unsigned cx=(unsigned)((unsigned long long)x*cc->w/W),cy=(unsigned)((unsigned long long)y*cc->h/H);
   long long s=cc->data[(size_t)cy*cc->w+cx];unsigned pr=cc->prec;if(cc->sgnd)s+=1LL<<(pr-1);
   long long mx=(pr>=31)?0x7fffffffLL:((1LL<<pr)-1);if(s<0)s=0;if(s>mx)s=mx;
   uint8_t o=(uint8_t)((s*255+mx/2)/mx);
   if(c<colour){if(colour==1){v[0]=v[1]=v[2]=o;}else v[c]=o;} else v[3]=o;}
  memcpy(px+((size_t)y*W+x)*4,v,4);}
 cimg_t fr={(int)W,(int)H,px};char ex[200];
 snprintf(ex,sizeof ex,",\"container\":\"%s\",\"components\":%d,\"prec0\":%u,\"sgnd0\":%u,\"colour_space\":\"%s\",\"alpha\":%d",kind,nc,im->comps[0].prec,im->comps[0].sgnd,cs,alpha);
 char orig[64];snprintf(orig,sizeof orig,"%s/%dc/%ubit",cs,nc,im->comps[0].prec);
 jok(&fr,1,orig,dump,ex);return 0;}
