#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdpError {
    Pip(rawr_pip::PipError),
}

impl std::fmt::Display for PdpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdpError::Pip(err) => write!(f, "failed to decide: {}", err),
        }
    }
}

impl std::error::Error for PdpError {}

impl From<rawr_pip::PipError> for PdpError {
    fn from(err: rawr_pip::PipError) -> Self {
        PdpError::Pip(err)
    }
}
