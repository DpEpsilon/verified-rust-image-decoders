//! Reentrancy safety of the module's `stop_load`/`load_increment` pair, driven
//! through a *real* `GdkPixbufLoader` and the installed `libgdk_pixbuf`.
//!
//! gdk-pixbuf emits `size-prepared`, `area-prepared` and `area-updated`
//! synchronously from inside `stop_load`, and it sets `priv->closed` only
//! *after* `stop_load` returns (gdk-pixbuf-loader.c: the signal emits are at
//! lines 311/386/401, `priv->closed = TRUE` at line 887). An application
//! handler can therefore re-enter the loader while the module is still inside
//! `stop_load`:
//!
//!   * calling `gdk_pixbuf_loader_close(loader)` re-enters `stop_load` with the
//!     same context pointer, and
//!   * calling `gdk_pixbuf_loader_write(loader, ...)` re-enters `load_increment`
//!     with the same context pointer.
//!
//! Before the fix, the outer `stop_load` freed the context with
//! `Box::from_raw` while still holding a `&Context` across the callback, so the
//! nested `stop_load` freed it a second time (double free) and a nested
//! `load_increment` created a `&mut Context` aliasing the outer `&`.
//!
//! The test compiles `tests/reentrancy.c` with AddressSanitizer and runs all
//! (action, signal) combinations. ASAN intercepts the Rust module's
//! `malloc`/`free` (Rust's default allocator on Linux is the system allocator),
//! so a double free inside the `.so` aborts before any output. It also enables
//! LeakSanitizer, so a context that is never freed is reported.
//!
//! Requires: `cc` with `-fsanitize=address`, `pkg-config` for gdk-pixbuf-2.0,
//! and `gdk-pixbuf-query-loaders`. If any is missing the test prints why and
//! returns without failing (there is nothing to run).

use std::path::{Path, PathBuf};
use std::process::Command;

/// One (action, signal) case: what the trigger handler does, and which signal's
/// handler does it.
struct Case {
    action: &'static str,
    signal: &'static str,
}

const ACTIONS: [&str; 3] = ["close", "write", "closewrite"];
const SIGNALS: [&str; 3] = ["size", "prepared", "updated"];

fn manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The directory cargo built into, at run time. `cargo test` exports
/// `CARGO_TARGET_DIR` to the test process when it was set on the command line.
fn target_dir() -> PathBuf {
    match std::env::var_os("CARGO_TARGET_DIR") {
        Some(d) => PathBuf::from(d),
        None => manifest().join("target"),
    }
}

fn cargo_path() -> String {
    let base = std::env::var("PATH").unwrap_or_default();
    format!("/opt/cargo/bin:{base}")
}

fn run(cmd: &mut Command) -> std::process::Output {
    cmd.output().unwrap_or_else(|e| panic!("failed to run {cmd:?}: {e}"))
}

/// Build the module, generate a one-module loaders.cache, and compile the C
/// driver with ASAN. Returns the driver path and cache path, or `None` with a
/// printed reason if the toolchain cannot run the harness here.
fn setup() -> Option<(PathBuf, PathBuf)> {
    let crate_dir = manifest();
    let target = target_dir();
    let work = target.join("reentrancy-test");
    std::fs::create_dir_all(&work).ok()?;

    // 1. Build the cdylib (a nested cargo invocation; this is allowed and does
    //    not deadlock on the build lock).
    let mut build = Command::new("cargo");
    build
        .args(["build", "--offline", "--release", "--lib"])
        .env("PATH", cargo_path())
        .env("CARGO_TARGET_DIR", &target)
        .current_dir(&crate_dir);
    let out = run(&mut build);
    if !out.status.success() {
        eprintln!("reentrancy: `cargo build --lib` failed:\n{}", String::from_utf8_lossy(&out.stderr));
        return None;
    }
    let so = target.join("release/libpixbufloader_verified_bmp.so");
    if !so.is_file() {
        eprintln!("reentrancy: module {} not built", so.display());
        return None;
    }

    // 2. A cache listing only our module (GDK_PIXBUF_MODULE_FILE for the child).
    let query = match pkg_config("--variable=gdk_pixbuf_query_loaders", &["gdk-pixbuf-2.0"]) {
        Some(q) => q,
        None => {
            eprintln!("reentrancy: gdk-pixbuf-query-loaders not found via pkg-config; skipping");
            return None;
        }
    };
    let cache = work.join("loaders-ours.cache");
    let mut q = Command::new(query);
    q.arg(&so);
    let out = run(&mut q);
    if !out.status.success() {
        eprintln!("reentrancy: gdk-pixbuf-query-loaders failed; skipping");
        return None;
    }
    std::fs::write(&cache, &out.stdout).ok()?;

    // 3. Compile tests/reentrancy.c with ASAN against gdk-pixbuf.
    let cflags = Command::new("pkg-config")
        .args(["--cflags", "--libs", "gdk-pixbuf-2.0"])
        .output();
    let (cflags, cflags_ok) = match cflags {
        Ok(o) if o.status.success() => (
            String::from_utf8_lossy(&o.stdout).split_whitespace().map(String::from).collect::<Vec<_>>(),
            true,
        ),
        _ => (Vec::new(), false),
    };
    if !cflags_ok {
        eprintln!("reentrancy: `pkg-config --cflags --libs gdk-pixbuf-2.0` failed; skipping");
        return None;
    }
    let driver = work.join("reentrancy");
    let mut cc = Command::new("cc");
    cc.arg("-O1")
        .arg("-g")
        .arg("-fsanitize=address")
        .arg("-o")
        .arg(&driver)
        .arg(crate_dir.join("tests/reentrancy.c"))
        .args(&cflags);
    let out = run(&mut cc);
    if !out.status.success() {
        eprintln!(
            "reentrancy: cannot compile tests/reentrancy.c with -fsanitize=address \
             (no ASAN runtime?); skipping:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
        return None;
    }
    Some((driver, cache))
}

/// Run `cc`/`pkg-config` style helper: first stdout line of `pkg-config ARGS`.
fn pkg_config(flag: &str, pkgs: &[&str]) -> Option<String> {
    let out = Command::new("pkg-config").arg(flag).args(pkgs).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    (!s.is_empty()).then_some(s)
}

/// Run one case and return (exit code, combined output). The ASAN options ask
/// for leak reports, which is how a never-freed context is detected.
fn run_case(driver: &Path, cache: &Path, action: &str, signal: &str) -> (i32, String) {
    let mut cmd = Command::new(driver);
    cmd.arg(action)
        .arg(signal)
        .env("GDK_PIXBUF_MODULE_FILE", cache)
        .env("ASAN_OPTIONS", "detect_leaks=1:abort_on_error=1")
        .env("LSAN_OPTIONS", "exitcode=23");
    let out = run(&mut cmd);
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    (out.status.code().unwrap_or(-1), text)
}

/// The JSON object the driver prints, as a tiny field lookup (the test has no
/// JSON dependency).
fn json_field(text: &str, key: &str) -> Option<String> {
    let line = text.lines().find(|l| l.trim_start().starts_with('{'))?;
    let needle = format!("\"{key}\":");
    let start = line.find(&needle)? + needle.len();
    let rest = &line[start..];
    let end = rest.find([',', '}']).unwrap_or(rest.len());
    Some(rest[..end].trim().to_string())
}

fn assert_case(driver: &Path, cache: &Path, case: &Case) {
    let (rc, text) = run_case(driver, cache, case.action, case.signal);
    let label = format!("{}/{}", case.action, case.signal);

    // A double free/use-after-free inside the module aborts under ASAN.
    assert!(
        !text.contains("ERROR: AddressSanitizer"),
        "{label}: AddressSanitizer reported an error:\n{text}"
    );
    assert!(
        !text.contains("LeakSanitizer") && !text.contains("detected memory leaks"),
        "{label}: LeakSanitizer reported a leak (an unfreed context?):\n{text}"
    );
    assert_eq!(rc, 0, "{label}: driver exited with {rc}:\n{text}");
    assert!(text.contains("\"status\":\"ok\""), "{label}: no success line:\n{text}");

    let f = |k: &str| json_field(&text, k).unwrap_or_else(|| panic!("{label}: missing {k}:\n{text}"));
    // Normal completion: the whole file was written, the pixbuf exists, and the
    // outer close succeeded. Exactly one of each signal fired: a second
    // stop_load must not allocate a second pixbuf or re-emit.
    assert_eq!(f("wrote"), "1", "{label}: outer write failed:\n{text}");
    assert_eq!(f("outer_close_rc"), "1", "{label}: outer close failed:\n{text}");
    assert_eq!(f("pixbuf"), "true", "{label}: no pixbuf:\n{text}");
    assert_eq!(f("n_size"), "1", "{label}: {text}");
    assert_eq!(f("n_prepared"), "1", "{label}: {text}");
    assert_eq!(f("n_updated"), "1", "{label}: {text}");
    // The drive has no bug of our own. An expected critical is counted below.
    assert_eq!(f("warning"), "0", "{label}: GLib warning:\n{text}");

    // `closewrite` calls close and then write from one handler: the write
    // trips gdk-pixbuf's own `g_return_val_if_fail (priv->closed == FALSE)` (a
    // CRITICAL under the "GdkPixbuf" domain), so the driver sees exactly one.
    // Every other action must produce none. The module itself never logs.
    let expected_criticals = if case.action == "closewrite" { "1" } else { "0" };
    assert_eq!(f("critical"), expected_criticals, "{label}: GLib critical count:\n{text}");

    match case.action {
        "close" => {
            // The nested close re-entered stop_load and returned success without
            // freeing the context that the outer stop_load still owns.
            assert_eq!(f("re_close_rc"), "1", "{label}: nested close failed:\n{text}");
            assert_eq!(f("re_write_rc"), "-1", "{label}: no write expected:\n{text}");
        }
        "write" => {
            // The nested write re-entered load_increment and was refused with
            // GDK_PIXBUF_ERROR_FAILED (code 5); it must not have mutated state.
            assert_eq!(f("re_write_rc"), "0", "{label}: nested write accepted:\n{text}");
            assert_eq!(f("re_write_code"), "5", "{label}: nested write not GDK_PIXBUF_ERROR_FAILED:\n{text}");
        }
        "closewrite" => {
            // Nested close first; the following nested write is refused (by the
            // module, or by gdk-pixbuf once closed). Either way it must not crash
            // or reopen the load.
            assert_eq!(f("re_close_rc"), "1", "{label}: nested close failed:\n{text}");
            assert_eq!(f("re_write_rc"), "0", "{label}: nested write accepted:\n{text}");
        }
        _ => unreachable!(),
    }
}

#[test]
fn reentrant_callbacks_do_not_double_free_or_alias() {
    let Some((driver, cache)) = setup() else {
        return; // setup printed why
    };

    // Control: no reentry, just the normal callback order.
    let (rc, text) = run_case(&driver, &cache, "none", "size");
    assert_eq!(rc, 0, "control run failed:\n{text}");
    assert!(text.contains("\"status\":\"ok\""), "control run:\n{text}");
    assert_eq!(json_field(&text, "n_prepared").as_deref(), Some("1"), "control run:\n{text}");

    let mut cases = Vec::new();
    for action in ACTIONS {
        for signal in SIGNALS {
            cases.push(Case { action, signal });
        }
    }
    for case in &cases {
        assert_case(&driver, &cache, case);
    }
}
