use crate::Error;
use crate::GlobalDescriptor;
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
        0x06
    }
}

impl Default for GlobalSection {
    fn default() -> Self {
        Self::new()
    }
}
