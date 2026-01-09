// Sync endpoints

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post, Router},
};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;

#[derive(Serialize, Deserialize)]
pub struct SyncPullRequest {
    pub last_sync_time: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SyncPushRequest {
    pub changes: Vec<serde_json::Value>,
    pub last_sync_time: Option<String>,
}

#[derive(Serialize)]
pub struct SyncPullResponse {
    pub changes: Vec<serde_json::Value>,
    pub sync_time: String,
}

#[derive(Serialize)]
pub struct SyncPushResponse {
    pub synced: usize,
    pub sync_time: String,
}

#[derive(Serialize)]
pub struct SyncStatusResponse {
    pub last_sync_time: Option<String>,
    pub pending_changes: usize,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/pull", post(pull_changes))
        .route("/push", post(push_changes))
        .route("/status", get(sync_status))
}

async fn pull_changes(
    State(_state): State<AppState>,
    Json(_payload): Json<SyncPullRequest>,
) -> Result<Json<SyncPullResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implement sync pull logic
    Ok(Json(SyncPullResponse {
        changes: vec![],
        sync_time: chrono::Utc::now().to_rfc3339(),
    }))
}

async fn push_changes(
    State(_state): State<AppState>,
    Json(payload): Json<SyncPushRequest>,
) -> Result<Json<SyncPushResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implement sync push logic
    Ok(Json(SyncPushResponse {
        synced: payload.changes.len(),
        sync_time: chrono::Utc::now().to_rfc3339(),
    }))
}

async fn sync_status(
    State(_state): State<AppState>,
) -> Result<Json<SyncStatusResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Query sync status from database
    Ok(Json(SyncStatusResponse {
        last_sync_time: None,
        pending_changes: 0,
    }))
}

