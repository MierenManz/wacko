use crate::Error;
use crate::Section;
use leb128::write;
use std::io::Write;

/// Editor Note: This struct relies ony on external validation.
pub struct FunctionSection {
    declarations: Vec<u32>,
}

impl FunctionSection {
    pub fn new() -> Self {
        Self {
            declarations: Vec::new(),
        }
    }
    pub fn add_fn_decl<T: Into<String>>(&mut self, type_index: u32) -> usize {
        self.declarations.push(type_index);
        self.declarations.len() - 1
    }

    pub fn remove_fn_decl(&mut self, index: usize) -> bool {
        if self.declarations.len() < index {
            return false;
        }

        self.declarations.remove(index);
        true
    }
}

impl Section for FunctionSection {
    fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        written += writer.write(&[self.id()])?;
        written += write::unsigned(writer, self.count() as u64)?;
        for x in self.declarations {
            written += write::unsigned(writer, x as u64)?;
        }

        writer.flush()?;
        Ok(written)
    }

    fn id(&self) -> u8 {
        0x03
    }

    fn count(&self) -> usize {
        self.declarations.len()
    }
}

impl Default for FunctionSection {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Add for FunctionSection {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut declarations = Vec::with_capacity(self.declarations.len() + rhs.declarations.len());
        declarations.extend(self.declarations);
        declarations.extend(rhs.declarations);

        Self {
            declarations
        }
    }
}