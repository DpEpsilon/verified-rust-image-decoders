//! Exact R3 events from reconstructed indexed samples, independent of pixels.
use vstd::prelude::*;
use crate::raster_model as r;
verus! {
pub open spec fn pixel(ct: u8,depth: u8,entries: nat,line: Seq<u8>,x: nat,black: bool) -> bool {
    black && ct==3 && r::palette_sample(line,depth,x) as nat>=entries
}
#[verifier::opaque]
pub open spec fn row_prefix(ct: u8,depth: u8,entries: nat,line: Seq<u8>,n: nat,black: bool) -> bool {
    exists|x: nat| x<n && pixel(ct,depth,entries,line,x,black)
}
pub proof fn row_start(ct: u8,depth: u8,entries: nat,line: Seq<u8>,black: bool)
    ensures !row_prefix(ct,depth,entries,line,0,black),
{reveal(row_prefix);}
pub proof fn row_step(ct: u8,depth: u8,entries: nat,line: Seq<u8>,n: nat,black: bool)
    ensures row_prefix(ct,depth,entries,line,n+1,black)
        ==(row_prefix(ct,depth,entries,line,n,black) || pixel(ct,depth,entries,line,n,black)),
{
    reveal(row_prefix);
    if exists|x: nat| x<n+1 && pixel(ct,depth,entries,line,x,black) {
        let x=choose|x: nat| x<n+1 && pixel(ct,depth,entries,line,x,black);
        assert(x<n || x==n);
    }
    if exists|x: nat| x<n && pixel(ct,depth,entries,line,x,black) {
        let x=choose|x: nat| x<n && pixel(ct,depth,entries,line,x,black);
        assert(x<n+1);
    }
}
pub proof fn row_complete(ct: u8,depth: u8,entries: nat,line: Seq<u8>,n: nat,black: bool)
    ensures row_prefix(ct,depth,entries,line,n,black)
        ==(black && ct==3 && !r::palette_valid(line,depth,n,entries)),
{
    reveal(row_prefix);
    if row_prefix(ct,depth,entries,line,n,black) {
        let x=choose|x: nat| x<n && pixel(ct,depth,entries,line,x,black);
        assert(!r::palette_valid(line,depth,n,entries));
    }
    if black && ct==3 && !r::palette_valid(line,depth,n,entries) {
        let x=choose|x: nat| x<n && r::palette_sample(line,depth,x) as nat>=entries;
        assert(pixel(ct,depth,entries,line,x,black));
        assert(row_prefix(ct,depth,entries,line,n,black));
    }
}
pub open spec fn row(input: Seq<u8>,a: r::Raster,k: nat,y: nat) -> bool {
    !a.reject_index && a.ihdr.color_type==3
    && !r::palette_valid(r::line(input,a,k,y),a.ihdr.depth,r::pw(a,k),a.entries)
}
#[verifier::opaque]
pub open spec fn prefix(input: Seq<u8>,a: r::Raster,k: nat,y: nat) -> bool {
    exists|p: nat,q: nat| p<r::count(a) && r::pw(a,p)>0 && q<r::ph(a,p)
        && (p<k || (p==k && q<y)) && row(input,a,p,q)
}
pub proof fn start(input: Seq<u8>,a: r::Raster)
    ensures !prefix(input,a,0,0),
{reveal(prefix);}
pub proof fn step(input: Seq<u8>,a: r::Raster,k: nat,y: nat)
    requires k<r::count(a),r::pw(a,k)>0,y<r::ph(a,k),
    ensures prefix(input,a,k,y+1)==(prefix(input,a,k,y) || row(input,a,k,y)),
{
    reveal(prefix);
    if prefix(input,a,k,y+1) {
        let (p,q)=choose|p: nat,q: nat| p<r::count(a) && r::pw(a,p)>0 && q<r::ph(a,p)
            && (p<k || (p==k && q<y+1)) && row(input,a,p,q);
        assert(p<k || (p==k && q<y) || (p==k && q==y));
    }
    if prefix(input,a,k,y) {
        let (p,q)=choose|p: nat,q: nat| p<r::count(a) && r::pw(a,p)>0 && q<r::ph(a,p)
            && (p<k || (p==k && q<y)) && row(input,a,p,q);
        assert(p<k || (p==k && q<y+1));
    }
}
pub proof fn pass_finished(input: Seq<u8>,a: r::Raster,k: nat)
    requires k<r::count(a),
    ensures prefix(input,a,k,r::ph(a,k))==prefix(input,a,k+1,0),
{reveal(prefix);}
pub proof fn empty_pass(input: Seq<u8>,a: r::Raster,k: nat)
    requires k<r::count(a),r::pw(a,k)==0 || r::ph(a,k)==0,
    ensures prefix(input,a,k,0)==prefix(input,a,k+1,0),
{reveal(prefix);}
pub open spec fn R3(input: Seq<u8>,a: r::Raster) -> bool {prefix(input,a,r::count(a),0)}
}
