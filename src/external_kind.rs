use crate::Error;
use crate::GlobalDescriptor;
use crate::ResizableLimits;
use crate::ValType;
use leb128::write;
use std::io::Write;

#[derive(Copy, Clone)]
pub enum ExternalKind {
    Function(u32),
    Table(ResizableLimits),
    Memory(ResizableLimits),
    Global(GlobalDescriptor),
}

impl ExternalKind {
    pub(crate) fn encode(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        written += writer.write(&[self.into()])?;
        match self {
            Self::Function(sig_idx) => {
                written += write::unsigned(writer, sig_idx as u64)?;
            }
            Self::Table(mem_desc) => {
                written += writer.write(&[ValType::FuncRef.into()])?;
                written += mem_desc.encode(writer)?;
            }
            Self::Memory(mem_desc) => {
                written += mem_desc.encode(writer)?;
            }
            Self::Global(descriptor) => {
                written += descriptor.encode(writer)?;
            }
        }

        Ok(written)
    }
}

impl From<ExternalKind> for u8 {
    fn from(kind: ExternalKind) -> Self {
        match kind {
            ExternalKind::Function(_) => 0x00,
            ExternalKind::Table(_) => 0x01,
            ExternalKind::Memory(_) => 0x02,
            ExternalKind::Global(_) => 0x03,
        }
    }
}
