use std::sync::Arc;

use axum::{
    Router,
    extract::{self, State},
    response::Json,
    routing::get,
};
use libqbase::schema::Schema;
use serde_json::{Value, json};

struct AppState {
    pub schema: Schema,
}

type App = State<Arc<AppState>>;
#[tokio::main]
async fn main() {
    let mut schema = Schema::default_schema();

    let mut state = Arc::new(AppState { schema });
    let app = Router::new()
        .route("/api/v1/schema", get(get_schema))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

//schema routes
async fn get_schema(State(app): App) -> Json<Value> {
    return Json(json!(app.schema));
}
