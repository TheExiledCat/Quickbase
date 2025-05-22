use std::{collections::HashMap, sync::Arc};

use axum::{
    Router,
    extract::{self, State},
    response::Json,
    routing::get,
};
use axum_folder_router::folder_router;
use libqbase::schema::Schema;
use serde_json::{Value, json};

struct AppState {
    pub schema: Schema,
}

type App = Arc<AppState>;

#[folder_router("./api", App)]
struct ControllerRouter();

#[tokio::main]
async fn main() {
    let schema = Schema::default_schema();
    let mut state = Arc::new(AppState { schema });
    let controller: Router<App> = ControllerRouter::into_router();
    let app = controller.with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
