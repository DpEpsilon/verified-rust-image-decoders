/* Copy of ../shared/gdk.c with an additional rgba16_sha256 (8-bit samples expanded by *257, little-endian). */
#include <gdk-pixbuf/gdk-pixbuf.h>
#include <gio/gio.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

static void fail(const char *s) {
    char *v=g_utf8_make_valid(s?s:"decode error",-1);
    GString *j=g_string_new(NULL);
    for(const unsigned char *p=(const unsigned char*)v;*p;p++){
        if(*p=='"'||*p=='\\'){g_string_append_c(j,'\\');g_string_append_c(j,*p);}
        else if(*p<32)g_string_append_printf(j,"\\u%04x",*p);
        else g_string_append_c(j,*p);
    }
    printf("{\"status\":\"error\",\"error_class\":\"gdk\",\"message\":\"%s\"}\n",j->str);
    g_string_free(j,TRUE);g_free(v);
}
int main(int argc,char **argv) {
    if(argc<4){fail("usage: runner FORMAT MODE FILE [DUMP]");return 0;}
    const char *fmt=argv[1], *mode=argv[2];
    GError *e=NULL;GdkPixbuf *p=NULL;
    if(!strcmp(mode,"file")) p=gdk_pixbuf_new_from_file(argv[3],&e);
    else {
        gchar *bytes=NULL;gsize len=0;
        if(!g_file_get_contents(argv[3],&bytes,&len,&e))goto end;
        if(!strcmp(mode,"stream")) {
            GInputStream *stream=g_memory_input_stream_new_from_data(bytes,len,NULL);
            p=gdk_pixbuf_new_from_stream(stream,NULL,&e);g_object_unref(stream);
        } else {
            GdkPixbufLoader *loader=!strncmp(mode,"forced-",7)?gdk_pixbuf_loader_new_with_type(fmt,&e):gdk_pixbuf_loader_new();
            if(loader){
                gsize step=strstr(mode,"byte")?1:(strstr(mode,"chunk")?4096:(len?len:1));
                for(gsize i=0;i<len;i+=step)if(!gdk_pixbuf_loader_write(loader,(guchar*)bytes+i,MIN(step,len-i),&e))break;
                if(!e)gdk_pixbuf_loader_close(loader,&e);
                if(!e && gdk_pixbuf_loader_get_pixbuf(loader))p=g_object_ref(gdk_pixbuf_loader_get_pixbuf(loader));
                g_object_unref(loader);
            }
        }
        g_free(bytes);
    }
end:
    if(!p){fail(e?e->message:"no pixbuf");if(e)g_error_free(e);return 0;}
    int w=gdk_pixbuf_get_width(p),h=gdk_pixbuf_get_height(p),n=gdk_pixbuf_get_n_channels(p);
    if(w<=0||h<=0||(guint64)w*h>16000000||n<3||n>4){fail("output size or channels outside harness limit");g_object_unref(p);return 0;}
    guchar *out=g_malloc((gsize)w*h*4),*src=gdk_pixbuf_get_pixels(p);
    gboolean alpha=gdk_pixbuf_get_has_alpha(p),trans=FALSE;
    int stride=gdk_pixbuf_get_rowstride(p);
    for(int y=0;y<h;y++)for(int x=0;x<w;x++){
        guchar *a=src+(gsize)y*stride+x*n,*b=out+((gsize)y*w+x)*4;
        b[0]=a[0];b[1]=a[1];b[2]=a[2];b[3]=alpha?a[3]:255;
        if(b[3]!=255)trans=TRUE;
    }
    char *hash=g_compute_checksum_for_data(G_CHECKSUM_SHA256,out,(gsize)w*h*4);
    guint16 *wide=g_malloc((gsize)w*h*8);
    for(gsize i=0;i<(gsize)w*h*4;i++){guint16 v=out[i]*257;wide[i]=GUINT16_TO_LE(v);}
    char *hash16=g_compute_checksum_for_data(G_CHECKSUM_SHA256,(guchar*)wide,(gsize)w*h*8);
    if(argc>=5)g_file_set_contents(argv[4],(char*)out,(gssize)w*h*4,NULL);
    printf("{\"status\":\"ok\",\"width\":%d,\"height\":%d,\"orig_color_type\":\"%s\",\"alpha_present\":%s,\"rgba_sha256\":\"%s\",\"rgba16_sha256\":\"%s\"}\n",w,h,alpha?"RGBA8":"RGB8",trans?"true":"false",hash,hash16);
    g_free(hash);g_free(hash16);g_free(wide);g_free(out);g_object_unref(p);if(e)g_error_free(e);return 0;
}
