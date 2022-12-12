use crate::ValType;
use crate::ValidationError;

pub struct Validator {
    depth: usize,
    stack: Vec<ValType>,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            depth: 1,
            stack: Vec::new(),
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn increase_depth(&mut self, offset: usize) {
        self.depth += offset;
    }
    pub fn decrease_depth(&mut self, offset: usize) {
        self.depth -= offset;
    }

    pub fn pop(&mut self, expect: ValType) -> Result<(), ValidationError> {
        if expect == ValType::Func || expect == ValType::FuncRef {
            return Err(ValidationError::InvalidType);
        }

        if expect == ValType::Void && self.stack.is_empty() {
            return Ok(());
        }

        if expect != self.stack.pop().unwrap() {
            return Err(ValidationError::InvalidType);
        }

        Ok(())
    }

    pub fn push(&mut self, value_type: ValType) {
        self.stack.push(value_type);
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}
