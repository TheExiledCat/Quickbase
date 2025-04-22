use std::io::Error;

use crate::schema::{DTO, Entity, EntityField, Schema};

pub trait Migrator {
    fn migrate(&self, changes: Vec<SchemaChange>, new_schema: &Schema) -> Result<(), Error>;
}

pub enum SchemaChange {
    ADD_ENTITY(Entity),
    REMOVE_ENTITY(Entity),
    RENAME_ENTITY(Entity, String),
    CHANGE_ENTITY(Vec<FieldChange>),
}
pub enum FieldChange {
    CHANGE_NULLABLE(bool),
    RENAME(String),
    CHANGE_RULE(String, String),
    CHANGE_DTO(DTO),
    ADD_FIELD(EntityField),
    REMOVE_FIELD(EntityField),
}
pub struct SchemaComparator {}
impl SchemaComparator {
    pub fn compare(&self, source: &Schema, new: &Schema) -> Result<Vec<SchemaChange>, Error> {
        //TODO COMPARE THE SCHEMAS
        return Ok(vec![]);
    }
}
