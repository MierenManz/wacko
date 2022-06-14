use crate::CodeSection;
use crate::Error;
use crate::ExportKind;
use crate::ExportSection;
use crate::ExternalKind;
use crate::FnBody;
use crate::FunctionSection;
use crate::GlobalDescriptor;
use crate::GlobalSection;
use crate::ImportSection;
use crate::MemorySection;
use crate::ResizableLimits;
use crate::TableSection;
use crate::TypeSection;
use crate::ValType;
use crate::ValidationError;

pub struct Module {
    optimize: bool,
    validate: bool,
    type_section: TypeSection,
    fn_section: FunctionSection,
    code_section: CodeSection,
    import_section: ImportSection,
    table_section: TableSection,
    memory_section: MemorySection,
    global_section: GlobalSection,
    export_section: ExportSection,
}

impl Module {
    pub fn new(optimize: bool, validate: bool) -> Self {
        Self {
            optimize,
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
        }
    }

    pub(crate) fn add_type<T: Into<Vec<ValType>>>(&mut self, params: T, return_type: T) -> usize {
        self.type_section.add_type_def(params, return_type)
    }

    pub(crate) fn add_fn_decl(&mut self, type_def: u32) -> usize {
        self.fn_section.add_fn_decl(type_def)
    }

    pub(crate) fn add_export(&mut self, export_kind: ExportKind, export_name: &str) {
        self.export_section.add_export(export_kind, export_name);
    }

    /// Footgun. Needs to be used after `add_import` otherwise this may generate a corrupt binary
    pub fn add_function<'a>(&mut self, fn_body: FnBody, export_name: Option<&'a str>) -> usize {
        let (params, return_type) = fn_body.get_fn_type();
        let type_id = self.add_type(params, return_type) as u32;
        let fn_index = self.add_fn_decl(type_id) as u32;

        if let Some(name) = export_name {
            self.add_export(ExportKind::Function(fn_index), name.into());
        }

        self.code_section.add_fn_body(fn_body);

        fn_index as usize
    }

    pub fn add_global_descriptor<'a>(
        &mut self,
        descriptor: GlobalDescriptor,
        export_name: Option<&'a str>,
    ) {
        let global_index = self.global_section.add_descriptor(descriptor) as u32;
        if let Some(name) = export_name {
            self.add_export(ExportKind::Global(global_index), name.into());
        }
    }

    pub fn add_memory_descriptor<'a>(
        &mut self,
        descriptor: ResizableLimits,
        export_name: Option<&'a str>,
    ) {
        let mem_index = self.memory_section.add_descriptor(descriptor) as u32;
        if let Some(name) = export_name {
            self.add_export(ExportKind::Memory(mem_index), name.into());
        }
    }

    /// Footgun. Needs to be used before `add_function` otherwise this may generate a corrupt binary
    pub fn add_import<T: Into<String>>(&mut self, module: T, external_name: T, kind: ExternalKind) {
        self.import_section.add_import(module, external_name, kind);
        match kind {
            ExternalKind::Function(type_def) => {self.add_fn_decl(type_def);},
            ExternalKind::Global(desc) => self.add_global_descriptor(desc, None),
            ExternalKind::Memory(desc) => self.add_memory_descriptor(desc, None),
            ExternalKind::Table(desc) => self.add_table_descriptor(desc, None),
        };
    }

    pub fn add_table_descriptor<'a>(
        &mut self,
        descriptor: ResizableLimits,
        export_name: Option<&'a str>,
    ) {
        let table_index = self.table_section.add_descriptor(descriptor) as u32;
        if let Some(name) = export_name {
            self.add_export(ExportKind::Table(table_index), name.into());
        }
    }

    pub fn compile(self) -> Result<Vec<u8>, Error> {
        if self.optimize {
            self.optimize();
        }

        if self.validate {
            self.validate()?;
        }

        Ok(Vec::new())
    }

    fn optimize(&self) {}
    fn validate(&self) -> Result<(), ValidationError> {
        if self.import_section.count() > 0 {
            self.import_section.validate()?;
        }

        if self.table_section.count() > 0 {
            self.table_section.validate()?;
        }

        if self.memory_section.count() > 0 {
            self.memory_section.validate()?;
        }

        if self.global_section.count() > 0 {
            self.global_section.validate()?;
        }

        self.type_section.validate()?;
        Ok(())
    }
}
