// Link against the installed gdk-pixbuf (and, transitively, gobject/glib).
// The module only calls exported, stable API: gdk_pixbuf_new,
// gdk_pixbuf_get_pixels_with_length, gdk_pixbuf_error_quark, g_object_unref,
// g_set_error_literal.
fn main() {
    pkg_config::Config::new()
        .atleast_version("2.26") // gdk_pixbuf_get_pixels_with_length
        .probe("gdk-pixbuf-2.0")
        .expect("gdk-pixbuf-2.0 development files (pkg-config) are required");
}
