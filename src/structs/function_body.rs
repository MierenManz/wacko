use crate::Error;
use crate::Instruction;
use crate::ValType;
use leb128::write;
use std::io::Write;

pub struct FnBody {
    /// `Vec<(count, ValType)>`
    locals: Vec<(u32, ValType)>,
    instructions: Vec<Instruction>,
}

impl FnBody {
    pub fn new() -> Self {
        Self {
            locals: Vec::new(),
            instructions: Vec::new(),
        }
    }

    pub fn add_local(&mut self, kind: ValType) {
        let index = self.locals.len() - 1;

        if self.locals[index].1 == kind {
            self.locals[index].0 += 1;
            return;
        }

        self.locals.push((0, kind));
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn add_instructions<T>(&mut self, instructions: T)
    where
        T: IntoIterator<Item = Instruction>,
    {
        self.instructions.extend(instructions)
    }

    pub fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        let mut buff: Vec<u8> = Vec::with_capacity(self.locals.len() * 2);

        written += write::unsigned(&mut buff, self.locals.len() as u64)?;

        for local in self.locals {
            written += write::unsigned(&mut buff, local.0 as u64)?;
            written += (&mut buff).write(&[local.1.into()])?;
        }
        written += self.instructions.len();
        
        written += write::unsigned(writer, written as u64)?;
        writer.write_all(&buff)?;

        for x in self.instructions {
            writer.write(&[x.into()])?;
        }

        Ok(written)
    }
}
