use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub search: SearchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub keywords: String,
    pub location: String,
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {:?}", path.as_ref()))?;
        
        let config: Config = toml::from_str(&content)
            .with_context(|| "Failed to parse config file")?;
        
        Ok(config)
    }

    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Result<Self> {
        if path.as_ref().exists() {
            Self::load_from_file(path)
        } else {
            // Return default config
            Ok(Config {
                search: SearchConfig {
                    keywords: "rust developer".to_string(),
                    location: "San Francisco Bay Area".to_string(),
                },
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let config = Config {
            search: SearchConfig {
                keywords: "test".to_string(),
                location: "Test Location".to_string(),
            },
        };

        let toml = toml::to_string(&config).unwrap();
        assert!(toml.contains("keywords"));
        assert!(toml.contains("location"));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_content = r#"
[search]
keywords = "rust developer"
location = "San Francisco Bay Area"
"#;

        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.search.keywords, "rust developer");
        assert_eq!(config.search.location, "San Francisco Bay Area");
    }
}
