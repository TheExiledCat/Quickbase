use crate::version::QBASEVERSION;
use chrono::{DateTime, Datelike, Utc};
use semver::Version;
#[derive(Debug)]
pub struct Schema {
    version: Version,
    entities: Vec<Entity>,
    settings: SchemaSettings,
}
#[derive(Debug)]
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
            EntityFieldType::DATE {
                min_date: DateTime::<Utc>::MIN_UTC,
                max_date: DateTime::<Utc>::MAX_UTC,
            },
        ));
        fields.push(EntityField::new(
            "updated",
            false,
            true,
            EntityFieldType::DATE {
                min_date: DateTime::<Utc>::MIN_UTC,
                max_date: DateTime::<Utc>::MAX_UTC,
            },
        ));
        return fields;
    }
}
#[derive(Debug)]
pub struct Entity {
    name: String,
    kind: EntityType,
    fields: Vec<EntityField>,
}

#[derive(Debug)]
pub struct EntityField {
    name: String,
    nullable: bool,
    base: bool,
    kind: EntityFieldType,
}
impl EntityField {
    pub fn new(name: &str, nullable: bool, base: bool, kind: EntityFieldType) -> Self {
        return EntityField {
            name: String::from(name),
            nullable,
            base,
            kind,
        };
    }
}
#[derive(Debug)]
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
pub struct SchemaSettings {}
impl Schema {
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

        return Schema::new(version, entities, SchemaSettings {});
    }
}
impl Entity {
    pub fn new(name: &str, kind: EntityType) -> Self {
        let mut entity = Entity {
            name: String::from(name),
            kind,
            fields: vec![],
        };
        entity.add_fields(&mut entity.kind.generate_base_fields());
        return entity;
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

impl SchemaSettings {}
