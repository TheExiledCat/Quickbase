use rusqlite::Error;
use serde::{Deserialize, Serialize};

use crate::{
    migrator::SchemaChange,
    schema::{Entity, Schema},
};
#[derive(Debug, Deserialize, Serialize)]
pub enum DataSourceType {
    SQLITE { connection_string: String },
    PSQL { connection_string: String },
    MARIA_DB { connection_string: String },
}
#[derive(Debug)]
pub enum DataSourceError {
    CONNECTION_FAILED,
    SYNTAX_ERROR,
    CREATION_FAILED,
    ENTITY_CREATION_FAILED,
}
pub type DataResult = Result<(), Error>;
pub trait DataSource {
    //Create
    fn create_database(&mut self, schema: &Schema) -> DataResult;
    fn create_table(&self, table: &Entity) -> DataResult;

    //Remove
    fn delete_database(&self) -> DataResult;
    fn delete_table(&self, table: &Entity) -> DataResult;

    //Update
    fn update_table_name(&self, change: &SchemaChange) -> DataResult;
    fn update_table_field(&self, change: &SchemaChange) -> DataResult;

    //Validate
    fn database_exists(&self) -> bool;
}
