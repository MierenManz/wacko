use crate::indices::AnyIndex;
use crate::indices::FnIndex;
use crate::indices::GlobalIndex;
use crate::indices::MemoryIndex;
use crate::indices::TableIndex;

#[derive(Copy, Clone, Debug)]
pub(crate) enum ExternalKind {
    Func = 0x00,
    Table = 0x01,
    Memory = 0x02,
    Global = 0x03,
}

impl From<ExternalKind> for u8 {
    fn from(kind: ExternalKind) -> Self {
        kind as Self
    }
}

#[derive(Debug)]
pub struct Export {
    pub(crate) export_name: String,
    pub(crate) index: AnyIndex,
}

impl Export {
    pub fn func(idx: FnIndex, export_name: String) -> Self {
        Self { index: idx.into(), export_name }
    }
    pub fn table(idx: TableIndex, export_name: String) -> Self {
        Self { index: idx.into(), export_name }
    }
    pub fn memory(idx: MemoryIndex, export_name: String) -> Self {
        Self { index: idx.into(), export_name }
    }
    pub fn global(idx: GlobalIndex, export_name: String) -> Self {
        Self { index: idx.into(), export_name }
    }
}