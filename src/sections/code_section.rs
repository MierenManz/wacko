use crate::Error;
use crate::FnBody;
// use crate::ValidationError;
use leb128::write;
use std::io::Write;

pub struct CodeSection<'a> {
    code_blocks: Vec<FnBody<'a>>,
}

impl<'a> CodeSection<'a> {
    pub fn new() -> Self {
        Self {
            code_blocks: Vec::new(),
        }
    }

    pub fn add_fn_body(&mut self, code_block: FnBody<'a>) {
        self.code_blocks.push(code_block);
    }

    pub fn optimize(self) -> Self {
        let mut bodies = Vec::new();
        for mut fn_body in self.code_blocks {
            fn_body.optimize();
            bodies.push(fn_body);
        }

        Self {
            code_blocks: bodies,
        }
    }
    // pub fn validate(&self) -> Result<(), ValidationError> {
    //     Ok(())
    // }

    pub fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        writer.write_all(&[Self::id()])?;
        let mut buff = Vec::new();
        write::unsigned(&mut buff, self.code_blocks.len() as u64)?;
        for fn_body in self.code_blocks {
            fn_body.compile(&mut buff)?;
        }
        write::unsigned(writer, buff.len() as u64)?;
        writer.write_all(&buff)?;

        Ok(buff.len() + 1)
    }

    fn id() -> u8 {
        0x0A
    }
}

impl Default for CodeSection<'_> {
    fn default() -> Self {
        Self::new()
    }
}
