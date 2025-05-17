use std::{collections::HashMap, io::Error};

use crate::schema::{DTO, Entity, EntityField, Schema};

pub trait Migrator {
    fn migrate(&self, changes: Vec<SchemaChange>) -> Result<(), Error>;
}
pub struct SqliteMigrator {
    pub connection_string: String,
}
impl Migrator for SqliteMigrator {
    fn migrate(&self, changes: Vec<SchemaChange>) -> Result<(), Error> {
        todo!()
    }
}
pub enum SchemaChange<'a> {
    ADD_ENTITY(&'a Entity),
    REMOVE_ENTITY(&'a Entity),
    RENAME_ENTITY(&'a Entity, String),
    CHANGE_ENTITY_FIELD(&'a Entity, Vec<FieldChange<'a>>),
}

pub enum FieldChange<'a> {
    CHANGE_NULLABLE(&'a EntityField, bool),
    RENAME(&'a EntityField, String),
    CHANGE_RULE(&'a EntityField, String, String),
    CHANGE_DTO(&'a EntityField, DTO),
    ADD_FIELD(&'a EntityField),
    REMOVE_FIELD(&'a EntityField),
}
pub struct SchemaComparator {}
impl SchemaComparator {
    pub fn new(&self) -> Self {
        return SchemaComparator {};
    }
    pub fn compare(&self, source: &Schema, new: &Schema) -> Result<Vec<SchemaChange>, Error> {
        //TODO COMPARE THE SCHEMAS
        let mut changes: Vec<SchemaChange> = vec![];

        //CHECK SETTINGS CHANGES

        //CHECK ENTITIES
        let source_entities: HashMap<String, &Entity> = source
            .get_entities()
            .iter()
            .map(|e| (e.uuid.clone(), e))
            .collect();
        let new_entities: HashMap<String, &Entity> = source
            .get_entities()
            .iter()
            .map(|e| (e.uuid.clone(), e))
            .collect();
        //Check for changed or remove entities
        for (uuid, entity) in &source_entities {
            let other = new_entities.get(uuid);
            match other {
                Some(e) => {
                    // entity is still there
                    // check for other differences
                    changes.append(&mut SchemaComparator::compare_entity(entity, e));
                }
                None => {
                    //entity got removed.
                    changes.push(SchemaChange::REMOVE_ENTITY(entity));
                }
            }
        }

        //check new schema for any new entities
        for (uuid, new_entity) in &new_entities {
            if (source_entities.contains_key(uuid)) {
                continue;
            }
            changes.push(SchemaChange::ADD_ENTITY(new_entity))
        }

        //debug
        for change in changes {
            match change {
                SchemaChange::ADD_ENTITY(entity) => println!("Added entity {0}", entity.name),
                SchemaChange::REMOVE_ENTITY(entity) => println!("Removed entity {0}", entity.name),
                SchemaChange::RENAME_ENTITY(entity, name) => {
                    println!("Renamed {0} to {1}", entity.name, name)
                }
                SchemaChange::CHANGE_ENTITY_FIELD(entity, field_changes) => {
                    for fieldchange in field_changes {}
                }
            }
        }
        //?
        return Ok(vec![]);
    }
    fn compare_entity<'a>(old: &'a Entity, new: &'a Entity) -> Vec<SchemaChange<'a>> {
        let mut changes: Vec<SchemaChange> = vec![];

        return changes;
    }
}
