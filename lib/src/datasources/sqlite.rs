use std::fs;

use rusqlite::fallible_iterator::Unwrap;
use rusqlite::{self, Connection, Result};

use crate::datasource::{DataResult, DataSource, DataSourceError};
use crate::migrator::SchemaChange;
use crate::schema::{Entity, EntityFieldType, Schema};
pub struct SqliteDataSource {
    source: String,
    conn: Connection,
    logger: Option<fn(&str) -> ()>,
}
impl SqliteDataSource {
    pub fn new(database_name: String) -> Self {
        return SqliteDataSource {
            source: database_name.clone(),
            conn: Connection::open(&database_name).expect("Cannot open database"),
            logger: None,
        };
    }
    pub fn new_with_logger(database_name: String, log_func: fn(&str) -> ()) -> Self {
        return SqliteDataSource {
            source: database_name.clone(),
            conn: Connection::open(&database_name).expect("Cannot open database"),
            logger: Some(log_func),
        };
    }
}

impl DataSource for SqliteDataSource {
    fn create_database(&mut self, schema: &Schema) -> DataResult {
        // if there can only be one database at once, if it exists already, delete it.
        match fs::exists(&self.source) {
            Ok(val) => {
                if val == true {
                    fs::remove_file(&self.source);
                    self.conn = Connection::open(&self.source).unwrap();
                }
            }
            Err(_) => panic!("Something went horribly wrong"),
        }
        println!("creating tables");
        for entity in schema.get_entities() {
            if let Err(e) = self.create_table(entity) {
                return Err(e);
            }
        }

        return Ok(());
    }

    fn create_table(&self, table: &Entity) -> DataResult {
        let mut query = String::from(format!("CREATE TABLE {}(\n", table.name));
        let fields = table.get_fields();
        for (i, field) in table.get_fields().iter().enumerate() {
            query.push_str(&format!("{} ", field.get_name()));
            query.push_str(match field.get_type() {
                EntityFieldType::TEXT {
                    min,
                    max,
                    validate,
                    generate,
                } => "TEXT ",
                EntityFieldType::NUMBER { min, max, is_int } => {
                    if *is_int {
                        "INTEGER "
                    } else {
                        "REAL "
                    }
                }
                EntityFieldType::BOOL
                | EntityFieldType::DATE { .. }
                | EntityFieldType::RELATION { .. } => "INTEGER ",
                EntityFieldType::RELATION_MANY { entity_names } => todo!(),
            });

            if !field.is_nullable() {
                query.push_str("NOT NULL");
            }

            if i < fields.len() - 1 {
                query.push_str(",");
            }
        }
        query.push_str(")");
        if let Some(log) = self.logger {
            log(&query);
        };
        println!("{}", &query);

        match self.conn.execute(&query, []) {
            Ok(_) => (),
            Err(e) => {
                if let Some(log) = self.logger {
                    log(&format!("{}", e));
                    return Err(e);
                }
            }
        }

        return Ok(());
    }

    fn delete_database(&self) -> DataResult {
        todo!()
    }

    fn delete_table(&self, table: &Entity) -> DataResult {
        todo!()
    }

    fn update_table_name(&self, change: &SchemaChange) -> DataResult {
        todo!()
    }

    fn update_table_field(&self, change: &SchemaChange) -> DataResult {
        todo!()
    }

    fn database_exists(&self) -> bool {
        return fs::exists(&self.source).unwrap();
    }
}
