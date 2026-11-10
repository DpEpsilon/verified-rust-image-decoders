/* Deterministic synthetic JPEG 2000 seed generator on libopenjp2 (OpenJPEG 2.5.x) encoder API.
   Usage: mkseeds OUTDIR. Pixel data are closed-form gradients; no external image. */
#include <openjpeg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
typedef struct{const char*name;int jp2;int w,h,nc,prec,sgnd;OPJ_COLOR_SPACE cs;int alpha_last;int lossy;int mct;int nres;int cbw,cbh;int prog;int tile;int prec_on;int sop_eph;int layers;int icc;int bypass;}S;
static const char*OUT_DIR;
static void gen(const S*s){
 opj_cparameters_t p;opj_set_default_encoder_parameters(&p);
 p.cp_disto_alloc=1;p.tcp_numlayers=s->layers?s->layers:1;
 if(s->lossy){p.irreversible=1;for(int i=0;i<p.tcp_numlayers;i++)p.tcp_rates[i]=(float)(s->lossy>>i>0?(s->lossy>>i):1);if(p.tcp_numlayers==1)p.tcp_rates[0]=(float)s->lossy;}
 else {for(int i=0;i<p.tcp_numlayers;i++)p.tcp_rates[i]=(float)((p.tcp_numlayers-1-i)*4);p.tcp_rates[p.tcp_numlayers-1]=0;}
 p.tcp_mct=s->mct;if(s->nres)p.numresolution=s->nres;else{int m=s->w<s->h?s->w:s->h;int n=1;while(n<6&&(1<<n)<=m)n++;p.numresolution=n;if(s->tile&&(1<<(n-1))>s->tile){n=1;while(n<6&&(1<<n)<=s->tile)n++;p.numresolution=n;}}if(s->cbw){p.cblockw_init=s->cbw;p.cblockh_init=s->cbh;}
 if(s->prog>=0)p.prog_order=(OPJ_PROG_ORDER)s->prog;
 if(s->tile){p.tile_size_on=1;p.cp_tx0=0;p.cp_ty0=0;p.cp_tdx=s->tile;p.cp_tdy=s->tile;}
 if(s->prec_on){p.csty|=0x01;p.res_spec=p.numresolution>1?p.numresolution-1:1;for(int i=0;i<p.numresolution;i++){p.prcw_init[i]=1<<(i+3>7?7:i+3);p.prch_init[i]=p.prcw_init[i];}}
 if(s->sop_eph)p.csty|=0x06;
 if(s->bypass)p.mode|=0x01;
 opj_image_cmptparm_t cp[8];memset(cp,0,sizeof cp);
 for(int i=0;i<s->nc;i++){cp[i].dx=cp[i].dy=1;cp[i].w=s->w;cp[i].h=s->h;cp[i].prec=s->prec;cp[i].bpp=s->prec;cp[i].sgnd=s->sgnd;}
 opj_image_t*im=opj_image_create(s->nc,cp,s->cs);im->x0=0;im->y0=0;im->x1=s->w;im->y1=s->h;
 int maxv=(1<<s->prec)-1;
 for(int c=0;c<s->nc;c++){for(int y=0;y<s->h;y++)for(int x=0;x<s->w;x++){
   long v=(c==0)?(x*maxv)/(s->w>1?s->w-1:1):(c==1)?(y*maxv)/(s->h>1?s->h-1:1):(((x^y)*37+c*61)&maxv);
   if(s->alpha_last&&c==s->nc-1)v=((x*13+y*7)%5==0)?0:maxv-(x+y)%3;
   if(s->sgnd)v-=(1<<(s->prec-1));
   im->comps[c].data[y*s->w+x]=(OPJ_INT32)v;}}
 if(s->alpha_last)im->comps[s->nc-1].alpha=1;
 if(s->icc){im->icc_profile_len=132;im->icc_profile_buf=calloc(1,132);im->icc_profile_buf[3]=132;memcpy(im->icc_profile_buf+12,"mntr",4);memcpy(im->icc_profile_buf+16,"RGB ",4);memcpy(im->icc_profile_buf+20,"XYZ ",4);memcpy(im->icc_profile_buf+36,"acsp",4);}
 opj_codec_t*cd=opj_create_compress(s->jp2?OPJ_CODEC_JP2:OPJ_CODEC_J2K);
 if(!opj_setup_encoder(cd,&p,im)){fprintf(stderr,"setup failed %s\n",s->name);return;}
 char path[512];snprintf(path,sizeof path,"%s/%s.%s",OUT_DIR,s->name,s->jp2?"jp2":"j2k");
 opj_stream_t*st=opj_stream_create_default_file_stream(path,0);
 int ok=opj_start_compress(cd,im,st)&&opj_encode(cd,st)&&opj_end_compress(cd,st);
 if(!ok)fprintf(stderr,"encode failed %s\n",s->name);
 opj_stream_destroy(st);opj_destroy_codec(cd);opj_image_destroy(im);}
int main(int argc,char**argv){
 OUT_DIR=argc>1?argv[1]:".";
 const OPJ_COLOR_SPACE G=OPJ_CLRSPC_GRAY,R=OPJ_CLRSPC_SRGB,Y=OPJ_CLRSPC_SYCC,K=OPJ_CLRSPC_CMYK,U=OPJ_CLRSPC_UNSPECIFIED;
 /* name jp2 w h nc prec sgnd cs alpha lossy mct nres cbw cbh prog tile prec_on sop layers icc bypass */
 S L[]={
 {"j2k-gray8-ll",0,16,16,1,8,0,G,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-gray8-1x1",0,1,1,1,8,0,G,0,0,0,1,0,0,-1,0,0,0,1,0,0},
 {"j2k-gray8-17x13",0,17,13,1,8,0,G,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-rgb8-ll-mct",0,16,16,3,8,0,R,0,0,1,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-rgb8-ll-nomct",0,16,16,3,8,0,R,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-rgb8-lossy-r8",0,32,32,3,8,0,R,0,8,1,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-rgb8-lossy-r30",0,32,32,3,8,0,R,0,30,1,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-gray8-lossy-r10",0,32,32,1,8,0,G,0,10,0,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-gray12-ll",0,16,16,1,12,0,G,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-gray16-ll",0,16,16,1,16,0,G,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-gray16-lossy",0,24,24,1,16,0,G,0,6,0,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-gray1-ll",0,16,16,1,1,0,G,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-gray4-ll",0,16,16,1,4,0,G,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-gray10-ll",0,16,16,1,10,0,G,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-gray8-signed-ll",0,16,16,1,8,1,G,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-rgb12-ll",0,16,16,3,12,0,R,0,0,1,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-rgb16-ll",0,16,16,3,16,0,R,0,0,1,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-rgba8-ll",0,16,16,4,8,0,R,1,0,1,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-ga8-ll",0,16,16,2,8,0,G,1,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-5comp-unspec",0,16,16,5,8,0,U,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-cmyk8-ll",0,16,16,4,8,0,K,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"j2k-rgb8-tiles8",0,24,16,3,8,0,R,0,0,1,0,0,0,-1,8,0,0,1,0,0},
 {"j2k-rgb8-prec-sop-eph",0,32,32,3,8,0,R,0,0,1,4,0,0,-1,0,1,1,1,0,0},
 {"j2k-gray8-layers3-lossy",0,32,32,1,8,0,G,0,20,0,3,0,0,-1,0,0,0,3,0,0},
 {"j2k-gray8-rpcl",0,32,32,1,8,0,G,0,0,0,3,0,0,2,0,1,0,1,0,0},
 {"j2k-gray8-pcrl",0,32,32,1,8,0,G,0,0,0,3,0,0,3,0,1,0,1,0,0},
 {"j2k-rgb8-cprl",0,16,16,3,8,0,R,0,0,0,2,0,0,4,0,1,0,1,0,0},
 {"j2k-rgb8-rlcp",0,16,16,3,8,0,R,0,0,0,2,0,0,1,0,0,0,2,0,0},
 {"j2k-gray8-cblk4x4",0,32,32,1,8,0,G,0,0,0,0,4,4,-1,0,0,0,1,0,0},
 {"j2k-gray8-cblk64x64-nres1",0,64,64,1,8,0,G,0,0,0,1,64,64,-1,0,0,0,1,0,0},
 {"j2k-gray8-bypass",0,32,32,1,8,0,G,0,0,0,0,0,0,-1,0,0,0,1,0,1},
 {"jp2-gray8-ll",1,16,16,1,8,0,G,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"jp2-rgb8-ll",1,16,16,3,8,0,R,0,0,1,0,0,0,-1,0,0,0,1,0,0},
 {"jp2-rgb8-lossy",1,32,32,3,8,0,R,0,12,1,0,0,0,-1,0,0,0,1,0,0},
 {"jp2-rgba8-ll-cdef",1,16,16,4,8,0,R,1,0,1,0,0,0,-1,0,0,0,1,0,0},
 {"jp2-ga8-cdef",1,16,16,2,8,0,G,1,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"jp2-gray16-ll",1,16,16,1,16,0,G,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"jp2-gray12-lossy",1,24,24,1,12,0,G,0,8,0,0,0,0,-1,0,0,0,1,0,0},
 {"jp2-rgb8-icc",1,16,16,3,8,0,R,0,0,1,0,0,0,-1,0,0,0,1,1,0},
 {"jp2-sycc8",1,16,16,3,8,0,Y,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"jp2-cmyk8",1,16,16,4,8,0,K,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"jp2-unspec-3c",1,16,16,3,8,0,U,0,0,0,0,0,0,-1,0,0,0,1,0,0},
 {"jp2-rgb8-tiles-sop",1,24,16,3,8,0,R,0,0,1,0,0,0,-1,8,0,1,1,0,0},
 {"jp2-gray8-1x1",1,1,1,1,8,0,G,0,0,0,1,0,0,-1,0,0,0,1,0,0},
 };
 for(size_t i=0;i<sizeof L/sizeof*L;i++)gen(&L[i]);
 return 0;}
