use goblin::elf64::sym::{bind_to_str, type_to_str, visibility_to_str};
use goblin::{elf::Elf, elf64::section_header};
use std::error::Error;

const SYMFAIL: &str = "<FAILED TO RETRIEVE>";

#[derive(Debug)]
pub struct FileData<'a> {
    path: &'a str,
    bin: Option<Elf<'a>>,
    dynsyms: Vec<ResolvedSym<'a>>,
    syms: Vec<ResolvedSym<'a>>,
}

impl FileData<'_> {
    // Dpes not fully process the elf
    pub fn new<'a>(path: &'a str, bytes: &'a [u8]) -> FileData<'a> {
        match Elf::parse(bytes) {
            Ok(bin) => FileData {
                path,
                bin: Some(bin),
                dynsyms: Default::default(),
                syms: Default::default(),
            },
            Err(e) => {
                println!("{e}");
                FileData::default()
            }
        }
    }

    fn process_dynsyms(&mut self) {
        unimplemented!();
    }
}

impl Default for FileData<'_> {
    fn default() -> FileData<'static> {
        FileData {
            path: Default::default(),
            bin: Default::default(),
            dynsyms: Default::default(),
            syms: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
struct ResolvedSym<'a> {
    /// Resolved symbol a human readable string
    symbol: &'a str,
    /// Symbol version. If no version found this will be None
    version: Option<&'a str>,
    /// Raw data from elf
    info: goblin::elf::Sym,
}

impl ResolvedSym<'_> {
    fn ndx_to_str(&self) -> &str {
        match self.info.st_shndx as u32 {
            section_header::SHN_UNDEF => &"UND",
            section_header::SHN_LORESERVE => &"LORESRVE",
            section_header::SHN_ABS => &"ABS",
            _ => "NDX_ERR",
        }
    }
}

// fn display_syms(syms: &Vec<ResolvedSym>) {
//     let num = syms.len();
//     if num > 0 {
//         println!("Symbol table '.dynsym' contains {num} entries:");
//         println!(
//             "{:>3}: {:>6} {:>15} {:<8} {:<6} {:<8} {:>4} {:<4}",
//             "Num", "Value", "Size", "Type", "Bind", "Vis", "Ndx", "Name",
//         );
//     }

//     for (i, s) in syms.iter().enumerate() {
//         println!(
//             "{:>3}: {:0>16} {:>5} {:<8} {:<6} {:>8} {:>4} {:<4}",
//             i,
//             s.info.st_value,
//             s.info.st_size,
//             type_to_str(s.info.st_type()),
//             bind_to_str(s.info.st_bind()),
//             visibility_to_str(s.info.st_visibility()),
//             s.ndx_to_str(),
//             s.symbol,
//         )
//     }
// }

// fn resolve_dynamic_symbols<'a>(
//     bin: &'a Elf<'a>,
//     symstrs: &Vec<ResolvedSym<'a>>,
// ) -> Option<Vec<ResolvedSym<'a>>> {
//     if bin.interpreter.is_none() {
//         // TODO
//         // Add feedback??
//         None
//     } else {
//         let mut dynsyms_resolved = vec![];
//         // Iterate over each Dynamic Symbol
//         // Index into .dynsymtab, pull symbol string
//         // Check if string pulled is in the symbol table
//         // if so, use string from symbol table
//         //
//         // Not a massive fan of this, I feel there must
//         // be a more efficient way to resolve this
//         for dynsym in bin.dynsyms.iter().by_ref() {
//             let mut final_sym = bin.dynstrtab.get_at(dynsym.st_name).unwrap_or(SYMFAIL);

//             for s in symstrs.iter().map(|sym| sym.symbol) {
//                 if s.contains(final_sym) {
//                     final_sym = s;
//                 }
//             }

//             let tmp = ResolvedSym {
//                 symbol: final_sym,
//                 info: dynsym,
//             };

//             dynsyms_resolved.push(tmp);
//         }
//         Some(dynsyms_resolved)
//     }
// }

// fn resolve_symbols<'a>(bin: &'a Elf<'a>) -> Vec<ResolvedSym<'a>> {
//     let mut syms_resolved = vec![];
//     for sym in bin.syms.iter().by_ref() {
//         let tmp = ResolvedSym {
//             symbol: bin.strtab.get_at(sym.st_name).unwrap_or(SYMFAIL),
//             info: sym,
//         };
//         syms_resolved.push(tmp);
//     }
//     syms_resolved
// }

// pub fn get_all_syms(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
//     match Elf::parse(bytes) {
//         Ok(bin) => {
//             let syms_resolved = resolve_symbols(&bin);
//             if !syms_resolved.is_empty() {
//                 if let Some(dynsyms_resolved) = resolve_dynamic_symbols(&bin, &syms_resolved) {
//                     display_syms(&dynsyms_resolved);
//                 }
//                 Ok(())
//             } else {
//                 Err("[!] No symbols found....something is busted".into())
//             }
//         }
//         Err(e) => Err(Box::new(e)),
//     }
// }
