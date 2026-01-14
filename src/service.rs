use crate::{filters, scraper, storage};
use anyhow::{Context, Result};
use chrono::{DateTime, Local};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScrapeSummary {
    pub total_jobs: usize,
    pub today_jobs: usize,
    pub new_jobs: usize,
    pub updated_at: DateTime<Local>,
    pub search_url: String,
}

pub async fn run_scrape_once(paths: &storage::Paths) -> Result<ScrapeSummary> {
    let config = storage::load_config_or_default(paths)
        .context("Failed to load config")?;

    let search_url = scraper::build_search_url(
        &config.search.keywords,
        &config.search.location,
        config.search.remote,
        config.search.salary_min,
    );

    let client = scraper::create_client()?;

    let cookies = if storage::file_exists(&paths.cookies) {
        Some(scraper::load_cookies_from_file(
            paths.cookies.to_string_lossy().as_ref(),
        )?)
    } else {
        None
    };

    let html = if let Some(cookie_header) = &cookies {
        scraper::fetch_jobs_page_with_cookies(&client, &search_url, Some(cookie_header))
            .await
            .context("Failed to fetch jobs page")?
    } else {
        scraper::fetch_jobs_page(&client, &search_url)
            .await
            .context("Failed to fetch jobs page")?
    };

    std::fs::write(&paths.debug_html, &html)
        .with_context(|| format!("Failed to save debug HTML: {:?}", paths.debug_html))?;

    let all_jobs = scraper::parse_jobs_from_html(&html)
        .context("Failed to parse jobs from HTML")?;

    let today_jobs = filters::filter_today_only(all_jobs);

    let mut app_state = storage::load_state(paths)
        .context("Failed to load state")?;

    let new_jobs = app_state.filter_new_jobs(&today_jobs);

    app_state.mark_jobs_seen(&new_jobs);
    storage::save_state(paths, &app_state)
        .context("Failed to save state")?;

    let updated_at = Local::now();

    let snapshot = storage::JobsSnapshot {
        updated_at: Some(updated_at),
        jobs: today_jobs.clone(),
        new_jobs: new_jobs.clone(),
    };

    storage::save_latest_jobs(paths, &snapshot)
        .context("Failed to save latest jobs")?;

    Ok(ScrapeSummary {
        total_jobs: snapshot.jobs.len(),
        today_jobs: snapshot.jobs.len(),
        new_jobs: snapshot.new_jobs.len(),
        updated_at,
        search_url,
    })
}
