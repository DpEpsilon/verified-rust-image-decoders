p="README.md"; s=open(p).read()
a="Every deliberate difference from gdk-pixbuf is listed in"
assert s.count(a)==1
b="""**ICO/CUR** (`ICO-CORE-SPEC 1.0`, `ico/spec-frozen/`). Standalone `.ico`/`.cur` byte slices: directory parse into entry records (invalid entries keep a defect status instead of failing the parse), then `try_decode_entry` for DIB entries by calling the DIB core in `IcoEntryDibSlack` mode. PNG entries are classified, not decoded. Limits: 64 MiB input, 1,024 directory entries. The ICO core lives in `ico/` (`ico/ico_verus`) and depends on `dib/dib_verus` by path.

"""
s=s.replace(a,b+a); open(p,"w").write(s)
