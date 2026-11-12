//! Pre-fix N11 testcase as Sol reviewed it: `unapproved_grow` uses `resize`
//! (which reaches reserve->grow_amortized, not the guard's push_mut instance).

pub mod guard {
    pub fn try_zeroed(n: usize) -> Vec<u8> {
        let mut v = Vec::new();
        v.reserve_exact(n);
        let mut i = 0usize;
        while i < n {
            v.push(0u8);
            i += 1;
        }
        v
    }
}

fn unapproved_reserve(m: usize) -> Vec<u32> {
    let mut v = Vec::new();
    v.reserve_exact(m);
    v
}

fn unapproved_grow(k: usize) -> Vec<u8> {
    let mut v = Vec::new();
    v.resize(k, 1u8);
    v
}

pub fn n11_shared_instance(n: usize, m: usize, k: usize) -> usize {
    let a = guard::try_zeroed(n);
    let b = unapproved_reserve(m);
    let c = unapproved_grow(k);
    a.len() + b.len() + c.len()
}
