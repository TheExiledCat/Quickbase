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

use crate::App;
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Serialize, Deserialize)]
pub struct EntityCreationDTO {
    pub kind: EntityType,
    pub name: String,
}
pub async fn post(
    State(state): State<App>,
    Json(entity): Json<EntityCreationDTO>,
) -> impl IntoResponse {
    let new_entity = Entity::new(&entity.name, entity.kind);
    let new_change: SchemaChange;
    {
        match state.write().unwrap().schema.add_entity(new_entity) {
            Ok(change) => new_change = change,
            Err(e) => {
                return (
                    StatusCode::CONFLICT,
                    Html("<h1>Entity Creation Failed</h1>".to_owned()),
                );
            }
        }
    }
    state.read().unwrap().migrator.migrate(vec![new_change]);
    return (
        StatusCode::OK,
        Html(String::from(format!(
            "<h1>Entity Created: {}</h1>",
            &entity.name
        ))),
    );
}
