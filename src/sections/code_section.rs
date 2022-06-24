use crate::Error;
use crate::FnBody;
// use crate::ValidationError;
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

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn encode() {
        let mut buff = Vec::new();
        let mut fn_body = FnBody::new(vec![ValType::I32], vec![ValType::I32]);
        fn_body.add_instructions([
            Instruction::LocalGet(0),
            Instruction::LocalGet(1),
            Instruction::I32Add,
            Instruction::End,
        ]);
        let mut code = CodeSection::new();
        code.add_fn_body(fn_body.clone());
        code.add_fn_body(fn_body);
        code.compile(&mut buff).unwrap();
        assert_eq!(
            buff,
            vec![
                0x0A, 0x02, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6A, 0x0B, 0x07, 0x00, 0x20, 0x00, 0x20,
                0x01, 0x6A, 0x0B
            ]
        )
    }
}
