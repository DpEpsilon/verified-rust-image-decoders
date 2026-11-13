//! Negative control N5: `collect` allocates for the target `Vec`.
//! Expected: allocation reached through `FromIterator`/`RawVec` growth, from
//! `n5_collect`.

pub fn n5_collect(src: &[u16]) -> Vec<u16> {
    src.iter().map(|x| x.wrapping_mul(3)).collect()
}
