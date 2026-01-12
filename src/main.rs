use anyhow::{Context, Result};
use fresh_post::{config, filters, scraper, state};
use std::path::PathBuf;

const STATE_FILE: &str = ".notifier_state.json";
const COOKIE_FILE: &str = "linkedin_cookies.txt";
const CONFIG_FILE: &str = "config.toml";

#[tokio::main]
async fn main() -> Result<()> {
    // Load config
    let config = config::Config::load_or_default(CONFIG_FILE)
        .context("Failed to load config")?;
    
    let keywords = &config.search.keywords;
    let location = &config.search.location;
    
    println!("🔍 Searching LinkedIn jobs...");
    println!("   Keywords: {}", keywords);
    println!("   Location: {}", location);
    if config.search.remote {
        println!("   Remote: Yes");
    }
    if let Some(salary) = config.search.salary_min {
        println!("   Min Salary: ${}", salary);
    }
    println!();
    
    // Load state
    let state_path = PathBuf::from(STATE_FILE);
    let mut app_state = state::State::load_from_file(&state_path)
        .context("Failed to load state")?;
    
    println!("📊 Previously seen jobs: {}\n", app_state.seen_count());
    
    // Build search URL
    let search_url = scraper::build_search_url(
        keywords, 
        location,
        config.search.remote,
        config.search.salary_min,
    );
    println!("🌐 Fetching: {}\n", search_url);
    
    // Create HTTP client
    let client = scraper::create_client()?;
    
    // Load cookies if available
    let cookies = if std::path::Path::new(COOKIE_FILE).exists() {
        Some(scraper::load_cookies_from_file(COOKIE_FILE)?)
    } else {
        println!("ℹ️  No cookie file found. Using guest mode. (Create '{}' to use your account)\n", COOKIE_FILE);
        None
    };
    
    // Fetch page with cookies if available
    let html = if let Some(cookie_header) = &cookies {
        scraper::fetch_jobs_page_with_cookies(&client, &search_url, Some(cookie_header))
            .await
            .context("Failed to fetch jobs page")?
    } else {
        scraper::fetch_jobs_page(&client, &search_url)
            .await
            .context("Failed to fetch jobs page")?
    };
    
    // DEBUG: Save HTML for inspection
    std::fs::write("debug_linkedin.html", &html)
        .context("Failed to save debug HTML")?;
    println!("💾 Saved HTML to debug_linkedin.html ({} bytes)", html.len());
    
    // DEBUG: Show snippet
    println!("📄 HTML snippet (first 500 chars):");
    println!("{}\n", &html[..html.len().min(500)]);
    
    // Parse jobs from HTML
    let all_jobs = scraper::parse_jobs_from_html(&html)
        .context("Failed to parse jobs from HTML")?;
    
    println!("📋 Found {} total jobs\n", all_jobs.len());
    
    // Filter to today only
    let today_jobs = filters::filter_today_only(all_jobs);
    println!("📅 Jobs posted today: {}\n", today_jobs.len());
    
    // Filter out already seen jobs
    let new_jobs = app_state.filter_new_jobs(&today_jobs);
    println!("✨ New jobs (not seen before): {}\n", new_jobs.len());
    
    // Display new jobs
    if new_jobs.is_empty() {
        println!("No new jobs found! 🎉");
    } else {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        for (idx, job) in new_jobs.iter().enumerate() {
            println!("\n{}. {}", idx + 1, job.title);
            println!("   Company: {}", job.company);
            println!("   Location: {}", job.location);
            if let Some(date) = job.posted_date {
                println!("   Posted: {}", date.format("%Y-%m-%d"));
            }
            println!("   URL: {}", job.url);
        }
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
    
    // Update state with new jobs
    app_state.mark_jobs_seen(&new_jobs);
    app_state.save_to_file(&state_path)
        .context("Failed to save state")?;
    
    println!("💾 State saved. Total seen jobs: {}", app_state.seen_count());
    
    Ok(())
}
