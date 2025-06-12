use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use hyper::StatusCode;

use crate::{App, decode_jwt};

pub async fn get(State(state): State<App>, Path(token): Path<String>) -> impl IntoResponse {
    if let Some(decoded) = decode_jwt(&token) {
        match decoded.1 {
            crate::AuthRoleClaim::AUTH => {
                return (StatusCode::UNAUTHORIZED, "NOT AUTHORIZED");
            }
            crate::AuthRoleClaim::ADMIN => return (StatusCode::OK, "VALID TOKEN"),
        }
    }

    return (StatusCode::UNAUTHORIZED, "NOT AUTHORIZED");
}
