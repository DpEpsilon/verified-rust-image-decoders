//! Negative control N8: an allocation in a user helper, reached transitively.
//! Expected: `n8_caller` -> `helper_grow` -> `Vec::push` growth; the tool must
//! follow the user helper (a direct call) and report the allocation.

fn helper_grow(n: usize) -> Vec<u32> {
    let mut v = Vec::new();
    let mut i = 0usize;
    while i < n {
        v.push(i as u32);
        i += 1;
    }
    v
}

pub fn n8_caller(n: usize) -> Vec<u32> {
    helper_grow(n)
}
