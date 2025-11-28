use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    InvalidValue { field: String, message: String },
    Envconfig(envconfig::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidValue { field, message } => {
                write!(f, "`{field}` has an invalid value: {message}")
            }
            ConfigError::Envconfig(err) => write!(f, "{err}"),
        }
    }
}

impl From<envconfig::Error> for ConfigError {
    fn from(err: envconfig::Error) -> Self {
        ConfigError::Envconfig(err)
    }
}

