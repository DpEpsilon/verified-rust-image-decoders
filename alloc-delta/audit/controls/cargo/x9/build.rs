// x9_build_script_untracked_read (SPEC §6 required control): a build script
// that reads an undeclared file with std::fs::read and an undeclared variable
// with std::env::var, and emits no cargo:rerun-if-* line for either. Neither
// read appears in rustc's dep-info. The inventory must block, naming both.
fn main() {
    let n = std::fs::read("x9-undeclared-input.txt").map(|v| v.len()).unwrap_or(0);
    let s = std::env::var("X9_SECRET").unwrap_or_default();
    println!("cargo:rustc-env=X9_LEN={}", n + s.len());
}
