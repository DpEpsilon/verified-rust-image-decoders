#![no_std]
// x33b: the same shape as x33_callback_direct_alloc (an admitted std item,
// `to_vec`, calls back into a private crate `Clone` impl), but the callback
// allocates nothing. Must PASS: resetting authorisation on entry to crate
// code must not fail callbacks that do not allocate.
extern crate alloc;
struct X<const K: bool>(u8);
impl<const K: bool> Clone for X<K> {
    #[inline(never)]
    fn clone(&self) -> Self {
        core::hint::black_box(self.0);
        X::<K>(self.0)
    }
}
#[inline(never)]
pub fn guard(n: u8) -> usize {
    let xs = [X::<false>(n)];
    let ys = xs.to_vec();
    core::hint::black_box(ys.len())
}
