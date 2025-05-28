use rusqlite::Error;

use crate::{
    datasource::{DataResult, DataSource},
    migrator::Migrator,
    schema::Schema,
};

use super::sqlite::SqliteDataSource;

pub struct SqliteMigrator {
    database_file: String,
}
impl SqliteMigrator {
    pub fn new(database_file: &str) -> Self {
        let source = SqliteDataSource::new(database_file.into());
        return Self {
            database_file: database_file.into(),
        };
    }
}
impl Migrator for SqliteMigrator {
    fn migrate(&self, changes: Vec<crate::migrator::SchemaChange>) -> Result<(), rusqlite::Error> {
        todo!()
    }

    fn ensure_created(&self, schema: &Schema) -> Result<(), Error> {
        let mut source = SqliteDataSource::new(self.database_file.clone());
        println!("Creating db");
        return source.create_database(schema);
    }
}
