//! Negative control N6: a function pointer hides an allocating callee.
//! Expected: `n6_fnptr` makes an *indirect* call through a `fn` pointer; the
//! tool cannot resolve the target set to a single instance, so it fails closed
//! with a `fn-ptr-call` finding — even though the pointee (`alloc_helper`)
//! allocates. This is the "fn-pointer indirect call hiding an allocation" case:
//! the *right* outcome is a failure, not a silent pass.

pub fn alloc_helper() -> Vec<u8> {
    let mut v = Vec::new();
    v.push(7u8);
    v
}

pub fn n6_fnptr() -> Vec<u8> {
    let f: fn() -> Vec<u8> = alloc_helper;
    f()
}
