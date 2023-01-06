use clap::Parser;
use goblin::elf::Elf;
use std::{
    error::Error,
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author = "HBchevelle68", version="0.1", about, long_about = None)]
struct Args {
    /// Path to binary to disassemble
    path: PathBuf,

    /// Function you wish to disassemble
    //#[arg(short, long)]
    func: String,
}

// gdb -batch -ex 'file ~/test' -ex 'disassemble add'
fn no_src_disasm(path: &PathBuf, func: &str) -> Result<(), Box<dyn Error>> {
    // let args = format!(
    //     "-batch -ex 'file {}' -ex 'disassemble {}\'",
    //     path.to_string_lossy(),
    //     func,
    // );
    let tmpf = format!("\'file {}\'", path.to_string_lossy());
    let tmpfnc = format!("\'disassemble {}\'", func);
    let args = vec!["-batch", "-ex", &tmpf, "-ex", &tmpfnc];
    let mut output = Command::new("gdb").args(args).output();

    println!("{}", String::from_utf8_lossy(&output.unwrap().stderr));

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
            no_src_disasm(&args.path, &args.func)
        }
        Ok(false) => {
            println!("NO DEBUG!");
            no_src_disasm(&args.path, &args.func)
        }
        Err(e) => Err(e),
    }
}
