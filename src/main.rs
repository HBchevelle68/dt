use std::{fs, path::PathBuf};

mod cli;
mod disasm;
mod dtelf;
use dtelf::FileData;

mod codes;
use codes::DtCode;

fn dt() -> DtCode {
    let matches = cli::dt_cli().get_matches();

    let arg_path = matches.get_one::<PathBuf>("FILE").unwrap();
    let res = match fs::read(arg_path) {
        Ok(bytes) => {
            let mut result = DtCode::Success;
            if let Some(arg_func) = matches.get_one::<String>("func") {
                // Disassemble a targeted function
                match disasm::has_debug(&bytes) {
                    Ok(true) => {
                        println!("[+] Debug symbols found");
                        disasm::src_disasm(arg_path, arg_func);
                    }
                    Ok(false) => {
                        println!("[*] No debug symbols found");
                        disasm::no_src_disasm(arg_path, arg_func);
                    }
                    Err(e) => {
                        println!("{e}");
                        result = DtCode::ElfParse;
                    }
                };
            } else if let Some(_lsym) = matches.get_one::<bool>("list") {
                let elf_file = FileData::new(arg_path.to_str().unwrap(), &bytes);
                dbg!(elf_file);
            }
            result
        }
        Err(e) => {
            println!("{e}");
            DtCode::IoReadFail
        }
    };

    res
}

fn main() {
    std::process::exit(dt() as i32);
}
