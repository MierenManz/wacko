use crate::Error;
use crate::Instruction;
use crate::ValType;
use leb128::write;
use std::io::Write;

#[derive(Clone)]
pub struct FnBody<'a> {
    fn_type: (Vec<ValType>, Vec<ValType>),
    /// `Vec<(count, ValType)>`
    locals: Vec<(u32, ValType)>,
    instructions: Vec<Instruction<'a>>,
}

impl<'a> FnBody<'a> {
    pub fn new(arguments: Vec<ValType>, return_type: Vec<ValType>) -> Self {
        let returns = if return_type.is_empty() {
            vec![ValType::Void]
        } else {
            return_type
        };
        Self {
            fn_type: (arguments, returns),
            locals: Vec::new(),
            instructions: Vec::new(),
        }
    }

    pub fn add_local(&mut self, kind: ValType) {
        if self.locals.is_empty() {
            self.locals.push((1, kind));
            return;
        }

        let index = self.locals.len() - 1;
        if self.locals[index].1 == kind {
            self.locals[index].0 += 1;
            return;
        }

        self.locals.push((1, kind));
        return;
    }

    pub fn add_instruction(&mut self, instruction: Instruction<'a>) {
        self.instructions.push(instruction);
    }

    pub fn add_instructions<T>(&mut self, instructions: T)
    where
        T: IntoIterator<Item = Instruction<'a>>,
    {
        self.instructions.extend(instructions)
    }

    pub(crate) fn get_fn_type(&self) -> (&[ValType], &[ValType]) {
        (&self.fn_type.0, &self.fn_type.1)
    }

    pub(crate) fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut buff = Vec::with_capacity(self.locals.len() * 3 + self.instructions.len());

        write::unsigned(&mut buff, self.locals.len() as u64)?;
        for local in self.locals {
            write::unsigned(&mut buff, local.0 as u64)?;
            (&mut buff).write(&[local.1.into()])?;
        }

        for x in self.instructions {
            x.encode(&mut buff)?;
        }

        write::unsigned(writer, buff.len() as u64)?;
        writer.write_all(&buff)?;
        Ok(buff.len() + 1)
    }

    pub(crate) fn optimize(&mut self) {}
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn encode_fn_body() {
        let mut buff = Vec::new();
        let args = vec![ValType::I32, ValType::I32];
        let return_type = vec![ValType::I32];
        let mut fn_body = FnBody::new(args, return_type);
        fn_body.add_instructions(vec![
            Instruction::LocalGet(0),
            Instruction::LocalGet(1),
            Instruction::I32Add,
            Instruction::End,
        ]);
        fn_body.compile(&mut buff).unwrap();
        assert_eq!(buff, vec![0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6A, 0x0B]);
    }
}
