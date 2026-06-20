use std::collections::HashMap;
use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::{
    ClearResult, ClearSummary, ExpiringPoint, ExpiringPointsResponse, PointRecord, User,
    UserClearDetail,
};
use crate::time_utils;

pub const SYSTEM_TIMEZONE: &str = "UTC";

#[derive(Clone, Default)]
pub struct AppState {
    pub users: Arc<RwLock<HashMap<Uuid, User>>>,
    pub points: Arc<RwLock<HashMap<Uuid, PointRecord>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            points: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_user(&self, user: User) {
        let mut users = self.users.write().await;
        users.insert(user.id, user);
    }

    pub async fn add_point(&self, point: PointRecord) {
        let mut points = self.points.write().await;
        points.insert(point.id, point);
    }

    pub async fn get_user(&self, user_id: Uuid) -> Option<User> {
        let users = self.users.read().await;
        users.get(&user_id).cloned()
    }

    pub async fn get_expiring_points(
        &self,
        user_id: Uuid,
        expire_within_days: i64,
    ) -> Option<ExpiringPointsResponse> {
        let user = self.get_user(user_id).await?;
        let points = self.points.read().await;
        let now = time_utils::now_utc();
        let cutoff = now + Duration::days(expire_within_days);

        let mut expiring_points: Vec<ExpiringPoint> = Vec::new();
        let mut total: i64 = 0;

        for point in points.values() {
            if point.user_id == user_id && point.balance > 0 && point.expire_at <= cutoff {
                let days_until_expiry =
                    time_utils::days_until_expiry(point.expire_at, now);
                let expire_date_utc = point.expire_at.format("%Y-%m-%d").to_string();
                total += point.balance;
                expiring_points.push(ExpiringPoint {
                    id: point.id,
                    amount: point.amount,
                    balance: point.balance,
                    reason: point.reason.clone(),
                    expire_at: point.expire_at,
                    days_until_expiry,
                    expire_date_utc,
                });
            }
        }

        expiring_points.sort_by_key(|p| p.expire_at);

        Some(ExpiringPointsResponse {
            user_id: user.id,
            username: user.username,
            total_expiring_points: total,
            expiring_points,
            query_time: now,
            expire_within_days,
            timezone: SYSTEM_TIMEZONE.to_string(),
        })
    }

    pub async fn get_clear_summary(&self, expire_within_days: i64) -> ClearSummary {
        let users = self.users.read().await;
        let points = self.points.read().await;
        let now = time_utils::now_utc();
        let cutoff = now + Duration::days(expire_within_days);

        let mut user_totals: HashMap<Uuid, i64> = HashMap::new();

        for point in points.values() {
            if point.balance > 0 && point.expire_at <= cutoff {
                *user_totals.entry(point.user_id).or_insert(0) += point.balance;
            }
        }

        let mut details: Vec<UserClearDetail> = Vec::new();
        let mut total_points: i64 = 0;

        for (user_id, points_to_clear) in user_totals {
            if let Some(user) = users.get(&user_id) {
                total_points += points_to_clear;
                details.push(UserClearDetail {
                    user_id: user.id,
                    username: user.username.clone(),
                    points_to_clear,
                });
            }
        }

        details.sort_by(|a, b| b.points_to_clear.cmp(&a.points_to_clear));

        ClearSummary {
            total_users: details.len(),
            total_points_to_clear: total_points,
            clear_cutoff_time: cutoff,
            query_time: now,
            expire_within_days,
            timezone: SYSTEM_TIMEZONE.to_string(),
            details,
        }
    }

    pub async fn clear_expired_points(&self, before: DateTime<Utc>) -> ClearResult {
        let mut points = self.points.write().await;
        let mut total_cleared: i64 = 0;
        let mut affected_users: HashMap<Uuid, bool> = HashMap::new();

        for point in points.values_mut() {
            if point.balance > 0 && point.expire_at <= before {
                total_cleared += point.balance;
                affected_users.insert(point.user_id, true);
                point.balance = 0;
            }
        }

        ClearResult {
            cleared_points: total_cleared,
            affected_users: affected_users.len(),
            clear_before: before,
            executed_at: time_utils::now_utc(),
            timezone: SYSTEM_TIMEZONE.to_string(),
        }
    }
}
