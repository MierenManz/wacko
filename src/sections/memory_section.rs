use crate::Error;
use crate::ResizableLimits;
use crate::ValidationError;
use leb128::write;
use std::io::Write;

pub struct MemorySection {
    descriptors: Vec<ResizableLimits>,
}

impl MemorySection {
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

    pub fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        if self.descriptors.is_empty() {
            return Ok(0);
        }
        let mut written = writer.write(&[Self::id()])?;
        written += write::unsigned(writer, self.descriptors.len() as u64)?;

        for x in self.descriptors {
            written += x.encode(writer)?;
        }
        writer.flush()?;
        Ok(written)
    }

    fn id() -> u8 {
        0x05
    }
}

impl Default for MemorySection {
    fn default() -> Self {
        Self::new()
    }
}
