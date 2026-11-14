# Reproducer: animated-GIF disposal-2 out-of-bounds `memset`

Files here: `trigger.gif` (80 bytes), `trigger.hexdump.txt` (annotated bytes),
`trigger.c`, this README.

## Build

    cc -fsanitize=address trigger.c -o trigger $(pkg-config --cflags --libs gdk-pixbuf-2.0)

`trigger.c` loads the file with `gdk_pixbuf_animation_new_from_file()`, takes an
iterator with `gdk_pixbuf_animation_get_iter()`, then calls
`gdk_pixbuf_animation_iter_get_pixbuf()` and
`gdk_pixbuf_animation_iter_advance()` a few times. Loading alone does not reach
the bug, and neither does `gdk_pixbuf_new_from_file()`, which takes the static
path; the disposal runs only when a frame after the bad one is composited.

## Run

    ./trigger trigger.gif

## Expected output against the library installed here

    loaded 4x2, static=0
    frame 0: 4x2
    =================================================================
    ==13108==ERROR: AddressSanitizer: negative-size-param: (size=-8)
        #0 0x2b2e75fb7008 in memset ../../../../src/libsanitizer/sanitizer_common/sanitizer_common_interceptors_memintrinsics.inc:87
        #1 0x2b2e773520d8  (/usr/lib/x86_64-linux-gnu/gdk-pixbuf-2.0/2.10.0/loaders/libpixbufloader-gif.so+0x40d8) (BuildId: 6100c8407fca00873c2ff1327c0df79a329704cd)
        #2 0x564b22ec44ec in main (.../trigger+0x14ec)
        ...
    0x503000001888 is located 24 bytes inside of 32-byte region [0x503000001870,0x503000001890)

    SUMMARY: AddressSanitizer: negative-size-param ... in memset
    ==13108==ABORTING

Addresses, PIDs and BuildIds vary per run; the load line, the `frame 0` line, the
`negative-size-param: (size=-8)` summary and the "24 bytes inside of 32-byte
region" line are the stable parts. The trailing `-8` is
`(MIN(6 + 2, 4) - 6) * 4 = -8`, a value the `memset` prototype converts to
`size_t` 18446744073709551608.

## Without a sanitizer

Without AddressSanitizer the outcome depends on the libc and on the allocation's
placement. On the glibc build tested here the `memset` call returned after writing
zeros outside the allocation rather than faulting, so the run does not necessarily
crash and produces no diagnostic; ASan is what makes the fault deterministic and
visible.

Tested here with libgdk-pixbuf 2.42.12+dfsg-4+deb13u1 on Debian 13, glibc
2.41-12+deb13u4, x86-64, compiled with that GCC's default `-fsanitize=address`
and run as above. The report also reproduces against a locally built pristine
2.42.12 tree, where the frame above `main` names
`gdk_pixbuf_gif_anim_iter_get_pixbuf ../gdk-pixbuf/io-gif-animation.c:456`.
