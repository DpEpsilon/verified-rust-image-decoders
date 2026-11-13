//! x11_std_reachable (SPEC §9 required control): a std crate whose root calls
//! std::fs::read. Must fail with `std-instance`.
pub fn x11_root() -> usize {
    match std::fs::read("/nonexistent") {
        Ok(v) => v.len(),
        Err(_) => 0,
    }
}
