use fresh_post::{filters, scraper, state};
use std::fs;
use std::path::PathBuf;

#[tokio::test]
async fn test_full_scrape_flow() {
    // Load test fixture
    let fixture_path = PathBuf::from("tests/fixtures/sample_linkedin_jobs.html");
    let html = fs::read_to_string(&fixture_path)
        .expect("Failed to read test fixture");
    
    // Parse jobs
    let jobs = scraper::parse_jobs_from_html(&html)
        .expect("Failed to parse jobs");
    
    assert!(jobs.len() >= 2, "Should parse at least 2 jobs");
    
    // Verify job data
    let first_job = &jobs[0];
    assert!(!first_job.title.is_empty());
    assert!(!first_job.company.is_empty());
    assert!(!first_job.url.is_empty());
    
    // Test date filtering (will depend on current date)
    let today_jobs = filters::filter_today_only(jobs.clone());
    
    // Test state management
    let temp_state_path = PathBuf::from(".test_state.json");
    let mut app_state = state::State::load_from_file(&temp_state_path)
        .expect("Failed to load state");
    
    // Mark first job as seen
    app_state.mark_jobs_seen(&[jobs[0].clone()]);
    
    // Filter new jobs
    let new_jobs = app_state.filter_new_jobs(&jobs);
    assert!(new_jobs.len() < jobs.len(), "Should filter out seen jobs");
    
    // Cleanup
    let _ = std::fs::remove_file(&temp_state_path);
}

#[test]
fn test_build_search_url() {
    let url = scraper::build_search_url("rust", "SF", false, None);
    assert!(url.starts_with("https://www.linkedin.com/jobs/search"));
    assert!(url.contains("keywords=rust"));
    assert!(url.contains("location=SF"));
}
