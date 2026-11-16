from pathlib import Path
p=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src/gif.rs');s=p.read_text()
a='''        let report=decode_image_pixels(self.b,self.hdr,self.pol,image,&mut self.table,&mut self.canvas);
        proof {
            self.work@=self.work@+report.work@;
            self.charged_pixels@=(self.charged_pixels@ as int+rect_area(image.fh.rect)) as nat;
        }
        proof {lemma_measured_pixels(prepared,*self,image,report.work@);}''';assert s.count(a)==1;s=s.replace(a,'        let report=self.render_pixels(image);')
pos=s.index('    #[verifier::spinoff_prover]\n    fn image_prepared(')
s=s[:pos]+'''    #[verifier::spinoff_prover]
    fn render_pixels(&mut self,image:parse::ImageStep)->(report:crate::lzw::LzwReport)
        requires storage_wf(*old(self)),image_ready(*old(self),image),
        ensures storage_wf(*final(self)),measured_pixels(*old(self),*final(self),image),
            pixels_observation(old(self).b@,old(self).hdr,old(self).pol,image,old(self).canvas@,final(self).canvas@,report),
            final(self).b==old(self).b,final(self).hdr==old(self).hdr,final(self).pol==old(self).pol,
            final(self).info==old(self).info,final(self).p==old(self).p,walk_state_same(*old(self),*final(self)),
            final(self).still==old(self).still,final(self).state==old(self).state,final(self).prev==old(self).prev,
            saved_view(*final(self))==saved_view(*old(self)),(final(self).saved is Some)==(old(self).saved is Some),
            final(self).log@.view()==old(self).log@.view(),
    {
        hide(storage_wf);hide(pixels_observation);hide(measured_pixels);
        hide(crate::model::mp);hide(crate::parse::hdr_ok);hide(crate::parse::image_data_post);hide(crate::lzw::read_d);
        proof {lemma_storage_bounds(*self);}
        let ghost before=*self;
        let report=decode_image_pixels(self.b,self.hdr,self.pol,image,&mut self.table,&mut self.canvas);
        proof {
            self.work@=self.work@+report.work@;
            self.charged_pixels@=(self.charged_pixels@ as int+rect_area(image.fh.rect)) as nat;
            lemma_measured_pixels(before,*self,image,report.work@);
            reveal(prev_bounds);lemma_storage_reframe(before,*self);
        }
        report
    }

'''+s[pos:];p.write_text(s)
