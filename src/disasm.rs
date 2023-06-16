//use std::{error::Error, path::Path, process::Command};

use goblin::elf::Elf;
use std::error::Error;
use std::path::Path;
use std::process::Command;

pub fn has_debug(bytes: &[u8]) -> Result<bool, Box<dyn Error>> {
    // This could be improved by only reading in
    // only the elf header and not the whole file
    // let bytes: Vec<u8> = fs::read(path)?;
    match Elf::parse(bytes) {
        Ok(bin) => {
            let shstrtab = bin.shdr_strtab.to_vec().unwrap();
            if shstrtab.contains(&".debug_info") {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

// gdb -batch -ex "disassemble/rs $FUNCTION" "$EXECUTABLE"
pub fn src_disasm(path: &Path, func: &str) {
    let tmpfnc = format!("disassemble/rs {func}");
    let output = Command::new("/usr/bin/gdb")
        .arg("-batch")
        .arg("-ex")
        .arg(tmpfnc)
        .arg(path.to_str().unwrap())
        .output();
    
    println!();
    
    println!(
        "{}",
        String::from_utf8_lossy(&output.as_ref().unwrap().stderr)
    );
    println!("{}", String::from_utf8_lossy(&output.unwrap().stdout));
}

// gdb -batch -ex 'file ~/test' -ex 'disassemble add'
pub fn no_src_disasm(path: &Path, func: &str) {
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
}
