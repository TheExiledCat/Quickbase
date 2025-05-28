use crate::App;
use axum::response::IntoResponse;
use axum::{Json, extract::State};
use libqbase::schema::Schema;
use serde_json::Value;
use serde_json::json;
pub async fn get(State(app): State<App>) -> impl IntoResponse {
    println!("Schema requested");
    return Json(json!(app.read().unwrap().schema)).into_response();
}
pub async fn post(State(app): State<App>, Json(schema): Json<Schema>) {}
