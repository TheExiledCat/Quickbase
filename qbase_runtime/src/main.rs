use std::{collections::HashMap, sync::Arc};

use axum::{
    Router,
    extract::{self, Request, State},
    middleware::{self, Next},
    response::{IntoResponse, Json, Response},
    routing::get,
};
use axum_folder_router::folder_router;
use libqbase::schema::Schema;
use serde_json::{Value, json};
use tower::ServiceBuilder;

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
    let app = controller
        .layer(middleware::from_fn(logger_middleware))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}

enum LogLevel {
    Info,
}
const log_level: LogLevel = LogLevel::Info;
async fn logger_middleware(request: Request, next: Next) -> Response {
    println!("{} {}", request.method(), request.uri());
    return next.run(request).await;
}
