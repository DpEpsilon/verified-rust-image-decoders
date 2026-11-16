from pathlib import Path
p=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src/gif.rs');s=p.read_text()
a='''            final(self).charged_pixels@ as int==old(self).charged_pixels@ as int+rect_area(image.fh.rect),
            old(self).work@<=final(self).work@,
            final(self).work@ as int<=old(self).work@ as int+5*image.data.data_len as int+2*rect_area(image.fh.rect),'''
assert s.count(a)==1;s=s.replace(a,'            measured_pixels(*old(self),*final(self),image),')
a='hide(prepared_matches);hide(pixels_observation);';assert s.count(a)==1;s=s.replace(a,a+'hide(measured_pixels);')
a='''        let ghost observed=report;
        let overrun=match report.public_result()''';assert s.count(a)==1;s=s.replace(a,'''        proof {lemma_measured_pixels(prepared,*self,image,report.work@);}
        let ghost costed=*self;
        let ghost observed=report;
        let overrun=match report.public_result()''')
a='''                    reveal(prev_bounds);lemma_storage_reframe(prepared,*self);
                    lemma_image_finish''';assert s.count(a)==1;s=s.replace(a,'''                    reveal(prev_bounds);lemma_storage_reframe(prepared,*self);
                    lemma_pixels_cost_frame(prepared,costed,*self,image);
                    lemma_image_finish''')
a='''        proof {lemma_image_finish(original,image,prepared,*self,observed,Ok(meta));}''';assert s.count(a)==1;s=s.replace(a,'''        proof {lemma_pixels_cost_frame(prepared,costed,*self,image);lemma_image_finish(original,image,prepared,*self,observed,Ok(meta));}''')
pos=s.index('proof fn lemma_previous_area(')
s=s[:pos]+'''spec fn measured_pixels(a:Frames,d:Frames,im:parse::ImageStep)->bool {
    &&& d.charged_pixels@ as int==a.charged_pixels@ as int+rect_area(im.fh.rect)
    &&& a.work@<=d.work@
    &&& d.work@ as int<=a.work@ as int+5*im.data.data_len as int+2*rect_area(im.fh.rect)
}
proof fn lemma_measured_pixels(a:Frames,d:Frames,im:parse::ImageStep,cost:nat)
    requires d.work@==a.work@+cost,d.charged_pixels@==a.charged_pixels@+rect_area(im.fh.rect),
        cost as int<=5*im.data.data_len as int+2*rect_area(im.fh.rect),
    ensures measured_pixels(a,d,im),
{}
proof fn lemma_pixels_cost_frame(a:Frames,p:Frames,d:Frames,im:parse::ImageStep)
    requires measured_pixels(a,p,im),d.work@==p.work@,d.charged_pixels@==p.charged_pixels@,
    ensures measured_pixels(a,d,im),
{}
'''+s[pos:];p.write_text(s)
