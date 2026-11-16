from pathlib import Path
p=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src/compose_new.rs');s=p.read_text()
def rep(a,b,n=1):
 global s
 assert s.count(a)==n,(a,s.count(a));s=s.replace(a,b)
rep('        c.3 as int <= sh as int,','''        c.3 as int <= sh as int,
        c.0 == r.left, c.2 == r.top,
        c.1 as int <= r.left as int + r.width as int,
        c.3 as int <= r.top as int + r.height as int,''')
for name in ['clear_rect','copy_rect']:
 start=s.index('pub fn '+name+'(');end=s.index('\n}',start)+2
 f=s[start:end]
 assert f.count('    ensures\n')==1
 f=f.replace('    ensures\n','    ensures\n        work@ as int <= 2 * (r.width as int) * (r.height as int),\n',1)
 sigend=f.index('\n    requires');f=f[:sigend]+' -> (work: Ghost<nat>)'+f[sigend:]
 f=f.replace('    let (x0, x1, y0, y1) = clip_of(r, sw, sh);','    let ghost mut steps = 0nat;\n    let (x0, x1, y0, y1) = clip_of(r, sw, sh);')
 assert f.count('        return;')==1;f=f.replace('        return;','        return Ghost(steps);')
 # Frame facts survive the loop through scalar span bounds.
 f=f.replace('            y0 <= y <= y1,','''            y0 <= y <= y1,
            steps as int == (y as int-y0 as int) * (1 + x1 as int-x0 as int),
            x1 as int-x0 as int <= r.width as int,
            y1 as int-y0 as int <= r.height as int,
            r.width>=1, r.height>=1,''',1)
 f=f.replace('        let mut x: u32 = x0;','        proof { steps = steps + 1; }\n        let mut x: u32 = x0;')
 f=f.replace('                x0 <= x <= x1,','''                x0 <= x <= x1,
                steps as int == (y as int-y0 as int) * (1+x1 as int-x0 as int) + 1 + x as int-x0 as int,
                y0<=y,
                x1 as int-x0 as int <= r.width as int,
                y1 as int-y0 as int <= r.height as int,
                r.width>=1, r.height>=1,''',1)
 # Count the column body before the four straight-line byte assignments.
 assert f.count('            x += 1;')==1;f=f.replace('            x += 1;','            proof { steps = steps + 1; }\n            x += 1;')
 assert f.count('        y += 1;')==1
 f=f.replace('        y += 1;','''        proof {
            assert(steps as int == (y as int + 1-y0 as int) * (1+x1 as int-x0 as int)) by(nonlinear_arith)
                requires steps as int == (y as int-y0 as int) * (1+x1 as int-x0 as int) + 1+x1 as int-x0 as int;
        }
        y += 1;''')
 f=f[:-2]+'''    proof {
        assert(steps as int <= 2 * (r.width as int) * (r.height as int)) by(nonlinear_arith)
            requires steps as int == (y1 as int-y0 as int) * (1+x1 as int-x0 as int),
                0<=x1 as int-x0 as int<=r.width as int,0<=y1 as int-y0 as int<=r.height as int,
                r.width>=1,r.height>=1;
    }
    Ghost(steps)
}'''
 s=s[:start]+f+s[end:]
p.write_text(s)
print('Instrumented exact clear/copy row and column costs')
