#![allow(dead_code)]
use axum::{Extension, Json};
use serde::Serialize;
use serde_json::json;
use sqlx::{PgPool, Row, query_as};
use tokio::join;
use redis::Client;

// ---------- Data Models ----------

#[derive(Serialize, Debug, sqlx::FromRow, Clone)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct OrderStats {
    pub total_orders: i64,
}

// ---------- Async Helper Functions ----------

pub async fn fetch_user_info(pool: &PgPool, user_id: i32) -> sqlx::Result<UserInfo> {
    let user = query_as::<_, UserInfo>(
        "SELECT id, username FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn fetch_order_stats(pool: &PgPool, user_id: i32) -> sqlx::Result<OrderStats> {
    let total = sqlx::query(
        "SELECT COUNT(*) as total_orders FROM orders WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?
    .get::<i64, _>("total_orders");

    Ok(OrderStats {
        total_orders: total,
    })
}

// ---------- Redis Helper Functions ----------

pub fn get_user_cache(redis: &Client, user_id: i32) -> Option<UserInfo> {
    let mut conn = redis.get_connection().ok()?;
    let cached: String = redis::cmd("GET")
        .arg(format!("user:{}", user_id))
        .query(&mut conn)
        .ok()?;
    
    serde_json::from_str(&cached).ok()
}

pub fn set_user_cache(redis: &Client, user_id: i32, user: &UserInfo) {
    if let Ok(mut conn) = redis.get_connection() {
        let json = serde_json::to_string(user).unwrap_or_default();
        let _: () = redis::cmd("SETEX")
            .arg(format!("user:{}", user_id))
            .arg(3600) 
            .arg(json)
            .query(&mut conn)
            .unwrap_or(());
    }
}

pub fn get_stats_cache(redis: &Client, user_id: i32) -> Option<OrderStats> {
    let mut conn = redis.get_connection().ok()?;
    let cached: String = redis::cmd("GET")
        .arg(format!("stats:{}", user_id))
        .query(&mut conn)
        .ok()?;
    
    serde_json::from_str(&cached).ok()
}

pub fn set_stats_cache(redis: &Client, user_id: i32, stats: &OrderStats) {
    if let Ok(mut conn) = redis.get_connection() {
        let json = serde_json::to_string(stats).unwrap_or_default();
        let _: () = redis::cmd("SETEX")
            .arg(format!("stats:{}", user_id))
            .arg(3600) 
            .arg(json)
            .query(&mut conn)
            .unwrap_or(());
    }
}

// ---------- Main Handler ----------

pub async fn get_user_dashboard(
    Extension(pool): Extension<PgPool>,
    Extension(redis): Extension<Client>,
) -> Result<Json<serde_json::Value>, String> {
    let user_id = 1; // Hardcoded for demo

    // Try to get from cache first
    let user_future = async {
        if let Some(cached_user) = get_user_cache(&redis, user_id) {
            Ok(cached_user)
        } else {
            let user = fetch_user_info(&pool, user_id).await?;
            set_user_cache(&redis, user_id, &user);
            Ok(user)
        }
    };

    let stats_future = async {
        if let Some(cached_stats) = get_stats_cache(&redis, user_id) {
            Ok(cached_stats)
        } else {
            let stats = fetch_order_stats(&pool, user_id).await?;
            set_stats_cache(&redis, user_id, &stats);
            Ok(stats)
        }
    };

    let (user_result, stats_result) = join!(user_future, stats_future);

    let user = user_result.map_err(|e| e.to_string())?;
    let stats = stats_result.map_err(|e| e.to_string())?;

    Ok(Json(json!({
        "user": user,
        "stats": stats
    })))
}