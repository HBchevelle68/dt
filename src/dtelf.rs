use goblin::elf::symver::{Verneed, Versym};
use goblin::elf::Elf;
use goblin::elf64::section_header;
use goblin::elf64::sym::{bind_to_str, type_to_str, visibility_to_str};
use std::error::Error;

// Unwrap failure
const UWFAIL: &str = "<ERR>";

fn display_symbol_table(tab: &Vec<ResolvedSym>, name: &str) {
    let num = tab.len();
    if num > 0 {
        println!("Symbol table '{name}' contains {num} entries:");
        println!(
            "{:>3}: {:>6} {:>15} {:<8} {:<6} {:<8} {:>4} {:<4}",
            "Num", "Value", "Size", "Type", "Bind", " Vis", "Ndx", "Name",
        );
        for (i, s) in tab.iter().enumerate() {
            s.print_symbol(i);
        }
        println!();
    }
}

#[derive(Debug)]
pub struct FileData<'a> {
    path: &'a str,
    bin: Elf<'a>,
    dynsyms: Vec<ResolvedSym<'a>>,
    symtab: Vec<ResolvedSym<'a>>,
    verneed: Vec<Verneed<'a>>,
    versyms: Vec<Versym>,
}

impl FileData<'_> {
    // Does not fully process the binary
    // Instead this simply sets up the bare minimum needed
    // to do as much or as little further processing as
    // requested
    pub fn lazy_parse<'a>(path: &'a str, bytes: &'a [u8]) -> Result<FileData<'a>, Box<dyn Error>> {
        // If parsing fails, dt as a whole  should really
        //  just bail. Just pass back the error
        match Elf::parse(bytes) {
            Ok(bin) => Ok(FileData {
                path,
                bin,
                dynsyms: Default::default(),
                symtab: Default::default(),
                versyms: Default::default(),
                verneed: Default::default(),
            }),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn diplay_symbol_tables(&mut self) {
        self.display_dynsyms();
        self.display_symtab();
    }

    pub fn display_dynsyms(&mut self) {
        if self.dynsyms.is_empty() {
            self.process_dynsyms();
        }
        self.get_dynsym_versions();
        display_symbol_table(&self.dynsyms, ".dynsym");
    }

    pub fn display_symtab(&mut self) {
        if self.symtab.is_empty() {
            self.process_symtab();
        }
        display_symbol_table(&self.symtab, ".symtab");
    }

    pub fn process_dynsyms(&mut self) {
        for dsym in &self.bin.dynsyms {
            self.dynsyms.push(ResolvedSym {
                symbol: self.bin.dynstrtab.get_at(dsym.st_name).unwrap_or(UWFAIL),
                version: None,
                info: dsym,
            })
        }
    }

    pub fn process_symtab(&mut self) {
        for sym in &self.bin.syms {
            self.symtab.push(ResolvedSym {
                symbol: self.bin.strtab.get_at(sym.st_name).unwrap_or(UWFAIL),
                version: None,
                info: sym,
            })
        }
    }

    // pub get_sym()

    fn get_dynsym_versions(&mut self) {
        // Both verneed and versyms are required
        if self.process_sym_version_info() {
            // TODO? -- I feel there is a better way to handle this in Rust
            for (dsym_idx, mut dsym) in self.dynsyms.iter_mut().enumerate() {
                // Its possible to have mutiple Verneed sections
                // each with its own optional Vernaux structures following.
                // Goblin makes use of iter() to access the Vernaux
                // entries.
                for verneed in self.verneed.iter() {
                    for vernaux in verneed.iter() {
                        if self.versyms[dsym_idx].vs_val == vernaux.vna_other {
                            dsym.version = Some(
                                self.bin
                                    .dynstrtab
                                    .get_at(vernaux.vna_name)
                                    .unwrap_or(UWFAIL),
                            );
                        }
                    }
                }
            }
        }
    }

    fn process_sym_version_info(&mut self) -> bool {
        // The Elfxx_Verneed section is an optional section. Its completely possible, this info
        // is just simply not here. This is not an error and is in compliance with ELF standard
        // https://refspecs.linuxfoundation.org/LSB_3.0.0/LSB-PDA/LSB-PDA.junk/symversion.html
        //
        // Goblin stores this data raw and Elf files store this data somewhat deeply nested.
        // Let's bring this data more to the forefront for easier access
        let mut res = false;
        if self.verneed.is_empty() && self.versyms.is_empty() {
            if let Some(versym_sec) = &self.bin.versym {
                self.versyms = versym_sec.into_iter().collect();

                if let Some(verneed_sec) = &self.bin.verneed {
                    self.verneed = verneed_sec.into_iter().collect()
                };
                res = true;
            };
        } else {
            res = true;
        }
        res
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
    // TODO document
    fn shndx_to_str(&self) -> String {
        match self.info.st_shndx as u32 {
            section_header::SHN_UNDEF => "UND".to_string(),
            section_header::SHN_ABS => "ABS".to_string(),
            val => val.to_string(),
        }
    }

    fn print_symbol(&self, idx: usize) {
        let symstr = match self.version {
            Some(ver) => {
                let tmp = format!("{}@{}", self.symbol, ver);
                tmp.to_owned()
            }
            _ => self.symbol.to_string(),
        };
        println!(
            "{:>3}: {:0>16} {:>5} {:<8} {:<6} {:<8} {:>4} {:<4}",
            idx,
            self.info.st_value,
            self.info.st_size,
            type_to_str(self.info.st_type()),
            bind_to_str(self.info.st_bind()),
            visibility_to_str(self.info.st_visibility()),
            self.shndx_to_str(),
            symstr,
        )
    }
}
