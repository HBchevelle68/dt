//use clap::Parser;
//use goblin::elf::Elf;
use std::{fs, path::PathBuf};

mod cli;
mod disasm;
mod dt_elf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let mut args = Args::parse();

    let matches = cli::dt_cli().get_matches();

    let arg_path = matches.get_one::<PathBuf>("FILE").unwrap();
    let bytes: Vec<u8> = fs::read(arg_path)?;

    // Figure out what we need to do next
    if let Some(arg_func) = matches.get_one::<String>("func") {
        return match dt_elf::has_debug(&bytes) {
            Ok(true) => {
                println!("[+] Debug symbols found");
                disasm::src_disasm(&arg_path, &arg_func)
            }
            Ok(false) => {
                println!("[*] No debug symbols found");
                disasm::no_src_disasm(&arg_path, &arg_func)
            }
            Err(e) => Err(e),
        };
    } else if let Some(_lsym) = matches.get_one::<bool>("list") {
        dt_elf::get_syms(&bytes);
    }

    Ok(())
}
