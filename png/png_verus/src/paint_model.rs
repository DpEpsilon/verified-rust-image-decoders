//! Independent reconstructed source pixels and per-position paint progress.
use vstd::prelude::*;
use crate::{colour_model as c,raster_model as r,layout_model as l};

verus! {
pub open spec fn pass_geometry(a: r::Raster,k: nat) -> (nat,nat,nat,nat) {
    if a.ihdr.interlace==0 {(0,0,1,1)}else{l::geometry(k)}
}
pub open spec fn source_cell(a: r::Raster,x: nat,y: nat) -> (nat,nat,nat) {
    if a.ihdr.interlace==0 {(0,x,y)}else{l::inverse(x,y)}
}
pub open spec fn domain(a: r::Raster) -> bool {
    a.ihdr.interlace<=1 && 1<=a.width<=crate::limits::PNG_WIRE_MAX && 1<=a.height<=crate::limits::PNG_WIRE_MAX
}
pub proof fn source_coordinate(a: r::Raster,x: nat,y: nat)
    requires domain(a),x<a.width,y<a.height,
    ensures source_cell(a,x,y).0<r::count(a),source_cell(a,x,y).1<r::pw(a,source_cell(a,x,y).0),
        source_cell(a,x,y).2<r::ph(a,source_cell(a,x,y).0),
        pass_geometry(a,source_cell(a,x,y).0).0+source_cell(a,x,y).1*pass_geometry(a,source_cell(a,x,y).0).2==x,
        pass_geometry(a,source_cell(a,x,y).0).1+source_cell(a,x,y).2*pass_geometry(a,source_cell(a,x,y).0).3==y,
{
    if a.ihdr.interlace!=0 {
        l::point_inverse(a.width,a.height,x,y);
        let at=source_cell(a,x,y);let g=l::geometry(at.0);
        assert(l::point(at.0,at.1,at.2)==(g.0+at.1*g.2,g.1+at.2*g.3));
        assert(pass_geometry(a,at.0)==g);
    }else{assert(source_cell(a,x,y)==(0,x,y));assert(pass_geometry(a,0)==(0,0,1,1));}
}
pub proof fn pass_coordinate(a: r::Raster,k: nat,col: nat,row: nat)
    requires domain(a),k<r::count(a),col<r::pw(a,k),row<r::ph(a,k),
    ensures pass_geometry(a,k).0+col*pass_geometry(a,k).2<a.width,
        pass_geometry(a,k).1+row*pass_geometry(a,k).3<a.height,
        source_cell(a,pass_geometry(a,k).0+col*pass_geometry(a,k).2,
            pass_geometry(a,k).1+row*pass_geometry(a,k).3)==(k,col,row),
{
    if a.ihdr.interlace!=0 {
        l::cell_inverse(a.width,a.height,k,col,row);
        let g=l::geometry(k);let p=l::point(k,col,row);
        assert(p==(g.0+col*g.2,g.1+row*g.3));assert(pass_geometry(a,k)==g);
        assert(source_cell(a,p.0,p.1)==l::inverse(p.0,p.1));
        assert(g.1+row*g.3<a.height);
        assert(source_cell(a,g.0+col*g.2,g.1+row*g.3)==(k,col,row));
    }else{
        assert(k==0);assert(pass_geometry(a,k)==(0,0,1,1));
        assert(r::pw(a,0)==a.width);assert(r::ph(a,0)==a.height);
        assert(row<a.height);assert(source_cell(a,col,row)==(0,col,row));
    }
}
pub proof fn row_location(a: r::Raster,k: nat,row: nat,x: nat,y: nat)
    requires domain(a),k<r::count(a),r::pw(a,k)>0,row<r::ph(a,k),x<a.width,y<a.height,
    ensures c::row_contains(pass_geometry(a,k).0,pass_geometry(a,k).2,
            pass_geometry(a,k).1+row*pass_geometry(a,k).3,r::pw(a,k),x,y)
        ==(source_cell(a,x,y).0==k && source_cell(a,x,y).2==row),
        source_cell(a,x,y).0==k && source_cell(a,x,y).2==row ==> source_cell(a,x,y).1==x/pass_geometry(a,k).2,
{
    source_coordinate(a,x,y);
    let g=pass_geometry(a,k);let at=source_cell(a,x,y);
    assert(g.2>=1 && g.0<g.2);
    if at.0==k && at.2==row {l::axis_forward(g.0,g.2,at.1);}
    if c::row_contains(g.0,g.2,g.1+row*g.3,r::pw(a,k),x,y) {
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod(x as int,g.2 as int);
        pass_coordinate(a,k,x/g.2,row);
    }
}

pub open spec fn source_pixel(input: Seq<u8>,a: r::Raster,colour: c::Context,x: nat,y: nat,wide: bool,black: bool)
    -> Result<Seq<u16>,crate::png::Error>
{
    let at=source_cell(a,x,y);
    c::pixel(colour,r::line(input,a,at.0,at.2),at.1,wide,black)
}
pub open spec fn visited(a: r::Raster,x: nat,y: nat,k: nat,row: nat) -> bool {
    let at=source_cell(a,x,y);at.0<k || at.0==k && at.2<row
}
#[verifier::opaque]
pub open spec fn progress(input: Seq<u8>,a: r::Raster,colour: c::Context,shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,state: Seq<u8>,k: nat,row: nat,black: bool) -> bool {
    before.len()==state.len() && forall|x: nat,y: nat| x<a.width && y<a.height ==>
        if visited(a,x,y,k,row) {
            (#[trigger] source_pixel(input,a,colour,x,y,c::is_wide(shape),black)).is_ok()
            && c::observe(shape,state,a.width,x,y)==c::converted(shape,
                source_pixel(input,a,colour,x,y,c::is_wide(shape),black).get_Ok_0(),c::observe(shape,before,a.width,x,y))
        }else{c::observe(shape,state,a.width,x,y)==c::observe(shape,before,a.width,x,y)}
}
pub proof fn start(input: Seq<u8>,a: r::Raster,colour: c::Context,shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,black: bool)
    ensures progress(input,a,colour,shape,before,before,0,0,black),
{reveal(progress);}

pub proof fn pass_shape(a: r::Raster,k: nat)
    requires domain(a),k<r::count(a),
    ensures pass_geometry(a,k).2>=1,pass_geometry(a,k).0<pass_geometry(a,k).2,
        r::pw(a,k)==l::span(a.width,pass_geometry(a,k).0,pass_geometry(a,k).2),
{
    if a.ihdr.interlace==0 {assert(k==0);}else if k==0 {}else if k==1 {}else if k==2 {}else if k==3 {}else if k==4 {}else if k==5 {}else {assert(k==6);}
}
pub proof fn row_step(input: Seq<u8>,a: r::Raster,colour: c::Context,shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,state: Seq<u8>,after: Seq<u8>,k: nat,row: nat,black: bool,rows: Seq<Seq<u16>>)
    requires domain(a),k<r::count(a),r::pw(a,k)>0,row<r::ph(a,k),
        crate::model::valid_depth(colour.ct,colour.depth),c::geometry(shape,a.width,a.height,before.len()),
        progress(input,a,colour,shape,before,state,k,row,black),
        c::MC(colour,r::line(input,a,k,row),r::pw(a,k),c::is_wide(shape),black)==Some(Ok(rows)),
        after==c::render_row(rows,shape,state,a.width,pass_geometry(a,k).0,pass_geometry(a,k).2,
            pass_geometry(a,k).1+row*pass_geometry(a,k).3,r::pw(a,k)),
    ensures progress(input,a,colour,shape,before,after,k,row+1,black),
{
    hide(c::pixel);hide(c::observe);hide(c::converted);
    reveal(progress);
    let g=pass_geometry(a,k);
    pass_coordinate(a,k,0,row);pass_shape(a,k);
    assert(g.2>=1 && g.0<g.2 && r::pw(a,k)==l::span(a.width,g.0,g.2));
    assert forall|col: int| 0<=col<r::pw(a,k) implies (#[trigger] rows[col]).len()==4 by {
        c::MC_pixel(colour,r::line(input,a,k,row),r::pw(a,k),c::is_wide(shape),black,col as nat,rows);
    }
    c::render_row_observe(rows,shape,state,a.width,a.height,g.0,g.2,g.1+row*g.3,r::pw(a,k),0,0);
    assert(after.len()==before.len());
    assert forall|x: nat,y: nat| x<a.width && y<a.height implies
        if visited(a,x,y,k,row+1) {
            (#[trigger] source_pixel(input,a,colour,x,y,c::is_wide(shape),black)).is_ok()
            && c::observe(shape,after,a.width,x,y)==c::converted(shape,
                source_pixel(input,a,colour,x,y,c::is_wide(shape),black).get_Ok_0(),c::observe(shape,before,a.width,x,y))
        }else{c::observe(shape,after,a.width,x,y)==c::observe(shape,before,a.width,x,y)} by {
        row_location(a,k,row,x,y);
        c::render_row_observe(rows,shape,state,a.width,a.height,g.0,g.2,g.1+row*g.3,r::pw(a,k),x,y);
        let at=source_cell(a,x,y);
        if at.0==k && at.2==row {
            c::MC_pixel(colour,r::line(input,a,k,row),r::pw(a,k),c::is_wide(shape),black,at.1,rows);
            assert(!visited(a,x,y,k,row));
        }else{assert(visited(a,x,y,k,row+1)==visited(a,x,y,k,row));}
    }
}
pub proof fn pass_finished(input: Seq<u8>,a: r::Raster,colour: c::Context,shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,state: Seq<u8>,k: nat,black: bool)
    requires domain(a),k<r::count(a),progress(input,a,colour,shape,before,state,k,r::ph(a,k),black),
    ensures progress(input,a,colour,shape,before,state,k+1,0,black),
{
    reveal(progress);
    assert forall|x: nat,y: nat| x<a.width && y<a.height implies
        visited(a,x,y,k,r::ph(a,k))==visited(a,x,y,k+1,0) by {source_coordinate(a,x,y);}
}
pub proof fn empty_pass(input: Seq<u8>,a: r::Raster,colour: c::Context,shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,state: Seq<u8>,k: nat,black: bool)
    requires domain(a),k<r::count(a),r::pw(a,k)==0 || r::ph(a,k)==0,progress(input,a,colour,shape,before,state,k,0,black),
    ensures progress(input,a,colour,shape,before,state,k+1,0,black),
{
    reveal(progress);
    assert forall|x: nat,y: nat| x<a.width && y<a.height implies
        visited(a,x,y,k,0)==visited(a,x,y,k+1,0) by {source_coordinate(a,x,y);}
}
pub proof fn complete_pixels(input: Seq<u8>,a: r::Raster,colour: c::Context,shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,state: Seq<u8>,black: bool)
    requires domain(a),progress(input,a,colour,shape,before,state,r::count(a),0,black),
    ensures forall|x: nat,y: nat| x<a.width && y<a.height ==>
        (#[trigger] source_pixel(input,a,colour,x,y,c::is_wide(shape),black)).is_ok()
        && c::observe(shape,state,a.width,x,y)==c::converted(shape,
            source_pixel(input,a,colour,x,y,c::is_wide(shape),black).get_Ok_0(),c::observe(shape,before,a.width,x,y)),
{
    hide(c::pixel);hide(c::observe);hide(c::converted);
    reveal(progress);
    assert forall|x: nat,y: nat| x<a.width && y<a.height implies
        (#[trigger] source_pixel(input,a,colour,x,y,c::is_wide(shape),black)).is_ok()
        && c::observe(shape,state,a.width,x,y)==c::converted(shape,
            source_pixel(input,a,colour,x,y,c::is_wide(shape),black).get_Ok_0(),c::observe(shape,before,a.width,x,y)) by {
        source_coordinate(a,x,y);assert(visited(a,x,y,r::count(a),0));
    }
}
/// A complete RGBA image, with no dependency on its overwritten destination.
#[verifier::opaque]
pub open spec fn image_pixels(input: Seq<u8>,a: r::Raster,colour: c::Context,state: Seq<u8>,wide: bool,black: bool) -> bool {
    let shape=(if wide {1nat}else{0nat},state.len(),0nat,0nat,0nat,false);
    state.len()==(if wide {8nat}else{4nat})*a.width*a.height
    && forall|x: nat,y: nat| x<a.width && y<a.height ==>
        (#[trigger] source_pixel(input,a,colour,x,y,wide,black)).is_ok()
        && c::observe(shape,state,a.width,x,y)==c::physical(source_pixel(input,a,colour,x,y,wide,black).get_Ok_0(),wide)
}
#[verifier::spinoff_prover]
pub proof fn image_from_progress(input: Seq<u8>,a: r::Raster,colour: c::Context,before: Seq<u8>,state: Seq<u8>,wide: bool,black: bool)
    requires domain(a),before.len()==(if wide {8nat}else{4nat})*a.width*a.height,
        progress(input,a,colour,(if wide {1nat}else{0nat},before.len(),0nat,0nat,0nat,false),before,state,r::count(a),0,black),
    ensures image_pixels(input,a,colour,state,wide,black),
{
    hide(c::pixel);hide(c::observe);hide(c::physical);
    reveal(image_pixels);reveal(progress);
    complete_pixels(input,a,colour,(if wide {1nat}else{0nat},before.len(),0nat,0nat,0nat,false),before,state,black);
}
pub open spec fn default_raster(doc: crate::document_model::Doc,pol: crate::policy::Policy) -> r::Raster {
    r::shape(doc.ihdr,doc.ihdr.width as nat,doc.ihdr.height as nat,crate::document_model::entries(doc),pol)
}
#[verifier::opaque]
pub open spec fn initial_canvas(b: Seq<u8>,doc: crate::document_model::Doc,pol: crate::policy::Policy,canvas: Seq<u8>) -> bool {
    if doc.actl.is_some() && !doc.info.default_is_frame {
        canvas.len()==4*(doc.ihdr.width as nat)*(doc.ihdr.height as nat)
        && forall|j: int| 0<=j<canvas.len() ==> #[trigger] canvas[j]==0
    }else{
        match crate::document_model::default_stream(b,doc) {
            Some(z)=>image_pixels(r::inflated(z,default_raster(doc,pol),pol).out,default_raster(doc,pol),
                c::from_doc(doc,b),canvas,false,pol.palette_index==crate::policy::PaletteIndex::Black),
            None=>false,
        }
    }
}
pub proof fn initial_transparent(b: Seq<u8>,doc: crate::document_model::Doc,pol: crate::policy::Policy,canvas: Seq<u8>)
    requires doc.actl.is_some(),!doc.info.default_is_frame,
        canvas.len()==4*(doc.ihdr.width as nat)*(doc.ihdr.height as nat),
        forall|j: int| 0<=j<canvas.len() ==> #[trigger] canvas[j]==0,
    ensures initial_canvas(b,doc,pol,canvas),
{reveal(initial_canvas);}
pub proof fn initial_decoded(b: Seq<u8>,doc: crate::document_model::Doc,pol: crate::policy::Policy,canvas: Seq<u8>,z: Seq<u8>)
    requires !(doc.actl.is_some() && !doc.info.default_is_frame),crate::document_model::default_stream(b,doc)==Some(z),
        image_pixels(r::inflated(z,default_raster(doc,pol),pol).out,default_raster(doc,pol),c::from_doc(doc,b),canvas,false,
            pol.palette_index==crate::policy::PaletteIndex::Black),
    ensures initial_canvas(b,doc,pol,canvas),
{reveal(initial_canvas);}
pub proof fn initial_metadata_preserved(b: Seq<u8>,before: crate::document_model::Doc,after: crate::document_model::Doc,pol: crate::policy::Policy,canvas: Seq<u8>)
    requires before.ihdr==after.ihdr,before.plte==after.plte,before.trns==after.trns,
        before.idat==after.idat,before.actl==after.actl,before.info.default_is_frame==after.info.default_is_frame,
    ensures initial_canvas(b,before,pol,canvas)==initial_canvas(b,after,pol,canvas),
{reveal(initial_canvas);}

pub proof fn initial_default_source(b: Seq<u8>,doc: crate::document_model::Doc,pol: crate::policy::Policy,canvas: Seq<u8>)
    requires (doc.actl.is_none() || doc.info.default_is_frame),initial_canvas(b,doc,pol,canvas),
    ensures crate::document_model::default_stream(b,doc).is_some(),
        image_pixels(r::inflated(crate::document_model::default_stream(b,doc).unwrap(),default_raster(doc,pol),pol).out,
            default_raster(doc,pol),c::from_doc(doc,b),canvas,false,pol.palette_index==crate::policy::PaletteIndex::Black),
{reveal(initial_canvas);}
/// Reusing the decoded IDAT source is the first MK paint on transparent black,
/// not OVER of the source on itself.
pub proof fn reuse_progress(input: Seq<u8>,a: r::Raster,colour: c::Context,source: Seq<u8>,blend: bool,black: bool)
    requires domain(a),image_pixels(input,a,colour,source,false,black),
    ensures progress(input,a,colour,(2,source.len(),a.width,0,0,blend),Seq::new(source.len(),|j: int| 0u8),
        if blend {crate::composite_model::zero_alpha(source)}else{source},r::count(a),0,black),
{
    hide(c::pixel);reveal(image_pixels);reveal(progress);
    let zero=Seq::new(source.len(),|j: int| 0u8);
    let after=if blend {crate::composite_model::zero_alpha(source)}else{source};
    let shape=(2nat,source.len(),a.width,0nat,0nat,blend);
    assert(4*(a.height)*(a.width)==4*a.width*a.height) by (nonlinear_arith);
    assert(c::geometry(shape,a.width,a.height,source.len()));
    assert forall|x: nat,y: nat| x<a.width && y<a.height implies
        if visited(a,x,y,r::count(a),0) {
            (#[trigger] source_pixel(input,a,colour,x,y,false,black)).is_ok()
            && c::observe(shape,after,a.width,x,y)==c::converted(shape,
                source_pixel(input,a,colour,x,y,false,black).get_Ok_0(),c::observe(shape,zero,a.width,x,y))
        }else{c::observe(shape,after,a.width,x,y)==c::observe(shape,zero,a.width,x,y)} by {
        source_coordinate(a,x,y);assert(visited(a,x,y,r::count(a),0));
        c::observed_bounds(shape,a.width,a.height,source.len(),x,y);
        let o=c::destination_offset(shape,a.width,x,y);
        assert(o==4*(y*a.width+x));
        assert(c::observe(shape,source,a.width,x,y)==c::observe((0,source.len(),0,0,0,false),source,a.width,x,y));
        assert(c::observe(shape,zero,a.width,x,y)=~=seq![0u8,0u8,0u8,0u8]);
        if blend {
            crate::composite_model::zero_alpha_pixel(source,y*a.width+x);
        }
    }
}

/// P14 disposal through the actual previous-frame reference, not Saved's layout.
pub open spec fn disposed_reference(before: Seq<u8>,reference: Option<Seq<u8>>,cw: nat,previous: Option<(crate::composite_model::Rect,u8)>) -> Seq<u8> {
    match previous {
        Some((rect,1))=>crate::composite_model::clear_rect(before,cw,rect),
        Some((_,2))=>match reference {Some(value)=>value,None=>Seq::empty()},
        _=>before,
    }
}
pub proof fn reference_disposal(before: Seq<u8>,saved: Option<Seq<u8>>,reference: Option<Seq<u8>>,cw: nat,previous: Option<(crate::composite_model::Rect,u8)>)
    requires match previous {Some((rect,2))=>crate::composite_model::saved_before_frame(before,saved,reference,cw,rect),_=>true},
    ensures crate::composite_model::dispose_regions(before,saved,cw,previous)==Some(disposed_reference(before,reference,cw,previous)),
{
    if let Some((rect,2))=previous {crate::composite_model::saved_before_frame_restore(before,saved,reference,cw,rect);}
}
/// One mathematical compositor step: independent MF/MC pixels at each source
/// coordinate, integer OVER on the disposed destination, and its byte footprint.
#[verifier::opaque]
pub open spec fn MK(z: Seq<u8>,a: r::Raster,colour: c::Context,pol: crate::policy::Policy,cw: nat,rect: crate::composite_model::Rect,blend: bool,
    previous: Option<(crate::composite_model::Rect,u8)>,reference: Option<Seq<u8>>,before: Seq<u8>,after: Seq<u8>,reuse: bool) -> bool
{
    let initial=if reuse {Seq::new(before.len(),|j: int| 0u8)}else{disposed_reference(before,reference,cw,previous)};
    let shape=(2nat,before.len(),cw,rect.x,rect.y,blend);
    progress(r::inflated(z,a,pol).out,a,colour,shape,initial,after,r::count(a),0,
        pol.palette_index==crate::policy::PaletteIndex::Black)
    && crate::composite_model::drawing_frame(initial,after,shape,rect.w,rect.h)
}

/// Public M5 pixel-content projection, independent of native decode helpers.
#[verifier::opaque]
pub closed spec fn still_pixels(b: Seq<u8>,p: crate::policy::Policy,wide: bool,state: Seq<u8>) -> bool {
    match crate::document_model::MW_p(b,p.walk_view()) {
        crate::document_model::Outcome::Done(doc,_)=>match crate::document_model::default_stream(b,doc) {
            Some(z)=>image_pixels(r::inflated(z,default_raster(doc,p),p).out,default_raster(doc,p),c::from_doc(doc,b),state,wide,
                p.palette_index==crate::policy::PaletteIndex::Black),
            None=>false,
        },_=>false,
    }
}
pub(crate) proof fn still_pixels_intro(b: Seq<u8>,doc: crate::document_model::Doc,p: crate::policy::Policy,wide: bool,state: Seq<u8>,z: Seq<u8>,ev: crate::document_model::Events)
    requires crate::document_model::MW_p(b,p.walk_view())==crate::document_model::Outcome::Done(doc,ev),
        crate::document_model::default_stream(b,doc)==Some(z),
        image_pixels(r::inflated(z,default_raster(doc,p),p).out,default_raster(doc,p),c::from_doc(doc,b),state,wide,
            p.palette_index==crate::policy::PaletteIndex::Black),
    ensures still_pixels(b,p,wide,state),
{reveal(still_pixels);}
pub(crate) proof fn still_pixels_context(b: Seq<u8>,p: crate::policy::Policy,wide: bool,state: Seq<u8>,doc: crate::document_model::Doc,ev: crate::document_model::Events,z: Seq<u8>)
    requires still_pixels(b,p,wide,state),crate::document_model::MW_p(b,p.walk_view())==crate::document_model::Outcome::Done(doc,ev),
        crate::document_model::default_stream(b,doc)==Some(z),
    ensures image_pixels(r::inflated(z,default_raster(doc,p),p).out,default_raster(doc,p),c::from_doc(doc,b),state,wide,
        p.palette_index==crate::policy::PaletteIndex::Black),
{reveal(still_pixels);}

pub proof fn mk_intro(z: Seq<u8>,a: r::Raster,colour: c::Context,p: crate::policy::Policy,cw: nat,rect: crate::composite_model::Rect,blend: bool,
    previous: Option<(crate::composite_model::Rect,u8)>,reference: Option<Seq<u8>>,before: Seq<u8>,after: Seq<u8>,reuse: bool)
    requires progress(r::inflated(z,a,p).out,a,colour,(2nat,before.len(),cw,rect.x,rect.y,blend),
        if reuse {Seq::new(before.len(),|j: int|0u8)}else{disposed_reference(before,reference,cw,previous)},after,r::count(a),0,
        p.palette_index==crate::policy::PaletteIndex::Black),
        crate::composite_model::drawing_frame(if reuse {Seq::new(before.len(),|j: int|0u8)}else{disposed_reference(before,reference,cw,previous)},
            after,(2nat,before.len(),cw,rect.x,rect.y,blend),rect.w,rect.h),
    ensures MK(z,a,colour,p,cw,rect,blend,previous,reference,before,after,reuse),
{reveal(MK);}
pub proof fn mk_context(z: Seq<u8>,a: r::Raster,colour: c::Context,p: crate::policy::Policy,cw: nat,rect: crate::composite_model::Rect,blend: bool,
    previous: Option<(crate::composite_model::Rect,u8)>,reference: Option<Seq<u8>>,before: Seq<u8>,after: Seq<u8>,reuse: bool)
    requires MK(z,a,colour,p,cw,rect,blend,previous,reference,before,after,reuse),
    ensures progress(r::inflated(z,a,p).out,a,colour,(2nat,before.len(),cw,rect.x,rect.y,blend),
        if reuse {Seq::new(before.len(),|j: int|0u8)}else{disposed_reference(before,reference,cw,previous)},after,r::count(a),0,
        p.palette_index==crate::policy::PaletteIndex::Black),
        crate::composite_model::drawing_frame(if reuse {Seq::new(before.len(),|j: int|0u8)}else{disposed_reference(before,reference,cw,previous)},
            after,(2nat,before.len(),cw,rect.x,rect.y,blend),rect.w,rect.h),
{reveal(MK);}
pub proof fn mk_reference(z: Seq<u8>,a: r::Raster,colour: c::Context,p: crate::policy::Policy,cw: nat,rect: crate::composite_model::Rect,blend: bool,
    previous: Option<(crate::composite_model::Rect,u8)>,r1: Option<Seq<u8>>,r2: Option<Seq<u8>>,before: Seq<u8>,after: Seq<u8>,reuse: bool)
    requires MK(z,a,colour,p,cw,rect,blend,previous,r1,before,after,reuse),
        match previous {Some((_,2))=>r1==r2,_=>true},
    ensures MK(z,a,colour,p,cw,rect,blend,previous,r2,before,after,reuse),
{reveal(MK);}

/// A complete image's actual physical bytes at one independently decoded pixel.
pub(crate) proof fn image_observe(input: Seq<u8>,a: r::Raster,colour: c::Context,state: Seq<u8>,wide: bool,black: bool,x: nat,y: nat)
    requires image_pixels(input,a,colour,state,wide,black),x<a.width,y<a.height,
    ensures state.len()==(if wide {8nat}else{4nat})*a.width*a.height,
        source_pixel(input,a,colour,x,y,wide,black).is_ok(),
        c::observe((if wide {1nat}else{0nat},state.len(),0nat,0nat,0nat,false),state,a.width,x,y)
            ==c::physical(source_pixel(input,a,colour,x,y,wide,black).get_Ok_0(),wide),
{reveal(image_pixels);}

pub(crate) proof fn still_projection_context(b: Seq<u8>,wide: bool,state: Seq<u8>,h: crate::model::Ihdr,z: Seq<u8>,entries: nat,colour: c::Context)
    requires still_pixels(b,crate::policy::STRICT,wide,state),r::default_case(b,h,z,entries),
        match crate::document_model::MW_p(b,crate::document_model::strict()) {
            crate::document_model::Outcome::Done(doc,_)=>c::from_doc(doc,b)==colour,_=>false},
    ensures image_pixels(r::inflated(z,r::shape(h,h.width as nat,h.height as nat,entries,crate::policy::STRICT),crate::policy::STRICT).out,
        r::shape(h,h.width as nat,h.height as nat,entries,crate::policy::STRICT),colour,state,wide,false),
{reveal(still_pixels);crate::walk_policy::profiles_project_exactly();}

/// Equality of every physical pixel block implies equality of the byte slices.
pub(crate) proof fn pixel_blocks_equal(a: Seq<u8>,b: Seq<u8>,bytes: nat,pixels: nat)
    requires bytes>=1,a.len()==bytes*pixels,b.len()==a.len(),
        forall|i: nat| i<pixels ==> #[trigger] a.subrange((bytes*i) as int,(bytes*(i+1)) as int)
            ==b.subrange((bytes*i) as int,(bytes*(i+1)) as int),
    ensures a==b,
{
    assert forall|j: int| 0<=j<a.len() implies #[trigger] a[j]==b[j] by {
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod(j,bytes as int);
        vstd::arithmetic::div_mod::lemma_mod_pos_bound(j,bytes as int);
        vstd::arithmetic::div_mod::lemma_div_pos_is_pos(j,bytes as int);
        let i=(j/(bytes as int)) as nat;
        assert(i<pixels) by (nonlinear_arith) requires j<(bytes*pixels),j==bytes*i+j%(bytes as int),0<=j%(bytes as int),bytes>=1;
        assert(0<=bytes*i<=j<bytes*(i+1)<=a.len()) by (nonlinear_arith)
            requires j==bytes*i+j%(bytes as int),0<=j%(bytes as int)<bytes,i<pixels,a.len()==bytes*pixels,bytes>=1;
        assert(a.subrange((bytes*i) as int,(bytes*(i+1)) as int)==b.subrange((bytes*i) as int,(bytes*(i+1)) as int));
        assert(a[j]==a.subrange((bytes*i) as int,(bytes*(i+1)) as int)[j-bytes*i]);
        assert(b[j]==b.subrange((bytes*i) as int,(bytes*(i+1)) as int)[j-bytes*i]);
    }
    assert(a=~=b);
}

pub(crate) proof fn initial_transparent_context(b: Seq<u8>,doc: crate::document_model::Doc,p: crate::policy::Policy,state: Seq<u8>)
    requires doc.actl.is_some(),!doc.info.default_is_frame,initial_canvas(b,doc,p,state),
    ensures state.len()==4*(doc.ihdr.width as nat)*(doc.ihdr.height as nat),
        forall|j: int| 0<=j<state.len() ==> #[trigger] state[j]==0,
{reveal(initial_canvas);}

pub(crate) proof fn pixel_observe(input: Seq<u8>,a: r::Raster,colour: c::Context,shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,state: Seq<u8>,black: bool,x: nat,y: nat)
    requires domain(a),progress(input,a,colour,shape,before,state,r::count(a),0,black),x<a.width,y<a.height,
    ensures source_pixel(input,a,colour,x,y,c::is_wide(shape),black).is_ok(),
        c::observe(shape,state,a.width,x,y)==c::converted(shape,source_pixel(input,a,colour,x,y,c::is_wide(shape),black).get_Ok_0(),c::observe(shape,before,a.width,x,y)),
{complete_pixels(input,a,colour,shape,before,state,black);}

/// Defining-module introduction bridge for the existing opaque image predicate.
pub(crate) proof fn image_pixels_intro(input: Seq<u8>,a: r::Raster,colour: c::Context,state: Seq<u8>,wide: bool,black: bool)
    requires state.len()==(if wide {8nat}else{4nat})*a.width*a.height,
        forall|x: nat,y: nat| x<a.width && y<a.height ==>
            (#[trigger] source_pixel(input,a,colour,x,y,wide,black)).is_ok()
            && c::observe((if wide {1nat}else{0nat},state.len(),0nat,0nat,0nat,false),state,a.width,x,y)
                ==c::physical(source_pixel(input,a,colour,x,y,wide,black).get_Ok_0(),wide),
    ensures image_pixels(input,a,colour,state,wide,black),
{reveal(image_pixels);}

}
