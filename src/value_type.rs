#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ValType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
    FuncRef = 0x70,
    Func = 0x60,
    Void = 0x40,
}
