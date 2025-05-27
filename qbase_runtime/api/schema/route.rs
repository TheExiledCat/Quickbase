use crate::App;
use axum::response::IntoResponse;
use axum::{Json, extract::State};
use serde_json::Value;
use serde_json::json;
pub async fn get(State(app): State<App>) -> impl IntoResponse {
    println!("Schema requested");
    return Json(json!(app.schema)).into_response();
}
