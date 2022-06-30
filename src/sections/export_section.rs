use crate::Error;
use crate::ValidationError;
use leb128::write;
use std::io::Write;

#[derive(Clone, Copy, PartialEq)]
pub enum ExportKind {
    Function(u32),
    Table(u32),
    Memory(u32),
    Global(u32),
}

impl From<ExportKind> for u8 {
    fn from(kind: ExportKind) -> Self {
        match kind {
            ExportKind::Function(_) => 0x00,
            ExportKind::Table(_) => 0x01,
            ExportKind::Memory(_) => 0x02,
            ExportKind::Global(_) => 0x03,
        }
    }
}

pub struct ExportSection {
    exports: Vec<(String, ExportKind)>,
}

impl ExportSection {
    pub fn new() -> Self {
        Self {
            exports: Vec::new(),
        }
    }

    pub fn add_export(&mut self, export_kind: ExportKind, export_name: &str) -> usize {
        self.exports.push((export_name.to_string(), export_kind));

        self.exports.len() - 1
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        for i in 0..self.exports.len() {
            let (base_name, base_kind) = &self.exports[i];
            if i + 1 < self.exports.len() {
                for j in i + 1..self.exports.len() {
                    let (cmp_name, cmp_kind) = &self.exports[j];
                    if base_name == cmp_name && base_kind == cmp_kind {
                        return Err(ValidationError::Duplicate);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn compile(self, writer: &mut impl Write) -> Result<(), Error> {
        if self.exports.is_empty() {
            return Ok(());
        }
        writer.write_all(&[Self::id()])?;
        let mut buff = Vec::new();
        write::unsigned(&mut buff, self.exports.len() as u64)?;
        for (name, kind) in self.exports {
            write::unsigned(&mut buff, name.len() as u64)?;
            (&mut buff).write_all(name.as_bytes())?;
            let v = match kind {
                ExportKind::Function(id) => id,
                ExportKind::Table(id) => id,
                ExportKind::Memory(id) => id,
                ExportKind::Global(id) => id,
            };

            (&mut buff).write_all(&[kind.into()])?;
            write::unsigned(&mut buff, v as u64)?;
        }

        write::unsigned(writer, buff.len() as u64)?;
        writer.write_all(&buff)?;

        Ok(())
    }

    pub fn count(&self) -> usize {
        self.exports.len()
    }

    fn id() -> u8 {
        0x07
    }
}

impl Default for ExportSection {
    fn default() -> Self {
        Self::new()
    }
}
