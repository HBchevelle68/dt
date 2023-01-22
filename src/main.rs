use clap::Parser;
use goblin::elf::Elf;
use std::{error::Error, fs, path::PathBuf};

mod disassem;

/// Small disassembly tool to help with other projects and as a learning project to learn Rust
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
        Err(e) => Err(Box::new(e)),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args::parse();

    args.path = args.path.canonicalize()?;

    match has_debug(&args.path) {
        Ok(true) => {
            println!("HAS DEBUG!");
            disassem::src_disasm(&args.path, &args.func)
        }
        Ok(false) => {
            println!("NO DEBUG!");
            disassem::no_src_disasm(&args.path, &args.func)
        }
        Err(e) => Err(e),
    }
}
