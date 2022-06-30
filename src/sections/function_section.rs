use crate::Error;
use leb128::write;
use std::io::Write;

/// Editor Note: This struct relies ony on external validation.
pub struct FunctionSection {
    declarations: Vec<u32>,
}

impl FunctionSection {
    pub fn new() -> Self {
        Self {
            declarations: Vec::new(),
        }
    }

    pub fn add_fn_decl(&mut self, type_index: u32) -> usize {
        self.declarations.push(type_index);
        self.declarations.len() - 1
    }

    pub fn compile(self, writer: &mut impl Write) -> Result<(), Error> {
        writer.write_all(&[Self::id()])?;
        let mut buff = Vec::new();
        write::unsigned(&mut buff, self.declarations.len() as u64)?;
        for x in self.declarations {
            write::unsigned(&mut buff, x as u64)?;
        }

        write::unsigned(writer, buff.len() as u64)?;
        writer.write_all(&buff)?;
        Ok(())
    }

    fn id() -> u8 {
        0x03
    }
}

impl Default for FunctionSection {
    fn default() -> Self {
        Self::new()
    }
}
