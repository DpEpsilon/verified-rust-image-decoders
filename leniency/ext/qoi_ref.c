/* Reference QOI decoder (phoboslab/qoi qoi.h) wrapped to the leniency JSON contract. Usage: qoi-ref FILE [RGBA8_DUMP] */
#include <stdio.h>
#include <stdlib.h>
#include <glib.h>
#define QOI_IMPLEMENTATION
#include "qoi.h"
static void fail(const char *c,const char *m){printf("{\"status\":\"error\",\"error_class\":\"%s\",\"message\":\"%s\"}\n",c,m);}
int main(int argc,char **argv){
    gchar *data=NULL;gsize len=0;
    if(argc<2||!g_file_get_contents(argv[1],&data,&len,NULL)){fail("io","cannot read input");return 0;}
    qoi_desc d;
    /* the reference decoder only takes the header's own channel count when channels=0; it does not bound w*h before allocating */
    if(len>=14){unsigned long w=((unsigned char)data[4]<<24)|((unsigned char)data[5]<<16)|((unsigned char)data[6]<<8)|(unsigned char)data[7];
        unsigned long h=((unsigned char)data[8]<<24)|((unsigned char)data[9]<<16)|((unsigned char)data[10]<<8)|(unsigned char)data[11];
        if(w&&h&&w<(1ul<<32)&&h<(1ul<<32)&&w*h>16000000ul&&!memcmp(data,"qoif",4)){fail("harness-limit","more than 16,000,000 pixels; not decoded");return 0;}}
    unsigned char *px=qoi_decode(data,(int)len,&d,0);
    if(!px){fail("QoiRejected","qoi_decode returned NULL");return 0;}
    size_t n=(size_t)d.width*d.height,c=d.channels;
    guchar *r8=g_malloc(n*4);guint16 *r16=g_malloc(n*8);int alpha=0;
    for(size_t i=0;i<n;i++){unsigned char *p=px+i*c;unsigned char q[4]={p[0],p[1],p[2],c==4?p[3]:255};
        for(int k=0;k<4;k++){r8[i*4+k]=q[k];r16[i*4+k]=GUINT16_TO_LE((guint16)(q[k]*257));}
        if(q[3]!=255)alpha=1;}
    char *h8=g_compute_checksum_for_data(G_CHECKSUM_SHA256,r8,n*4),*h16=g_compute_checksum_for_data(G_CHECKSUM_SHA256,(guchar*)r16,n*8);
    if(argc>=3)g_file_set_contents(argv[2],(char*)r8,n*4,NULL);
    printf("{\"status\":\"ok\",\"width\":%u,\"height\":%u,\"orig_color_type\":\"%s\",\"alpha_present\":%s,\"rgba_sha256\":\"%s\",\"rgba16_sha256\":\"%s\",\"selection\":\"single\"}\n",
        d.width,d.height,c==4?"RGBA8":"RGB8",alpha?"true":"false",h8,h16);
    return 0;}
