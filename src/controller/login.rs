use axum::{http::status, response::Response, Extension, Json};
use hyper::Body;
use serde_json::json;
use sqlx::PgPool;

use crate::model::login::LoginReq;


pub async fn get_authenticate(
    Extension(pg_pool): Extension<PgPool>,
    Json(LoginReq { username, password }): Json<LoginReq>,
) -> Result<Response<Body>, Response<Body>> {
        
    let res = sqlx::query(
        r#"SELECT 
        username, password
        from users where username=$1 and password=$2"#,
    )
    .bind(&username)
    .bind(password)
    .fetch_one(&pg_pool)
    .await;

    match res {
        Ok(_user) => {
            let json = json!({
                "username": username,
                "staus": "Authenticated"
            });
            let body = Body::from(serde_json::to_vec(&json).unwrap());
            Ok(Response::builder()
                .status(status::StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap())
        }
        Err(_e) => {
            let json = json!({
                "status": "Failed"
            });
            let body = Body::from(serde_json::to_vec(&json).unwrap());
            Ok(Response::builder()
                .status(status::StatusCode::UNAUTHORIZED)
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap())
        }
    }
}
