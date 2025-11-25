#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdpError {
    FailedToDecide(String),
}

impl std::fmt::Display for PdpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdpError::FailedToDecide(msg) => write!(f, "failed to decide: {}", msg),
        }
    }
}

impl std::error::Error for PdpError {}

