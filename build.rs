use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    if probe("fn main() { 0i128; }") {
        println!("cargo:rustc-cfg=has_i128");
    }

    let try_fold = "fn main() { let _ = std::iter::once(0).try_fold(1, |_, _| Err(2)); }";
    if probe(try_fold) {
        println!("cargo:rustc-cfg=has_try_fold");
    }
}

/// Test if a code snippet can be compiled
fn probe(code: &str) -> bool {
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let out_dir = env::var_os("OUT_DIR").expect("environment variable OUT_DIR");

    let mut child = Command::new(rustc)
        .arg("--out-dir")
        .arg(out_dir)
        .arg("--emit=obj")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .expect("rustc probe");

    child
        .stdin
        .as_mut()
        .expect("rustc stdin")
        .write_all(code.as_bytes())
        .expect("write rustc stdin");

    child.wait().expect("rustc probe").success()
}
