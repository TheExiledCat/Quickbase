use axum::{
    Json,
    extract::State,
    response::{Html, IntoResponse, Response},
};
use hyper::StatusCode;
use libqbase::{datasource::DataSource, datasources::sqlite::SqliteDataSource, hashing};
use serde_json::{Value, json};

use crate::App;

pub struct LoginDto {
    pub identifier: String,
    pub password: String,
}

pub enum JsonOrFail {
    Json(Json<Value>),
    Fail(StatusCode, String),
}
pub async fn post(
    State(state): State<App>,
    Json(login): Json<LoginDto>,
) -> Result<Response, Response> {
    let hashed_pw = hashing::hash_sha256(&login.password);
    let admin_id: Option<u32> = match state
        .read()
        .unwrap()
        .schema
        .get_settings()
        .get_source_type()
    {
        libqbase::datasource::DataSourceType::SQLITE { connection_string } => {
            let source = SqliteDataSource::new(connection_string.clone());
            source.get_admin_login(login.identifier, hashed_pw)
        }
        libqbase::datasource::DataSourceType::PSQL { connection_string } => todo!(),
        libqbase::datasource::DataSourceType::MARIA_DB { connection_string } => todo!(),
    };
    let token;
    if let Some(id) = admin_id {
        token = crate::create_jwt(&id.to_string());
    } else {
        return Err((
            StatusCode::UNAUTHORIZED,
            Html("<h1>Unknown identifier or password</h1>"),
        )
            .into_response())
        .into_response();
    }

    return Ok((StatusCode::OK, Json(json!(token))).into_response());
}
