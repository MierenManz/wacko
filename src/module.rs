use crate::Error;
use crate::ExportSection;
use crate::FunctionSection;
use crate::GlobalSection;
use crate::ImportSection;
use crate::MemorySection;
use crate::RequiredSection;
use crate::Section;
use crate::TableSection;
use crate::TypeSection;
use crate::ValidationError;

pub struct Module {
    optimize: bool,
    validate: bool,
    type_section: TypeSection,
    import_section: Option<ImportSection>,
    fn_section: FunctionSection,
    table_section: Option<TableSection>,
    memory_section: Option<MemorySection>,
    global_section: Option<GlobalSection>,
    export_section: Option<ExportSection>,
}

impl Module {
    pub fn new(optimize: bool, validate: bool) -> Self {
        Self {
            optimize,
            validate,
            type_section: TypeSection::default(),
            import_section: None,
            fn_section: FunctionSection::default(),
            table_section: None,
            memory_section: None,
            global_section: None,
            export_section: None,
        }
    }

    pub fn set_type_section(&mut self, section: TypeSection) {
        self.type_section = section;
    }

    pub fn set_import_section(&mut self, section: ImportSection) {
        self.import_section = Some(section);
    }

    pub fn set_fn_section(&mut self, section: FunctionSection) {
        self.fn_section = section;
    }

    pub fn set_table_section(&mut self, section: TableSection) {
        self.table_section = Some(section);
    }

    pub fn set_memory_section(&mut self, section: MemorySection) {
        self.memory_section = Some(section);
    }

    pub fn set_global_section(&mut self, section: GlobalSection) {
        self.global_section = Some(section);
    }

    pub fn set_export_section(&mut self, section: ExportSection) {
        self.export_section = Some(section);
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
        if self.type_section.count() == 0 {
            return Err(ValidationError::SectionMissing(
                RequiredSection::TypeSection,
            ));
        }

        if self.fn_section.count() == 0 {
            return Err(ValidationError::SectionMissing(
                RequiredSection::FunctionSection,
            ));
        }

        // if self.code_section.count() == 0 {
        //     return Err(ValidationError::SectionMissing(RequiredSection::CodeSection))
        // }

        // if self.code_section.count() < self.fn_section.count() {
        //     return Err(ValidationError::TooManyFnDeclarations);
        // }

        // if self.code_section.count() > self.fn_section.count() {
        //     return Err(ValidationError::TooManyFnBodies);
        // }

        self.type_section.validate()?;
        // self.code_section.validate()?;

        if let Some(section) = &self.import_section {
            section.validate()?;
        }

        if let Some(section) = &self.table_section {
            section.validate()?;
        }
        if let Some(section) = &self.memory_section {
            section.validate()?;
        }

        if let Some(section) = &self.global_section {
            section.validate()?;
        }
        Ok(())
    }
}
