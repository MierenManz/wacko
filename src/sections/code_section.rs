use crate::Instruction;
use crate::Section;

pub struct CodeSection {
    code_blocks: Vec<Vec<Instruction>>
}