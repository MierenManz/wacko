use crate::Error;
use crate::Section;
use leb128::write;
use std::io::Write;

#[repr(u8)]
#[derive(Clone, Copy)]
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

    pub fn remove_export(&mut self, index: usize) -> bool {
        if self.exports.len() < index {
            return false;
        }

        self.exports.remove(index);

        true
    }
}

impl Section for ExportSection {
    fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        written += writer.write(&[self.id()])?;
        written += write::unsigned(writer, self.count() as u64)?;
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

        Ok(written)
    }

    fn id(&self) -> u8 {
        0x07
    }

    fn count(&self) -> usize {
        self.exports.len()
    }
}

impl Default for ExportSection {
    fn default() -> Self {
        Self::new()
    }
}
