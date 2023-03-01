use clap::arg;
use clap::value_parser;
use clap::ArgAction;
use clap::ArgMatches;
use clap::Command;
use std::path::PathBuf;

use crate::codes::DtCode;
use crate::disasm;
use crate::dtelf::FileData;

/// TODO! Document
pub fn dt_cli() -> Command {
    Command::new("dt")
        .about("Small disassembly tool to help with other projects and as a learning project to learn Rust")
        .version("0.1.0")
        .arg_required_else_help(true)
        .arg(
            arg!(<FILE>)
            .help("Path to file")
            .value_parser(value_parser!(std::path::PathBuf))
            .required(true)
        )
        .arg(
            arg!(func: -f <FUNC>)
            .long("func")
            .help("Function to disassemble")
            .value_parser(value_parser!(String))
            .requires("FILE")
            .conflicts_with("list_symbols") 
        )
        .arg(
            arg!(list_symbols: -l --"list-symbols")
            .help("List symbols")
            .action(ArgAction::SetTrue)
            .conflicts_with("func")
        )
        .arg(
            arg!(dynsyms: --"dyn-syms")
            .help("List dynamic symbol table")
            .action(ArgAction::SetTrue)
            .conflicts_with("func")
            .overrides_with("list_symbols")
            .overrides_with("syms")
        )
        .arg(
            arg!(syms: -s --syms)
            .help("List symbol table")
            .action(ArgAction::SetTrue)
            .conflicts_with("func")
            .overrides_with("list_symbols")
            .overrides_with("dynsyms")
        )
}

// This function will be very long and somewhat chunky
// Not too many ways around this. Keeping it out of main() and dt())
pub fn process_command(matches: ArgMatches) -> DtCode {
    let arg_path = matches.get_one::<PathBuf>("FILE").unwrap();

    return match std::fs::read(arg_path) {
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
            } else {
                // Here we know we'll need to utilize dtelf::FileData parsing
                // There is some setup we can do that all options will benefit from
                match FileData::lazy_parse(arg_path.to_str().unwrap(), &bytes) {
                    Ok(mut elf_file) => {
                        if matches.get_flag("list_symbols") {
                            elf_file.diplay_symbol_tables();
                        } else if matches.get_flag("dynsyms") {
                            elf_file.display_dynsyms();
                        } else if matches.get_flag("syms") {
                            elf_file.display_symtab();
                        }
                    }
                    Err(e) => {
                        println!("{e}");
                        result = DtCode::ElfParse;
                    }
                };
            }
            result
        }
        Err(e) => {
            // fs::read() failed
            println!("{e}");
            DtCode::IoReadFail
        }
    };
}
