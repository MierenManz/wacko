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
        if self.exports.is_empty() {
            return Ok(0);
        }
        writer.write(&[Self::id()])?;
        let mut buff = Vec::new();
        write::unsigned(&mut buff, self.exports.len() as u64)?;
        for (name, kind) in self.exports {
            write::unsigned(&mut buff, name.len() as u64)?;
            (&mut buff).write(name.as_bytes())?;
            let v = match kind {
                ExportKind::Function(id) => id,
                ExportKind::Table(id) => id,
                ExportKind::Memory(id) => id,
                ExportKind::Global(id) => id,
            };

            (&mut buff).write(&[kind.into()])?;
            write::unsigned(&mut buff, v as u64)?;
        }

        write::unsigned(writer, buff.len() as u64)?;
        writer.write_all(&buff)?;

        Ok(buff.len() + 1)
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
