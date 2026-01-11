use crate::models::JobPosting;

/// Filter jobs to only include those posted today
pub fn filter_today_only(jobs: Vec<JobPosting>) -> Vec<JobPosting> {
    let today = chrono::Local::now().date_naive();
    
    jobs
        .into_iter()
        .filter(|job| {
            match job.posted_date {
                Some(date) => date == today,
                None => false, // Conservative: exclude if date unknown
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::JobPosting;

    #[test]
    fn test_filter_today_only() {
        let today = chrono::Local::now().date_naive();
        let yesterday = today - chrono::Duration::days(1);
        
        let jobs = vec![
            JobPosting::new(
                "1".to_string(),
                "Today Job".to_string(),
                "Company A".to_string(),
                "Location".to_string(),
                "https://example.com/1".to_string(),
                Some(today),
                None,
            ),
            JobPosting::new(
                "2".to_string(),
                "Yesterday Job".to_string(),
                "Company B".to_string(),
                "Location".to_string(),
                "https://example.com/2".to_string(),
                Some(yesterday),
                None,
            ),
            JobPosting::new(
                "3".to_string(),
                "No Date Job".to_string(),
                "Company C".to_string(),
                "Location".to_string(),
                "https://example.com/3".to_string(),
                None,
                None,
            ),
        ];

        let filtered = filter_today_only(jobs);
        
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].title, "Today Job");
    }

    #[test]
    fn test_filter_empty_list() {
        let jobs = vec![];
        let filtered = filter_today_only(jobs);
        assert_eq!(filtered.len(), 0);
    }
}
