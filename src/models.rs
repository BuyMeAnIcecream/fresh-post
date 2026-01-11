use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::hash::{Hash, Hasher};

fn serialize_date<S>(date: &Option<chrono::NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(d) => serializer.serialize_str(&d.format("%Y-%m-%d").to_string()),
        None => serializer.serialize_none(),
    }
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<Option<chrono::NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(date_str) => {
            chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
        None => Ok(None),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JobPosting {
    pub id: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub url: String,
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    pub posted_date: Option<chrono::NaiveDate>,
    pub description: Option<String>,
}

impl Hash for JobPosting {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl JobPosting {
    pub fn new(
        id: String,
        title: String,
        company: String,
        location: String,
        url: String,
        posted_date: Option<chrono::NaiveDate>,
        description: Option<String>,
    ) -> Self {
        Self {
            id,
            title,
            company,
            location,
            url,
            posted_date,
            description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_posting_serialization() {
        let job = JobPosting::new(
            "123".to_string(),
            "Software Engineer".to_string(),
            "Tech Corp".to_string(),
            "San Francisco, CA".to_string(),
            "https://linkedin.com/jobs/view/123".to_string(),
            Some(chrono::Local::now().date_naive()),
            Some("Great opportunity".to_string()),
        );

        let json = serde_json::to_string(&job).unwrap();
        let deserialized: JobPosting = serde_json::from_str(&json).unwrap();
        
        assert_eq!(job.id, deserialized.id);
        assert_eq!(job.title, deserialized.title);
    }

    #[test]
    fn test_job_posting_hash() {
        let job1 = JobPosting::new(
            "123".to_string(),
            "Engineer".to_string(),
            "Corp".to_string(),
            "SF".to_string(),
            "https://example.com".to_string(),
            None,
            None,
        );
        
        let job2 = JobPosting::new(
            "123".to_string(),
            "Different Title".to_string(),
            "Different Corp".to_string(),
            "NYC".to_string(),
            "https://example.com".to_string(),
            None,
            None,
        );

        // Same ID should produce same hash
        use std::collections::hash_map::DefaultHasher;
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        job1.hash(&mut hasher1);
        job2.hash(&mut hasher2);
        
        assert_eq!(hasher1.finish(), hasher2.finish());
    }
}
