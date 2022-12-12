use crate::validator::Validator;
use crate::Error;
use crate::FnBody;
use crate::ValidationError;
use leb128::write;
use std::io::Write;

#[derive(Default)]
pub struct CodeSection<'a> {
    code_blocks: Vec<FnBody<'a>>,
}

impl<'a> CodeSection<'a> {
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

    pub fn compile(self, writer: &mut impl Write) -> Result<(), Error> {
        if self.code_blocks.is_empty() {
            return Ok(());
        }

        writer.write_all(&[Self::id()])?;
        let mut buff = Vec::new();
        write::unsigned(&mut buff, self.code_blocks.len() as u64)?;
        for fn_body in self.code_blocks {
            fn_body.compile(&mut buff)?;
        }
        write::unsigned(writer, buff.len() as u64)?;
        writer.write_all(&buff)?;

        Ok(())
    }

    pub(crate) fn count(&self) -> usize {
        self.code_blocks.len()
    }

    pub(crate) fn validate(&self) -> Result<(), ValidationError> {
        for x in &self.code_blocks {
            let mut validator = Validator::new();
            x.validate(&mut validator)?;

            if !validator.is_empty() {
                return Err(ValidationError::StackNotEmpty);
            }
        }

        Ok(())
    }

    fn id() -> u8 {
        0x0A
    }
}
