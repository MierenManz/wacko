use crate::Error;
use leb128::write;
use std::io::Write;

#[derive(Copy, Clone, PartialEq)]
pub enum GlobalValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl GlobalValue {
    pub fn encode(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        written += writer.write(&[self.into()])?;
        written += match self {
            GlobalValue::I32(v) => write::signed(writer, v as i64)?,
            GlobalValue::I64(v) => write::signed(writer, v as i64)?,
            GlobalValue::F32(v) => writer.write(&v.to_le_bytes())?,
            GlobalValue::F64(v) => writer.write(&v.to_le_bytes())?,
        };

        Ok(written)
    }
}

impl From<GlobalValue> for u8 {
    fn from(other: GlobalValue) -> Self {
        match other {
            GlobalValue::I32(_) => 0x7F,
            GlobalValue::I64(_) => 0x7E,
            GlobalValue::F32(_) => 0x7D,
            GlobalValue::F64(_) => 0x7C,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct GlobalDescriptor {
    valtype: GlobalValue,
    mutable: bool,
}

impl GlobalDescriptor {
    pub fn new(valtype: GlobalValue, mutable: bool) -> Self {
        Self { valtype, mutable }
    }

    pub(crate) fn encode(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = self.valtype.encode(writer)?;
        written += writer.write(&[self.mutable as u8])?;
        Ok(written)
    }

    pub(crate) fn is_mut(&self) -> bool {
        self.mutable
    }
}
