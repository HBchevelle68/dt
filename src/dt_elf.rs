use goblin::elf::Elf;
use std::error::Error;

pub fn get_syms(bytes: &Vec<u8>) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn has_debug(bytes: &Vec<u8>) -> Result<bool, Box<dyn Error>> {
    // This could be improved by only reading in
    // only the elf header and not the whole file
    // let bytes: Vec<u8> = fs::read(path)?;
    match Elf::parse(&bytes) {
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
