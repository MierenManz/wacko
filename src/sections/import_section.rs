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
    ) {
        self.imports
            .push((module_name.into(), external_name.into(), kind));
    }

    pub(crate) fn validate(&self) -> Result<(), ValidationError> {
        if self.imports.len() > u32::MAX as usize {
            return Err(ValidationError::ArrayOverflow);
        }

        for (_, _, kind) in &self.imports {
            match kind {
                ExternalKind::Memory(descriptor) | ExternalKind::Table(descriptor) => {
                    descriptor.validate()?;
                }
                ExternalKind::Global(descriptor) => {
                    descriptor.validate()?;
                    if descriptor.is_mut() {
                        return Err(ValidationError::MutatableImport);
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

        writer.flush()?;
        Ok(written)
    }

    fn id(&self) -> u8 {
        0x02
    }

    fn count(&self) -> usize {
        self.imports.len()
    }
}

impl Default for ImportSection {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Add for ImportSection {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut imports = Vec::with_capacity(self.imports.len() + rhs.imports.len());
        imports.extend(self.imports);
        imports.extend(rhs.imports);

        Self {
            imports
        }
    }
}