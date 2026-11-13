// x8_real_build_script: a real build script, run by cargo. It emits a cfg the
// library reads. The production tool must report `undeclared-build-script`
// until the allowlist declares build.rs with its hash.
fn main() {
    println!("cargo:rustc-cfg=x8_from_build_script");
    println!("cargo:rerun-if-changed=build.rs");
}
