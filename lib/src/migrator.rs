use std::collections::HashMap;

use rusqlite::Error;

use crate::schema::{Entity, EntityField, Schema};

pub trait Migrator: Send + Sync {
    fn migrate(&self, changes: Vec<SchemaChange>) -> Result<(), Error>;
    fn ensure_created(&self, schema: &Schema) -> Result<Option<Schema>, Error>;
}
pub enum SchemaChange {
    ADD_ENTITY(Entity),
    REMOVE_ENTITY(Entity),
    RENAME_ENTITY(Entity, String),
    CHANGE_ENTITY_FIELD(Entity, Vec<FieldChange>),
}

pub enum FieldChange {
    CHANGE_NULLABLE(EntityField, bool),
    RENAME(EntityField, String),
    CHANGE_RULE(EntityField, String, String),
    // CHANGE_DTO(&'a EntityField, DTO),
    ADD_FIELD(EntityField),
    REMOVE_FIELD(EntityField),
}
pub struct SchemaComparator {}
impl SchemaComparator {
    pub fn new() -> Self {
        return SchemaComparator {};
    }
    pub fn compare<'a>(&self, source: &'a Schema, new: &Schema) -> Vec<SchemaChange> {
        //TODO COMPARE THE SCHEMAS
        let mut changes: Vec<SchemaChange> = vec![];

        //CHECK SETTINGS CHANGES

        //CHECK ENTITIES
        let source_entities: HashMap<String, &Entity> = source
            .get_entities()
            .iter()
            .map(|e| (e.name.clone(), e))
            .collect();
        let new_entities: HashMap<String, &Entity> = source
            .get_entities()
            .iter()
            .map(|e| (e.name.clone(), e))
            .collect();
        //Check for changed or remove entities
        for (name, entity) in &source_entities {
            let other = new_entities.get(name);
            let cloned_entity = (*entity).clone();
            match other {
                Some(e) => {
                    // entity is still there
                    // check for other differences
                    changes.append(&mut SchemaComparator::compare_entity(
                        cloned_entity,
                        (**e).clone(),
                    ));
                }
                None => {
                    //entity got removed.
                    changes.push(SchemaChange::REMOVE_ENTITY(cloned_entity));
                }
            }
        }

        //check new schema for any new entities
        for (name, new_entity) in &new_entities {
            let cloned_entity = (*new_entity).clone();
            if source_entities.contains_key(name) {
                continue;
            }
            changes.push(SchemaChange::ADD_ENTITY(cloned_entity));
        }

        //debug
        for change in &changes {
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
        return changes;
    }
    fn compare_entity(old: Entity, new: Entity) -> Vec<SchemaChange> {
        let mut changes: Vec<SchemaChange> = vec![];

        return changes;
    }
}
