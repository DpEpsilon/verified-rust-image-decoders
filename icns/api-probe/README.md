# `parsed-api-review-probe`

A compile-fail regression probe for `icns_verus`'s public API after the E14
opaque `Parsed` handle (see `alloc-delta/ERRATA.md` E14/E15). It exists to check,
on a real downstream crate, that the property the handle was introduced for
holds from **outside** the verified crate: safe Rust cannot forge a `Parsed`,
cannot mutate the entry list a decode will use, and cannot reach the
theorem-facing internals.

`cargo build` with no features must **succeed**. Each feature turns on one
negative case that must **fail** to compile with the code below.

| Feature | What it tries | Expected error |
|---|---|---|
| `literal` | `Parsed { bytes, entries }` — construct the struct literally | `E0451` (fields are private) |
| `default_impl` | `Parsed::default()` | `E0277` (`Parsed: Default` not satisfied) |
| `clone` | `parsed.clone()` | `E0599` (no `clone` method) |
| `copy` | `assert_copy::<Parsed>()` | `E0277` (`Parsed: Copy` not satisfied) |
| `mut_entries` | `parsed.entries()[0].payload[1] = ..` — mutate through the shared view | `E0594` (cannot assign through `&`) |
| `mut_bytes` | `parsed.bytes()[0] = ..` | `E0594` (cannot assign through `&`) |
| `old_api` | the removed `try_decode_rgb(bytes, entries, i)` etc. and the `_logged` internals | `E0603` (private) |
| `spec_api` | `parsed.spec_bytes()` / `spec_entries()` / `inv()` | `E0599` (no such method) |
| `borrow_alias` | mutate an owner of the bytes while the handle borrows it | `E0502` (already borrowed as immutable) |

Run one case, for example:

```
cargo build --offline -j1 --features literal
```

and check that stderr names the expected code. `cargo build --offline -j1` (no
features) must print no error.

The `Send + Sync` assertion in the neutral body is deliberate: the handle holds
only a shared slice and a `Vec`, so it may cross a thread boundary. It is not a
negative case.
