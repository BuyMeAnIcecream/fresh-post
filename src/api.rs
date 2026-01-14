use axum::{extract::State, http::StatusCode, routing::{get, post}, Json, Router};
use chrono::{DateTime, Local};
use std::sync::Arc;
use tokio::sync::Mutex;

use fresh_post::{config::Config, service, storage};

#[derive(Clone)]
pub struct AppState {
    pub paths: storage::Paths,
    pub last_run: Arc<Mutex<Option<DateTime<Local>>>>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/config", get(get_config).post(update_config))
        .route("/api/jobs", get(get_jobs))
        .route("/api/run", post(run_scrape))
        .with_state(state)
}

type ApiResult<T> = Result<Json<T>, (StatusCode, String)>;

async fn get_config(State(state): State<AppState>) -> ApiResult<Config> {
    let config = storage::load_config_or_default(&state.paths)
        .map_err(internal_error)?;
    Ok(Json(config))
}

async fn update_config(State(state): State<AppState>, Json(config): Json<Config>) -> ApiResult<Config> {
    storage::save_config(&state.paths, &config)
        .map_err(internal_error)?;
    Ok(Json(config))
}

async fn get_jobs(State(state): State<AppState>) -> ApiResult<storage::JobsSnapshot> {
    let snapshot = storage::load_latest_jobs(&state.paths)
        .map_err(internal_error)?;
    Ok(Json(snapshot))
}

async fn run_scrape(State(state): State<AppState>) -> ApiResult<service::ScrapeSummary> {
    let summary = service::run_scrape_once(&state.paths).await
        .map_err(internal_error)?;

    let mut guard = state.last_run.lock().await;
    *guard = Some(summary.updated_at);

    Ok(Json(summary))
}

fn internal_error<E: std::fmt::Display>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
