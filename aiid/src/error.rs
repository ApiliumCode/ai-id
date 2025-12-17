/// A simple struct(String) for reporting aiid errors
#[derive(Debug, PartialEq, Clone)]
pub struct AiidError(pub String);

/// aiid Result type
pub type AiidResult<T> = Result<T, AiidError>;

impl std::fmt::Display for AiidError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "AiidError(\"{}\")", self.0)
    }
}

impl std::error::Error for AiidError {}

impl From<String> for AiidError {
    fn from(error: String) -> Self {
        Self(error)
    }
}

impl<'a> From<&'a str> for AiidError {
    fn from(error: &'a str) -> Self {
        Self(error.to_string())
    }
}

impl From<reed_solomon::DecoderError> for AiidError {
    fn from(error: reed_solomon::DecoderError) -> Self {
        Self(format!("{:?}", error))
    }
}

impl From<std::num::ParseIntError> for AiidError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self(format!("{:?}", error))
    }
}
