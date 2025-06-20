#![allow(dead_code)]
use hyper::{Body, Response, StatusCode};
use serde::Serialize;
use serde_json::json;

// Handles successful responses
pub fn success_response<T: Serialize>(
    status_code: StatusCode,
    response_type: &str,
    message: &str,
    data: Option<T>,
) -> Response<Body> {
    let response_obj = json!({
        "message": message,
        "type": response_type,
        "code": status_code.as_u16(),
        "trace_id": "",
        "data": data
    });

    let response = json!({
        "apiresponse": "success",
        "success": [response_obj]
    });

    Response::builder()
        .status(status_code)
        .header("content-type", "application/json")
        .body(Body::from(response.to_string()))
        .unwrap()
}
 // Handles error responses
pub fn error_response(
    status_code: StatusCode,
    response_type: &str,
    message: &str,
) -> Response<Body> {
    let response_obj = json!({
        "message": message,
        "type": response_type,
        "code": status_code.as_u16(),
        "trace_id": ""
    });

    let response = json!({
        "apiresponse": "error",
        "error": [response_obj]
    });

    Response::builder()
        .status(status_code)
        .header("content-type", "application/json")
        .body(Body::from(response.to_string()))
        .unwrap()
}
