use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use hyper::StatusCode;
use libqbase::schema_records::{EntityFieldRecord, EntityRecord};
use serde::Deserialize;

use crate::App;
#[derive(Debug, Deserialize)]
pub struct DatabaseFilterParams {
    filter: Option<String>,
    sortBy: Option<String>,
    select: Option<String>,
}

pub async fn post(
    State(state): State<App>,
    Path(name): Path<String>,
    Query(params): Query<DatabaseFilterParams>,
    Json(entity_record): Json<EntityRecord>,
) -> impl IntoResponse {
    if let Some(entity) = state
        .read()
        .unwrap()
        .schema
        .get_entity_by_name(&entity_record.name)
    {
        match state
            .read()
            .unwrap()
            .schema
            .get_settings()
            .get_source()
            .insert_entity(&entity, entity_record)
        {
            Ok(_) => return (StatusCode::OK, "0"),
            Err(err) => match err {
                _ => return (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL SERVER ERROR"),
            },
        }
    } else {
        return (StatusCode::NOT_FOUND, "ENTITY NOT FOUND");
    }
}
