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

    pub fn compile(self, writer: &mut impl Write) -> Result<(), Error> {
        if self.data.is_empty() {
            return Ok(());
        }
        let mut buff: Vec<u8> = Vec::new();

        writer.write_all(&[Self::id()])?;
        write::unsigned(&mut buff, self.data.len() as u64)?;

        for (mem_idx, offset, slice) in self.data {
            write::unsigned(&mut buff, mem_idx as u64)?;
            Instruction::I32Const(offset).encode(&mut buff)?;
            (&mut buff).write_all(&[Self::id()])?;
            write::unsigned(&mut buff, slice.len() as u64)?;
            (&mut buff).write_all(&slice)?;
        }

        write::unsigned(writer, buff.len() as u64)?;
        writer.write_all(&buff)?;

        Ok(())
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
