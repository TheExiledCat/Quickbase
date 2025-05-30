use chrono::NaiveWeek;
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
        return Self {
            database_file: database_file.into(),
        };
    }
}
impl Migrator for SqliteMigrator {
    fn migrate(&self, changes: Vec<crate::migrator::SchemaChange>) -> Result<(), rusqlite::Error> {
        let source = SqliteDataSource::new(self.database_file.clone());

        for change in changes {
            match change {
                crate::migrator::SchemaChange::ADD_ENTITY(entity) => {
                    source.create_table(&entity).unwrap()
                }
                crate::migrator::SchemaChange::REMOVE_ENTITY(entity) => todo!(),
                crate::migrator::SchemaChange::RENAME_ENTITY(entity, _) => todo!(),
                crate::migrator::SchemaChange::CHANGE_ENTITY_FIELD(entity, field_changes) => {
                    todo!()
                }
            }
        }

        return Ok(());
    }

    fn ensure_created(&self, schema: &Schema) -> Result<Option<Schema>, Error> {
        let mut source = SqliteDataSource::new(self.database_file.clone());
        println!("Creating db");
        return match source.create_database(schema) {
            Ok(_) => Ok(None),
            Err(e) => match e {
                Error::ExecuteReturnedResults => {
                    //return the schema from the db

                    let loaded_schema = source.load_schema();
                    return Ok(Some(loaded_schema));
                }
                _ => {
                    return Err(e);
                }
            },
        };
    }
}
