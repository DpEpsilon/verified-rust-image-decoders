//! Negative control N9: a `Drop` impl that allocates.
//! Expected: `n9_drop` stores a `Dropper` in a `Box`; dropping it runs the
//! `Drop::drop` glue, whose body allocates (`Vec::new` + `push`). The tool must
//! follow drop glue into the `Drop` impl and report the allocation.

pub struct Dropper;

impl Drop for Dropper {
    fn drop(&mut self) {
        let mut v = Vec::new();
        v.push(1u8);
        std::hint::black_box(&v);
    }
}

pub fn n9_drop() {
    let d = Dropper;
    std::hint::black_box(&d);
    // `d` is dropped here, running `Dropper::drop`.
}
