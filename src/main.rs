mod handlers;
mod models;
mod store;
mod time_utils;

use axum::routing::{get, post};
use axum::Router;
use chrono::{Duration, Utc};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

use crate::models::{PointRecord, User};
use crate::store::AppState;

async fn init_test_data(state: &AppState) {
    let user1 = User {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
        timezone: Some("Asia/Shanghai".to_string()),
        created_at: Utc::now(),
    };

    let user2 = User {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap(),
        username: "bob".to_string(),
        email: "bob@example.com".to_string(),
        timezone: Some("America/New_York".to_string()),
        created_at: Utc::now(),
    };

    let user3 = User {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440003").unwrap(),
        username: "charlie".to_string(),
        email: "charlie@example.com".to_string(),
        timezone: Some("Europe/London".to_string()),
        created_at: Utc::now(),
    };

    state.add_user(user1.clone()).await;
    state.add_user(user2.clone()).await;
    state.add_user(user3.clone()).await;

    let points = vec![
        PointRecord {
            id: Uuid::new_v4(),
            user_id: user1.id,
            amount: 1000,
            balance: 1000,
            reason: "签到奖励".to_string(),
            expire_at: Utc::now() + Duration::days(7),
            created_at: Utc::now(),
        },
        PointRecord {
            id: Uuid::new_v4(),
            user_id: user1.id,
            amount: 500,
            balance: 500,
            reason: "购物返现".to_string(),
            expire_at: Utc::now() + Duration::days(15),
            created_at: Utc::now(),
        },
        PointRecord {
            id: Uuid::new_v4(),
            user_id: user1.id,
            amount: 2000,
            balance: 2000,
            reason: "活动奖励".to_string(),
            expire_at: Utc::now() + Duration::days(45),
            created_at: Utc::now(),
        },
        PointRecord {
            id: Uuid::new_v4(),
            user_id: user2.id,
            amount: 3000,
            balance: 3000,
            reason: "新用户注册".to_string(),
            expire_at: Utc::now() + Duration::days(3),
            created_at: Utc::now(),
        },
        PointRecord {
            id: Uuid::new_v4(),
            user_id: user2.id,
            amount: 800,
            balance: 800,
            reason: "评价奖励".to_string(),
            expire_at: Utc::now() + Duration::days(25),
            created_at: Utc::now(),
        },
        PointRecord {
            id: Uuid::new_v4(),
            user_id: user3.id,
            amount: 1500,
            balance: 1500,
            reason: "积分兑换".to_string(),
            expire_at: Utc::now() + Duration::days(60),
            created_at: Utc::now(),
        },
        PointRecord {
            id: Uuid::new_v4(),
            user_id: user1.id,
            amount: 300,
            balance: 0,
            reason: "已使用".to_string(),
            expire_at: Utc::now() + Duration::days(5),
            created_at: Utc::now(),
        },
    ];

    for point in points {
        state.add_point(point).await;
    }

    tracing::info!("测试数据初始化完成");
    tracing::info!("用户ID: alice (Asia/Shanghai) -> 550e8400-e29b-41d4-a716-446655440001");
    tracing::info!("用户ID: bob (America/New_York) -> 550e8400-e29b-41d4-a716-446655440002");
    tracing::info!("用户ID: charlie (Europe/London) -> 550e8400-e29b-41d4-a716-446655440003");
}

fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/users", get(handlers::list_users))
        .route("/points", get(handlers::list_points))
        .route(
            "/users/:user_id/expiring-points",
            get(handlers::get_user_expiring_points),
        )
        .route("/clear-summary", get(handlers::get_clear_summary))
        .route("/clear-expired", post(handlers::execute_clear_expired_points))
        .layer(cors)
        .with_state(state)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "points_expiry_api=debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::new();
    init_test_data(&state).await;

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("服务启动，监听端口: 3000");
    tracing::info!("系统时区: UTC (所有时间均使用UTC标准时间)");
    tracing::info!("API 文档:");
    tracing::info!("  GET  /health -> 健康检查");
    tracing::info!("  GET  /users -> 获取所有用户");
    tracing::info!("  GET  /points -> 获取所有积分记录");
    tracing::info!("  GET  /users/:user_id/expiring-points?days=30 -> 查询用户即将过期积分");
    tracing::info!("  GET  /clear-summary?days=30 -> 统计待清零积分总额");
    tracing::info!("  POST /clear-expired?days=0 -> 执行过期积分清零");

    axum::serve(listener, app).await.unwrap();
}
