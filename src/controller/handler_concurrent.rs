use axum::{Extension, Json};
use serde::Serialize;
use serde_json::json;
use sqlx::{PgPool, Row, query_as};
use tokio::join;

// ---------- Data Models ----------

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
}

#[derive(Serialize, Debug)]
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

// ---------- Main Handler ----------

pub async fn get_user_dashboard(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<serde_json::Value>, String> {
    let user_id = 1; // Hardcoded for demo

    let (user_result, stats_result) = join!(
        fetch_user_info(&pool, user_id),
        fetch_order_stats(&pool, user_id)
    );

    let user = user_result.map_err(|e| e.to_string())?;
    let stats = stats_result.map_err(|e| e.to_string())?;

    Ok(Json(json!({
        "user": user,
        "stats": stats
    })))
}
