use crate::Error;
use crate::ResizableLimits;
use crate::ValType;
use crate::ValidationError;
use leb128::write;
use std::io::Write;

pub struct TableSection {
    descriptors: Vec<ResizableLimits>,
}

impl TableSection {
    pub fn new() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub fn add_descriptor(&mut self, descriptor: ResizableLimits) -> usize {
        self.descriptors.push(descriptor);
        self.descriptors.len() - 1
    }

    pub fn count(&self) -> usize {
        self.descriptors.len()
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        for x in &self.descriptors {
            x.validate()?;
        }
        Ok(())
    }

    pub fn compile(self, writer: &mut impl Write) -> Result<(), Error> {
        if self.descriptors.is_empty() {
            return Ok(());
        }
        writer.write_all(&[Self::id()])?;
        write::unsigned(writer, self.descriptors.len() as u64)?;

        for x in self.descriptors {
            writer.write_all(&[ValType::FuncRef.into()])?;
            x.encode(writer)?;
        }

        Ok(())
    }

    fn id() -> u8 {
        0x04
    }
}

impl Default for TableSection {
    fn default() -> Self {
        Self::new()
    }
}
