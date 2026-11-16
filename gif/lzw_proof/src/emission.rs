//! Stack-free prefix-chain emission into the caller's output slice.
use vstd::prelude::*;

verus! {
#[cfg(verus_only)]
use crate::chain::*;

pub open spec fn used(n: int, q: int, a: int) -> int {
    if n <= a - q { n } else { a - q }
}

/// Write the chain's indices backwards at their forward raster positions.
/// Positions beyond the raster are skipped without computing an overflowing sum.
/// The returned first index also supplies the dictionary insertion's suffix.
pub fn emit_backwards(c: usize, clear: usize, q: usize, a: usize, n: usize,
    sf: &[u8], pf: &[u16], out: &mut [u8]) -> (first: u8)
    requires
        sf@.len() == 4096, pf@.len() == 4096,
        c < 4096, clear < 4096,
        chain_ok(pf@, clear as int, c as int),
        q <= a <= old(out)@.len(),
        n == str_of(sf@, pf@, clear as int, c as int, 4096).len(),
    ensures
        first == str_of(sf@, pf@, clear as int, c as int, 4096)[0],
        final(out)@.len() == old(out)@.len(),
        forall|i: int| 0 <= i < used(n as int, q as int, a as int) ==>
            #[trigger] final(out)@[q + i] == str_of(sf@, pf@, clear as int, c as int, 4096)[i],
        forall|i: int| 0 <= i < old(out)@.len()
            && (i < q || i >= q + used(n as int, q as int, a as int)) ==>
            #[trigger] final(out)@[i] == old(out)@[i],
{
    let _ = clear; // The verified length determines where the literal terminates the walk.
    let _out_len = out.len();
    let ghost original = out@;
    let ghost s = str_of(sf@, pf@, clear as int, c as int, 4096);
    let mut cur = c;
    let mut left = n;
    let mut fuel: usize = 4096;
    proof { lemma_str_len_bound(sf@, pf@, clear as int, c as int, 4096); }
    while left > 1
        invariant
            sf@.len() == 4096, pf@.len() == 4096,
            c < 4096, clear < 4096,
            q <= a <= out@.len(), out@.len() <= usize::MAX,
            out@.len() == original.len(),
            1 <= left <= n <= 4096,
            cur < 4096,
            chain_ok(pf@, clear as int, cur as int),
            fuel_above(cur as int, fuel as nat),
            root_of(pf@, clear as int, cur as int, fuel as nat)
                == root_of(pf@, clear as int, c as int, 4096),
                s.len() == n,
            s.take(left as int) == str_of(sf@, pf@, clear as int, cur as int, fuel as nat),
            forall|i: int| left <= i < used(n as int, q as int, a as int) ==>
                #[trigger] out@[q + i] == s[i],
            forall|i: int| 0 <= i < out@.len()
                && (i < q || i >= q + used(n as int, q as int, a as int)) ==>
                #[trigger] out@[i] == original[i],
        decreases left,
    {
        proof {
            if cur < clear { reveal(str_of); assert(false); }
            lemma_chain_step(sf@, pf@, clear as int, cur as int, fuel as nat);
        }
        let byte = sf[cur];
        let parent = pf[cur] as usize;
        left = left - 1;
        if left < a - q { out[q + left] = byte; }
        proof {
            let inner = str_of(sf@, pf@, clear as int, parent as int, (fuel - 1) as nat);
            assert(s.take(left as int + 1) == inner.push(byte));
            assert(inner.len() == left);
            assert(s.take(left as int + 1)[left as int] == inner.push(byte)[left as int]);
            assert(s[left as int] == byte);
            assert(s.take(left as int) =~= inner);
        }
        cur = parent;
        fuel = fuel - 1;
    }
    proof {
        lemma_str_len_bound(sf@, pf@, clear as int, cur as int, fuel as nat);
        if cur >= clear {
            lemma_chain_step(sf@, pf@, clear as int, cur as int, fuel as nat);
            lemma_str_len_bound(sf@, pf@, clear as int, pf@[cur as int] as int, (fuel - 1) as nat);
            assert(false);
        }
        assert(root_of(pf@, clear as int, cur as int, fuel as nat) == cur);
        assert(str_of(sf@, pf@, clear as int, cur as int, fuel as nat) == seq![cur as u8]);
        assert(s.take(1)[0] == s[0]);
        assert((cur as u8) == ((cur % 256) as u8)) by(bit_vector);
        assert(s[0] == (cur % 256) as u8);
    }
    let first = (cur % 256) as u8;
    if q < a { out[q] = first; }
    first
}
pub open spec fn resolved(sf:Seq<u8>,pf:Seq<u16>,clear:int,next:int,prev:int,code:int)->Seq<u8> {
    if code<next {str_of(sf,pf,clear,code,4096)} else {
        let p=str_of(sf,pf,clear,prev,4096); p.push(p[0])
    }
}

/// Resolve an ordinary or KwKwK code and emit its raster-clipped prefix.
/// Keeping this sequence-joining proof outside the decoder isolates its
/// quantified output facts from the main loop's model-fold invariants.
pub fn emit_code(code:usize,clear:usize,next:usize,prev:usize,q:usize,a:usize,
    sf:&[u8],pf:&[u16],ls:&[u16],out:&mut [u8])->(r:(usize,u8))
    requires crate::dictionary::table_ok(pf@,ls@,clear as int,next as int),
        sf@.len()==4096, clear<4096,
        crate::dictionary::valid(clear as int,next as int,code as int)
            || code==next && next<4096 && crate::dictionary::valid(clear as int,next as int,prev as int),
        q<a<=old(out)@.len(),
    ensures
        r.0==resolved(sf@,pf@,clear as int,next as int,prev as int,code as int).len(),
        r.0 as int==if code<next {crate::refinement::clen(ls@,clear as int,code as int)}
            else {crate::refinement::clen(ls@,clear as int,prev as int)+1},
        1<=r.0<=4097,
        r.1==resolved(sf@,pf@,clear as int,next as int,prev as int,code as int)[0],
        r.1==root_of(pf@,clear as int,if code<next {code as int} else {prev as int},4096) as u8,
        final(out)@.len()==old(out)@.len(),
        final(out)@.take(q as int+used(r.0 as int,q as int,a as int))
            ==old(out)@.take(q as int)+resolved(sf@,pf@,clear as int,next as int,prev as int,code as int)
                .take(used(r.0 as int,q as int,a as int)),
{
    let kw=code==next;
    let base=if kw {prev} else {code};
    let nbase=crate::dictionary::cached(base,clear,ls);
    proof {
        crate::dictionary::chain_valid(pf@,ls@,clear as int,next as int,base as int);
        crate::refinement::cache_shape(sf@,pf@,ls@,clear as int,base as int,4096);
    }
    let ghost before=out@;
    let ghost bs=str_of(sf@,pf@,clear as int,base as int,4096);
    let ghost s=resolved(sf@,pf@,clear as int,next as int,prev as int,code as int);
    let first=emit_backwards(base,clear,q,a,nbase,sf,pf,out);
    let ghost emitted=out@;
    let n=if kw {nbase+1} else {nbase};
    if kw && nbase<a-q {out[q+nbase]=first;}
    proof {
        assert(s==if kw {bs.push(first)} else {bs});
        let take=used(n as int,q as int,a as int);
        assert(out@.take(q as int+take) =~= before.take(q as int)+s.take(take)) by {
            assert forall|i:int| 0<=i<q+take implies
                #[trigger] out@.take(q as int+take)[i]==(before.take(q as int)+s.take(take))[i] by {
                if i<q {
                    assert(emitted[i]==before[i]);
                    assert(out@[i]==emitted[i]);
                } else if kw && i==q+nbase {
                    assert(out@[i]==first);
                    assert(s[i-q]==first);
                } else {
                    assert(0<=i-q<used(nbase as int,q as int,a as int));
                    assert(emitted[q+(i-q)]==bs[i-q]);
                    assert(out@[i]==emitted[i]);
                    assert(s[i-q]==bs[i-q]);
                }
            }
        }
    }
    (n,first)
}

} // verus!
