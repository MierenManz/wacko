use crate::Error;
use crate::ExternalKind;
use crate::Section;
use crate::ValidationError;
use leb128::write;
use std::io::Write;

pub struct ImportSection {
    imports: Vec<(String, String, ExternalKind)>,
}

impl ImportSection {
    pub fn new() -> Self {
        Self {
            imports: Vec::new(),
        }
    }

    pub fn add_import<T: Into<String>>(
        &mut self,
        module_name: T,
        external_name: T,
        kind: ExternalKind,
    ) -> usize {
        self.imports
            .push((module_name.into(), external_name.into(), kind));
        self.imports.len() - 1
    }

    pub fn remove_import(&mut self, index: usize) -> bool {
        if self.imports.len() < index {
            return false;
        }

        self.imports.remove(index);
        true
    }

    pub(crate) fn validate(&self) -> Result<(), ValidationError> {
        if self.imports.len() > u32::MAX as usize {
            return Err(ValidationError::ArrayOverflow);
        }

        for (_, _, kind) in &self.imports {
            match kind {
                ExternalKind::Memory(mem_descriptor) | ExternalKind::Table(mem_descriptor) => {
                    if let Some(v) = mem_descriptor.maximum {
                        if v < mem_descriptor.minimum {
                            return Err(ValidationError::InvalidMemorySetting);
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl Section for ImportSection {
    fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        if self.count() == 0 {
            return Ok(0);
        }

        let mut written = 0;
        written += writer.write(&[self.id()])?;
        written += write::unsigned(writer, self.count() as u64)?;

        for (module_name, external_name, kind) in self.imports {
            written += write::unsigned(writer, module_name.len() as u64)?;
            written += writer.write(module_name.as_bytes())?;

            written += write::unsigned(writer, external_name.len() as u64)?;
            written += writer.write(external_name.as_bytes())?;

            written += kind.encode(writer)?;
        }

        Ok(written)
    }

    fn id(&self) -> u8 {
        0x02
    }

    fn count(&self) -> usize {
        self.imports.len()
    }
}
