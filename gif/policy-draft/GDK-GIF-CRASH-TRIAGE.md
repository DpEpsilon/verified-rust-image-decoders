# GdkPixbuf GIF crash triage

Written by the Sol triage agent on 2026-11-14 and saved here verbatim by the coordinator (the agent's instructions prohibited it from writing the file). Not independently reproduced by the coordinator.

**Classify this as potentially exploitable heap memory corruption, provisionally High severity—not crash-only, and not demonstrated Critical/code execution.** The existing probe makes a negative `memset` length become a huge `size_t`, but the installed libc does **not** necessarily sweep forward until it faults: the native call returned after zeroing neighbouring heap memory. That return invalidates the strongest argument for treating this as denial of service only.

Reviewed 2026-11-14. Source: upstream 2.42.12 GIF snapshots. Runtime: Debian `libgdk-pixbuf-2.0-0` **2.42.12+dfsg-4+deb13u1**, glibc **2.41-12+deb13u4**, x86-64 in the workspace's gVisor sandbox. Only the recorded 80-byte `probe:clip-dispose2-x-beyond` input and existing runner were used.

## Faulting operation and file control

In `io-gif-animation.c:447–456`, disposal 2 calls:

`memset(B + R*y + 4*x, 0, (x_end - x)*4)`

Here `B` is the pixel buffer of `anim->last_frame_data`, `R` its rowstride, `x` the previous frame's left offset, and `x_end = MIN(x + frame_width, screen_width)`. The loop runs while `y < MIN(y_offset + frame_height, screen_height)`.

The file controls screen dimensions, frame dimensions/offsets, disposal and the existence of a following frame. Those fields control the allocation size, relative destination offset and length; the allocator chooses `B`, so this is not an arbitrary absolute-address write. The destination is a heap pixel allocation, not a stack buffer. Large allocator allocations could use mmap; the recorded native allocation lies in the process's `[heap]` mapping.

All operands of `(x_end - x)*4` are signed `int` after integer promotion. With 16-bit GIF geometry, this example needs no signed arithmetic overflow: screen width 4, frame x=6, width=2 yield `x_end=4`, `(4-6)*4 = -8`. The `memset` prototype converts -8 to `size_t`: **18,446,744,073,709,551,608 bytes** on the tested 64-bit build, or 4,294,967,288 bytes on a 32-bit build. The checked offset multiplication/addition checks numerical overflow, not membership in the pixel allocation, and does not validate the length.

The recorded canvas is 4×2 RGBA: 32 allocated bytes, rowstride 16. On the first disposal row, the destination is `B+24`, eight bytes before the allocation ends. The following row would use `B+40` if execution reaches it. The operation is an out-of-bounds **write**, not a source over-read or `memcpy`. Its fill byte is always **zero**, including alpha; neither GIF palette bytes nor the declared background colour supply the fill pattern.

## What actually happens before the crash

ASan intercepted the existing runner's call and reported `negative-size-param (size=-8)`, destination 24 bytes into a 32-byte heap allocation. That interception aborts before the native fill and cannot establish the native overwrite distance.

A bounded ptrace run placed ordinary debugger breakpoints immediately before and after the installed loader's `memset` call. It captured `size_t=-8`, fill 0, and then hit the **post-call breakpoint**. Comparing the surrounding 1 KiB before and after showed 209 previously nonzero bytes changed to zero, including bytes before and after the canvas allocation; no changed byte lay outside the store ranges predicted by disassembly.

The selected glibc implementation uses AVX-512 non-temporal stores for this huge length. Its end-address arithmetic wraps, so it performs bounded head/loop/tail stores and returns rather than filling the entire nominal length. For the observed destination alignment, disassembly and memory comparison establish **536 distinct bytes written** in two ranges: destination−264 through destination−9, and destination through destination+279. Relative to the 32-byte canvas, the stores extend **240 bytes before the allocation and 272 bytes beyond its end**. No unmapped page stopped this first call. A separate native run eventually received SIGSEGV at NULL; a debugger captured RIP=0, rather than an instruction inside `memset`. The agent did not investigate that later null control-flow target further.

Other libc/CPU paths can instead fault during a forward fill, after corrupting whatever writable mappings precede the first inaccessible address; the distance depends on allocation placement and the selected implementation. Neither the source nor the nominal huge length establishes a universal crash-at-buffer-end property. This runtime observation is specific to the tested build, but disproves a general claim that the process necessarily dies inside `memset`.

## Control flow and reachability

A single-threaded ordinary `memset` invokes no application callbacks or allocator operations. If it really faults and SIGSEGV has its default disposition, the process terminates without normal GObject finalizers or C++ stack unwinding. Other threads or installed signal handlers can execute around such a fault, but their existence alone does not demonstrate exploitation.

Here the call **returns**, so corrupted neighbouring objects or allocator metadata can be consumed by subsequent disposal/compositing, allocation or destruction. Fixed-zero writes are more restricted than attacker-supplied byte copies, but can still corrupt pointers, lengths and object state. No concrete code-execution chain was developed or attempted. The demonstrated overwrite-and-return is enough to reject a crash-only classification.

The recorded description also conflates loading and playback: `gdk_pixbuf_animation_new_from_file()` returns successfully; the ASan stack reaches the bug later from `run_anim()` line 45, through `gdk_pixbuf_animation_iter_get_pixbuf()` while advancing to a frame after the bad disposal-2 frame. Loading alone is insufficient.

- The animation constructor followed by playback reaches the bug.
- `gdk_pixbuf_loader_write/close` followed by `get_animation` and playback reaches the same bug; the existing runner's `forced-anim` mode reproduced the ASan report.
- Static `gdk_pixbuf_loader_get_pixbuf()` paths did not trigger it; existing whole-buffer and byte-at-a-time modes returned the first frame successfully.
- `gdk_pixbuf_new_from_file()` selects the GIF static load function, which requests the first static image, not disposal of later frames; the recorded `gdk-file` probe succeeds.

Source-confirmed consumers include **GTK 3.24.43 `GtkImage` file loading**, which retrieves a loader animation and advances/renders its iterator, and **Eye of GNOME 47.0**, which does the same in `eog_image_real_load()` and `eog_image_iter_advance()`. These accept file contents supplied by users or downloads when applications open/display them. This does not imply all GTK apps, current GTK 4 apps or browser GIF decoders use this path.

**Tumbler 4.20.0's pixbuf thumbnailer is not evidence of automatic exposure:** it creates an iterator only to check whether the first frame is loaded, then retrieves the static pixbuf without advancing/rendering later frames. Do not classify thumbnailers as affected merely because they link GdkPixbuf; a thumbnailer must actually traverse the animation compositor to trigger this defect.

## Maintainer decision and limits

Treat this as a security-relevant out-of-bounds write and pursue private upstream triage, not merely a robustness crash. Provisional **High** reflects untrusted-file reachability, file-influenced relative writes outside the canvas, and a demonstrated return to normal code after heap corruption. **Critical or reliable RCE is not established**: the writes are zeros, no particular victim application was exploited, and heap layout, libc and application behaviour determine what the corruption permits.

Confidence is **high** in the root cause, tested overwrite-and-return, and animation/static API distinction; confidence is **moderate** in exploitation beyond DoS. The agent did not test other architectures/libcs, run actual GUI consumers, establish affected/fixed version ranges, or reproduce against a separately built pristine 2.42.12 library. The installed Debian build remains affected despite its other patches. ASan, native tracing and breakpoints used bounded foreground runs of the existing runner; no heavy builds, Verus, new attack inputs or background jobs were used.

Evidence under `gif/triage-evidence-20261114/`:

- `asan-anim.stderr` and `asan-forced-anim.stderr` record the invalid lengths and allocations.
- `memset-debug.txt` records pre/post-call registers, memory snapshots, changed bytes and mappings.
- `libc-memset-disassembly.txt` and `gif-disposal-disassembly.txt` record the installed instructions.
- `native.strace`, `native-debug.txt` and `package-versions.txt` record the later null fault and environment.

Sources:

- Local GIF source: `gif/sources/gdk-pixbuf-2.42.12/io-gif-animation.c:397–472`, `io-gif.c:363–488,612–624,832–879,959–1017`, and `io-gif-animation.h:130–159`.
- Recorded input/results: `gif/policy-draft/probes/results.json:23078–23219`.
- Existing runner: `leniency/gif/runners/gdk_anim.c:29–58,75–103`.
- GdkPixbuf 2.42.12 allocation implementation: https://raw.githubusercontent.com/GNOME/gdk-pixbuf/2.42.12/gdk-pixbuf/gdk-pixbuf.c
- GdkPixbuf 2.42.12 animation constructor: https://raw.githubusercontent.com/GNOME/gdk-pixbuf/2.42.12/gdk-pixbuf/gdk-pixbuf-animation.c
- GTK 3.24.43 GtkImage: https://raw.githubusercontent.com/GNOME/gtk/3.24.43/gtk/gtkimage.c
- Eye of GNOME 47.0: https://raw.githubusercontent.com/GNOME/eog/47.0/src/eog-image.c
- Tumbler 4.20.0 pixbuf thumbnailer: https://raw.githubusercontent.com/xfce-mirror/tumbler/tumbler-4.20.0/plugins/pixbuf-thumbnailer/pixbuf-thumbnailer.c
