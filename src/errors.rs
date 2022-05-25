use std::io::Error as StdioError;

#[derive(Debug)]
pub enum Error {
    ValidationError(ValidationError),
    IoError(Box<StdioError>), // OptimizationError(OptimizationError),
}

#[derive(Debug, Clone, Copy)]
pub enum RequiredSection {
    TypeSection,
    FunctionSection,
    CodeSection,
}

#[derive(Debug, Clone, Copy)]
pub enum ValidationError {
    InvalidSignature,
    InvalidIndex,
    UnexpectedEOF,
    ArrayOverflow,
    ArrayTooLittleElements,
    InvalidMemorySetting,
    SectionMissing(RequiredSection),
    InvalidType,
    TooManyFnBodies,
    TooManyFnDeclarations,
    MutatableImport,
}

impl From<ValidationError> for Error {
    fn from(err: ValidationError) -> Self {
        Self::ValidationError(err)
    }
}

impl From<StdioError> for Error {
    fn from(err: StdioError) -> Self {
        Self::IoError(Box::new(err))
    }
}
