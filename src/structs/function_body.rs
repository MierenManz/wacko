use crate::Instruction;
use crate::ValType;
use crate::Error;
use std::io::Write;
use leb128::write;

pub struct FnBody {
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

        for local in self.locals {
            write::unsigned(&mut buff, local.0 as u64)?;
            (&mut buff).write(&[local.1.into()])?;
        }

        for instr in self.instructions {
            // (&mut buff).write(instr.encode());
        }

        Ok(0)
    }
}
