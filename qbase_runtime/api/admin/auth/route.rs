use axum::{
    Json,
    extract::State,
    response::{Html, IntoResponse, Response},
};
use hyper::StatusCode;
use libqbase::{datasource::DataSource, datasources::sqlite::SqliteDataSource, hashing};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{App, AuthRoleClaim, decode_jwt};
#[derive(Serialize, Deserialize)]
pub struct LoginDto {
    pub identifier: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginTokenDto {
    pub token: String,
    pub expiration: usize,
}
#[derive(Serialize, Deserialize)]
pub struct ValidateTokenDto {
    pub token: String,
}
pub enum JsonOrFail {
    Json(Json<Value>),
    Fail(StatusCode, String),
}
impl IntoResponse for JsonOrFail {
    fn into_response(self) -> Response {
        match self {
            JsonOrFail::Json(j) => (StatusCode::OK, j).into_response(),
            JsonOrFail::Fail(status, html) => (status, Html(html)).into_response(),
        }
    }
}

pub async fn post(State(state): State<App>, Json(login): Json<LoginDto>) -> impl IntoResponse {
    println!("login attempt");
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
        let token_data = crate::create_jwt(&id.to_string(), crate::AuthRoleClaim::ADMIN);
        token = LoginTokenDto {
            token: token_data.0,
            expiration: token_data.1,
        };
    } else {
        return JsonOrFail::Fail(
            StatusCode::UNAUTHORIZED,
            "Incorrect identifier or password".to_owned(),
        );
    }

    return JsonOrFail::Json(Json(json!(token)));
}
