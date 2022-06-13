use crate::FnBody;
use crate::Section;
use std::io::Write;
use crate::Error;
use leb128::write;

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
}

impl Section for CodeSection {
    fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        written += writer.write(&[self.id()])?;
        written += write::unsigned(writer, self.code_blocks.len() as u64)?;
        for fn_body in self.code_blocks {
            written += fn_body.compile(writer)?;
        }

        Ok(written)
    }

    fn id(&self) -> u8 {
        0x0A
    }

    fn count(&self) -> usize {
        0
    }
}

impl Default for CodeSection {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Add for CodeSection {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut code_blocks = Vec::with_capacity(self.code_blocks.len() + rhs.code_blocks.len());
        code_blocks.extend(self.code_blocks);
        code_blocks.extend(rhs.code_blocks);

        Self {
            code_blocks
        }
    }
}