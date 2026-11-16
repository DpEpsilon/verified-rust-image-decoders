from pathlib import Path
root=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src')
p=root/'compose_new.rs';s=p.read_text();a='assert(steps as int <= 2 * (r.width as int) * (r.height as int)) by(nonlinear_arith)';assert s.count(a)==2;s=s.replace(a,'assert(steps as int <= 2 * ((r.width as int) * (r.height as int))) by(nonlinear_arith)');p.write_text(s)
p=root/'gif.rs';s=p.read_text()
a='spec fn rect_area(r:Rect)->int {(r.width as int)*(r.height as int)}\nspec fn prior_area(d:Frames)->int';assert s.count(a)==1;s=s.replace(a,'spec fn rect_area(r:Rect)->nat {(r.width as nat)*(r.height as nat)}\nspec fn prior_area(d:Frames)->nat')
a='        final(self).charged_pixels@ as int==old(self).charged_pixels@ as int+rect_area(image.fh.rect),';assert s.count(a)==1;s=s.replace(a,'''        saved_view(*final(self))==saved_view(*old(self)),
            (final(self).saved is Some)==(old(self).saved is Some),
            final(self).charged_pixels@ as int==old(self).charged_pixels@ as int+rect_area(image.fh.rect),''')
a='            old(self).p<=final(self).p,old(self).work@<=final(self).work@,';assert s.count(a)==1;s=s.replace(a,'            old(self).p<=final(self).p<=old(self).b.len(),old(self).work@<=final(self).work@,')
p.write_text(s)
