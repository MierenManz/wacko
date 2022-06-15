use crate::Error;
use crate::FnBody;
use crate::ValidationError;
use leb128::write;
use std::io::Write;

pub struct CodeSection {
    code_blocks: Vec<FnBody>,
}

impl CodeSection {
    pub fn new() -> Self {
        Self {
            code_blocks: Vec::new(),
        }
    }

    pub fn add_fn_body(&mut self, code_block: FnBody) {
        self.code_blocks.push(code_block);
    }

    pub fn optimize(&mut self) {}
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }

    pub fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        written += writer.write(&[Self::id()])?;
        written += write::unsigned(writer, self.code_blocks.len() as u64)?;
        for fn_body in self.code_blocks {
            written += fn_body.compile(writer)?;
        }

        Ok(written)
    }

    fn id() -> u8 {
        0x0A
    }
}

impl Default for CodeSection {
    fn default() -> Self {
        Self::new()
    }
}
