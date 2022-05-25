use crate::Instruction;
use crate::Section;
use crate::FnBody;

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

// impl Section for CodeSection {}

impl Default for CodeSection {
    fn default() -> Self {
        Self::new()
    }
}
