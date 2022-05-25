use crate::ValType;
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Instruction {
    /// `Block(return_value)`
    Block(ValType),
    /// `Loop(return_value)`
    Loop(ValType),
    /// `Br(depth)`
    Br(u32),
    /// `BrIf(depth)`
    BrIf(u32),
    /// `BrTable(table_id, default/fallback)`
    BrTable(u32, u32),
    /// `If(return_value)`
    If(ValType),
    Else,
    End,
    Return,
    Unreachable,

    Nop,
    Drop,
    /// `I32Const(value)`
    I32Const(i32),
    /// `I64Const(value)`
    I64Const(i64),
    /// `F32Const(value)`
    F32Const(f32),
    /// `F64Const(value)`
    F64Const(f64),


    /// `LocalGet(local_id)`
    LocalGet(u32),
    /// `LocalSet(local_id)`
    LocalSet(u32),
    /// `LocalTee(local_id)`
    LocalTee(u32),

    /// `GlobalGet(global_id)`
    GlobalGet(u32),
    /// `GlobalSet(global_id)`
    GlobalSet(u32),

    Select,

    /// `Call(fn_id)`
    Call(u32),
    /// `CallIndirect(type_id)`
    CallIndirect(u32),

    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I32Clz,
    I32Ctz,
    I32PopCnt,
    I32Eqz,

    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    I64Clz,
    I64Ctz,
    I64PopCnt,
    I64Eqz,

    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Sqrt,

    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Sqrt,
}

impl From<Instruction> for u8 {
    fn from(instr: Instruction) -> Self {
        match instr {
            Instruction::Block(_) => 0x02,
            Instruction::Loop(_) => 0x03,
            Instruction::Br(_) => 0x0C,
            Instruction::BrIf(_) => 0x0D,
            Instruction::BrTable(_, _) => 0x0E,
            Instruction::If(_) => 0x04,
            Instruction::Else => 0x05,
            Instruction::End => 0x0B,
            Instruction::Return => 0x0F,
            Instruction::Unreachable => 0x00,
            Instruction::Nop => 0x01,
            Instruction::Drop => 0x1A,
            Instruction::I32Const(_) => 0x41,
            Instruction::I64Const(_) => 0x42,
            Instruction::F32Const(_) => 0x43,
            Instruction::F64Const(_) => 0x44,
            Instruction::LocalGet(_) => 0x20,
            Instruction::LocalSet(_) => 0x21,
            Instruction::LocalTee(_) => 0x22,
            Instruction::GlobalGet(_) => 0x23,
            Instruction::GlobalSet(_) => 0x24,
            Instruction::Select => 0x1B,
            Instruction::Call(_) => 0x10,
            Instruction::CallIndirect(_) => 0x11,
            _ => unreachable!(),
        }
    }
}
