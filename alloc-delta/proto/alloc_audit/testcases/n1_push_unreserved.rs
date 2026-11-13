//! Negative control N1: `Vec::new` + `push` without a prior reservation.
//! Expected: the tool reports an allocator entry reached through `Vec::push`
//! (`alloc::raw_vec::RawVecInner::grow_amortized` / `finish_grow`) from
//! `n1_push_unreserved`, and fails closed (unexplained allocation).

pub fn n1_push_unreserved(n: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut i = 0usize;
    while i < n {
        v.push(i as u8);
        i += 1;
    }
    v
}
