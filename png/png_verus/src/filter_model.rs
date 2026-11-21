//! Independent P11 byte reconstruction (MF), including its growing row prefix.
//! The model reads filtered bytes from an immutable row and predictors from the
//! reconstructed prefix and the previous reconstructed row of the same pass.
use vstd::prelude::*;

verus! {

pub open spec fn bpp(bits: nat) -> nat {if bits/8==0 {1}else{bits/8}}

pub open spec fn abs(x: int) -> int {if x<0 {-x}else{x}}
pub open spec fn distances(a: u8,b: u8,c: u8) -> (int,int,int) {
    let p=a as int+b as int-c as int;
    (abs(p-a as int),abs(p-b as int),abs(p-c as int))
}
pub open spec fn paeth(a: u8,b: u8,c: u8) -> u8 {
    let d=distances(a,b,c);
    if d.0<=d.1 && d.0<=d.2 {a} else if d.1<=d.2 {b}else{c}
}

/// Minimum-distance ties prefer a, then b, then c, for every byte operand.
pub proof fn paeth_ties(a: u8,b: u8,c: u8)
    ensures
        distances(a,b,c).0<=distances(a,b,c).1 && distances(a,b,c).0<=distances(a,b,c).2 ==> paeth(a,b,c)==a,
        distances(a,b,c).1<distances(a,b,c).0 && distances(a,b,c).1<=distances(a,b,c).2 ==> paeth(a,b,c)==b,
        distances(a,b,c).2<distances(a,b,c).0 && distances(a,b,c).2<distances(a,b,c).1 ==> paeth(a,b,c)==c,
        distances(a,b,c).0==distances(a,b,c).1 && distances(a,b,c).0<=distances(a,b,c).2 ==> paeth(a,b,c)==a,
        distances(a,b,c).1==distances(a,b,c).2 && distances(a,b,c).1<distances(a,b,c).0 ==> paeth(a,b,c)==b,
{}

pub open spec fn predictor(ft: u8,a: u8,b: u8,c: u8) -> nat {
    if ft==0 {0} else if ft==1 {a as nat} else if ft==2 {b as nat}
    else if ft==3 {(a as nat+b as nat)/2} else {paeth(a,b,c) as nat}
}
pub open spec fn byte(ft: u8,x: u8,a: u8,b: u8,c: u8) -> u8 {
    ((x as nat+predictor(ft,a,b,c))%256) as u8
}
pub open spec fn left(row: Seq<u8>,bpp: nat,i: nat) -> u8 {
    if i>=bpp {row[(i-bpp) as int]}else{0}
}
pub open spec fn above(prev: Option<Seq<u8>>,i: nat) -> u8 {
    match prev {Some(p)=>p[i as int],None=>0}
}
pub open spec fn above_left(prev: Option<Seq<u8>>,bpp: nat,i: nat) -> u8 {
    if i>=bpp {above(prev,(i-bpp) as nat)}else{0}
}

/// MF's first n reconstructed bytes. The executable never evaluates this recursion.
pub closed spec fn mf(ft: u8,input: Seq<u8>,prev: Option<Seq<u8>>,bpp: nat,n: nat) -> Seq<u8>
    decreases n,
{
    if n==0 {Seq::empty()} else {
        let i=(n-1) as nat;
        let out=mf(ft,input,prev,bpp,i);
        out.push(byte(ft,input[i as int],left(out,bpp,i),above(prev,i),above_left(prev,bpp,i)))
    }
}
pub proof fn mf_len(ft: u8,input: Seq<u8>,prev: Option<Seq<u8>>,bpp: nat,n: nat)
    ensures mf(ft,input,prev,bpp,n).len()==n,
    decreases n,
{
    reveal_with_fuel(mf,2);
    if n>0 {mf_len(ft,input,prev,bpp,(n-1) as nat);}
}

/// The prefix is reconstructed; every unread filtered byte is still untouched.
pub open spec fn ready(ft: u8,input: Seq<u8>,prev: Option<Seq<u8>>,bpp: nat,state: Seq<u8>,n: nat) -> bool {
    state.len()==input.len() && n<=input.len()
    && state.take(n as int)==mf(ft,input,prev,bpp,n)
    && forall|j: int| n<=j<input.len() ==> #[trigger] state[j]==input[j]
}
pub proof fn in_place_start(ft: u8,input: Seq<u8>,prev: Option<Seq<u8>>,bpp: nat)
    ensures ready(ft,input,prev,bpp,input,0),
{
    reveal(ready);reveal_with_fuel(mf,2);
    assert(mf(ft,input,prev,bpp,0)==Seq::<u8>::empty());
    assert(input.take(0) =~= Seq::<u8>::empty());
}

/// One write advances MF by one byte and preserves every future input byte.
pub proof fn unfilter_in_place(ft: u8,input: Seq<u8>,prev: Option<Seq<u8>>,bpp: nat,state: Seq<u8>,i: nat)
    requires bpp>=1,i<input.len(),ready(ft,input,prev,bpp,state,i),
    ensures ready(ft,input,prev,bpp,state.update(i as int,
        byte(ft,state[i as int],left(state,bpp,i),above(prev,i),above_left(prev,bpp,i))),i+1),
        forall|j: int| 0<=j<state.len() && j!=i ==>
            #[trigger] state.update(i as int,byte(ft,state[i as int],left(state,bpp,i),above(prev,i),above_left(prev,bpp,i)))[j]==state[j],
{
    reveal_with_fuel(mf,2);
    let out=mf(ft,input,prev,bpp,i);
    mf_len(ft,input,prev,bpp,i);
    assert(state[i as int]==input[i as int]);
    if i>=bpp {assert(left(state,bpp,i)==left(out,bpp,i));}
    let v=byte(ft,state[i as int],left(state,bpp,i),above(prev,i),above_left(prev,bpp,i));
    let next=state.update(i as int,v);
    assert(next.take((i+1) as int)==out.push(v)) by {
        assert forall|j: int| 0<=j<i+1 implies
            #[trigger] next.take((i+1) as int)[j]==out.push(v)[j] by {
            if j<i {assert(state[j]==state.take(i as int)[j]);}
        }
    }
    assert forall|j: int| i+1<=j<input.len() implies #[trigger] next[j]==input[j] by {}
}

pub proof fn in_place_done(ft: u8,input: Seq<u8>,prev: Option<Seq<u8>>,bpp: nat,state: Seq<u8>)
    requires ready(ft,input,prev,bpp,state,input.len()),
    ensures state==mf(ft,input,prev,bpp,input.len()),
{}

pub open spec fn position(base: nat,rb: nat,row: nat) -> nat {base+row*(1+rb)}
pub open spec fn row_data(input: Seq<u8>,base: nat,rb: nat,row: nat) -> Seq<u8> {
    let pos=position(base,rb,row);
    input.subrange(pos as int+1,pos as int+1+rb as int)
}
pub closed spec fn pass_row(input: Seq<u8>,base: nat,rb: nat,bpp: nat,row: nat) -> Seq<u8>
    decreases row,
{
    mf(input[position(base,rb,row) as int],row_data(input,base,rb,row),
        if row==0 {None}else{Some(pass_row(input,base,rb,bpp,(row-1) as nat))},bpp,rb)
}
pub open spec fn previous_row(input: Seq<u8>,base: nat,rb: nat,bpp: nat,row: nat) -> Option<Seq<u8>> {
    if row==0 {None}else{Some(pass_row(input,base,rb,bpp,(row-1) as nat))}
}
pub proof fn pass_row_equation(input: Seq<u8>,base: nat,rb: nat,bpp: nat,row: nat)
    ensures pass_row(input,base,rb,bpp,row)==mf(input[position(base,rb,row) as int],
        row_data(input,base,rb,row),previous_row(input,base,rb,bpp,row),bpp,rb),
        pass_row(input,base,rb,bpp,row).len()==rb,
{
    reveal_with_fuel(pass_row,2);
    mf_len(input[position(base,rb,row) as int],row_data(input,base,rb,row),
        previous_row(input,base,rb,bpp,row),bpp,rb);
}

/// Completed rows equal MF, and no write has reached the next filter or row.
pub open spec fn pass_ready(input: Seq<u8>,state: Seq<u8>,base: nat,rb: nat,bpp: nat,n: nat) -> bool {
    state.len()==input.len() && position(base,rb,n)<=input.len()
    && (forall|row: nat| row<n ==> #[trigger] input[position(base,rb,row) as int]<=4)
    && (forall|row: nat| row<n ==> #[trigger] row_data(state,base,rb,row)==pass_row(input,base,rb,bpp,row))
    && (forall|j: int| position(base,rb,n)<=j<input.len() ==> #[trigger] state[j]==input[j])
}
pub proof fn pass_start(input: Seq<u8>,state: Seq<u8>,base: nat,rb: nat,bpp: nat)
    requires state.len()==input.len(),base<=input.len(),
        forall|j: int| base<=j<input.len() ==> #[trigger] state[j]==input[j],
    ensures pass_ready(input,state,base,rb,bpp,0),
{reveal(pass_ready);}

/// Prefix/suffix frame conditions keep every earlier reconstructed row intact.
pub proof fn advance_pass(input: Seq<u8>,before: Seq<u8>,after: Seq<u8>,base: nat,rb: nat,bpp: nat,row: nat)
    requires pass_ready(input,before,base,rb,bpp,row),after.len()==input.len(),
        position(base,rb,row+1)<=input.len(),input[position(base,rb,row) as int]<=4,
        row_data(after,base,rb,row)==pass_row(input,base,rb,bpp,row),
        forall|j: int| 0<=j<=position(base,rb,row) ==> #[trigger] after[j]==before[j],
        forall|j: int| position(base,rb,row+1)<=j<input.len() ==> #[trigger] after[j]==before[j],
    ensures pass_ready(input,after,base,rb,bpp,row+1),
{
    reveal(pass_ready);
    assert forall|r: nat| r<row+1 implies #[trigger] input[position(base,rb,r) as int]<=4 by {
        if r<row {assert(input[position(base,rb,r) as int]<=4);}else{assert(r==row);}
    }
    assert forall|r: nat| r<row+1 implies
        #[trigger] row_data(after,base,rb,r)==pass_row(input,base,rb,bpp,r) by {
        if r<row {
            assert(row_data(before,base,rb,r)==pass_row(input,base,rb,bpp,r));
            assert(position(base,rb,r+1)<=position(base,rb,row)) by (nonlinear_arith)
                requires r<row;
            assert(position(base,rb,r)+1+rb==position(base,rb,r+1)) by (nonlinear_arith);
            assert(row_data(after,base,rb,r)==row_data(before,base,rb,r)) by {
                assert forall|j: int| 0<=j<rb implies
                    #[trigger] row_data(after,base,rb,r)[j]==row_data(before,base,rb,r)[j] by {}
            }
        }
    }
    assert(position(base,rb,row)<=position(base,rb,row+1)) by (nonlinear_arith);
    assert forall|j: int| position(base,rb,row+1)<=j<input.len() implies #[trigger] after[j]==input[j] by {
        assert(before[j]==input[j]);
        assert(after[j]==before[j]);
    }
}

pub open spec fn pass_width(interlace: u8,w: nat,k: nat) -> nat {
    if interlace==0 {w}else{crate::layout_model::cols(w,k)}
}
pub open spec fn pass_height(interlace: u8,h: nat,k: nat) -> nat {
    if interlace==0 {h}else{crate::layout_model::rows(h,k)}
}
pub open spec fn filter_position(interlace: u8,bits: nat,w: nat,h: nat,k: nat,row: nat) -> nat {
    position(crate::layout_model::prefix(interlace,bits,w,h,k),
        crate::layout_model::scan_bytes(bits,pass_width(interlace,w,k)),row)
}
pub open spec fn valid_pass(input: Seq<u8>,interlace: u8,bits: nat,w: nat,h: nat,k: nat) -> bool {
    pass_width(interlace,w,k)==0 || pass_height(interlace,h,k)==0
    || forall|r: nat| r<pass_height(interlace,h,k) ==>
        #[trigger] input[filter_position(interlace,bits,w,h,k,r) as int]<=4
}
pub open spec fn valid_prefix(input: Seq<u8>,interlace: u8,bits: nat,w: nat,h: nat,n: nat) -> bool {
    forall|k: nat| k<n ==> #[trigger] valid_pass(input,interlace,bits,w,h,k)
}
pub proof fn valid_pass_from_rows(input: Seq<u8>,state: Seq<u8>,interlace: u8,bits: nat,w: nat,h: nat,k: nat,bpp: nat)
    requires pass_ready(input,state,crate::layout_model::prefix(interlace,bits,w,h,k),
        crate::layout_model::scan_bytes(bits,pass_width(interlace,w,k)),bpp,pass_height(interlace,h,k)),
    ensures valid_pass(input,interlace,bits,w,h,k),
{
    reveal(pass_ready);reveal(valid_pass);
    assert forall|r: nat| r<pass_height(interlace,h,k) implies
        #[trigger] input[filter_position(interlace,bits,w,h,k,r) as int]<=4 by {
        assert(input[position(crate::layout_model::prefix(interlace,bits,w,h,k),
            crate::layout_model::scan_bytes(bits,pass_width(interlace,w,k)),r) as int]<=4);
    }
}
pub proof fn valid_prefix_extend(input: Seq<u8>,interlace: u8,bits: nat,w: nat,h: nat,k: nat)
    requires valid_prefix(input,interlace,bits,w,h,k),valid_pass(input,interlace,bits,w,h,k),
    ensures valid_prefix(input,interlace,bits,w,h,k+1),
{
    assert forall|p: nat| p<k+1 implies #[trigger] valid_pass(input,interlace,bits,w,h,p) by {
        if p<k {}else{assert(p==k);}
    }
}

/// Lexicographic pass/row order is P11's scanline stream order; empty passes have no filter byte.
pub open spec fn first_invalid(input: Seq<u8>,interlace: u8,bits: nat,w: nat,h: nat,k: nat,row: nat) -> bool {
    k<crate::layout_model::count(interlace) && pass_width(interlace,w,k)>0
    && row<pass_height(interlace,h,k) && input[filter_position(interlace,bits,w,h,k,row) as int]>4
    && forall|p: nat,r: nat| p<crate::layout_model::count(interlace)
        && pass_width(interlace,w,p)>0 && r<pass_height(interlace,h,p)
        && (p<k || (p==k && r<row)) ==>
            #[trigger] input[filter_position(interlace,bits,w,h,p,r) as int]<=4
}
pub proof fn first_invalid_intro(input: Seq<u8>,state: Seq<u8>,interlace: u8,bits: nat,w: nat,h: nat,k: nat,row: nat,bpp: nat)
    requires k<crate::layout_model::count(interlace),pass_width(interlace,w,k)>0,row<pass_height(interlace,h,k),
        valid_prefix(input,interlace,bits,w,h,k),
        pass_ready(input,state,crate::layout_model::prefix(interlace,bits,w,h,k),
            crate::layout_model::scan_bytes(bits,pass_width(interlace,w,k)),bpp,row),
        input[filter_position(interlace,bits,w,h,k,row) as int]>4,
    ensures first_invalid(input,interlace,bits,w,h,k,row),
{
    reveal(pass_ready);reveal(first_invalid);
    assert forall|p: nat,r: nat| p<crate::layout_model::count(interlace)
        && pass_width(interlace,w,p)>0 && r<pass_height(interlace,h,p)
        && (p<k || (p==k && r<row)) implies
            #[trigger] input[filter_position(interlace,bits,w,h,p,r) as int]<=4 by {
        if p<k {assert(valid_pass(input,interlace,bits,w,h,p));}
        else {
            assert(p==k && r<row);
            assert(input[position(crate::layout_model::prefix(interlace,bits,w,h,k),
                crate::layout_model::scan_bytes(bits,pass_width(interlace,w,k)),r) as int]<=4);
        }
    }
}

/// Filter None is the identity on every prefix; a bridge for authored witnesses.
pub proof fn mf_none(input: Seq<u8>,prev: Option<Seq<u8>>,bpp: nat,n: nat)
    requires n<=input.len(),
    ensures mf(0,input,prev,bpp,n)==input.take(n as int),
    decreases n,
{
    reveal_with_fuel(mf,2);
    if n>0 {
        mf_none(input,prev,bpp,(n-1) as nat);
        let i=(n-1) as nat;
        assert(byte(0,input[i as int],left(mf(0,input,prev,bpp,i),bpp,i),above(prev,i),above_left(prev,bpp,i))==input[i as int]);
        assert(input.take(n as int) =~= input.take(i as int).push(input[i as int]));
    } else {assert(input.take(0) =~= Seq::<u8>::empty());}
}

} // verus!
