use crate::Error;
use crate::ExternalKind;
use crate::ValidationError;
use leb128::write;
use std::io::Write;

#[derive(Default)]
pub struct ImportSection {
    imports: Vec<(String, String, ExternalKind)>,
}

impl ImportSection {
    pub fn add_import<T: Into<String>>(
        &mut self,
        module_name: T,
        external_name: T,
        external_kind: ExternalKind,
    ) -> usize {
        let mod_name = module_name.into();
        let extern_name = external_name.into();
        self.imports.push((mod_name, extern_name, external_kind));

        self.imports.len() - 1
    }

    pub fn count(&self) -> usize {
        self.imports.len()
    }

    pub(crate) fn validate(&self) -> Result<(), ValidationError> {
        if self.imports.len() > u32::MAX as usize {
            return Err(ValidationError::ArrayOverflow);
        }

        for i in 0..self.imports.len() {
            let (base_ns, base_name, base_kind) = &self.imports[i];
            if i + 1 < self.imports.len() {
                for j in i + 1..self.imports.len() {
                    let (cmp_ns, cmp_name, cmp_kind) = &self.imports[j];
                    if base_ns == cmp_ns && base_name == cmp_name && base_kind == cmp_kind {
                        return Err(ValidationError::Duplicate);
                    }
                }
            }

            match base_kind {
                ExternalKind::Memory(descriptor) | ExternalKind::Table(descriptor) => {
                    descriptor.validate()?;
                }
                ExternalKind::Global(descriptor) => {
                    if descriptor.is_mut() {
                        return Err(ValidationError::MutatableImport);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn compile(self, writer: &mut impl Write) -> Result<(), Error> {
        if self.imports.is_empty() {
            return Ok(());
        };
        let mut buff = Vec::new();
        writer.write_all(&[Self::id()])?;
        write::unsigned(&mut buff, self.imports.len() as u64)?;

        for (module_name, external_name, kind) in self.imports {
            write::unsigned(&mut buff, module_name.len() as u64)?;
            (&mut buff).write_all(module_name.as_bytes())?;

            write::unsigned(&mut buff, external_name.len() as u64)?;
            (&mut buff).write_all(external_name.as_bytes())?;

            kind.encode(&mut buff)?;
        }
        write::unsigned(writer, buff.len() as u64)?;
        writer.write_all(&buff)?;
        Ok(())
    }

    fn id() -> u8 {
        0x02
    }
}
