use crate::ResizableLimits;
use crate::ValType;

#[derive(Copy, Clone)]
pub enum ExternalKind {
    Function(u32),
    Table(ResizableLimits),
    Memory(ResizableLimits),
    Global(ValType, bool),
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
