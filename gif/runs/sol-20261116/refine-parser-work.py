from pathlib import Path
p=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src/parse.rs');s=p.read_text()
for name in ['parse_extension_verified','parse_extension']:
 start=s.index('fn '+name+'(');end=s.index('\n}',start)+2;f=s[start:end]
 a='3 * (b.len() as int - p as int + 1)';assert f.count(a)==1
 f=f.replace(a,'3 * (b.len() as int - p as int)');s=s[:start]+f+s[end:]
start=s.index('fn loop_count_of(');end=s.index('\n}',start)+2;f=s[start:end]
a='            payloads = payloads.push(payload);';assert f.count(a)==1
f=f.replace(a,a+'\n            let tail=crate::model::sub_scan(b@,p as int).unwrap();\n            lemma_sub_scan_bounds(b@,p as int,tail.0,tail.1);')
s=s[:start]+f+s[end:];p.write_text(s)
