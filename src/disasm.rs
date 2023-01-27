//use std::{error::Error, path::Path, process::Command};

use std::error::Error;
use std::path::Path;
use std::process::Command;

// gdb -batch -ex "disassemble/rs $FUNCTION" "$EXECUTABLE"
pub fn src_disasm(path: &Path, func: &str) -> Result<(), Box<dyn Error>> {
    let tmpfnc = format!("disassemble/rs {func}");
    let output = Command::new("/usr/bin/gdb")
        .arg("-batch")
        .arg("-ex")
        .arg(tmpfnc)
        .arg(path.to_str().unwrap())
        .output();

    println!(
        "{}",
        String::from_utf8_lossy(&output.as_ref().unwrap().stderr)
    );
    println!("{}", String::from_utf8_lossy(&output.unwrap().stdout));

    Ok(())
}

// gdb -batch -ex 'file ~/test' -ex 'disassemble add'
pub fn no_src_disasm(path: &Path, func: &str) -> Result<(), Box<dyn Error>> {
    let tmpf = format!("file {}", path.to_str().unwrap());
    let tmpfnc = format!("disassemble {func}");
    let output = Command::new("/usr/bin/gdb")
        .arg("-batch")
        .arg("-ex")
        .arg(tmpf)
        .arg("-ex")
        .arg(tmpfnc)
        .output();

    println!(
        "{}",
        String::from_utf8_lossy(&output.as_ref().unwrap().stderr)
    );
    println!("{}", String::from_utf8_lossy(&output.unwrap().stdout));

    Ok(())
}
