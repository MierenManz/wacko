use crate::Error;
use crate::RequiredSection;
use crate::Section;
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
        if self.definitions.len() == 0 {
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
}

impl Section for TypeSection {
    fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        written += writer.write(&[self.id()])?;
        written += write::unsigned(writer, self.definitions.len() as u64)?;

        for (params, results) in self.definitions {
            written += writer.write(&[ValType::Func.into()])?;

            written += write::unsigned(writer, params.len() as u64)?;
            for x in params {
                written += writer.write(&[x.into()])?;
            }

            written += write::unsigned(writer, results.len() as u64)?;
            for x in results {
                written += writer.write(&[x.into()])?;
            }
        }

        writer.flush()?;

        Ok(written)
    }

    fn id(&self) -> u8 {
        0x01
    }
}

impl Default for TypeSection {
    fn default() -> Self {
        Self::new()
    }
}
