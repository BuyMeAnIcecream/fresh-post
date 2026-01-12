use crate::models::JobPosting;
use anyhow::{Context, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use url::Url;

/// Build a LinkedIn job search URL
pub fn build_search_url(keywords: &str, location: &str) -> String {
    let base_url = "https://www.linkedin.com/jobs/search";
    let mut url = Url::parse(base_url).unwrap();
    
    url.query_pairs_mut()
        .append_pair("keywords", keywords)
        .append_pair("location", location)
        .append_pair("f_TPR", "r86400"); // Last 24 hours filter
    
    url.to_string()
}

/// Create an HTTP client with appropriate headers
pub fn create_client() -> Result<Client> {
    Client::builder()
        .cookie_store(true)
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()
        .context("Failed to create HTTP client")
}

/// Fetch HTML content from a URL
pub async fn fetch_jobs_page(client: &Client, url: &str) -> Result<String> {
    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to send HTTP request")?;
    
    if !response.status().is_success() {
        anyhow::bail!("HTTP request failed with status: {}", response.status());
    }
    
    response
        .text()
        .await
        .context("Failed to read response body")
}

/// Generate a unique ID for a job from its URL
fn generate_job_id(url: &str) -> String {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Parse relative date strings like "2 hours ago", "1 day ago" to NaiveDate
fn parse_relative_date(date_str: &str) -> Option<chrono::NaiveDate> {
    let today = chrono::Local::now().date_naive();
    let lower = date_str.to_lowercase();
    
    if lower.contains("hour") || lower.contains("minute") || lower.contains("just now") {
        return Some(today);
    }
    
    if lower.contains("day") {
        // Try to extract number
        if let Some(num_str) = lower.split_whitespace().next() {
            if let Ok(days) = num_str.parse::<i64>() {
                if days == 0 || days == 1 {
                    return Some(today);
                }
            }
        }
    }
    
    // Try to parse as absolute date
    // LinkedIn might use formats like "Dec 15" or "December 15, 2024"
    // For now, if we can't parse it, return None (will be filtered out)
    None
}

/// Parse job postings from HTML
pub fn parse_jobs_from_html(html: &str) -> Result<Vec<JobPosting>> {
    let document = Html::parse_document(html);
    let mut jobs = Vec::new();
    
    // LinkedIn job card selector - this may need adjustment based on actual HTML structure
    // Common selectors: .job-card-container, .base-card, [data-job-id]
    let job_card_selector = Selector::parse(".base-card, .job-card-container, [data-job-id]")
        .map_err(|e| anyhow::anyhow!("Failed to parse job card selector: {:?}", e))?;
    
    // DEBUG: Count how many job cards we find
    let job_card_count = document.select(&job_card_selector).count();
    println!("🔍 Found {} job card elements in HTML", job_card_count);
    
    let title_selector = Selector::parse("h3.base-search-card__title, .job-card-list__title, a.job-card-list__title")
        .map_err(|e| anyhow::anyhow!("Failed to parse title selector: {:?}", e))?;
    
    let company_selector = Selector::parse("h4.base-search-card__subtitle, .job-card-container__company-name, a.job-card-container__company-name")
        .map_err(|e| anyhow::anyhow!("Failed to parse company selector: {:?}", e))?;
    
    let location_selector = Selector::parse(".job-search-card__location, .job-card-container__metadata-item")
        .map_err(|e| anyhow::anyhow!("Failed to parse location selector: {:?}", e))?;
    
    let link_selector = Selector::parse("a.base-card__full-link, a.job-card-list__title")
        .map_err(|e| anyhow::anyhow!("Failed to parse link selector: {:?}", e))?;
    
    let date_selector = Selector::parse("time[class*='job-search-card__listdate'], time.job-search-card__listdate, .job-search-card__listdate, .job-search-card__listdate--new")
        .map_err(|e| anyhow::anyhow!("Failed to parse date selector: {:?}", e))?;
    
    for job_element in document.select(&job_card_selector) {
        // Extract title
        let title = job_element
            .select(&title_selector)
            .next()
            .and_then(|e| e.text().next())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        
        // Extract company (handle nested <a> tags)
        let company = job_element
            .select(&company_selector)
            .next()
            .map(|e| {
                // Get all text from the element (including nested elements)
                e.text().collect::<Vec<_>>().join(" ").trim().to_string()
            })
            .unwrap_or_else(|| "Unknown".to_string());
        
        // Extract location
        let location = job_element
            .select(&location_selector)
            .next()
            .and_then(|e| e.text().next())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        
        // Extract URL
        let url = job_element
            .select(&link_selector)
            .next()
            .and_then(|e| e.value().attr("href"))
            .map(|href| {
                if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("https://www.linkedin.com{}", href)
                }
            })
            .unwrap_or_else(|| "".to_string());
        
        if url.is_empty() {
            continue; // Skip jobs without URLs
        }
        
        // Extract posted date
        let posted_date = job_element
            .select(&date_selector)
            .next()
            .and_then(|e| {
                // Try datetime attribute first
                e.value().attr("datetime")
                    .or_else(|| e.text().next())
            })
            .and_then(|date_str| {
                // Try parsing as ISO date first
                chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                    .ok()
                    .or_else(|| parse_relative_date(date_str))
            });
        
        // DEBUG: Print what we extracted
        println!("   📝 Extracted: '{}' at '{}' in '{}' (date: {:?})", 
                 title, company, location, posted_date);
        
        let id = generate_job_id(&url);
        
        jobs.push(JobPosting::new(
            id,
            title,
            company,
            location,
            url,
            posted_date,
            None, // Description would require another request
        ));
    }
    
    // DEBUG: Show parsing results
    println!("✅ Successfully parsed {} jobs", jobs.len());
    if !jobs.is_empty() {
        println!("   First job: {} at {}", jobs[0].title, jobs[0].company);
    }
    
    Ok(jobs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_search_url() {
        let url = build_search_url("rust developer", "San Francisco");
        assert!(url.contains("keywords=rust+developer"));
        assert!(url.contains("location=San+Francisco"));
        assert!(url.contains("f_TPR=r86400"));
    }

    #[test]
    fn test_generate_job_id() {
        let url1 = "https://linkedin.com/jobs/view/123";
        let url2 = "https://linkedin.com/jobs/view/123";
        let url3 = "https://linkedin.com/jobs/view/456";
        
        assert_eq!(generate_job_id(url1), generate_job_id(url2));
        assert_ne!(generate_job_id(url1), generate_job_id(url3));
    }

    #[test]
    fn test_parse_relative_date() {
        assert_eq!(
            parse_relative_date("2 hours ago"),
            Some(chrono::Local::now().date_naive())
        );
        assert_eq!(
            parse_relative_date("1 day ago"),
            Some(chrono::Local::now().date_naive())
        );
        assert_eq!(parse_relative_date("5 days ago"), None);
    }

    #[test]
    fn test_parse_jobs_from_html() {
        let html = r#"
            <div class="base-card" data-job-id="123">
                <a class="base-card__full-link" href="/jobs/view/123">
                    <h3 class="base-search-card__title">Software Engineer</h3>
                    <h4 class="base-search-card__subtitle">Tech Corp</h4>
                    <span class="job-search-card__location">San Francisco, CA</span>
                    <time class="job-search-card__listdate" datetime="2024-01-15">1 day ago</time>
                </a>
            </div>
        "#;
        
        let jobs = parse_jobs_from_html(html).unwrap();
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].title, "Software Engineer");
        assert_eq!(jobs[0].company, "Tech Corp");
    }

    #[test]
    fn test_parse_empty_html() {
        let html = "<html><body></body></html>";
        let jobs = parse_jobs_from_html(html).unwrap();
        assert_eq!(jobs.len(), 0);
    }
}
