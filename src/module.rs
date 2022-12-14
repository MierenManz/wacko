use crate::components::Func;
use crate::components::Global;
use crate::components::Import;
use crate::components::ImportKind;
use crate::components::Memory;
use crate::components::Table;
use crate::components::Type;
use crate::components::Export;
use crate::indices::FnIndex;
use crate::indices::MemoryIndex;
use crate::indices::TableIndex;
use crate::indices::TypeIndex;
use std::rc::Rc;
use std::cell::Cell;

#[derive(Debug)]
pub struct Module<'a> {
    pub(crate) types: Vec<Type>,
    pub(crate) imports: Vec<Import>,
    // /// Includes code section
    pub(crate) funcs: Vec<Func<'a>>,
    // /// Includes element section
    pub(crate) tables: Vec<Table>,
    // /// Includes data section
    pub(crate) memory: Option<Memory>,
    pub(crate) globals: Vec<Global>,
    pub(crate) exports: Vec<Export>,
    /// Refers to a func in `Module.funcs`
    pub(crate) start_fn: Option<u32>,
}

impl<'a> Module<'a> {
    /// `validate` & `optimize` are currently ignored
    pub fn new(_validate: bool, _optimize: bool) -> Self {
        Module {
            types: Vec::new(),
            imports: Vec::new(),
            funcs: Vec::new(),
            tables: Vec::new(),
            memory: None,
            globals: Vec::new(),
            exports: Vec::new(),
            start_fn: None,
        }
    }

    pub fn add_type(&mut self, fn_type: Type) -> TypeIndex {
        self.types.push(fn_type);
        let idx = (self.types.len() - 1) as u32;
        TypeIndex { inner: Rc::new(Cell::new(idx)) }
    }

    pub fn add_import(&mut self, import: Import) {
        self.imports.push(import);
        // Reordering of `funcs` `tables` `memory` and `globals`
        // happens when compiling before validating Not here
    }

    pub fn add_func(&mut self, func: Func<'a>) -> FnIndex {
        self.funcs.push(func);
        let idx = (self.funcs.len() - 1) as u32;
        FnIndex { inner: Rc::new(Cell::new(idx)) }
    }

    pub fn add_table(&mut self, table: Table) -> TableIndex {
        self.tables.push(table);
        let idx = (self.tables.len() - 1) as u32;
        TableIndex { inner: Rc::new(Cell::new(idx)) }
    }
    
    pub fn set_mem(&mut self, memory: Memory) -> MemoryIndex {
        let old = self.memory;
        self.memory = Some(memory);

        MemoryIndex { inner: Rc::new(Cell::new(1)) }
    }
}
