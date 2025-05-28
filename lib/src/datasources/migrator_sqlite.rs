use crate::{
    datasource::{DataResult, DataSource},
    migrator::Migrator,
    schema::Schema,
};

use super::sqlite::SqliteDataSource;

pub struct SqliteMigrator<'a> {
    data_source: &'a SqliteDataSource,
}
impl<'a> SqliteMigrator<'a> {
    pub fn new(source: &'a SqliteDataSource) -> Self {
        return Self {
            data_source: source,
        };
    }
}
impl<'a> Migrator for SqliteMigrator<'a> {
    fn migrate(
        &mut self,
        changes: Vec<crate::migrator::SchemaChange>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn ensure_created(&mut self, schema: &Schema) -> Result<(), Box<dyn std::error::Error>> {
        if !self.data_source.database_exists() {
            if let Err(_e) = self.data_source.create_database(schema) {
                panic!("Something went wrong during database creation");
            }
        }

        return Ok(());
    }
}
