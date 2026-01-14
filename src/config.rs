use anyhow::{Context, Result};
use serde::{Deserialize, Deserializer, Serialize};
use std::path::Path;

fn deserialize_salary_min<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<u32> = Option::deserialize(deserializer)?;
    Ok(value.filter(|&v| v > 0 && v >= 40000))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub search: SearchConfig,
    #[serde(default)]
    pub schedule: ScheduleConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub keywords: String,
    pub location: String,
    /// Filter for remote jobs only (uses f_WT=2 parameter)
    #[serde(default)]
    pub remote: bool,
    /// Minimum salary in USD (e.g., 100000 for $100k). LinkedIn uses increments of $20k.
    /// Valid range: $40k - $200k. Set to 0 or omit to disable salary filter.
    #[serde(default, deserialize_with = "deserialize_salary_min")]
    pub salary_min: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleConfig {
    #[serde(default = "default_interval_hours")]
    pub interval_hours: u64,
}

fn default_interval_hours() -> u64 {
    4
}

impl Default for ScheduleConfig {
    fn default() -> Self {
        Self {
            interval_hours: default_interval_hours(),
        }
    }
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
                    remote: false,
                    salary_min: None,
                },
                schedule: ScheduleConfig {
                    interval_hours: default_interval_hours(),
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
                remote: false,
                salary_min: None,
            },
            schedule: ScheduleConfig {
                interval_hours: 4,
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
        assert_eq!(config.schedule.interval_hours, 4);
    }

    #[test]
    fn test_config_deserialization_with_schedule() {
        let toml_content = r#"
[search]
keywords = "rust developer"
location = "San Francisco Bay Area"
[schedule]
interval_hours = 5
"#;

        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.search.keywords, "rust developer");
        assert_eq!(config.search.location, "San Francisco Bay Area");
        assert_eq!(config.schedule.interval_hours, 5);
    }
}
