use crate::Error;
use crate::Section;
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
}

impl Section for FunctionSection {
    fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        written += writer.write(&[self.id()])?;
        written += write::unsigned(writer, self.declarations.len() as u64)?;
        for x in self.declarations {
            written += write::unsigned(writer, x as u64)?;
        }

        writer.flush()?;
        Ok(written)
    }

    fn id(&self) -> u8 {
        0x03
    }
}

impl Default for FunctionSection {
    fn default() -> Self {
        Self::new()
    }
}
