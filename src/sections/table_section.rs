use crate::Error;
use crate::ResizableLimits;
use crate::Section;
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

impl Section for TableSection {
    fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        written += writer.write(&[self.id()])?;
        written += write::unsigned(writer, self.count() as u64)?;

        for x in self.descriptors {
            written += writer.write(&[ValType::FuncRef.into()])?;
            written += x.encode(writer)?;
        }

        writer.flush()?;
        Ok(written)
    }

    fn id(&self) -> u8 {
        0x04
    }

    fn count(&self) -> usize {
        self.descriptors.len()
    }
}

impl Default for TableSection {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Add for TableSection {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut descriptors = Vec::with_capacity(self.descriptors.len() + rhs.descriptors.len());
        descriptors.extend(self.descriptors);
        descriptors.extend(rhs.descriptors);

        Self {
            descriptors
        }
    }
}