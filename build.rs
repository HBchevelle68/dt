// build.rs

use std::process::Command;

fn main() {
    // Build exmaple without debug
    Command::new("gcc")
        .args(&["example/test.c", "-o", "example/test_no_dbg"])
        .status()
        .unwrap();

    // Build example with debug
    Command::new("gcc")
        .args(&["example/test.c", "-g", "-o", "example/test_dbg"])
        .status()
        .unwrap();

    println!("cargo:rerun-if-changed=example/test.c");
}
