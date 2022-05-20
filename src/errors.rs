use std::io::Error as StdioError;

#[derive(Debug)]
pub enum Error {
    CompileError(CompileError),
    ValidationError(ValidationError),
    IoError(Box<StdioError>), // OptimizationError(OptimizationError),
}

#[derive(Debug, Clone, Copy)]
pub enum CompileError {}

#[derive(Debug, Clone, Copy)]
pub enum ValidationError {
    InvalidSignature,
    InvalidIndex,
    UnexpectedEOF,
    ArrayOverflow,
    ArrayTooLittleElements,
    InvalidMemorySetting,
    SectionMissing(&'static str),
    InvalidType,
}

impl From<CompileError> for Error {
    fn from(err: CompileError) -> Self {
        Self::CompileError(err)
    }
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
