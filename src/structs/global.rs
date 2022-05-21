use crate::Error;
use crate::ValType;
use crate::ValidationError;
use std::io::Write;

#[derive(Copy, Clone)]
pub struct GlobalDescriptor {
    valtype: ValType,
    mutable: bool,
}

impl GlobalDescriptor {
    pub fn new(valtype: ValType, mutable: bool) -> Self {
        Self { valtype, mutable }
    }

    pub(crate) fn encode(self, writer: &mut impl Write) -> Result<usize, Error> {
        let written = writer.write(&[self.valtype.into(), self.mutable as u8])?;
        Ok(written)
    }

    pub(crate) fn validate(&self) -> Result<(), ValidationError> {
        match self.valtype {
            ValType::Func | ValType::FuncRef | ValType::Void => Err(ValidationError::InvalidType),
            _ => Ok(()),
        }
    }
}
