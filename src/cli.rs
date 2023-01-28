use clap::{arg, value_parser, ArgAction, Command};
use std::path::PathBuf;

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
            arg!(func: -f)
            .long("func")
            .help("Function to disassemble")
            .value_parser(value_parser!(String))
            .requires("FILE")
        )
        .arg(
            arg!(list: -l)
            .help("List symbols")
            .long("list-syms")
            .action(ArgAction::SetTrue)
            .conflicts_with("func")
        )
}

// fn path_parser(path: &str) -> Result<PathBuf, String> {
//     match PathBuf::from(path).canonicalize() {
//         Ok(pb) => Ok(pb),
//         Err(e) => Err(format!("Error parsing FILE {}", e)),
//     }
// }
