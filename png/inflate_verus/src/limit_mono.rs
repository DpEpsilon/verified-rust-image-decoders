//! L-LIMIT-MONO is a theorem about the independent model, not an API assumption.
use crate::model as m;
use vstd::prelude::*;
verus! {

proof fn copy_shape(out:Seq<u8>,limit:nat,window:nat,d:nat,c:nat)
    ensures match m::MCopy(out,limit,window,d,c) {
        m::OutputEffect::Next{out:extended} => extended.len() == out.len()+c && out.len() <= extended.len(),
        _ => true,
    },
{
    if out.len() <= limit && 1 <= d <= out.len() && d <= window && 3 <= c <= crate::limits::LEN_MAX && 1 <= window <= crate::limits::WINDOW_MAX {
        let fit=if c <= limit-out.len() {c} else {(limit-out.len()) as nat};
        m::lemma_copy_distance(out,d,fit);
    }
}
proof fn match_shape(z:Seq<u8>,p:nat,out:Seq<u8>,limit:nat,window:nat,dist:Seq<u8>,s:nat)
    ensures match m::MMatch(z,p,out,limit,window,dist,s) {
        m::SymbolEffect::Next{out:extended,..} => out.len() <= extended.len(),
        _ => true,
    },
{
    hide(m::MDec);
    let le=m::length_extra(s);
    match m::MDec(dist,z,p+le) {
        m::Decoded::Symbol{symbol:ds,position:after} => {
            copy_shape(out,limit,window,m::distance_base(ds)+m::field_value(z,after,m::distance_extra(ds)),m::length_base(s)+m::field_value(z,p,le));
        },
        _ => {},
    }
}
proof fn step_shape(z:Seq<u8>,p:nat,out:Seq<u8>,limit:nat,window:nat,lit:Seq<u8>,dist:Seq<u8>)
    ensures match m::MCompressedStep(z,p,out,limit,window,lit,dist) {
        m::SymbolEffect::Next{out:extended,..} => out.len() <= extended.len(),
        m::SymbolEffect::End{out:extended,..} => extended == out,
        _ => true,
    },
{
    hide(m::MDec);
    match m::MDec(lit,z,p) {
        m::Decoded::Symbol{symbol:s,position:after} => {match_shape(z,after,out,limit,window,dist,s);},
        _ => {},
    }
}
proof fn compressed_shape(z:Seq<u8>,p:nat,out:Seq<u8>,limit:nat,window:nat,lit:Seq<u8>,dist:Seq<u8>)
    ensures match m::MCompressed(z,p,out,limit,window,lit,dist) {
        m::StoredEffect::End{out:extended,..} => out.len() <= extended.len(),
        _ => true,
    },
    decreases 8*z.len()-p,
{
    hide(m::MCompressedStep);
    step_shape(z,p,out,limit,window,lit,dist);
    match m::MCompressedStep(z,p,out,limit,window,lit,dist) {
        m::SymbolEffect::Next{out:extended,position:end} => {
            if p < end <= 8*z.len() {compressed_shape(z,end,extended,limit,window,lit,dist);}
        },
        _ => {},
    }
}
proof fn stored_shape(z:Seq<u8>,p:nat,out:Seq<u8>,limit:nat)
    ensures match m::MStoredBlock(z,p,out,limit) {
        m::StoredEffect::End{out:extended,..} => out.len() <= extended.len(),
        _ => true,
    },
{
    let cursor=(p+7)/8;
    if p <= 8*z.len() && out.len() <= limit && cursor+4 <= z.len() {
        let count=z[cursor as int] as nat+256*z[(cursor+1) as int] as nat;
        m::lemma_stored_payload(z,cursor+4,out,limit,count);
    }
}
proof fn block_shape(z:Seq<u8>,p:nat,out:Seq<u8>,limit:nat,window:nat,single:bool)
    ensures match m::MOneBlock(z,p,out,limit,window,single).1 {
        m::StoredEffect::End{out:extended,..} => out.len() <= extended.len(),
        _ => true,
    },
{
    hide(m::MDynamic);
    hide(m::MCompressed);
    hide(m::MStoredBlock);
    stored_shape(z,p+3,out,limit);
    compressed_shape(z,p+3,out,limit,window,m::fixed_lit_lengths(),m::fixed_dist_lengths());
    match m::MDynamic(z,p+3,single) {
        m::DynamicHeader::Tables{lit,dist,position:end} => {compressed_shape(z,end,out,limit,window,lit,dist);},
        _ => {},
    }
}
proof fn blocks_shape(z:Seq<u8>,p:nat,out:Seq<u8>,limit:nat,window:nat,single:bool)
    ensures match m::MBlk(z,p,out,limit,window,single) {
        m::StoredEffect::End{out:extended,..} => out.len() <= extended.len(),
        _ => true,
    },
    decreases 8*z.len()-p,
{
    hide(m::MOneBlock);
    block_shape(z,p,out,limit,window,single);
    let (last,effect)=m::MOneBlock(z,p,out,limit,window,single);
    match effect {
        m::StoredEffect::End{out:extended,position:end} => {
            if !last && p < end <= 8*z.len() {blocks_shape(z,end,extended,limit,window,single);}
        },
        _ => {},
    }
}

proof fn copy_limit(out:Seq<u8>,large:nat,small:nat,window:nat,d:nat,c:nat)
    requires small <= large,
    ensures match m::MCopy(out,large,window,d,c) {
        m::OutputEffect::Next{out:extended} => extended.len() <= small ==> m::MCopy(out,small,window,d,c) == m::MCopy(out,large,window,d,c),
        _ => true,
    },
{
    copy_shape(out,large,window,d,c);
}
proof fn match_limit(z:Seq<u8>,p:nat,out:Seq<u8>,large:nat,small:nat,window:nat,dist:Seq<u8>,s:nat)
    requires small <= large,
    ensures match m::MMatch(z,p,out,large,window,dist,s) {
        m::SymbolEffect::Next{out:extended,..} => extended.len() <= small ==> m::MMatch(z,p,out,small,window,dist,s) == m::MMatch(z,p,out,large,window,dist,s),
        _ => true,
    },
{
    hide(m::MDec);
    match_shape(z,p,out,large,window,dist,s);
    let le=m::length_extra(s);
    match m::MDec(dist,z,p+le) {
        m::Decoded::Symbol{symbol:ds,position:after} => {
            copy_limit(out,large,small,window,m::distance_base(ds)+m::field_value(z,after,m::distance_extra(ds)),m::length_base(s)+m::field_value(z,p,le));
        },
        _ => {},
    }
}
proof fn step_limit(z:Seq<u8>,p:nat,out:Seq<u8>,large:nat,small:nat,window:nat,lit:Seq<u8>,dist:Seq<u8>)
    requires small <= large,
    ensures match m::MCompressedStep(z,p,out,large,window,lit,dist) {
        m::SymbolEffect::Next{out:extended,..} => extended.len() <= small ==> m::MCompressedStep(z,p,out,small,window,lit,dist) == m::MCompressedStep(z,p,out,large,window,lit,dist),
        m::SymbolEffect::End{out:extended,..} => extended.len() <= small ==> m::MCompressedStep(z,p,out,small,window,lit,dist) == m::MCompressedStep(z,p,out,large,window,lit,dist),
        _ => true,
    },
{
    hide(m::MDec);
    step_shape(z,p,out,large,window,lit,dist);
    match m::MDec(lit,z,p) {
        m::Decoded::Symbol{symbol:s,position:after} => {match_limit(z,after,out,large,small,window,dist,s);},
        _ => {},
    }
}
proof fn compressed_limit(z:Seq<u8>,p:nat,out:Seq<u8>,large:nat,small:nat,window:nat,lit:Seq<u8>,dist:Seq<u8>)
    requires small <= large,
    ensures match m::MCompressed(z,p,out,large,window,lit,dist) {
        m::StoredEffect::End{out:extended,..} => extended.len() <= small ==> m::MCompressed(z,p,out,small,window,lit,dist) == m::MCompressed(z,p,out,large,window,lit,dist),
        _ => true,
    },
    decreases 8*z.len()-p,
{
    hide(m::MCompressedStep);
    match m::MCompressedStep(z,p,out,large,window,lit,dist) {
        m::SymbolEffect::Next{out:extended,position:end} => {
            if p < end <= 8*z.len() {
                compressed_shape(z,end,extended,large,window,lit,dist);
                compressed_limit(z,end,extended,large,small,window,lit,dist);
            }
        },
        _ => {},
    }
    step_limit(z,p,out,large,small,window,lit,dist);
}
proof fn stored_limit(z:Seq<u8>,p:nat,out:Seq<u8>,large:nat,small:nat)
    requires small <= large,
    ensures match m::MStoredBlock(z,p,out,large) {
        m::StoredEffect::End{out:extended,..} => extended.len() <= small ==> m::MStoredBlock(z,p,out,small) == m::MStoredBlock(z,p,out,large),
        _ => true,
    },
{
    let cursor=(p+7)/8;
    if p <= 8*z.len() && out.len() <= large && cursor+4 <= z.len() {
        let count=z[cursor as int] as nat+256*z[(cursor+1) as int] as nat;
        m::lemma_stored_payload(z,cursor+4,out,large,count);
        match m::MStoredBlock(z,p,out,large) {
            m::StoredEffect::End{out:extended,..} => {
                if extended.len() <= small {m::lemma_stored_payload(z,cursor+4,out,small,count);}
            },
            _ => {},
        }
    }
}
proof fn block_limit(z:Seq<u8>,p:nat,out:Seq<u8>,large:nat,small:nat,window:nat,single:bool)
    requires small <= large,
    ensures match m::MOneBlock(z,p,out,large,window,single).1 {
        m::StoredEffect::End{out:extended,..} => extended.len() <= small ==> m::MOneBlock(z,p,out,small,window,single) == m::MOneBlock(z,p,out,large,window,single),
        _ => true,
    },
{
    hide(m::MDynamic);
    hide(m::MCompressed);
    hide(m::MStoredBlock);
    block_shape(z,p,out,large,window,single);
    stored_limit(z,p+3,out,large,small);
    compressed_limit(z,p+3,out,large,small,window,m::fixed_lit_lengths(),m::fixed_dist_lengths());
    match m::MDynamic(z,p+3,single) {
        m::DynamicHeader::Tables{lit,dist,position:end} => {compressed_limit(z,end,out,large,small,window,lit,dist);},
        _ => {},
    }
}
proof fn blocks_limit(z:Seq<u8>,p:nat,out:Seq<u8>,large:nat,small:nat,window:nat,single:bool)
    requires small <= large,
    ensures match m::MBlk(z,p,out,large,window,single) {
        m::StoredEffect::End{out:extended,..} => extended.len() <= small ==> m::MBlk(z,p,out,small,window,single) == m::MBlk(z,p,out,large,window,single),
        _ => true,
    },
    decreases 8*z.len()-p,
{
    hide(m::MOneBlock);
    let (last,effect)=m::MOneBlock(z,p,out,large,window,single);
    match effect {
        m::StoredEffect::End{out:extended,position:end} => {
            if !last && p < end <= 8*z.len() {
                blocks_shape(z,end,extended,large,window,single);
                blocks_limit(z,end,extended,large,small,window,single);
            }
        },
        _ => {},
    }
    block_limit(z,p,out,large,small,window,single);
}
proof fn finish_limit(z:Seq<u8>,p:nat,out:Seq<u8>,large:nat,small:nat,ignore:bool)
    requires out.len() <= small <= large,
    ensures match m::MFinish(z,p,out,large,ignore) {
        m::StoredEffect::End{..} => m::MFinish(z,p,out,small,ignore) == m::MFinish(z,p,out,large,ignore),
        _ => true,
    },
{}

pub proof fn lemma_limit_mono(z:Seq<u8>,large:nat,small:nat,enforce_window:bool,ignore_tail:bool)
    requires m::Inflate(z,large,enforce_window,ignore_tail).status == m::InflateStatus::Done,
        m::Inflate(z,large,enforce_window,ignore_tail).out.len() <= small <= large,
    ensures m::Inflate(z,small,enforce_window,ignore_tail).status == m::InflateStatus::Done,
        m::Inflate(z,small,enforce_window,ignore_tail).out == m::Inflate(z,large,enforce_window,ignore_tail).out,
        m::Inflate(z,small,enforce_window,ignore_tail).r2 == m::Inflate(z,large,enforce_window,ignore_tail).r2,
{
    hide(m::MBlk);
    assert(small <= large);
    let window=m::window_size(z[0] as nat/16,enforce_window);
    blocks_limit(z,16,Seq::empty(),large,small,window,true);
    match m::MBlk(z,16,Seq::empty(),large,window,true) {
        m::StoredEffect::End{out,position:end} => {
            assert(out == m::Inflate(z,large,enforce_window,ignore_tail).out);
            finish_limit(z,end,out,large,small,ignore_tail);
        },
        _ => {},
    }
}
} // verus!
