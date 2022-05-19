use crate::Error;
use crate::ResizableLimits;
use crate::ValType;
use leb128::write;
use std::io::Write;

#[derive(Copy, Clone)]
pub enum ExternalKind {
    Function(u32),
    Table(ResizableLimits),
    Memory(ResizableLimits),
    Global(ValType, bool),
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
            Self::Global(v, is_mut) => {
                written += writer.write(&[v.into(), is_mut as u8])?;
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
            ExternalKind::Global(_, _) => 0x03,
        }
    }
}
