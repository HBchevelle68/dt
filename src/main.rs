use clap::Parser;
use goblin::elf::Elf;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author = "HBchevelle68", version="0.1", about, long_about = None)]
struct Args {
    /// Path to binary to disassemble
    path: PathBuf,

    /// Function you wish to disassemble
    func: String,

    /// List available symbols
    #[arg(short, long)]
    list: bool,
}

// gdb -batch -ex "disassemble/rs $FUNCTION" "$EXECUTABLE"
fn src_disasm(path: &Path, func: &str) -> Result<(), Box<dyn Error>> {
    let tmpfnc = format!("disassemble/rs {}", func);
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
fn no_src_disasm(path: &Path, func: &str) -> Result<(), Box<dyn Error>> {
    let tmpf = format!("file {}", path.to_str().unwrap());
    let tmpfnc = format!("disassemble {}", func);
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

fn has_debug(path: &PathBuf) -> Result<bool, Box<dyn Error>> {
    // This could be improved by only reading in
    // only the elf header and not the whole file
    let bytes: Vec<u8> = fs::read(path)?;
    match Elf::parse(&bytes) {
        Ok(bin) => {
            let shstrtab = bin.shdr_strtab.to_vec().unwrap();
            if shstrtab.contains(&".debug_info") {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) => Err(Box::new(e)), //Box::<dyn std::error::Error(e)>,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args::parse();

    args.path = args.path.canonicalize()?;

    match has_debug(&args.path) {
        Ok(true) => {
            println!("HAS DEBUG!");
            src_disasm(&args.path, &args.func)
        }
        Ok(false) => {
            println!("NO DEBUG!");
            no_src_disasm(&args.path, &args.func)
        }
        Err(e) => Err(e),
    }
}
