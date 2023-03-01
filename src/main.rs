mod cli;
mod codes;
mod disasm;
mod dtelf;
use codes::DtCode;

fn dt() -> DtCode {
    let matches = cli::dt_cli().get_matches();

    cli::process_command(matches)
}

fn main() {
    std::process::exit(dt() as i32);
}
