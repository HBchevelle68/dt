#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DtCode {
    Success = 0,
    MissingArg = -1,
    BadArg = -2,
    IoReadFail = -3,
    ElfParse = -4,
}
