use crate::indices::TypeIndex;

use super::ValType;

#[derive(Debug, Clone)]
pub enum ImportKind {
    Func(TypeIndex),
    Table { min: u32, max: Option<u32> },
    Memory { min: u32, max: Option<u32> },
    Global { val_type: ValType, mutable: bool },
}

#[derive(Debug)]
pub struct Import {
    pub(crate) import_kind: ImportKind,
    pub(crate) namespace: String,
    pub(crate) external_name: String,
}

impl Import {
    pub fn new(namespace: String, external_name: String, import_kind: ImportKind) -> Self {
        Self {
            namespace,
            external_name,
            import_kind,
        }
    }
}
