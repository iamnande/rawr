#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipError {
    AcmNotFound(String),
}

impl std::fmt::Display for PipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipError::AcmNotFound(msg) => write!(f, "access control model not found: {}", msg),
        }
    }
}

impl std::error::Error for PipError {}

// TODO(nick): we should wrap serde errors? maybe? probably?
