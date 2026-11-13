//! Negative control N7: a trait-object call hides an allocating impl.
//! Expected: `n7_dyn` makes a `dyn` call; the tool records a `dyn-call` finding
//! and follows the vtable methods, so it fails closed rather than missing the
//! allocation in `Allocating as Trait::make`.

pub trait Make {
    fn make(&self) -> Vec<u8>;
}

pub struct Allocating;

impl Make for Allocating {
    fn make(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.push(9u8);
        v
    }
}

pub fn n7_dyn(m: &dyn Make) -> Vec<u8> {
    m.make()
}
