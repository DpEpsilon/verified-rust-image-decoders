//! x13_dylib_not_proc_macro (defect 7): a crate linking a non-proc-macro
//! dylib. Must be reported as `dylib-dependency`, never as a proc-macro.
#![no_std]
extern crate dyldep;
pub fn x13_root(n: usize) -> usize {
    dyldep::plain(n)
}
