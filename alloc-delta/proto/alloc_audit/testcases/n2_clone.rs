//! Negative control N2: `clone` of a `Vec` (a non-`Copy` type).
//! Expected: allocation reached through `Vec::clone` /
//! `<[T]>::to_vec` -> `RawVec` growth, from `n2_clone`.

pub fn n2_clone(v: &[u8]) -> Vec<u8> {
    let owned: Vec<u8> = v.to_vec();
    owned.clone()
}
