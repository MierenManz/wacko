use crate::Error;
use crate::GlobalDescriptor;
use leb128::write;
use std::io::Write;

#[derive(Default)]
pub struct GlobalSection {
    descriptors: Vec<GlobalDescriptor>,
}

impl GlobalSection {
    pub fn add_descriptor(&mut self, descriptor: GlobalDescriptor) -> usize {
        self.descriptors.push(descriptor);
        self.descriptors.len() - 1
    }

    pub fn compile(self, writer: &mut impl Write) -> Result<(), Error> {
        if self.descriptors.is_empty() {
            return Ok(());
        }
        writer.write_all(&[Self::id()])?;
        write::unsigned(writer, self.descriptors.len() as u64)?;

        for x in self.descriptors {
            x.encode(writer)?;
        }

        Ok(())
    }

    fn id() -> u8 {
        0x06
    }
}
