#[derive(Copy, Clone, Debug)]
pub enum ValType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
    V128 = 0x7B,
    FuncRef = 0x70,
    ExternRef = 0x6F,
}

#[derive(Debug)]
pub struct Type {
    pub(crate) params: Vec<ValType>,
    pub(crate) result: Vec<ValType>,
}

impl Type {
    pub fn new(params: Vec<ValType>, result: Vec<ValType>) -> Self {
        Self { params, result }
    }
}
