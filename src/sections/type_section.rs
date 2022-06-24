use crate::Error;
use crate::RequiredSection;
use crate::ValType;
use crate::ValidationError;
use leb128::write;
use std::io::Write;

#[derive(Debug)]
pub struct TypeSection {
    definitions: Vec<(Vec<ValType>, Vec<ValType>)>,
}

impl TypeSection {
    pub fn new() -> Self {
        Self {
            definitions: Vec::new(),
        }
    }

    pub fn add_type_def<T: Into<Vec<ValType>>>(&mut self, params: T, return_type: T) -> usize {
        let new_definition = (params.into(), return_type.into());
        for i in 0..self.definitions.len() {
            let definition = &self.definitions[i];
            if definition == &new_definition {
                return i;
            }
        }

        self.definitions.push(new_definition);
        self.definitions.len() - 1
    }

    pub(crate) fn validate(&self) -> Result<(), ValidationError> {
        if self.definitions.is_empty() {
            return Err(ValidationError::SectionMissing(
                RequiredSection::CodeSection,
            ));
        }
        if self.definitions.len() > u32::MAX as usize {
            return Err(ValidationError::ArrayOverflow);
        }

        for (params, returns) in &self.definitions {
            if params.len() > u32::MAX as usize || returns.len() > u32::MAX as usize {
                return Err(ValidationError::ArrayOverflow);
            }

            if returns.is_empty() {
                return Err(ValidationError::ArrayTooLittleElements);
            }
        }

        Ok(())
    }

    pub fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = writer.write(&[Self::id()])?;
        let mut buff = Vec::new();
        write::unsigned(&mut buff, self.definitions.len() as u64)?;
        for (params, result) in self.definitions {
            (&mut buff).write(&[ValType::Func.into()])?;
            write::unsigned(&mut buff, params.len() as u64)?;
            for t in params {
                (&mut buff).write(&[t.into()])?;
            }

            write::unsigned(&mut buff, result.len() as u64)?;
            for t in result {
                (&mut buff).write(&[t.into()])?;
            }
        }
        write::unsigned(writer, buff.len() as u64)?;
        writer.write_all(&buff)?;
        written += buff.len();
        Ok(written)
    }

    fn id() -> u8 {
        0x01
    }
}

impl Default for TypeSection {
    fn default() -> Self {
        Self::new()
    }
}
