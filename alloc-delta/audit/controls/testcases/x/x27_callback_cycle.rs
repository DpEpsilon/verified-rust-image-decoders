#![no_std]
extern crate alloc;
struct X<const K: bool>(u8);
impl<const K: bool> Clone for X<K> {
    #[inline(never)]
    fn clone(&self) -> Self {
        if self.0 > 0 {
            let xs = [X::<K>(self.0 - 1)];
            let ys = xs.to_vec();
            core::hint::black_box(&ys);
        }
        X::<K>(self.0)
    }
}
#[inline(never)]
pub fn guard(n: u8) -> usize {
    let xs = [X::<false>(n)];
    let ys = xs.to_vec();
    core::hint::black_box(ys.len())
}
