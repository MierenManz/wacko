use crate::ValType;
#[repr(u8)]
pub enum Instruction {
    Block(ValType),
    Loop(ValType),
    /// `Br(depth)`
    Br(u32),
    /// `BrIf(depth)`
    BrIf(u32),
    /// `BrTable(table_id, default/fallback)`
    BrTable(u32, u32),
    If(ValType), 
    Else,
    End,
    
}

impl From<Instruction> for u8 {
    fn from(instr: Instruction) -> Self {
        match instr {
            Instruction::Block(_) => 0x02,
            Instruction::Loop(_) => 0x03,
            Instruction::Br(_) => 0x0C,
            Instruction::BrIf(_) => 0x0D,
            Instruction::BrTable(_, _,) => 0x0E,
            Instruction::If(_) => 0x04,
            Instruction::Else => 0x05,
            Instruction::End => 0x0B
        }
    }
}
