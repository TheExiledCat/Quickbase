use crate::{datasource::DataSourceType, version::QBASEVERSION};
use ::serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, serde};
use semver::Version;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
    fs::File,
    io::Write,
};
use uuid::Uuid;
// use semver::Version;
#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    version: Version,
    entities: Vec<Entity>,
    settings: SchemaSettings,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum EntityType {
    AUTH,
    DATA,
    COMPUTED,
}
impl EntityType {
    pub fn generate_base_fields(&self) -> Vec<EntityField> {
        let mut fields: Vec<EntityField> = vec![];
        fields.push(EntityField::new(
            "id",
            false,
            true,
            true,
            EntityFieldType::TEXT {
                min: 15,
                max: 15,
                validate: String::from("^[a-z0-9]+$"),
                generate: String::from("[a-z0-9]{15}"),
            },
        ));
        fields.push(EntityField::new(
            "created",
            false,
            true,
            false,
            EntityFieldType::DATE {
                min_date: DateTime::<Utc>::MIN_UTC,
                max_date: DateTime::<Utc>::MAX_UTC,
            },
        ));
        fields.push(EntityField::new(
            "updated",
            false,
            true,
            false,
            EntityFieldType::DATE {
                min_date: DateTime::<Utc>::MIN_UTC,
                max_date: DateTime::<Utc>::MAX_UTC,
            },
        ));
        match self {
            EntityType::AUTH => {}
            EntityType::DATA => {}
            EntityType::COMPUTED => {}
        }
        return fields;
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub uuid: String,
    pub name: String,
    pub kind: EntityType,
    fields: Vec<EntityField>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityField {
    name: String,
    nullable: bool,
    base: bool,
    primary_key: bool,
    kind: EntityFieldType,
}
impl EntityField {
    pub fn new(
        name: &str,
        nullable: bool,
        base: bool,
        primary_key: bool,
        kind: EntityFieldType,
    ) -> Self {
        return EntityField {
            name: String::from(name),
            nullable,
            base,
            primary_key,
            kind,
        };
    }
    pub fn get_name(&self) -> &String {
        return &self.name;
    }
    pub fn get_type(&self) -> &EntityFieldType {
        return &self.kind;
    }
    pub fn is_nullable(&self) -> &bool {
        return &self.nullable;
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EntityFieldType {
    TEXT {
        min: u32,
        max: u32,
        validate: String,
        generate: String,
    },
    NUMBER {
        min: f32,
        max: u32,
        is_int: bool,
    },
    BOOL,
    DATE {
        min_date: DateTime<Utc>,
        max_date: DateTime<Utc>,
    },
    RELATION {
        entity_name: String,
    },
    RELATION_MANY {
        entity_names: Vec<String>,
    },
}
#[derive(Debug)]
pub enum SchemaError {
    Invalid,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaSettings {
    data_source: DataSourceType,
}
impl Schema {
    pub fn validate(schema: &Schema) -> bool {
        //define invalid cases

        //ensure that there is atleast one AUTH table and that no two tables have the same name
        let mut entity_names: HashSet<String> = HashSet::new();
        let mut auth_found = false;
        for entity in schema.get_entities() {
            if entity.kind == EntityType::AUTH {
                auth_found = true;
            }
            if entity_names.contains(&entity.name) {
                return false;
            }
            entity_names.insert(entity.name.clone());
        }

        if auth_found == false {
            return false;
        }

        return true;
    }
    pub fn new(version: Version, entities: Vec<Entity>, settings: SchemaSettings) -> Self {
        return Schema {
            version,
            entities,
            settings,
        };
    }

    pub fn default_schema() -> Self {
        let version = Version::parse(QBASEVERSION).expect("Correct semver for qbase");
        let mut entities: Vec<Entity> = vec![];
        entities.push(Entity::new("Users", EntityType::AUTH));

        return Schema::new(version, entities, SchemaSettings::default());
    }

    pub fn export(&self) {
        let stringified = serde_json::to_string(self)
            .expect("There is something wrong with the schema serialization");
        let mut file = File::create("schema.json").expect("correct path");
        File::write(&mut file, stringified.into_bytes().as_slice()).expect("a valid utf8 string");
    }
    pub fn get_entities(&self) -> &[Entity] {
        return &self.entities;
    }
    pub fn get_entity_by_name(&self, name: &str) -> Option<&Entity> {
        return self
            .entities
            .iter()
            .find(|&e| e.name.to_lowercase() == name);
    }
    pub fn get_entity_by_uuid(&self, uuid: &String) -> Option<&Entity> {
        return self.entities.iter().find(|&e| e.uuid == *uuid);
    }
    pub fn get_settings(&self) -> &SchemaSettings {
        return &self.settings;
    }

    pub fn add_entity(&mut self, entity: Entity) {
        if let None = self.get_entity_by_name(&entity.name) {
            self.entities.push(entity);
        }
    }
}
impl Entity {
    pub fn new(name: &str, kind: EntityType) -> Self {
        let mut entity = Entity {
            name: String::from(name),
            kind,
            fields: vec![],
            uuid: Uuid::new_v4().simple().to_string(),
        };
        entity.add_fields(&mut entity.kind.generate_base_fields());
        return entity;
    }
    pub fn get_fields(&self) -> &[EntityField] {
        return &self.fields;
    }
    pub fn get_uuid(&self) -> &str {
        return &self.uuid;
    }

    pub fn add_field(&mut self, field: EntityField) {
        //TODO CHECK IF FIELD DOESNT EXIST
        self.fields.push(field);
    }
    pub fn add_fields(&mut self, fields: &mut Vec<EntityField>) {
        //TODO CHECK IF FIELD DOESNT EXIST
        self.fields.append(fields);
    }
}

impl SchemaSettings {
    pub fn default() -> Self {
        return SchemaSettings {
            data_source: DataSourceType::SQLITE {
                connection_string: "data.sqlite".into(),
            },
        };
    }

    pub fn get_source_type(&self) -> &DataSourceType {
        return &self.data_source;
    }
}
