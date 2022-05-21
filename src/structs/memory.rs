use crate::Error;
use crate::ValidationError;
use leb128::write;
use std::io::Write;

#[derive(Copy, Clone)]
pub struct ResizableLimits {
    pub minimum: u32,
    pub maximum: Option<u32>,
}

impl ResizableLimits {
    pub(crate) fn encode(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        let flags = if self.maximum.is_some() { 0x01 } else { 0x00 };
        written += writer.write(&[flags])?;
        written += write::unsigned(writer, self.minimum as u64)?;

        if let Some(v) = self.maximum {
            written += write::unsigned(writer, v as u64)?;
        }
        Ok(written)
    }

    pub(crate) fn validate(&self) -> Result<(), ValidationError> {
        if let Some(v) = self.maximum {
            if v < self.minimum {
                return Err(ValidationError::InvalidMemorySetting);
            }
        }

        Ok(())
    }
}
