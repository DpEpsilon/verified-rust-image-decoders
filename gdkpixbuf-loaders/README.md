# gdkpixbuf-loaders: verified BMP loader for gdk-pixbuf (scaffold)

`libpixbufloader_verified_bmp.so` is a gdk-pixbuf loader module that decodes BMP
with the verified `dib_verus` core, called as `decode_with_policy(bytes,
BmpMode::BmpFile, BmpProfile::GdkCompat)`: POLICY-SPEC's measured
gdk-pixbuf 2.42.12 profile (`GdkCompat-2.42.12`). No other profile is reachable
from the module. Applications and `libgdk_pixbuf` are
unchanged; the module is selected through a loaders.cache file.

## How this system provides BMP

gdk-pixbuf 2.42.12 here loads BMP from a loadable module,
`/usr/lib/x86_64-linux-gnu/gdk-pixbuf-2.0/2.10.0/loaders/libpixbufloader-bmp.so`,
listed in `/usr/lib/x86_64-linux-gnu/gdk-pixbuf-2.0/2.10.0/loaders.cache`
(`"bmp" 5 ... "BM" "" 100`). It is not built into `libgdk_pixbuf`, and
`GDK_PIXBUF_MODULEDIR` / `GDK_PIXBUF_MODULE_FILE` are unset, so the system
cache is authoritative. Replacing the BMP decoder therefore needs only a
different cache, passed per process with `GDK_PIXBUF_MODULE_FILE`:

* **ours only**: a cache listing just this module. Every BMP goes through the
  verified core; every other format becomes unloadable in that process.
* **combined**: every stock module plus this one, **with ours listed last**.
  gdk-pixbuf builds its module list by prepending as it reads the cache, so
  when two modules tie (same `BM` signature score, same name `bmp`), the one
  listed last wins. This holds for content sniffing
  (`gdk_pixbuf_new_from_file`, `gdk_pixbuf_loader_new`) and for lookup by name
  (`gdk_pixbuf_loader_new_with_type("bmp")`); `tests/compare.py` checks both.
  A cache that lists ours first silently keeps using the stock decoder.

A system-wide install (copying the `.so` into the module directory and
rerunning `gdk-pixbuf-query-loaders --update-cache`) would leave the order to
the directory listing, so this scaffold does not recommend it.

## Build and install

```sh
export PATH=/opt/cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/claude-0/gdkpixbuf-target
cargo build --offline --release          # never --locked
SO=$CARGO_TARGET_DIR/release/libpixbufloader_verified_bmp.so
Q=$(pkg-config --variable=gdk_pixbuf_query_loaders gdk-pixbuf-2.0)
D=$(pkg-config --variable=gdk_pixbuf_moduledir gdk-pixbuf-2.0)

$Q "$SO" > verified-bmp-only.cache                 # ours only
$Q "$D"/*.so "$SO" > verified-bmp-combined.cache   # stock + ours, ours LAST

GDK_PIXBUF_MODULE_FILE=$PWD/verified-bmp-combined.cache some-gtk-app image.bmp
```

The cache stores the absolute path of the `.so`; copy it somewhere stable
before generating a cache for real use.

## Interface implemented

From `gdk-pixbuf/gdk-pixbuf-io.h` (headers only; no gdk-pixbuf source was used):

* `fill_info`: name `bmp`, signature `"BM"` (no mask, relevance 100), MIME
  types `image/bmp image/x-bmp image/x-MS-bmp`, extension `bmp`, flags
  `GDK_PIXBUF_FORMAT_THREADSAFE` (the module has no global state; it is not
  `WRITABLE`), license `MIT`.
* `fill_vtable`: `begin_load`, `load_increment`, `stop_load`. The whole-file
  `load(FILE*)` is not provided: gdk-pixbuf then feeds files through the
  incremental functions itself, and leaving it out keeps `FILE*` handling out
  of the unsafe surface.
* `load_increment` appends to a buffer capped at the core's own input limit,
  `dib_verus::dib::MAX_INPUT` = 64 MiB. A write that would exceed it fails at
  once with `GDK_PIXBUF_ERROR_INSUFFICIENT_MEMORY`; nothing past 64 MiB is
  buffered, and the buffer never reserves more than 64 MiB.
* `stop_load` decodes the buffer, then calls `size_func(&w, &h)`; if that sets
  either value to 0 (the `gdk_pixbuf_get_file_info` convention) it returns
  success without allocating a pixbuf. Otherwise it creates the pixbuf, fills
  it, and calls `prepared_func(pixbuf, NULL)` then one
  `updated_func(pixbuf, 0, 0, w, h)`. A size request other than 0 is ignored;
  `GdkPixbufLoader` scales the finished image itself.
* Errors: core `FileTooLarge` and `OutputTooLarge` map to
  `INSUFFICIENT_MEMORY` (`InvalidDimensions`, the 8192 cap, maps to `CORRUPT_IMAGE`), every other core error to `CORRUPT_IMAGE` (the
  message names the core's R15 error), and internal failures, including caught
  panics, to `GDK_PIXBUF_ERROR_FAILED`. Every entry point runs under
  `catch_unwind`, and the release profile keeps `panic = "unwind"`.
* Reentrancy. gdk-pixbuf emits `size-prepared`, `area-prepared` and
  `area-updated` from inside `stop_load`, and sets its `priv->closed` flag only
  after `stop_load` returns, so an application handler can call
  `gdk_pixbuf_loader_close` or `gdk_pixbuf_loader_write` on the same loader
  while the module is still in `stop_load`. The module gives the context a
  lifecycle state (`Active`, `Stopping`, `Dead`) in a byte beside it:
  a nested `stop_load` returns success without freeing (only the outermost call
  frees, once, after its callbacks return); a nested `load_increment` fails with
  `GDK_PIXBUF_ERROR_FAILED` without touching the accumulated bytes. No Rust
  reference to the context is held across a C callback — `stop_load` copies the
  callbacks and user data out first. `tests/reentrancy.rs` runs the whole
  close/write/close-then-write matrix under AddressSanitizer.

## Behavioural differences from the stock loader

Measured by `tests/compare.py` on the 71 BMP files in `../dib/corpus/`,
against stock gdk-pixbuf 2.42.12 driven through a real `GdkPixbufLoader`
(`tests/chunk_driver.c`):

| outcome | files |
|---|---|
| both accept, same RGBA | 58 |
| both accept, different RGBA | 0 |
| stock accepts, ours rejects | 1 (`l2rgb_read`) |
| ours accepts, stock rejects | 0 |
| both reject | 12 |

So 58 files are accepted by both and 12 rejected by both; `l2rgb_read.bmp` is
the one known accept-vs-reject difference (an earlier runner-based count
listed it as a 13th shared reject): stock decodes its 3,158,064 x 48 header, the core rejects it because the
width exceeds the 8192 cap. (The leniency runner's `file` mode has its own
output-size limit and also rejects it, so `compare.py` takes stock's result
from the chunk driver, not the runner.) Per file, our module's result equals
the core's direct output (status, size, RGBA SHA-256) through
`new_from_file`, `new_from_stream`, loader writes of 1 byte, 7 bytes and the
whole file, and `gdk_pixbuf_loader_new_with_type("bmp")`.

* **has_alpha.** The module sets it by stock's rule: bit depth 32, or RLE4 /
  RLE8 compression, read from the header and not from pixel values. The core
  returns it as `Decoded::rle_or_32bpp`. It agrees with stock on all 58 files
  both accept (18 have an alpha channel in both). The core's blank-image and
  RLE fills are transparent exactly under that rule, and 16-bit BITFIELDS alpha
  is written opaque under GdkCompat, so no accepted input has alpha below 255
  without `has_alpha`. `shim::finish` still checks that, and returns a
  `GDK_PIXBUF_ERROR_FAILED` GError if a decoder ever breaks it
  (`has_alpha_guard_gives_failed_gerror` in `tests/limits.rs`).
* **Size caps.** The core rejects width or height above 8192 and input or
  output above 64 MiB. Stock has no such caps, so it accepts larger images
  (`l2rgb_read`).
* **Error codes.** Both caps are errors in ours: `FileTooLarge` and
  `OutputTooLarge` map to `INSUFFICIENT_MEMORY`, the 8192 dimension cap to
  `CORRUPT_IMAGE`. Stock's code for a malformed or oversized file can differ,
  as can the message text (`BMP image rejected: InvalidDimensions`); error
  domains are the same.
* **Callback timing.** Stock reports progress during writes: with 1-byte
  writes of `hopper.bmp` (49,290 bytes) its size and prepared signals fire
  after 4,096 bytes (when `GdkPixbufLoader` has sniffed the type) and it emits
  128 `area-updated` signals, one per row. Ours fires size, prepared and a
  single full-image update inside `gdk_pixbuf_loader_close`, so a progressive
  viewer shows nothing until the file is complete, and
  `gdk_pixbuf_get_file_info` reads the whole file instead of the header.
* **Memory.** Stock writes rows into the pixbuf as they arrive. Ours holds the
  whole file (at most 64 MiB; while the buffer grows from 32 to 64 MiB a
  realloc that cannot extend in place briefly holds both, 96 MiB), then the
  core's RGBA output (at most 64 MiB, its only allocation) alongside it, frees
  the input, and copies into the pixbuf (at most 64 MiB). Worst case is about
  128 MiB live during `stop_load`; for typical files it is 2-3 times the
  decoded image size.
* **Allocation failure.** The 64 MiB caps bound how much a hostile image can
  make this module *ask* for; they are a correctness limit (the core's R1/R5
  policy), **not** an out-of-memory safety net, and the module does not turn
  memory pressure into a clean rejection at every step. What is fallible: the
  accumulator buffer (`try_reserve_exact`, reporting
  `GDK_PIXBUF_ERROR_INSUFFICIENT_MEMORY`), the per-load context
  (`alloc` + null check, same error), and every error message (a `'static`
  `CStr`, built without `format!`, so reporting never allocates). What is not:
  the verified core's own RGBA output `Vec` (up to 64 MiB) and the
  `GdkPixbuf` pixel store, which use infallible Rust/`g_malloc` allocation, and
  the `malloc` inside `g_set_error_literal` and the metadata `Vec`s that
  `fill_info` builds **once at module load** (not per image; they are leaked by
  design). Under genuine memory exhaustion any of those can abort the host
  rather than fail the load.
* **Saving.** Not supported. Because ours is also named `bmp` and wins the
  lookup, `gdk_pixbuf_save(..., "bmp")` fails with "does not support saving
  the image format: bmp" under either cache. Renaming the module would restore
  saving but let `new_with_type("bmp")` bypass the verified decoder; this
  scaffold chooses full replacement for loading.
* **Truncated files.** Same as stock: a file shorter than the 54-byte headers
  is an error; a longer prefix decodes, with every pixel row that is not
  completely present zero-filled. Checked on all 70 prefixes of a 2x2 24-bit
  image (status, RGBA hash and alpha equal to stock).

## What is verified and what is not

The Verus proofs cover `dib_verus::decode` only: memory safety, termination,
the 64 MiB caps, and equality with the reference model. Nothing in this crate
is verified:

* `src/ffi.rs`: the hand-written C declarations and struct layouts (checked by
  reading the 2.42.12 headers, and exercised by the tests, not proven), every
  `unsafe` block, the callback order and the ownership of the context and the
  pixbuf reference. The reentrancy state machine and the context's byte layout
  are exercised by `tests/reentrancy.rs`, not proven.
* `src/shim.rs`: the input buffer and its cap, the RGBA-to-pixbuf copy with
  rowstride, and the check of the core's `has_alpha`. It is safe Rust (`forbid(unsafe_code)`).
* `src/bmp.rs`: the format metadata and the error mapping.
* gdk-pixbuf itself, including type sniffing and scaling done by
  `GdkPixbufLoader`.

## Tests

* `cargo test --offline --release` (`tests/limits.rs`): empty input, every
  prefix of a valid 2x2 file (1-53 bytes error, 54-70 decode with incomplete
  rows zero-filled, as stock), a header declaring a 64 MiB + 1 file and an
  8192x8192 32-bit image (errors with under 64 KiB allocated), a 256 MiB
  declared output (`INSUFFICIENT_MEMORY`, no allocation), 64 MiB + 1 MiB of
  actual input (refused at the cap), and through the extern functions: callback
  order `size, prepared, updated(0,0,w,h)` for 1-, 7- and 70-byte chunks, the
  `GError` codes, the `has_alpha` guard returning `GDK_PIXBUF_ERROR_FAILED`,
  and a panicking decoder returning `GDK_PIXBUF_ERROR_FAILED`.
* `cargo test --offline --release --test reentrancy` (`tests/reentrancy.rs`):
  builds the module, generates a one-module cache, compiles `tests/reentrancy.c`
  with `-fsanitize=address`, and runs a real `GdkPixbufLoader` over a BMP it
  builds in memory (no corpus bytes). For each of `size-prepared`,
  `area-prepared` and `area-updated`, the handler re-enters the loader with
  `gdk_pixbuf_loader_close`, with `gdk_pixbuf_loader_write`, and with close then
  write. It asserts (under AddressSanitizer and LeakSanitizer) no double free,
  no leak, exactly one of each signal, a successful outer close, the nested
  write refused with `GDK_PIXBUF_ERROR_FAILED`, and no GLib warnings. If
  `cc -fsanitize=address`, `pkg-config` or `gdk-pixbuf-query-loaders` is
  missing, the test prints why and skips (it cannot run the harness there).
* `python3 tests/compare.py` builds the module, generates both caches, compiles
  `tests/chunk_driver.c`, and runs the corpus through the leniency runner
  (`/tmp/claude-0/leniency-target/gdk-runner`) and the driver. It writes only
  under `/tmp/claude-0/gdkpixbuf-compare/` and exits non-zero if our module
  disagrees with the core or with itself across paths, or if `has_alpha`
  differs between the core and the module.

## Plan: ICO, TGA, ICNS

The shim is already generic: `ffi::{begin_load, load_increment, stop_load,
fill_vtable, fill_info}` are generic over `shim::Format`, which is static
metadata plus `fn decode(&[u8]) -> Result<Image, LoadError>`. Each further
module is one `impl Format` and one `export_loader!(...)` line, with no new
unsafe code.

1. Turn this crate into a workspace: `pixbuf-shim` (rlib: `shim.rs`, `ffi.rs`,
   the `export_loader!` macro) and one cdylib crate per format, because each
   `.so` may export only one `fill_vtable`/`fill_info` pair.
2. **ICO** (`ico_verus`): copy the stock cache entry's metadata: name `ico`,
   signature `"  \001   "` with mask `"zz znz"` at relevance 100, its eight
   MIME types, extensions `ico cur`. Which directory entry to return (stock
   returns one image) must be decided and documented. Stock ICO is writable, so
   the save conflict described above for BMP applies.
3. **TGA**: TGA has no magic number; the stock entry uses masked patterns
   (e.g. `" \001\001"` / `"x  "`) at relevance 100. Copy them from the stock
   cache so sniffing ties and listing order decides, as for BMP.
4. **ICNS**: `"icns"` at relevance 100, as in the stock entry.
5. `compare.py` gains a `--format` switch (glob, module path, expected
   description); `limits.rs` becomes a generic test over `Format` for the
   truncation, empty and cap cases, with format-specific oversized headers.
6. If a progressive display matters for some format, add an optional
   `fn header(&[u8]) -> Option<(u32, u32)>` to `Format` so `load_increment` can
   call `size_func` early. That needs a verified header parser to stay inside
   the "verified decode" boundary, so it is deferred.
