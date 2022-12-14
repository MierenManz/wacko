pub enum EncodingError {}

pub enum ValidationError {}

pub enum UnknownError {}

pub enum WackoError {
    EncodingError(EncodingError),
    ValidationError(ValidationError),
    UnknownError(UnknownError),
}
