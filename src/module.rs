use crate::Error;
use crate::ImportSection;
use crate::TypeSection;
use crate::ValidationError;

pub struct Module {
    optimize: bool,
    validate: bool,
    type_section: Option<TypeSection>,
    import_section: Option<ImportSection>,
}

impl Module {
    pub fn new(optimize: bool, validate: bool) -> Self {
        Self {
            optimize,
            validate,
            type_section: None,
            import_section: None,
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
        match &self.type_section {
            Some(v) => v.validate()?,
            None => return Err(ValidationError::SectionMissing("Type Section")),
        }

        if let Some(v) = &self.import_section {
            v.validate()?;
        }

        Ok(())
    }
}
