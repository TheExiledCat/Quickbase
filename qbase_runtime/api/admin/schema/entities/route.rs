use axum::{
    Json,
    extract::State,
    response::{Html, IntoResponse},
};
use hyper::StatusCode;
use libqbase::{
    migrator::SchemaChange,
    schema::{Entity, EntityFieldType, EntityType},
};
use ts_rs::TS;

use crate::App;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct EntityCreationDTO {
    pub kind: EntityType,
    pub name: String,
}
pub async fn post(
    State(state): State<App>,
    Json(entity): Json<EntityCreationDTO>,
) -> impl IntoResponse {
    let new_entity = Entity::new(&entity.name, entity.kind);
    let copy = new_entity.clone();
    let new_change: SchemaChange;
    {
        match state.write().unwrap().schema.add_entity(new_entity) {
            Ok(change) => new_change = change,
            Err(e) => {
                return (
                    StatusCode::CONFLICT,
                    Err(Json("<h1>Entity Creation Failed</h1>".to_owned())),
                );
            }
        }
    }
    state
        .read()
        .unwrap()
        .migrator
        .migrate(vec![new_change], &state.read().unwrap().schema);
    return (StatusCode::OK, Ok(Json(copy)));
}
