use std::fs;

use inquire::{Password, Text};
use rusqlite::fallible_iterator::Unwrap;
use rusqlite::{self, Connection, Result};
use serde_json::json;

use crate::datasource::{DataResult, DataSource, DataSourceError};
use crate::hashing;
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
        //if there are already tables, assume the database has been created fine.
        let query = "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name != 'android_metadata' AND name != 'sqlite_sequence';";
        let mut stmt = self.conn.prepare(query).unwrap();
        let mut rows = stmt.query([]).unwrap();
        println!("{}", query);
        if let Some(row) = rows.next().unwrap() {
            let table_count = row.get::<usize, u32>(0).unwrap();
            println!("tables: {}", table_count);
            if table_count > 0 {
                println!("database already exists");
                return Err(rusqlite::Error::ExecuteReturnedResults);
            }
        } else {
            panic!("error???")
        }
        let username = Text::new("username\n> ").prompt().expect("a valid string");
        let email = Text::new("email\n> ").prompt().expect("a valid string");
        let password = Password::new("password\n> ")
            .prompt()
            .expect("valid string");
        let hashed_pw = hashing::hash_sha256(&password);
        //create the admins table and root user, assume the password is already hashed
        let query = "CREATE TABLE qbase_admins(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            email TEXT NOT NULL,
            password_hash TEXT NOT NULL
        )";
        println!("{}", query);
        self.conn.execute(&query, []);

        let query = "
            INSERT INTO qbase_admins(username, email, password_hash) VALUES(?1,?2,?3)
        ";
        println!("{}", query);
        self.conn.execute(query, [username, email, hashed_pw]);
        //create schema
        println!("creating tables");
        for entity in schema.get_entities() {
            if let Err(e) = self.create_table(entity) {
                return Err(e);
            }
        }
        self.update_schema(&schema);
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

            if (field.get_name() == "id") {
                query.push_str("PRIMARY KEY");
            } else if !field.is_nullable() {
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

    fn load_schema(&self) -> Schema {
        let query = "SELECT schema FROM qbase_schema";
        let schema: Schema = serde_json::from_str(
            &self
                .conn
                .query_one(query, [], |row| row.get::<usize, String>(0))
                .unwrap(),
        )
        .unwrap();

        return schema;
    }
    fn get_admin_login(&self, identifier: String, hashed_password: String) -> Option<u32> {
        let query = "SELECT *
FROM qbase_admins
WHERE (username = ?1 OR email = ?1)
  AND password_hash = ?2;
";
        println!("{}", &query);
        println!(
            "ident: {}, password_hash: {}",
            &identifier, &hashed_password
        );
        let result = match self
            .conn
            .query_one(query, [identifier, hashed_password], |row| {
                return row.get::<usize, usize>(0);
            }) {
            Ok(id) => Some(id as u32),
            Err(_) => None,
        };
        return result;
    }
    fn update_schema(&self, schema: &Schema) -> DataResult {
        //insert schema into db
        let query = "CREATE TABLE qbase_schema(
            schema TEXT NOT NULL
        )";
        println!("{}", query);
        self.conn.execute(query, []).unwrap();
        let query = "INSERT INTO qbase_schema(schema) VALUES(?1)";
        println!("{}", query);
        self.conn
            .execute(query, [json!(schema).to_string()])
            .unwrap();
        return Ok(());
    }

    fn insert_entity(
        &self,
        entity: &Entity,
        record: crate::schema_records::EntityRecord,
    ) -> DataResult {
        let mut query: String = format!(
            "INSERT INTO {}({}) VALUES({})",
            &entity.name,
            entity
                .get_fields()
                .iter()
                .map(|f| f.get_name().to_owned())
                .collect::<Vec<String>>()
                .join(", "),
            entity
                .get_fields()
                .iter()
                .map(|f| format!(":{}", f.get_name()))
                .collect::<Vec<String>>()
                .join(", ")
        );

        println!("{}", query);
        return Ok(());
    }
}
