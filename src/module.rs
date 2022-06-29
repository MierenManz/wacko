use crate::CodeSection;
use crate::DataSection;
use crate::ElementSection;
use crate::Error;
use crate::ExportKind;
use crate::ExportSection;
use crate::ExternalKind;
use crate::FnBody;
use crate::FunctionSection;
use crate::GlobalDescriptor;
use crate::GlobalSection;
use crate::ImportSection;
use crate::Memory;
use crate::MemorySection;
use crate::ResizableLimits;
use crate::Table;
use crate::TableSection;
use crate::TypeSection;
use crate::ValType;
use crate::ValidationError;
use std::io::Write;

const MAGIC: [u8; 8] = *b"\0asm\x01\0\0\0";

pub struct Module<'a> {
    optimize: bool,
    validate: bool,
    type_section: TypeSection,
    fn_section: FunctionSection,
    code_section: CodeSection<'a>,
    import_section: ImportSection,
    table_section: TableSection,
    memory_section: MemorySection,
    global_section: GlobalSection,
    export_section: ExportSection,
    element_section: ElementSection,
    data_section: DataSection,
}

impl<'a> Module<'a> {
    pub fn new(validate: bool) -> Self {
        Self {
            // Will be implemented later
            optimize: false,
            validate,
            type_section: Default::default(),
            fn_section: Default::default(),
            code_section: Default::default(),
            // All of these are optional technically
            import_section: Default::default(),
            table_section: Default::default(),
            memory_section: Default::default(),
            global_section: Default::default(),
            export_section: Default::default(),
            element_section: Default::default(),
            data_section: Default::default(),
        }
    }

    pub(crate) fn add_type<T: Into<Vec<ValType>>>(&mut self, params: T, return_type: T) -> usize {
        self.type_section.add_type_def(params, return_type)
    }

    pub(crate) fn add_fn_decl(&mut self, type_def: u32) -> u32 {
        self.fn_section.add_fn_decl(type_def) as u32
    }

    pub(crate) fn add_export(&mut self, export_kind: ExportKind, export_name: &str) -> u32 {
        self.export_section.add_export(export_kind, export_name) as u32
    }

    pub(crate) fn add_memory_descriptor(
        &mut self,
        descriptor: ResizableLimits,
        export_name: Option<&'_ str>,
    ) -> u32 {
        let mem_index = self.memory_section.add_descriptor(descriptor) as u32;
        if let Some(name) = export_name {
            self.add_export(ExportKind::Memory(mem_index), name);
        }

        mem_index
    }

    pub(crate) fn add_table_descriptor(
        &mut self,
        descriptor: ResizableLimits,
        export_name: Option<&'_ str>,
    ) -> u32 {
        let table_index = self.table_section.add_descriptor(descriptor) as u32;
        if let Some(name) = export_name {
            self.add_export(ExportKind::Table(table_index), name);
        }

        table_index
    }

    /// FOOTGUN. Needs to be used before ANY other `add` function otherwise this may generate a corrupt binary
    pub fn add_import<T: Into<String>>(
        &mut self,
        module: T,
        external_name: T,
        kind: ExternalKind,
    ) -> u32 {
        self.import_section.add_import(module, external_name, kind);
        let idx = match kind {
            ExternalKind::Function(type_def) => self.add_fn_decl(type_def),
            ExternalKind::Global(desc) => self.add_global_descriptor(desc, None),
            ExternalKind::Memory(desc) => self.add_memory_descriptor(desc, None),
            ExternalKind::Table(desc) => self.add_table_descriptor(desc, None),
        };

        idx
    }

    /// Footgun. Needs to be used after `add_import` otherwise this may generate a corrupt binary
    pub fn add_function(&mut self, fn_body: FnBody<'a>, export_name: Option<&'_ str>) -> u32 {
        let (params, return_type) = fn_body.get_fn_type();
        let type_id = self.add_type(params, return_type) as u32;
        let fn_index = self.add_fn_decl(type_id) as u32;
        if let Some(name) = export_name {
            self.add_export(ExportKind::Function(fn_index), name);
        }

        self.code_section.add_fn_body(fn_body);

        fn_index
    }

    pub fn add_global_descriptor(
        &mut self,
        descriptor: GlobalDescriptor,
        export_name: Option<&'_ str>,
    ) -> u32 {
        let global_index = self.global_section.add_descriptor(descriptor) as u32;
        if let Some(name) = export_name {
            self.add_export(ExportKind::Global(global_index), name);
        }

        global_index
    }

    pub fn add_table(&mut self, table: Table, export_name: Option<&'_ str>) -> u32 {
        let table_idx = self.table_section.add_descriptor(table.inner()) as u32;
        self.element_section
            .add_elements(table_idx, 0, table.refs().to_vec());
        if let Some(name) = export_name {
            self.add_export(ExportKind::Table(table_idx), name);
        }

        table_idx
    }

    pub fn add_memory(&mut self, memory: Memory, export_name: Option<&'_ str>) -> u32 {
        let mem_idx = self.memory_section.add_descriptor(memory.inner()) as u32;
        self.data_section
            .add_data(mem_idx, 0, memory.mem_slice().to_vec());
        if let Some(name) = export_name {
            self.add_export(ExportKind::Memory(mem_idx), name);
        }

        mem_idx
    }

    pub fn compile(self) -> Result<Vec<u8>, Error> {
        let mut buff = Vec::new();
        self.compile_stream(&mut buff)?;

        Ok(buff)
    }

    // Does not need to validate fndecl and code section. These are always the same length
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.import_section.count() > 0 {
            self.import_section.validate()?;
        }

        if self.table_section.count() > 0 {
            self.table_section.validate()?;
        }

        if self.memory_section.count() > 0 {
            self.memory_section.validate()?;
        }

        if self.export_section.count() > 0 {
            self.export_section.validate()?;
        }

        self.type_section.validate()?;
        // Will be implemented later
        // self.code_section.validate()?;
        Ok(())
    }

    pub fn compile_stream(mut self, writer: &mut impl Write) -> Result<usize, Error> {
        if self.optimize {
            self.code_section = self.code_section.optimize();
        }

        if self.validate {
            self.validate()?;
        }

        let mut written = writer.write(&MAGIC)?;
        written += self.type_section.compile(writer)?;
        written += self.import_section.compile(writer)?;
        written += self.fn_section.compile(writer)?;
        written += self.table_section.compile(writer)?;
        written += self.memory_section.compile(writer)?;
        written += self.global_section.compile(writer)?;
        written += self.export_section.compile(writer)?;
        written += self.element_section.compile(writer)?;
        written += self.code_section.compile(writer)?;
        written += self.data_section.compile(writer)?;
        Ok(written)
    }
}
