use goblin::elf64::sym::{bind_to_str, type_to_str, visibility_to_str};
use goblin::{elf::Elf, elf64::section_header};
use std::error::Error;

const SYMFAIL: &str = "<FAILED TO RETRIEVE>";

// From
// #define SHN_UNDEF	0		/* Undefined section reference */
// #define SHN_LORESERVE	(-0x100u)	/* Begin range of reserved indices */
// #define SHN_LOPROC	(-0x100u)	/* Begin range of appl-specific */
// #define SHN_HIPROC	(-0xE1u)	/* End range of appl-specific */
// #define SHN_LOOS	(-0xE0u)	/* OS specific semantics, lo */
// #define SHN_HIOS	(-0xC1u)	/* OS specific semantics, hi */
// #define SHN_ABS		(-0xFu)		/* Associated symbol is absolute */
// #define SHN_COMMON	(-0xEu)		/* Associated symbol is in common */
// #define SHN_XINDEX	(-0x1u)		/* Section index is held elsewhere */
// #define SHN_HIRESERVE	(-0x1u)		/* End range of reserved indices */
// #define SHN_BAD		(-0x101u)	/* Used internally by bfd */
#[derive(Debug)]
struct ResolvedSym<'a> {
    /// Resolved symbol a human readable string
    symbol: &'a str,
    /// Raw data from elf
    info: goblin::elf::Sym,
}

impl ResolvedSym {
    fn ndx_to_str(&self) -> &str {
        match self.info.st_shndx as u32 {
            section_header::SHN_UNDEF => &"UNDEF",
            section_header::SHN_LORESERVE => &"LORESRVE",
            section_header::SHN_ABS => &"ABS",
            _ => ,
        }
    }
}

fn display_syms(syms: &Vec<ResolvedSym>) {
    for (i, s) in syms.iter().enumerate() {
        println!(
            "{:>3}: {:0>16} {:>5} {:<8} {:>8} {:>8} {:>4}",
            i,
            s.info.st_value,
            s.info.st_size,
            type_to_str(s.info.st_type()),
            bind_to_str(s.info.st_bind()),
            visibility_to_str(s.info.st_visibility()),
            s.info.st_shndx
        )
    }
}

fn resolve_dynamic_symbols<'a>(
    bin: &'a Elf<'a>,
    symstrs: &Vec<ResolvedSym<'a>>,
) -> Option<Vec<ResolvedSym<'a>>> {
    if bin.interpreter.is_none() {
        // TODO
        // Add feedback??
        None
    } else {
        let mut dynsyms_resolved = vec![];
        // Iterate over each Dynamic Symbol
        // Index into .dynsymtab, pull symbol string
        // Check if string pulled is in the symbol table
        // if so, use string from symbol table
        //
        // Not a massive fan of this, I feel there must
        // be a more efficient way to resolve this
        for dynsym in bin.dynsyms.iter().by_ref() {
            let mut final_sym = bin.dynstrtab.get_at(dynsym.st_name).unwrap_or(SYMFAIL);

            for s in symstrs.iter().map(|sym| sym.symbol) {
                if s.contains(final_sym) {
                    final_sym = s;
                }
            }

            let tmp = ResolvedSym {
                symbol: final_sym,
                info: dynsym,
            };

            dynsyms_resolved.push(tmp);
        }
        Some(dynsyms_resolved)
    }
}

fn resolve_symbols<'a>(bin: &'a Elf<'a>) -> Vec<ResolvedSym<'a>> {
    let mut syms_resolved = vec![];
    for sym in bin.syms.iter().by_ref() {
        let tmp = ResolvedSym {
            symbol: bin.strtab.get_at(sym.st_name).unwrap_or(SYMFAIL),
            info: sym,
        };
        syms_resolved.push(tmp);
    }
    syms_resolved
}

pub fn get_all_syms(bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    match Elf::parse(bytes) {
        Ok(bin) => {
            let syms_resolved = resolve_symbols(&bin);
            if !syms_resolved.is_empty() {
                if let Some(dynsyms_resolved) = resolve_dynamic_symbols(&bin, &syms_resolved) {
                    display_syms(&dynsyms_resolved);
                }
                Ok(())
            } else {
                Err("[!] No symbols found....something is busted".into())
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn has_debug(bytes: &[u8]) -> Result<bool, Box<dyn Error>> {
    // This could be improved by only reading in
    // only the elf header and not the whole file
    // let bytes: Vec<u8> = fs::read(path)?;
    match Elf::parse(bytes) {
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
