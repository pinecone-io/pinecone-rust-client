use std::error::Error;

#[derive(Debug)]
pub struct PineconeError {
    pub kind: PineconeErrorKind,
    pub message: String,
}

impl Error for PineconeError {}

impl std::fmt::Display for PineconeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PineconeError: {}", self.message)
    }
}

#[derive(Debug)]
pub enum PineconeErrorKind {
    CofigurationError,
}
