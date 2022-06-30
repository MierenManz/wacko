use crate::Error;
use crate::ValType;
use leb128::write;
use std::io::Write;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction<'a> {
    /// `Block(return_value)`
    Block(ValType),
    /// `Loop(return_value)`
    Loop(ValType),
    /// `Br(depth)`
    Br(u32),
    /// `BrIf(depth)`
    BrIf(u32),
    // this should have a better type
    /// `BrTable(table, default / fallback)`
    BrTable(&'a [u32], u32),
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
    I64Add,
    I32Sub,
    I64Sub,
    I32Mul,
    I64Mul,
    I32DivS,
    I64DivS,
    I32DivU,
    I64DivU,
    I32RemS,
    I64RemS,
    I32RemU,
    I64RemU,
    I32And,
    I64And,
    I32Or,
    I64Or,
    I32Xor,
    I64Xor,
    I32Shl,
    I64Shl,
    I32ShrS,
    I64ShrS,
    I32ShrU,
    I64ShrU,
    I32Rotl,
    I64Rotl,
    I32Rotr,
    I64Rotr,
    I32Clz,
    I64Clz,
    I32Ctz,
    I64Ctz,
    I32PopCnt,
    I64PopCnt,
    I32Eqz,
    I64Eqz,

    F32Add,
    F64Add,
    F32Sub,
    F64Sub,
    F32Mul,
    F64Mul,
    F32Div,
    F64Div,
    F32Sqrt,
    F64Sqrt,
    F32Min,
    F64Min,
    F32Max,
    F64Max,
    F32Ceil,
    F64Ceil,
    F32Floor,
    F64Floor,
    F32Trunc,
    F64Trunc,
    F32Nearest,
    F64Nearest,
    F32Abs,
    F64Abs,
    F32Neg,
    F64Neg,
    F32Copysign,
    F64Copysign,

    I32Eq,
    I64Eq,
    I32Ne,
    I64Ne,
    I32ltS,
    I64ltS,
    I32ltU,
    I64ltU,
    I32leS,
    I64leS,
    I32leU,
    I64leU,
    I32GtS,
    I64GtS,
    I32GtU,
    I64GtU,
    I32GeS,
    I64GeS,
    I32GeU,
    I64GeU,

    F32Eq,
    F64Eq,
    F32Ne,
    F64Ne,
    F32Lt,
    F64Lt,
    F32Le,
    F64Le,
    F32Gt,
    F64Gt,
    F32Ge,
    F64Ge,

    I32WrapI64,
    I64ExtendI32S,
    I64ExtendI32U,
    I32TruncF32S,
    I32TruncF64S,
    I64TruncF32S,
    I64TruncF64S,
    I32TruncF32U,
    I32TruncF64U,
    I64TruncF32U,
    I64TruncF64U,

    F32DemoteF64,
    F64PromoteF32,
    F32ConvertI32S,
    F32ConvertI64S,
    F64ConvertI32S,
    F64ConvertI64S,
    F32ConvertI32U,
    F32ConvertI64U,
    F64ConvertI32U,
    F64ConvertI64U,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,

    /// `I32Load(align, offset)`
    I32Load(u32, u32),
    /// `I64Load(align, offset)`
    I64Load(u32, u32),
    /// `F32Load(align, offset)`
    F32Load(u32, u32),
    /// `F64Load(align, offset)`
    F64Load(u32, u32),
    /// `I32Store(align, offset)`
    I32Store(u32, u32),
    /// `I64Store(align, offset)`
    I64Store(u32, u32),
    /// `F32Store(align, offset)`
    F32Store(u32, u32),
    /// `F64Store(align, offset)`
    F64Store(u32, u32),

    /// `I32Load8S(align, offset)`
    I32Load8S(u32, u32),
    /// `I32Load16S(align, offset)`
    I32Load16S(u32, u32),
    /// `I64Load8S(align, offset)`
    I64Load8S(u32, u32),
    /// `I64Load16S(align, offset)`
    I64Load16S(u32, u32),
    /// `I64Load32S(align, offset)`
    I64Load32S(u32, u32),

    /// `I32Load8U(align, offset)`
    I32Load8U(u32, u32),
    /// `I32Load16U(align, offset)`
    I32Load16U(u32, u32),
    /// `I64Load8U(align, offset)`
    I64Load8U(u32, u32),
    /// `I64Load16U(align, offset)`
    I64Load16U(u32, u32),
    /// `I64Load32U(align, offset)`
    I64Load32U(u32, u32),

    /// `I32Store8(align, offset)`
    I32Store8(u32, u32),
    /// `I32Store16(align, offset)`
    I32Store16(u32, u32),
    /// `I64Store8(align, offset)`
    I64Store8(u32, u32),
    /// `I64Store16(align, offset)`
    I64Store16(u32, u32),
    /// `I64Store32(align, offset)`
    I64Store32(u32, u32),

    MemoryGrow,
    MemorySize,

    // RefNull,
    // RefIsNull,
    // RefFunc,

    // 0xFC instructions
    I32TruncSatF32S,
    I32TruncSatF64S,
    I64TruncSatF32S,
    I64TruncSatF64S,
    I32TruncSatF32U,
    I32TruncSatF64U,
    I64TruncSatF32U,
    I64TruncSatF64U,
    // MemoryInit,
    // DataDrop,
    // MemoryCopy,
    // MemoryFill,
    // TableInit,
    // ElemDrop,
    // TableCopy,
    // TableGrow,
    // TableSize,
    // TableFill,
}

impl Instruction<'_> {
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), Error> {
        self.write_opcode(writer)?;
        match self {
            Instruction::Block(v) | Instruction::Loop(v) => writer.write_all(&[(*v).into()])?,
            Instruction::Br(depth) | Instruction::BrIf(depth) => {
                write::unsigned(writer, *depth as u64)?;
            }
            Instruction::BrTable(table, default) => {
                write::unsigned(writer, table.len() as u64)?;
                for i in 0..table.len() {
                    let idx = table[i];
                    write::unsigned(writer, idx as u64)?;
                }

                write::unsigned(writer, *default as u64)?;
            }
            Instruction::If(v) => writer.write_all(&[(*v).into()])?,
            Instruction::I32Const(v) => {
                write::signed(writer, *v as i64)?;
            }
            Instruction::I64Const(v) => {
                write::signed(writer, *v as i64)?;
            }
            Instruction::F32Const(v) => writer.write_all(&v.to_le_bytes())?,
            Instruction::F64Const(v) => writer.write_all(&v.to_le_bytes())?,

            Instruction::LocalGet(idx)
            | Instruction::LocalSet(idx)
            | Instruction::LocalTee(idx)
            | Instruction::GlobalGet(idx)
            | Instruction::GlobalSet(idx)
            | Instruction::Call(idx)
            | Instruction::CallIndirect(idx) => {
                write::unsigned(writer, *idx as u64)?;
            }

            Instruction::I32Load(align, offset)
            | Instruction::I64Load(align, offset)
            | Instruction::F32Load(align, offset)
            | Instruction::F64Load(align, offset)
            | Instruction::I32Store(align, offset)
            | Instruction::I64Store(align, offset)
            | Instruction::F32Store(align, offset)
            | Instruction::F64Store(align, offset)
            | Instruction::I32Load8S(align, offset)
            | Instruction::I32Load16S(align, offset)
            | Instruction::I64Load8S(align, offset)
            | Instruction::I64Load16S(align, offset)
            | Instruction::I64Load32S(align, offset)
            | Instruction::I32Load8U(align, offset)
            | Instruction::I32Load16U(align, offset)
            | Instruction::I64Load8U(align, offset)
            | Instruction::I64Load16U(align, offset)
            | Instruction::I64Load32U(align, offset)
            | Instruction::I32Store8(align, offset)
            | Instruction::I32Store16(align, offset)
            | Instruction::I64Store8(align, offset)
            | Instruction::I64Store16(align, offset)
            | Instruction::I64Store32(align, offset) => {
                write::unsigned(writer, *align as u64)?;
                write::unsigned(writer, *offset as u64)?;
            }
            _ => {}
        };

        Ok(())
    }

    pub fn write_opcode(self, writer: &mut impl Write) -> Result<(), Error> {
        let byte = match self {
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
            Instruction::I32Add => 0x6A,
            Instruction::I64Add => 0x7C,
            Instruction::I32Sub => 0x6B,
            Instruction::I64Sub => 0x7D,
            Instruction::I32Mul => 0x6C,
            Instruction::I64Mul => 0x7E,
            Instruction::I32DivS => 0x6D,
            Instruction::I64DivS => 0x7F,
            Instruction::I32DivU => 0x6E,
            Instruction::I64DivU => 0x80,
            Instruction::I32RemS => 0x6F,
            Instruction::I64RemS => 0x81,
            Instruction::I32RemU => 0x70,
            Instruction::I64RemU => 0x82,
            Instruction::I32And => 0x71,
            Instruction::I64And => 0x83,
            Instruction::I32Or => 0x72,
            Instruction::I64Or => 0x84,
            Instruction::I32Xor => 0x73,
            Instruction::I64Xor => 0x85,
            Instruction::I32Shl => 0x74,
            Instruction::I64Shl => 0x86,
            Instruction::I32ShrS => 0x75,
            Instruction::I64ShrS => 0x87,
            Instruction::I32ShrU => 0x76,
            Instruction::I64ShrU => 0x88,
            Instruction::I32Rotl => 0x77,
            Instruction::I64Rotl => 0x89,
            Instruction::I32Rotr => 0x78,
            Instruction::I64Rotr => 0x8A,
            Instruction::I32Clz => 0x67,
            Instruction::I64Clz => 0x79,
            Instruction::I32Ctz => 0x68,
            Instruction::I64Ctz => 0x7A,
            Instruction::I32PopCnt => 0x69,
            Instruction::I64PopCnt => 0x7B,
            Instruction::I32Eqz => 0x45,
            Instruction::I64Eqz => 0x50,
            Instruction::F32Add => 0x92,
            Instruction::F64Add => 0xA0,
            Instruction::F32Sub => 0x93,
            Instruction::F64Sub => 0xA1,
            Instruction::F32Mul => 0x94,
            Instruction::F64Mul => 0xA2,
            Instruction::F32Div => 0x95,
            Instruction::F64Div => 0xA3,
            Instruction::F32Sqrt => 0x91,
            Instruction::F64Sqrt => 0x9F,
            Instruction::F32Min => 0x96,
            Instruction::F64Min => 0xA4,
            Instruction::F32Max => 0x97,
            Instruction::F64Max => 0xA5,
            Instruction::F32Ceil => 0x8D,
            Instruction::F64Ceil => 0x9B,
            Instruction::F32Floor => 0x8E,
            Instruction::F64Floor => 0x9C,
            Instruction::F32Trunc => 0x8F,
            Instruction::F64Trunc => 0x9D,
            Instruction::F32Nearest => 0x90,
            Instruction::F64Nearest => 0x9E,
            Instruction::F32Abs => 0x8B,
            Instruction::F64Abs => 0x99,
            Instruction::F32Neg => 0x8C,
            Instruction::F64Neg => 0x9A,
            Instruction::F32Copysign => 0x98,
            Instruction::F64Copysign => 0xA6,
            Instruction::I32Eq => 0x46,
            Instruction::I64Eq => 0x51,
            Instruction::I32Ne => 0x47,
            Instruction::I64Ne => 0x52,
            Instruction::I32ltS => 0x48,
            Instruction::I64ltS => 0x53,
            Instruction::I32ltU => 0x49,
            Instruction::I64ltU => 0x54,
            Instruction::I32leS => 0x4C,
            Instruction::I64leS => 0x57,
            Instruction::I32leU => 0x4D,
            Instruction::I64leU => 0x58,
            Instruction::I32GtS => 0x4A,
            Instruction::I64GtS => 0x55,
            Instruction::I32GtU => 0x4B,
            Instruction::I64GtU => 0x56,
            Instruction::I32GeS => 0x4e,
            Instruction::I64GeS => 0x59,
            Instruction::I32GeU => 0x4F,
            Instruction::I64GeU => 0x5A,
            Instruction::F32Eq => 0x5B,
            Instruction::F64Eq => 0x61,
            Instruction::F32Ne => 0x5C,
            Instruction::F64Ne => 0x62,
            Instruction::F32Lt => 0x5D,
            Instruction::F64Lt => 0x63,
            Instruction::F32Le => 0x5F,
            Instruction::F64Le => 0x65,
            Instruction::F32Gt => 0x5E,
            Instruction::F64Gt => 0x64,
            Instruction::F32Ge => 0x60,
            Instruction::F64Ge => 0x66,
            Instruction::I32WrapI64 => 0xA7,
            Instruction::I64ExtendI32S => 0xAC,
            Instruction::I64ExtendI32U => 0xAD,
            Instruction::I32TruncF32S => 0xA8,
            Instruction::I32TruncF64S => 0xAA,
            Instruction::I64TruncF32S => 0xAE,
            Instruction::I64TruncF64S => 0xB0,
            Instruction::I32TruncF32U => 0xA9,
            Instruction::I32TruncF64U => 0xAB,
            Instruction::I64TruncF32U => 0xAF,
            Instruction::I64TruncF64U => 0xB1,
            Instruction::F32DemoteF64 => 0xB6,
            Instruction::F64PromoteF32 => 0xBB,
            Instruction::F32ConvertI32S => 0xB2,
            Instruction::F32ConvertI64S => 0xB4,
            Instruction::F64ConvertI32S => 0xB7,
            Instruction::F64ConvertI64S => 0xB9,
            Instruction::F32ConvertI32U => 0xB3,
            Instruction::F32ConvertI64U => 0xB5,
            Instruction::F64ConvertI32U => 0xB8,
            Instruction::F64ConvertI64U => 0xBA,
            Instruction::I32ReinterpretF32 => 0xBC,
            Instruction::I64ReinterpretF64 => 0xBD,
            Instruction::F32ReinterpretI32 => 0xBE,
            Instruction::F64ReinterpretI64 => 0xBF,
            Instruction::I32Extend8S => 0xC0,
            Instruction::I32Extend16S => 0xC1,
            Instruction::I64Extend8S => 0xC2,
            Instruction::I64Extend16S => 0xC3,
            Instruction::I64Extend32S => 0xC4,
            Instruction::I32Load(_, _) => 0x28,
            Instruction::I64Load(_, _) => 0x29,
            Instruction::F32Load(_, _) => 0x2A,
            Instruction::F64Load(_, _) => 0x2B,
            Instruction::I32Store(_, _) => 0x36,
            Instruction::I64Store(_, _) => 0x37,
            Instruction::F32Store(_, _) => 0x38,
            Instruction::F64Store(_, _) => 0x39,
            Instruction::I32Load8S(_, _) => 0x2C,
            Instruction::I32Load16S(_, _) => 0x2E,
            Instruction::I64Load8S(_, _) => 0x30,
            Instruction::I64Load16S(_, _) => 0x32,
            Instruction::I64Load32S(_, _) => 0x34,
            Instruction::I32Load8U(_, _) => 0x2D,
            Instruction::I32Load16U(_, _) => 0x2F,
            Instruction::I64Load8U(_, _) => 0x31,
            Instruction::I64Load16U(_, _) => 0x33,
            Instruction::I64Load32U(_, _) => 0x35,
            Instruction::I32Store8(_, _) => 0x3A,
            Instruction::I32Store16(_, _) => 0x3B,
            Instruction::I64Store8(_, _) => 0x3C,
            Instruction::I64Store16(_, _) => 0x3D,
            Instruction::I64Store32(_, _) => 0x3E,
            Instruction::MemoryGrow => 0x40,
            Instruction::MemorySize => 0x3F,
            _ => 0xFC,
        };
        writer.write_all(&[byte])?;
        if byte == 0xFC {
            let other_byte = match self {
                Instruction::I32TruncSatF32S => 0x00,
                Instruction::I32TruncSatF32U => 0x01,
                Instruction::I32TruncSatF64S => 0x02,
                Instruction::I32TruncSatF64U => 0x03,
                Instruction::I64TruncSatF32S => 0x04,
                Instruction::I64TruncSatF32U => 0x05,
                Instruction::I64TruncSatF64S => 0x06,
                Instruction::I64TruncSatF64U => 0x07,
                _ => unreachable!(),
            };
            writer.write_all(&[other_byte])?;
        }

        Ok(())
    }
}
