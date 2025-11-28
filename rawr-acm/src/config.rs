use envconfig::Envconfig;

use rawr_config::{ConfigError, load_config};

#[derive(Envconfig, Debug, Clone)]
pub struct AcmConfig {
    #[envconfig(from = "RAWR_ACM_MAX_LRU_SIZE", default = "25")]
    pub max_lru_size: usize,
}

impl Default for AcmConfig {
    fn default() -> Self {
        AcmConfig::load().unwrap_or(AcmConfig { max_lru_size: 25 })
    }
}

impl AcmConfig {
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
        let config = AcmConfig::init_from_hashmap(&hashmap).unwrap();
        assert_eq!(config.max_lru_size, 25);
    }

    #[test]
    fn test_load_with_supplied_value() {
        let mut hashmap = HashMap::new();
        hashmap.insert("RAWR_ACM_MAX_LRU_SIZE".to_string(), "50".to_string());

        let config = AcmConfig::init_from_hashmap(&hashmap).unwrap();
        assert_eq!(config.max_lru_size, 50);
    }

    #[test]
    fn test_load_with_large_value() {
        let mut hashmap = HashMap::new();
        hashmap.insert("RAWR_ACM_MAX_LRU_SIZE".to_string(), "1000".to_string());

        let config = AcmConfig::init_from_hashmap(&hashmap).unwrap();
        assert_eq!(config.max_lru_size, 1000);
    }

    #[test]
    fn test_default_implementation() {
        let config = AcmConfig::default();
        assert_eq!(config.max_lru_size, 25);
    }
}
