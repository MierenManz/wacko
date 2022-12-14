use super::components::ExternalKind;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct AnyIndex {
    pub(crate) kind: ExternalKind,
    pub(crate) rc: Rc<Cell<u32>>,
}

macro_rules! create_index {
    ($idx:ident) => {
        #[derive(Debug, Clone)]
        pub struct $idx {
            pub(crate) inner: Rc<Cell<u32>>,
        }

        impl $idx {
            #[inline]
            pub(crate) fn new(value: u32) -> Self {
                Self {
                    inner: Rc::new(Cell::new(value)),
                }
            }
        }
    };

    ($idx:ident, $kind:expr) => {
        #[derive(Debug, Clone)]
        pub struct $idx {
            pub(crate) inner: Rc<Cell<u32>>,
        }

        impl $idx {
            #[inline]
            pub(crate) fn new(value: u32) -> Self {
                Self {
                    inner: Rc::new(Cell::new(value)),
                }
            }
        }

        impl From<$idx> for AnyIndex {
            fn from(index: $idx) -> Self {
                Self {
                    kind: $kind,
                    rc: index.inner,
                }
            }
        }
    };
}

create_index!(TypeIndex);
create_index!(FnIndex, ExternalKind::Func);
create_index!(TableIndex, ExternalKind::Table);
create_index!(MemoryIndex, ExternalKind::Memory);
create_index!(GlobalIndex, ExternalKind::Global);
