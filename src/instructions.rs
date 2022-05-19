pub enum Instruction {}

impl From<Instruction> for u8 {
    fn from(_instr: Instruction) -> Self {
        0
    }
}
