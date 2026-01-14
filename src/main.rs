mod api;

use anyhow::Result;
use axum::routing::get_service;
use fresh_post::{scheduler, storage};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() -> Result<()> {
    let paths = storage::Paths::new()?;
    let last_run = Arc::new(Mutex::new(None));

    let app_state = api::AppState {
        paths: paths.clone(),
        last_run: last_run.clone(),
    };

    tokio::spawn(scheduler::run_scheduler(paths.clone(), last_run.clone()));

    let app = api::router(app_state)
        .route("/", get_service(ServeFile::new("web/index.html")))
        .route("/jobs", get_service(ServeFile::new("web/jobs.html")))
        .fallback_service(ServeDir::new("web"));

    let addr = "0.0.0.0:8080";
    println!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
