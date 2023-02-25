#[derive(Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum DtCode {
    Success = 0,
    //BadArg = 0x1,
    IoReadFail = 2,
    ElfParse = 3,
    //ElfNoDynSym = 0x4,
}
