use std::sync::{Arc, Mutex, RwLock};

use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
};
use axum_folder_router::folder_router;
use libqbase::{
    datasource::{DataSource, DataSourceType},
    datasources::{migrator_sqlite::SqliteMigrator, sqlite::SqliteDataSource},
    migrator::{Migrator, SchemaComparator},
    schema::{Schema, SchemaError},
};

struct AppState {
    schema: Schema,
    migrator: Box<dyn Migrator>,
}

impl AppState {
    pub fn get_schema(&self) -> &Schema {
        return &self.schema;
    }
    pub fn try_set_schema(&mut self, schema: Schema) -> Result<&Schema, SchemaError> {
        if Schema::validate(&schema) {
            let changes = SchemaComparator::new().compare(&self.schema, &schema);
            self.schema = schema;

            return Ok(&self.schema);
        }
        return Err(SchemaError::Invalid);
    }
}
type App = Arc<RwLock<AppState>>;

#[folder_router("./api", App)]
struct ControllerRouter();

#[tokio::main]
async fn main() {
    //for now just create the default schema every time
    let schema = Schema::default_schema();
    let migrator;
    if let DataSourceType::SQLITE { connection_string } = schema.get_settings().get_source_type() {
        migrator = SqliteMigrator::new(&connection_string);
    } else {
        panic!("no support for other migrators yet");
    }
    migrator.ensure_created(&schema);
    let mut state = Arc::new(RwLock::new(AppState {
        schema,
        migrator: Box::new(migrator),
    }));
    let controller: Router<App> = ControllerRouter::into_router();
    let app = controller
        .layer(middleware::from_fn(logger_middleware))
        .with_state(state);
    let host = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    println!("server listening on http://{}", host);

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
