/* gdk-pixbuf animated-GIF disposal-2 out-of-bounds memset trigger.
 * Build: cc -fsanitize=address trigger.c -o trigger $(pkg-config --cflags --libs gdk-pixbuf-2.0)
 */
#include <gdk-pixbuf/gdk-pixbuf.h>

int main (int argc, char **argv)
{
    GError *err = NULL;
    GdkPixbufAnimation *anim;
    GdkPixbufAnimationIter *iter;
    GTimeVal t = { 1000000, 0 };
    int i;

    if (argc < 2) { g_printerr ("usage: %s FILE\n", argv[0]); return 2; }

    anim = gdk_pixbuf_animation_new_from_file (argv[1], &err);
    if (anim == NULL) { g_printerr ("load failed: %s\n", err->message); return 1; }
    g_printerr ("loaded %dx%d, static=%d\n",
                gdk_pixbuf_animation_get_width (anim),
                gdk_pixbuf_animation_get_height (anim),
                gdk_pixbuf_animation_is_static_image (anim));

    /* The bug is in the frame compositor, so it needs playback, not just a load. */
    G_GNUC_BEGIN_IGNORE_DEPRECATIONS
    iter = gdk_pixbuf_animation_get_iter (anim, &t);
    for (i = 0; i < 4; i++) {
        GdkPixbuf *p = gdk_pixbuf_animation_iter_get_pixbuf (iter);   /* <-- invalid memset */
        g_printerr ("frame %d: %dx%d\n", i,
                    gdk_pixbuf_get_width (p), gdk_pixbuf_get_height (p));
        t.tv_sec += 1;                                                /* past every delay */
        gdk_pixbuf_animation_iter_advance (iter, &t);
    }
    G_GNUC_END_IGNORE_DEPRECATIONS
    return 0;
}
