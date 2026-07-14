use pgrx::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(PostgresType, Clone, Debug, Serialize, Deserialize)]
pub struct TaggedStringList {
    pairs: Vec<(String, String)>,
}

#[pg_extern]
fn tsl_new() -> TaggedStringList {
    TaggedStringList { pairs: Vec::new() }
}

#[pg_extern]
fn tsl_add(mut list: TaggedStringList, key: &str, value: & str) -> TaggedStringList {
    list.pairs.push((key.to_string(), value.to_string()));
    list
}

#[pg_extern]
fn tsl_get(list: TaggedStringList, key: &str) -> Option<String>{
    list.pairs.into_iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v)
} 
