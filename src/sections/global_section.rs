use crate::Error;
use crate::GlobalDescriptor;
use crate::Section;
use crate::ValidationError;
use leb128::write;
use std::io::Write;

pub struct GlobalSection {
    descriptors: Vec<GlobalDescriptor>,
}

impl GlobalSection {
    pub fn new() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub fn add_descriptor(&mut self, descriptor: GlobalDescriptor) -> usize {
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

impl Section for GlobalSection {
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
        0x06
    }

    fn count(&self) -> usize {
        self.descriptors.len()
    }
}

impl Default for GlobalSection {
    fn default() -> Self {
        Self::new()
    }
}
