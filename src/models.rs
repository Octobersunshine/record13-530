use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub timezone: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: i64,
    pub balance: i64,
    pub reason: String,
    pub expire_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpiringPoint {
    pub id: Uuid,
    pub amount: i64,
    pub balance: i64,
    pub reason: String,
    pub expire_at: DateTime<Utc>,
    pub days_until_expiry: i64,
    pub expire_date_utc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpiringPointsResponse {
    pub user_id: Uuid,
    pub username: String,
    pub total_expiring_points: i64,
    pub expiring_points: Vec<ExpiringPoint>,
    pub query_time: DateTime<Utc>,
    pub expire_within_days: i64,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearSummary {
    pub total_users: usize,
    pub total_points_to_clear: i64,
    pub clear_cutoff_time: DateTime<Utc>,
    pub query_time: DateTime<Utc>,
    pub expire_within_days: i64,
    pub timezone: String,
    pub details: Vec<UserClearDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClearDetail {
    pub user_id: Uuid,
    pub username: String,
    pub points_to_clear: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearResult {
    pub cleared_points: i64,
    pub affected_users: usize,
    pub clear_before: DateTime<Utc>,
    pub executed_at: DateTime<Utc>,
    pub timezone: String,
}
