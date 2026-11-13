#![no_std]
extern crate alloc;
struct X<const K: bool>(u8);
impl<const K: bool> Clone for X<K> {
    #[inline(never)]
    fn clone(&self) -> Self {
        unsafe {
            let l = core::alloc::Layout::new::<u8>();
            let p = alloc::alloc::alloc(l);
            core::hint::black_box(p);
            if !p.is_null() { alloc::alloc::dealloc(p, l); }
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
