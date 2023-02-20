use goblin::elf::symver::{Verneed, Versym};
use goblin::elf::Elf;
use goblin::elf64::section_header;
use goblin::elf64::sym::{bind_to_str, type_to_str, visibility_to_str};
use std::error::Error;

// Unwrap failure
const UWFAIL: &str = "<_error_>";

#[derive(Debug)]
pub struct FileData<'a> {
    path: &'a str,
    bin: Elf<'a>,
    dynsyms: Vec<ResolvedSym<'a>>,
    syms: Vec<ResolvedSym<'a>>,
    verneed: Vec<Verneed<'a>>,
    versyms: Vec<Versym>,
}

impl FileData<'_> {
    // Does not fully process the binary
    // Instead this simply sets up the bare minimum needed
    // to do as much or as little further processing as
    // requested
    pub fn lazy_parse<'a>(path: &'a str, bytes: &'a [u8]) -> Result<FileData<'a>, Box<dyn Error>> {
        // If parsing fails, dt as a whole
        // should really just bail. Just
        // pass back the error
        match Elf::parse(bytes) {
            Ok(bin) => Ok(FileData {
                path,
                bin,
                dynsyms: Default::default(),
                syms: Default::default(),
                versyms: Default::default(),
                verneed: Default::default(),
            }),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn display_dynsyms(&mut self) {
        self.get_sym_versions();
        let num = self.dynsyms.len();
        if num > 0 {
            println!("Symbol table '.dynsym' contains {num} entries:");
            println!(
                "{:>3}: {:>6} {:>15} {:<8} {:<6} {:<8} {:>4} {:<4}",
                "Num", "Value", "Size", "Type", "Bind", " Vis", "Ndx", "Name",
            );
            for (i, s) in self.dynsyms.iter().enumerate() {
                let symstr = match s.version {
                    Some(ver) => {
                        let tmp = format!("{}@{}", s.symbol, ver);
                        tmp.to_owned()
                    }
                    _ => s.symbol.to_string(),
                };

                println!(
                    "{:>3}: {:0>16} {:>5} {:<8} {:<6} {:>8} {:>4} {:<4}",
                    i,
                    s.info.st_value,
                    s.info.st_size,
                    type_to_str(s.info.st_type()),
                    bind_to_str(s.info.st_bind()),
                    visibility_to_str(s.info.st_visibility()),
                    s.ndx_to_str(),
                    symstr,
                )
            }
        }
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

    fn get_sym_versions(&mut self) {
        self.process_sym_version_info();

        // Both verneed and versyms are required
        if !self.verneed.is_empty() && !self.versyms.is_empty() {
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

    fn process_sym_version_info(&mut self) {
        // The Elfxx_Verneed section is an optional section. Its completely possible, this info
        // is just simply not here. This is not an error and is in compliance with ELF standard
        // https://refspecs.linuxfoundation.org/LSB_3.0.0/LSB-PDA/LSB-PDA.junk/symversion.html
        //
        // Goblin stores this data raw and Elf files store this data somewhat deeply nested.
        // Let's bring this data more to the forefront for easier access
        if let Some(versym_sec) = &self.bin.versym {
            self.versyms = versym_sec.into_iter().collect();

            if let Some(verneed_sec) = &self.bin.verneed {
                self.verneed = verneed_sec.into_iter().collect()
            };
        };
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
