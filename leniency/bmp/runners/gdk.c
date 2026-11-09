#include <gdk-pixbuf/gdk-pixbuf.h>
#include <gio/gio.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

static void fail(const char *message) {
    char *valid = g_utf8_make_valid(message ? message : "decode error", -1);
    GString *escaped = g_string_new(NULL);
    for (const unsigned char *p=(unsigned char*)valid; *p; p++) {
        if (*p == '"' || *p == '\\') {g_string_append_c(escaped,'\\');g_string_append_c(escaped,*p);}
        else if (*p < 32) g_string_append_printf(escaped,"\\u%04x",*p);
        else g_string_append_c(escaped,*p);
    }
    printf("{\"status\":\"error\",\"error_class\":\"gdk\",\"message\":\"%s\"}\n",escaped->str);
    g_string_free(escaped,TRUE);g_free(valid);
}
int main(int argc,char **argv) {
    if(argc < 3) {fail("usage: gdk-runner file|stream|whole|byte|chunk FILE [DUMP]");return 2;}
    GError *e=NULL; GdkPixbuf *p=NULL;
    if(!strcmp(argv[1],"file")) p=gdk_pixbuf_new_from_file(argv[2],&e);
    else {
        gchar *bytes=NULL; gsize len=0;
        if(!g_file_get_contents(argv[2],&bytes,&len,&e)) goto end;
        if(!strcmp(argv[1],"stream")) {
            GInputStream *s=g_memory_input_stream_new_from_data(bytes,len,NULL);
            p=gdk_pixbuf_new_from_stream(s,NULL,&e);
            g_object_unref(s);
        } else {
            GdkPixbufLoader *loader=gdk_pixbuf_loader_new();
            gsize chunk=(!strcmp(argv[1],"byte") ? 1 : !strcmp(argv[1],"chunk") ? 4096 : len ? len : 1);
            for(gsize i=0;i<len;i+=chunk) {
                gsize n=MIN(chunk,len-i);
                if(!gdk_pixbuf_loader_write(loader,(guchar*)bytes+i,n,&e)) break;
            }
            if(!e) gdk_pixbuf_loader_close(loader,&e);
            if(!e && gdk_pixbuf_loader_get_pixbuf(loader)) p=g_object_ref(gdk_pixbuf_loader_get_pixbuf(loader));
            g_object_unref(loader);
        }
        g_free(bytes);
    }
end:
    if(!p) {fail(e ? e->message : "no pixbuf");if(e)g_error_free(e);return 0;}
    int w=gdk_pixbuf_get_width(p),h=gdk_pixbuf_get_height(p),n=gdk_pixbuf_get_n_channels(p);
    if(w<=0||h<=0||(guint64)w*h>16000000||n<3||n>4) {fail("output size or channels outside harness limit");g_object_unref(p);return 0;}
    guchar *out=g_malloc((gsize)w*h*4); gboolean has_alpha=gdk_pixbuf_get_has_alpha(p),transparent=FALSE;
    guchar *src=gdk_pixbuf_get_pixels(p);int stride=gdk_pixbuf_get_rowstride(p);
    for(int y=0;y<h;y++)for(int x=0;x<w;x++) {
        guchar *v=src+(gsize)y*stride+x*n,*q=out+((gsize)y*w+x)*4;
        q[0]=v[0];q[1]=v[1];q[2]=v[2];q[3]=has_alpha?v[3]:255;
        if(q[3]!=255)transparent=TRUE;
    }
    char *hash=g_compute_checksum_for_data(G_CHECKSUM_SHA256,out,(gsize)w*h*4);
    if(argc>=4) g_file_set_contents(argv[3],(char*)out,(gssize)w*h*4,NULL);
    printf("{\"status\":\"ok\",\"width\":%d,\"height\":%d,\"orig_color_type\":\"%s\",\"alpha_present\":%s,\"rgba_sha256\":\"%s\"}\n",w,h,has_alpha?"RGBA8":"RGB8",transparent?"true":"false",hash);
    g_free(hash);g_free(out);g_object_unref(p);if(e)g_error_free(e);
    return 0;
}
