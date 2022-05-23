use crate::Error;
use crate::ResizableLimits;
use crate::Section;
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

    pub fn remove_import(&mut self, index: usize) -> bool {
        if self.descriptors.len() < index {
            return false;
        }

        self.descriptors.remove(index);
        true
    }

    pub(crate) fn validate(&self) -> Result<(), ValidationError> {
        for x in &self.descriptors {
            x.validate()?;
        }

        Ok(())
    }
}

impl Section for MemorySection {
    fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        written += writer.write(&[self.id()])?;
        written += write::unsigned(writer, self.count() as u64)?;

        for x in self.descriptors {
            written += x.encode(writer)?;
        }
        writer.flush()?;
        Ok(written)
    }

    fn id(&self) -> u8 {
        0x05
    }

    fn count(&self) -> usize {
        self.descriptors.len()
    }
}

impl Default for MemorySection {
    fn default() -> Self {
        Self::new()
    }
}
