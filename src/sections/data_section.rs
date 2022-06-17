use crate::Error;
use crate::Instruction;
use leb128::write;
use std::io::Write;

pub struct DataSection {
    data: Vec<(u32, i32, Vec<u8>)>,
}

impl DataSection {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add_data(&mut self, memory_idx: u32, offset: i32, data: Vec<u8>) {
        self.data.push((memory_idx, offset, data));
    }

    pub fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;

        written += writer.write(&[Self::id()])?;
        written += write::unsigned(writer, self.data.len() as u64)?;

        for (index, offset, data) in self.data {
            written += write::unsigned(writer, index as u64)?;
            written += Instruction::I32Const(offset).encode(writer)?;
            written += write::unsigned(writer, data.len() as u64)?;
            writer.write_all(&data)?;
            written += data.len();
        }

        Ok(written)
    }

    pub fn id() -> u8 {
        0x0B
    }
}

impl Default for DataSection {
    fn default() -> Self {
        Self::new()
    }
}
