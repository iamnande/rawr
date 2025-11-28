use crate::{load_config, ConfigError, Envconfig};

pub enum Cloud {
    Local,
    Hetzner,
}

impl std::str::FromStr for Cloud {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_ref() {
            "local" => Ok(Cloud::Local),
            "hetzner" => Ok(Cloud::Hetzner),
            _ => Err(ConfigError::InvalidValue {
                field: "RAWR_CLOUD".into(),
                message: format!("{s} is not a supported cloud"),
            }),
        }
    }
}

pub enum Region {
    UsPnwA,
}

impl std::str::FromStr for Region {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_ref() {
            "us-pnw-a" => Ok(Region::UsPnwA),
            _ => Err(ConfigError::InvalidValue {
                field: "RAWR_REGION".into(),
                message: format!("{s} is not a supported region"),
            }),
        }
    }
}

pub enum Environment {
    Local,
    Dev,
    Prod,
}

impl std::str::FromStr for Environment {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_ref() {
            "local" => Ok(Environment::Local),
            "dev" => Ok(Environment::Dev),
            "prod" => Ok(Environment::Prod),
            _ => Err(ConfigError::InvalidValue {
                field: "RAWR_ENVIRONMENT".into(),
                message: format!("{s} is not a supported environment"),
            }),
        }
    }
}

#[derive(Envconfig)]
pub struct Deployment {
    #[envconfig(from = "RAWR_CLOUD", default = "local")]
    pub cloud: Cloud,
    #[envconfig(from = "RAWR_REGION", default = "us-pnw-a")]
    pub region: Region,
    #[envconfig(from = "RAWR_ENVIRONMENT", default = "local")]
    pub environment: Environment,
}

impl Deployment {
    pub fn load() -> Result<Self, ConfigError> {
        load_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_load_with_defaults() {
        let hashmap = HashMap::new();
        let deployment = Deployment::init_from_hashmap(&hashmap).unwrap();
        assert!(matches!(deployment.cloud, Cloud::Local));
        assert!(matches!(deployment.region, Region::UsPnwA));
        assert!(matches!(deployment.environment, Environment::Local));
    }

    #[test]
    fn test_load_with_valid_supplied_values() {
        let mut hashmap = HashMap::new();
        hashmap.insert("RAWR_CLOUD".to_string(), "hetzner".to_string());
        hashmap.insert("RAWR_REGION".to_string(), "us-pnw-a".to_string());
        hashmap.insert("RAWR_ENVIRONMENT".to_string(), "prod".to_string());

        let deployment = Deployment::init_from_hashmap(&hashmap).unwrap();
        assert!(matches!(deployment.cloud, Cloud::Hetzner));
        assert!(matches!(deployment.region, Region::UsPnwA));
        assert!(matches!(deployment.environment, Environment::Prod));
    }

    #[test]
    fn test_load_with_partial_supplied_values() {
        let mut hashmap = HashMap::new();
        hashmap.insert("RAWR_ENVIRONMENT".to_string(), "dev".to_string());

        let deployment = Deployment::init_from_hashmap(&hashmap).unwrap();
        assert!(matches!(deployment.cloud, Cloud::Local));
        assert!(matches!(deployment.region, Region::UsPnwA));
        assert!(matches!(deployment.environment, Environment::Dev));
    }

    #[test]
    fn test_load_with_invalid_cloud() {
        let result = "invalid-cloud".parse::<Cloud>();
        assert!(result.is_err());
        if let Err(ConfigError::InvalidValue { field, .. }) = result {
            assert_eq!(field, "RAWR_CLOUD");
        }

        let mut hashmap = HashMap::new();
        hashmap.insert("RAWR_CLOUD".to_string(), "invalid-cloud".to_string());
        let result = Deployment::init_from_hashmap(&hashmap);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_with_invalid_region() {
        let result = "invalid-region".parse::<Region>();
        assert!(result.is_err());
        if let Err(ConfigError::InvalidValue { field, .. }) = result {
            assert_eq!(field, "RAWR_REGION");
        }

        let mut hashmap = HashMap::new();
        hashmap.insert("RAWR_REGION".to_string(), "invalid-region".to_string());
        let result = Deployment::init_from_hashmap(&hashmap);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_with_invalid_environment() {
        let result = "invalid-env".parse::<Environment>();
        assert!(result.is_err());
        if let Err(ConfigError::InvalidValue { field, .. }) = result {
            assert_eq!(field, "RAWR_ENVIRONMENT");
        }

        let mut hashmap = HashMap::new();
        hashmap.insert("RAWR_ENVIRONMENT".to_string(), "invalid-env".to_string());
        let result = Deployment::init_from_hashmap(&hashmap);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_with_case_insensitive_values() {
        let mut hashmap = HashMap::new();
        hashmap.insert("RAWR_CLOUD".to_string(), "HETZNER".to_string());
        hashmap.insert("RAWR_ENVIRONMENT".to_string(), "PROD".to_string());

        let deployment = Deployment::init_from_hashmap(&hashmap).unwrap();
        assert!(matches!(deployment.cloud, Cloud::Hetzner));
        assert!(matches!(deployment.environment, Environment::Prod));
    }

    #[test]
    fn test_load_with_whitespace_trimming() {
        let mut hashmap = HashMap::new();
        hashmap.insert("RAWR_CLOUD".to_string(), "  hetzner  ".to_string());
        hashmap.insert("RAWR_ENVIRONMENT".to_string(), "  dev  ".to_string());
        hashmap.insert("RAWR_REGION".to_string(), "us-pnw-a".to_string());

        let deployment = Deployment::init_from_hashmap(&hashmap).unwrap();
        assert!(matches!(deployment.cloud, Cloud::Hetzner));
        assert!(matches!(deployment.environment, Environment::Dev));
        assert!(matches!(deployment.region, Region::UsPnwA));
    }
}

