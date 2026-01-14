use crate::{service, storage};
use chrono::Local;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

pub async fn run_scheduler(paths: storage::Paths, last_run: Arc<Mutex<Option<chrono::DateTime<Local>>>>) {
    loop {
        match service::run_scrape_once(&paths).await {
            Ok(summary) => {
                let mut guard = last_run.lock().await;
                *guard = Some(summary.updated_at);
                println!(
                    "[scheduler] ran scrape: total={}, today={}, new={} url={} at={} ",
                    summary.total_jobs,
                    summary.today_jobs,
                    summary.new_jobs,
                    summary.search_url,
                    summary.updated_at
                );
            }
            Err(err) => {
                eprintln!("[scheduler] scrape failed: {:#}", err);
            }
        }

        let interval_hours = storage::load_config_or_default(&paths)
            .map(|config| config.schedule.interval_hours)
            .unwrap_or(4);

        let sleep_seconds = interval_hours.saturating_mul(3600);
        println!("[scheduler] next run in {} seconds", sleep_seconds);
        sleep(Duration::from_secs(sleep_seconds)).await;
    }
}
