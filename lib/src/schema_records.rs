use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct EntityRecord {
    pub fields: HashMap<String, Option<EntityFieldRecord>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(untagged)]
pub enum EntityFieldRecord {
    TEXT(String),
    NUMBER(f32),
    BOOL(bool),
    DATE(DateTime<Utc>),
    ENTITY_REF(EntityRecord),
    ENTITY_REF_MANY(Vec<EntityRecord>),
}
