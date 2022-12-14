use crate::indices::TypeIndex;
use crate::instructions::Instruction;

use super::ValType;

#[derive(Debug)]
pub struct Func<'a> {
    pub(crate) instructions: Vec<Instruction<'a>>,
    pub(crate) locals: Vec<ValType>,
    pub(crate) type_idx: TypeIndex,
}

impl<'a> Func<'a> {
    pub fn new(type_idx: TypeIndex) -> Self {
        Self {
            instructions: Vec::new(),
            locals: Vec::new(),
            type_idx,
        }
    }

    pub fn add_local(&mut self, kind: ValType) -> u32 {
        self.locals.push(kind);
        (self.locals.len() - 1) as u32
    }

    pub fn add_instructions(&mut self, instr: &[Instruction<'a>]) {
        self.instructions.extend(instr);
    }
}
