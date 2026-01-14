use crate::config::Config;
use crate::models::JobPosting;
use crate::state::State;
use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Paths {
    pub base_dir: PathBuf,
    pub config: PathBuf,
    pub cookies: PathBuf,
    pub state: PathBuf,
    pub latest_jobs: PathBuf,
    pub debug_html: PathBuf,
}

impl Paths {
    pub fn new() -> Result<Self> {
        let base_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| ".".to_string());
        let base_dir = PathBuf::from(base_dir);
        fs::create_dir_all(&base_dir)
            .with_context(|| format!("Failed to create data dir: {:?}", base_dir))?;

        Ok(Self {
            base_dir: base_dir.clone(),
            config: base_dir.join("config.toml"),
            cookies: base_dir.join("linkedin_cookies.txt"),
            state: base_dir.join(".notifier_state.json"),
            latest_jobs: base_dir.join("latest_jobs.json"),
            debug_html: base_dir.join("debug_linkedin.html"),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobsSnapshot {
    pub updated_at: Option<DateTime<Local>>,
    pub jobs: Vec<JobPosting>,
    pub new_jobs: Vec<JobPosting>,
}

impl JobsSnapshot {
    pub fn empty() -> Self {
        Self {
            updated_at: None,
            jobs: Vec::new(),
            new_jobs: Vec::new(),
        }
    }
}

pub fn load_config_or_default(paths: &Paths) -> Result<Config> {
    Config::load_or_default(&paths.config)
}

pub fn save_config(paths: &Paths, config: &Config) -> Result<()> {
    let content = toml::to_string_pretty(config).context("Failed to serialize config")?;
    fs::write(&paths.config, content)
        .with_context(|| format!("Failed to write config: {:?}", paths.config))?;
    Ok(())
}

pub fn load_state(paths: &Paths) -> Result<State> {
    State::load_from_file(&paths.state)
}

pub fn save_state(paths: &Paths, state: &State) -> Result<()> {
    state.save_to_file(&paths.state)
}

pub fn load_latest_jobs(paths: &Paths) -> Result<JobsSnapshot> {
    if !paths.latest_jobs.exists() {
        return Ok(JobsSnapshot::empty());
    }

    let content = fs::read_to_string(&paths.latest_jobs)
        .with_context(|| format!("Failed to read latest jobs: {:?}", paths.latest_jobs))?;

    let snapshot: JobsSnapshot = serde_json::from_str(&content)
        .context("Failed to parse latest jobs JSON")?;

    Ok(snapshot)
}

pub fn save_latest_jobs(paths: &Paths, snapshot: &JobsSnapshot) -> Result<()> {
    let content = serde_json::to_string_pretty(snapshot)
        .context("Failed to serialize latest jobs")?;

    fs::write(&paths.latest_jobs, content)
        .with_context(|| format!("Failed to write latest jobs: {:?}", paths.latest_jobs))?;

    Ok(())
}

pub fn file_exists(path: &Path) -> bool {
    path.exists()
}
