use crate::models::JobPosting;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    seen_job_ids: HashSet<String>,
}

impl State {
    pub fn new() -> Self {
        Self {
            seen_job_ids: HashSet::new(),
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        if !path.as_ref().exists() {
            return Ok(Self::new());
        }

        let content = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read state file: {:?}", path.as_ref()))?;
        
        let state: State = serde_json::from_str(&content)
            .with_context(|| "Failed to parse state file")?;
        
        Ok(state)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize state")?;
        
        std::fs::write(path.as_ref(), json)
            .with_context(|| format!("Failed to write state file: {:?}", path.as_ref()))?;
        
        Ok(())
    }

    pub fn mark_jobs_seen(&mut self, jobs: &[JobPosting]) {
        for job in jobs {
            self.seen_job_ids.insert(job.id.clone());
        }
    }

    pub fn filter_new_jobs(&self, jobs: &[JobPosting]) -> Vec<JobPosting> {
        jobs
            .iter()
            .filter(|job| !self.seen_job_ids.contains(&job.id))
            .cloned()
            .collect()
    }

    pub fn seen_count(&self) -> usize {
        self.seen_job_ids.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::JobPosting;
    use tempfile::TempDir;

    #[test]
    fn test_new_state() {
        let state = State::new();
        assert_eq!(state.seen_count(), 0);
    }

    #[test]
    fn test_mark_jobs_seen() {
        let mut state = State::new();
        let jobs = vec![
            JobPosting::new(
                "1".to_string(),
                "Job 1".to_string(),
                "Company".to_string(),
                "Location".to_string(),
                "https://example.com/1".to_string(),
                None,
                None,
            ),
            JobPosting::new(
                "2".to_string(),
                "Job 2".to_string(),
                "Company".to_string(),
                "Location".to_string(),
                "https://example.com/2".to_string(),
                None,
                None,
            ),
        ];

        state.mark_jobs_seen(&jobs);
        assert_eq!(state.seen_count(), 2);
    }

    #[test]
    fn test_filter_new_jobs() {
        let mut state = State::new();
        let job1 = JobPosting::new(
            "1".to_string(),
            "Seen Job".to_string(),
            "Company".to_string(),
            "Location".to_string(),
            "https://example.com/1".to_string(),
            None,
            None,
        );
        let job2 = JobPosting::new(
            "2".to_string(),
            "New Job".to_string(),
            "Company".to_string(),
            "Location".to_string(),
            "https://example.com/2".to_string(),
            None,
            None,
        );

        state.mark_jobs_seen(&[job1.clone()]);
        
        let all_jobs = vec![job1, job2.clone()];
        let new_jobs = state.filter_new_jobs(&all_jobs);
        
        assert_eq!(new_jobs.len(), 1);
        assert_eq!(new_jobs[0].id, job2.id);
    }

    #[test]
    fn test_save_and_load_state() {
        let temp_dir = TempDir::new().unwrap();
        let state_path = temp_dir.path().join("test_state.json");

        let mut state = State::new();
        let job = JobPosting::new(
            "123".to_string(),
            "Test Job".to_string(),
            "Company".to_string(),
            "Location".to_string(),
            "https://example.com".to_string(),
            None,
            None,
        );
        state.mark_jobs_seen(&[job]);

        state.save_to_file(&state_path).unwrap();
        assert!(state_path.exists());

        let loaded_state = State::load_from_file(&state_path).unwrap();
        assert_eq!(loaded_state.seen_count(), 1);
    }

    #[test]
    fn test_load_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let state_path = temp_dir.path().join("nonexistent.json");

        let state = State::load_from_file(&state_path).unwrap();
        assert_eq!(state.seen_count(), 0);
    }
}
