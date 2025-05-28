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
        return Self {
            database_file: database_file.into(),
        };
    }
}
impl Migrator for SqliteMigrator {
    fn migrate(
        &mut self,
        changes: Vec<crate::migrator::SchemaChange>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn ensure_created(&mut self, schema: &Schema) -> Result<(), Box<dyn std::error::Error>> {
        let source = SqliteDataSource::new(self.database_file.clone());
        if !source.database_exists() {
            if let Err(_e) = source.create_database(schema) {
                panic!("Something went wrong during database creation");
            }
        }

        return Ok(());
    }
}
