use anyhow::{Context, Result};
use fresh_post::{filters, scraper, state};
use std::path::PathBuf;

const STATE_FILE: &str = ".notifier_state.json";

#[tokio::main]
async fn main() -> Result<()> {
    // Hardcoded search parameters for now (will move to config/telegram later)
    let keywords = "rust developer";
    let location = "San Francisco Bay Area";
    
    println!("🔍 Searching LinkedIn jobs...");
    println!("   Keywords: {}", keywords);
    println!("   Location: {}\n", location);
    
    // Load state
    let state_path = PathBuf::from(STATE_FILE);
    let mut app_state = state::State::load_from_file(&state_path)
        .context("Failed to load state")?;
    
    println!("📊 Previously seen jobs: {}\n", app_state.seen_count());
    
    // Build search URL
    let search_url = scraper::build_search_url(keywords, location);
    println!("🌐 Fetching: {}\n", search_url);
    
    // Create HTTP client and fetch page
    let client = scraper::create_client()?;
    let html = scraper::fetch_jobs_page(&client, &search_url)
        .await
        .context("Failed to fetch jobs page")?;
    
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
