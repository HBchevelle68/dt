use clap::Parser;
use std::{fs, path::PathBuf};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author = "HBchevelle68", version="0.1", about, long_about = None)]
struct Args {
    /// Path to binary to disassemble
    path: String,

    /// Function you wish to disassemble
    //#[arg(short, long)]
    func: String,
}

fn has_debug(binary: &PathBuf) {}

fn main() {
    let args = Args::parse();
}
