//! An allocation-free bit reader, with a mathematical bit position wide enough
//! for eight times any representable Rust slice length on either target.
use vstd::prelude::*;
verus! {
#[cfg(verus_only)]
use crate::model::{ipow2,code_at,bit_at,nbits};

pub proof fn small_power(m:int)
    requires 0<=m<=12,
    ensures 1<=ipow2(m)<=4096, m<=11 ==> ipow2(m)<=2048,
        m>=2 ==> ipow2(m)>=4, m==1 ==> ipow2(m)==2,
{
    reveal_with_fuel(ipow2,13);
}

pub proof fn power_recurrence(m:int)
    requires 1<=m<=12,
    ensures ipow2(m)==2*ipow2(m-1),
{}

pub fn power(m:u8)->(r:usize)
    requires m<=12,
    ensures r as int==ipow2(m as int), 1<=r<=4096,
{
    let r=match m {0=>1,1=>2,2=>4,3=>8,4=>16,5=>32,6=>64,
        7=>128,8=>256,9=>512,10=>1024,11=>2048,_=>4096};
    proof { reveal_with_fuel(ipow2,13); }
    r
}

pub fn read_code(d:&[u8],pos:u128,width:u8)->(code:usize)
    requires 1<=width<=12, pos as int+width as int<=nbits(d@),
    ensures code as int==code_at(d@,pos as int,width as int,0),
        (code as int) < ipow2(width as int), code<4096,
{
    let _dl=d.len();
    let mut i=width;
    let mut acc:usize=0;
    proof { small_power(0); }
    while i>0
        invariant
            1<=width<=12, i<=width,
            pos as int+width as int<=nbits(d@), d@.len()<=usize::MAX,
            acc as int==code_at(d@,pos as int,width as int,i as int),
            (acc as int) < ipow2(width as int-i as int),
        decreases i,
    {
        i=i-1;
        proof { small_power(width as int-i as int-1); }
        let bitpos=pos+i as u128;
        let index=(bitpos/8) as usize;
        let shift=(bitpos%8) as u8;
        let raw=d[index];
        let one=(raw>>shift)&1u8;
        proof { assert(one<=1) by(bit_vector) requires one==((raw>>shift)&1u8); }
        let bit=one as usize;
        proof {
            assert(bit as int==bit_at(d@,pos as int+i as int));
            assert(ipow2(width as int-i as int)==2*ipow2(width as int-i as int-1));
            assert(code_at(d@,pos as int,width as int,i as int)
                ==bit as int+2*code_at(d@,pos as int,width as int,i as int+1));
        }
        acc=2*acc+bit;
    }
    proof { small_power(width as int); }
    acc
}
} // verus!
