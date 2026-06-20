use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::models::ClearSummary;
use crate::store::AppState;

#[derive(Debug, Deserialize)]
pub struct ExpireWithinQuery {
    pub days: Option<i64>,
}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "ok", "timezone": "UTC" })))
}

pub async fn get_user_expiring_points(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Query(query): Query<ExpireWithinQuery>,
) -> impl IntoResponse {
    let days = query.days.unwrap_or(30);

    if days <= 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "days must be greater than 0" })),
        );
    }

    match state.get_expiring_points(user_id, days).await {
        Some(response) => (StatusCode::OK, Json(serde_json::to_value(response).unwrap())),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "User not found" })),
        ),
    }
}

pub async fn get_clear_summary(
    State(state): State<AppState>,
    Query(query): Query<ExpireWithinQuery>,
) -> Json<ClearSummary> {
    let days = query.days.unwrap_or(30);
    let days = days.max(0);
    let summary = state.get_clear_summary(days).await;
    Json(summary)
}

pub async fn execute_clear_expired_points(
    State(state): State<AppState>,
    Query(query): Query<ExpireWithinQuery>,
) -> impl IntoResponse {
    let days = query.days.unwrap_or(0);
    let cutoff = Utc::now() + chrono::Duration::days(days);
    let result = state.clear_expired_points(cutoff).await;

    (StatusCode::OK, Json(result))
}

pub async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
    let users = state.users.read().await;
    let users: Vec<_> = users.values().cloned().collect();
    (StatusCode::OK, Json(users))
}

pub async fn list_points(State(state): State<AppState>) -> impl IntoResponse {
    let points = state.points.read().await;
    let points: Vec<_> = points.values().cloned().collect();
    (StatusCode::OK, Json(points))
}
