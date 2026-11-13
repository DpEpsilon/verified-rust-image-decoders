//! A macro crate whose exported `macro_rules!` macro expands to include_bytes!.
#![no_std]
#[macro_export]
macro_rules! embed {
    () => {
        include_bytes!("p.bin")
    };
}
