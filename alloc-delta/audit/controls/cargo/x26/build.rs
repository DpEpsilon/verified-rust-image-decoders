// x26_concurrent_untracked_read (Sol review finding 4): an undeclared read
// whose open is split by strace into `<unfinished ...>` / `<... openat
// resumed>` because another task's syscall is printed in between. A thread
// opens an undeclared FIFO in the package directory for reading, which blocks
// until the main thread opens it for writing (write-only opens are not inputs).
// The inventory must reassemble the pair and block, naming the FIFO.
use std::time::Duration;
fn main() {
    let fifo = "x26-undeclared-concurrent.fifo";
    let _ = std::fs::remove_file(fifo);
    let ok = std::process::Command::new("mkfifo").arg(fifo).status().map(|s| s.success()).unwrap_or(false);
    if !ok {
        panic!("mkfifo failed");
    }
    let reader = std::thread::spawn(move || {
        use std::io::Read;
        let mut f = std::fs::File::open(fifo).expect("open fifo for reading");
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        s.len()
    });
    std::thread::sleep(Duration::from_millis(300));
    // Something the main thread opens while the reader is blocked in openat.
    let _ = std::fs::metadata("Cargo.toml");
    let _ = std::fs::read("Cargo.toml");
    {
        use std::io::Write;
        let mut w = std::fs::OpenOptions::new().write(true).open(fifo).expect("open fifo for writing");
        let _ = w.write_all(b"x26");
    }
    let n = reader.join().unwrap();
    let _ = std::fs::remove_file(fifo);
    println!("cargo:rustc-env=X26_LEN={n}");
}
