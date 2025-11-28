pub use envconfig::Envconfig;

pub mod errors;
pub use errors::ConfigError;

pub mod deployment;
pub use deployment::{Cloud, Deployment, Environment, Region};

pub fn load_config<T: Envconfig>() -> Result<T, ConfigError> {
    T::init_from_env().map_err(ConfigError::from)
}
