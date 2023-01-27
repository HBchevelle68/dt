//use clap::Parser;
use goblin::elf::Elf;
use std::{error::Error, fs, path::PathBuf};

mod cli;
mod disasm;

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
    //let mut args = Args::parse();

    let matches = cli::dt_cli().get_matches();

    let arg_path = matches.get_one::<PathBuf>("FILE").unwrap();

    // Figure out what we need to do next
    if let Some(arg_func) = matches.get_one::<String>("FUNC") {
        return match has_debug(&arg_path) {
            Ok(true) => {
                println!("HAS DEBUG!");
                disasm::src_disasm(&arg_path, &arg_func)
            }
            Ok(false) => {
                println!("NO DEBUG!");
                disasm::no_src_disasm(&arg_path, &arg_func)
            }
            Err(e) => Err(e),
        };
    }

    Ok(())
}
