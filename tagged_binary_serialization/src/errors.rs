use std::error::Error;

#[derive(Debug)]
pub enum SerializationError {
    NoTag,
    MismatchedTag(u16, u16),
    NoValue,
}

impl std::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializationError::NoTag => write!(f, "Byte slice did not include a tag"),
            SerializationError::MismatchedTag(ex, got) => {
                write!(f, "Expected a tag of {ex} but received a tag of {got}")
            }
            SerializationError::NoValue => write!(f, "Byte slice did not include a value"),
        }
    }
}

impl Error for SerializationError {}
