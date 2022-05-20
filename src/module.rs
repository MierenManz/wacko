use crate::Error;
use crate::FunctionSection;
use crate::ImportSection;
use crate::TableSection;
use crate::TypeSection;
use crate::ValidationError;

pub struct Module {
    optimize: bool,
    validate: bool,
    type_section: Option<TypeSection>,
    import_section: Option<ImportSection>,
    fn_section: Option<FunctionSection>,
    table_section: Option<TableSection>,
}

impl Module {
    pub fn new(optimize: bool, validate: bool) -> Self {
        Self {
            optimize,
            validate,
            type_section: None,
            import_section: None,
            fn_section: None,
            table_section: None,
        }
    }

    pub fn set_type_section(&mut self, section: TypeSection) {
        self.type_section = Some(section);
    }

    pub fn set_import_section(&mut self, section: ImportSection) {
        self.import_section = Some(section);
    }

    pub fn set_fn_section(&mut self, section: FunctionSection) {
        self.fn_section = Some(section);
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
        match &self.type_section {
            Some(v) => v.validate()?,
            None => return Err(ValidationError::SectionMissing("Type Section")),
        }

        if let Some(v) = &self.import_section {
            v.validate()?;
        }
        // Function section is skipped because it doesn't have anything to validate internally

        if let Some(v) = &self.table_section {
            v.validate()?;
        }

        Ok(())
    }
}
