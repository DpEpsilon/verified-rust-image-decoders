/* JasPer (github.com/jasper-software/jasper) decode runner. Usage: jasper MODE FILE [DUMP]. Same normalisation policy as
   openjpeg.c; only gray and RGB colour models, with an optional opacity component. Others are skip. */
#include "../../modern/runner_common.h"
#include <jasper/jasper.h>
int main(int argc,char**argv){
 if(argc<3)return jerr("usage","jasper MODE FILE [DUMP]");
 const char*dump=argc>3?argv[3]:NULL;
 jas_conf_clear();jas_conf_set_max_mem_usage(400u*1024*1024);jas_conf_set_multithread(0);jas_conf_set_debug_level(0);
 if(jas_init_library())return jerr("init","jas_init_library");
 if(jas_init_thread())return jerr("init","jas_init_thread");
 jas_stream_t*st=jas_stream_fopen(argv[2],"rb");if(!st)return jerr("io","cannot open");
 jas_image_t*im=jas_image_decode(st,-1,"");
 if(!im)return jerr("decode","jas_image_decode failed");
 int w=jas_image_width(im),h=jas_image_height(im);
 if(w<=0||h<=0||(unsigned long long)w*h>MAX_PIXELS)return jerr("harness","output dimensions outside harness limit");
 int fam=jas_clrspc_fam(jas_image_clrspc(im));
 int comp[3]={-1,-1,-1};int colour;
 if(fam==JAS_CLRSPC_FAM_GRAY){colour=1;comp[0]=jas_image_getcmptbytype(im,JAS_IMAGE_CT_COLOR(JAS_CLRSPC_CHANIND_GRAY_Y));}
 else if(fam==JAS_CLRSPC_FAM_RGB){colour=3;for(int i=0;i<3;i++)comp[i]=jas_image_getcmptbytype(im,JAS_IMAGE_CT_COLOR(JAS_CLRSPC_CHANIND_RGB_R+i));}
 else{printf("{\"status\":\"skip\",\"error_class\":\"not-applicable\",\"message\":\"colour family %d not converted by runner\"}\n",fam);return 0;}
 for(int i=0;i<colour;i++)if(comp[i]<0)return jerr("layout","missing colour component");
 int al=jas_image_getcmptbytype(im,JAS_IMAGE_CT_OPACITY);
 uint8_t*px=malloc((size_t)w*h*4);
 for(int y=0;y<h;y++)for(int x=0;x<w;x++){uint8_t v[4]={0,0,0,255};
  for(int k=0;k<colour+1;k++){int c=k<colour?comp[k]:al;if(c<0)continue;
   int cw=jas_image_cmptwidth(im,c),ch=jas_image_cmptheight(im,c);
   long long s=jas_image_readcmptsample(im,c,(unsigned)((long long)x*cw/w),(unsigned)((long long)y*ch/h));
   int pr=jas_image_cmptprec(im,c);if(jas_image_cmptsgnd(im,c))s+=1LL<<(pr-1);
   long long mx=(1LL<<pr)-1;if(s<0)s=0;if(s>mx)s=mx;uint8_t o=(uint8_t)((s*255+mx/2)/mx);
   if(k<colour){if(colour==1)v[0]=v[1]=v[2]=o;else v[k]=o;}else v[3]=o;}
  memcpy(px+((size_t)y*w+x)*4,v,4);}
 cimg_t fr={w,h,px};char orig[48];snprintf(orig,sizeof orig,"%s/%dcmpt",colour==1?"gray":"rgb",jas_image_numcmpts(im));
 jok(&fr,1,orig,dump,"");return 0;}
