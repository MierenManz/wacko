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

    pub fn add_export(
        &mut self,
        export_kind: ExportKind,
        export_name: &str,
    ) -> Result<(), ValidationError> {
        for (name, kind) in &self.exports {
            if *kind == export_kind && *name == export_name {
                return Err(ValidationError::Duplicate);
            }
        }
        self.exports.push((export_name.to_string(), export_kind));

        Ok(())
    }

    pub fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        written += writer.write(&[Self::id()])?;
        written += write::unsigned(writer, self.exports.len() as u64)?;
        for (name, kind) in self.exports {
            written += write::unsigned(writer, name.len() as u64)?;
            written += writer.write(name.as_bytes())?;
            let v = match kind {
                ExportKind::Function(id) => id,
                ExportKind::Table(id) => id,
                ExportKind::Memory(id) => id,
                ExportKind::Global(id) => id,
            };

            written += writer.write(&[kind.into()])?;
            written += write::unsigned(writer, v as u64)?;
        }

        writer.flush()?;
        Ok(written)
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
