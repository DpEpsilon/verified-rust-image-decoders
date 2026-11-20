//! Exact keyword validation and first-NUL search (P8.2, T6 sub-obligations).
use vstd::prelude::*;
use crate::png::Error;
#[cfg(verus_only)]
use crate::model;

verus! {

fn valid_keyword(k: &[u8]) -> (r: bool)
    ensures r == model::keyword(k@),
{
    if k.len() == 0 || k.len() > 79 { return false; }
    let mut prev_space = false;
    let mut i = 0usize;
    while i < k.len()
        invariant 1 <= k.len() <= 79, i <= k.len(),
            prev_space == (i > 0 && k@[i as int - 1] == 0x20),
            forall|j: int| 0 <= j < i ==> (0x20 <= #[trigger] k@[j] <= 0x7E || k@[j] >= 0xA1),
            forall|j: int| 1 <= j < i ==> !(#[trigger] k@[j] == 0x20 && k@[j - 1] == 0x20),
        decreases k.len() - i,
    {
        let c = k[i];
        if !((0x20 <= c && c <= 0x7E) || c >= 0xA1) { return false; }
        if c == 0x20 && prev_space { return false; }
        prev_space = c == 0x20;
        i += 1;
    }
    k[0] != 0x20 && k[k.len() - 1] != 0x20
}

pub(crate) fn find_nul(d: &[u8], from: usize) -> (r: Option<usize>)
    ensures match r {
        Some(z) => from <= z < d.len() && d@[z as int] == 0
            && model::first_nul(d@, from as nat) == Some(z as nat),
        None => model::first_nul(d@, from as nat).is_none(),
    },
{
    let mut i = from;
    while i < d.len()
        invariant i >= from,
            model::first_nul(d@, from as nat) == model::first_nul(d@, i as nat),
        decreases d.len() - i,
    {
        if d[i] == 0 { return Some(i); }
        i += 1;
    }
    None
}

pub(crate) fn split_keyword(d: &[u8], err: Error) -> (r: Result<usize, Error>)
    ensures match r {
        Ok(z) => z < d.len() && model::split_keyword(d@, err) == Ok(z as nat),
        Err(e) => model::split_keyword(d@, err) == Err(e),
    },
{
    match find_nul(d, 0) {
        Some(z) => if valid_keyword(&d[..z]) { Ok(z) } else { Err(err) },
        None => Err(err),
    }
}

} // verus!
