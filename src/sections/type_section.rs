use crate::Error;
use crate::Section;
use crate::ValType;
use crate::ValidationError;
use leb128::write;

use std::io::Write;

#[derive(Debug)]
pub struct TypeSection {
    type_definitions: Vec<(Vec<ValType>, Vec<ValType>)>,
}

impl TypeSection {
    pub fn new() -> Self {
        Self {
            type_definitions: Vec::new(),
        }
    }

    pub fn add_type_def<T: Into<Vec<ValType>>>(&mut self, params: T, returns: T) -> usize {
        self.type_definitions.push((params.into(), returns.into()));
        self.type_definitions.len() - 1
    }

    pub fn remove_type_def(&mut self, index: usize) -> bool {
        if index < self.type_definitions.len() {
            return false;
        }

        self.type_definitions.remove(index);
        true
    }

    pub(crate) fn validate(&self) -> Result<(), ValidationError> {
        if self.type_definitions.len() > u32::MAX as usize {
            return Err(ValidationError::ArrayOverflow);
        }

        for (params, returns) in &self.type_definitions {
            if params.len() > u32::MAX as usize || returns.len() > u32::MAX as usize {
                return Err(ValidationError::ArrayOverflow);
            }

            if returns.len() == 0 {
                return Err(ValidationError::ArrayTooLittleElements);
            }
        }

        Ok(())
    }
}

impl Section for TypeSection {
    fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        writer.write(&[self.id()])?;
        written += write::unsigned(writer, self.type_definitions.len() as u64)?;
        
        for (params, results) in self.type_definitions {
            writer.write(&[0x60])?;

            write::unsigned(writer, params.len() as u64)?;
            for x in params {
                write::unsigned(writer, x as u64)?;
            }

            write::unsigned(writer, results.len() as u64)?;
            for x in results {
                write::unsigned(writer, x as u64)?;
            }
        }

        writer.flush()?;

        Ok(written)
    }

    fn id(&self) -> u8 {
        0x01
    }

    fn declaration_count(&self) -> usize {
        self.type_definitions.len()
    }
}
