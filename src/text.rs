use std::collections::HashMap;

use pgrx::prelude::*;
use pgrx::StringInfo;
use pgrx::JsonB;
use serde_json::Value;

#[pg_extern]
pub fn pg_lab_version() -> &'static str {
    "0.1.0"
}

#[pg_extern]
pub fn pg_lab_formal_greet(name: Option<String>) -> String {
    let name = match name{
                            None => return String::new(),
                            Some(s) => s,
                        };

    let mut initials = Vec::new();

    for word in name.split_whitespace(){
       let cleaned: String = word
                                    .chars()
                                    .filter(|c| c.is_alphabetic())
                                    .collect();

        if let Some(first) = cleaned.chars().next() {
            let capitalized: String = if first.is_lowercase() {
                first.to_uppercase().collect::<String>() + &cleaned[first.len_utf8()..]
            } else {
                cleaned
            };
            initials.push(capitalized);
        }
    }

    format!("Hello {}", initials.join(" "))
}

#[pg_extern]
pub fn pg_lab_hello() -> &'static str {
    "Hello from pg_lab!"
}

#[pg_extern]
pub fn pg_greet(name: Option<String>) -> Option<String>
{
    match name {
        Some(name) => Some(format!("Hello, {}!", name)),
        None => None,
    }
}

#[pg_extern]
pub fn pg_lab_repeat(input_string: Option<String>, freq: Option<i32>) -> Option<String> {
    let input_string = match input_string{
                                    Some(s) => s,
                                    None => error!("Please provide some input string")
                                };
    let freq = match freq{
                            Some(f) => f,
                            None => error!("Please provide some frequency")
    };

    let mut s = StringInfo::new();
    for _ in 0..freq {
                s.push_str(&input_string);
    }
    Some(s.to_string())
}

#[pg_extern]
pub fn pg_lab_reverse(input_string: Option<String>) -> Option<String> {
    let input_string = match input_string{
        Some(s) => s,
        None => error!("Please provide some input string")
    };

    let mut s = StringInfo::new();
    for c in input_string.chars().rev(){
        s.push(c);
    }
    Some(s.to_string())
}

#[pg_extern]
fn pg_lab_split_name(name: Option<String>) -> TableIterator<'static, (name!(first, String), name!(last, Option<String>))> {

    let name  = match name{
        Some(n) => n,
        None => error!("Please provide some name")
    };

    let mut parts = name.trim().splitn(2, ' ');
    let first = parts.next().unwrap_or("").to_string();
    let last = parts.next().map(|s| s.to_string());

    TableIterator::once((first, last))
}

#[pg_extern]
fn pg_lab_jsonb_keys(input: JsonB) -> Vec<String>{
    let value: Value = input.0;
    let mut result = Vec::new();

    match value{
        Value::Object(map) => {
            for key in map.keys(){
                result.push(key.to_string())
            }
        }
        _ => {}
    }
    result
}


#[pg_extern]
fn pg_lab_jsonb_values(input: JsonB) -> Vec<String>{
    let value: Value = input.0;
    let mut result = Vec::new();

    match value{
        Value::Object(map) => {
            for val in map.values(){
                let s = match val{
                    Value::String(s) => s.clone(),
                    other => other.to_string()
                };
                result.push(s);
            }
        }
        _ => {}
    }
    result
}

#[pg_extern]
fn pg_lab_extract_value(input: JsonB, key: String) -> Option<String> {
    let value = input.0;
    match value.get(&key){
        Some(Value::String(s)) => Some(s.clone()),
        Some(other) => Some(other.to_string()),
        None => None,
    }
}

#[pg_extern]
fn pg_lab_word_count(input: Option<String>) -> TableIterator< 'static,
                                                        (
                                                            name!(word, String),
                                                            name!(freq, i32)
                                                        )> 
{
    let input = match input{
                                None => error!("Please provide input"),
                                Some(s) => s
                            };
    
    let mut counts: HashMap<String, i32> = HashMap::new();

    for word in input.split_whitespace(){
        let cleaned: String = word
                                .chars() 
                                .filter(|c| c.is_alphabetic())
                                .collect::<String>()
                                .to_lowercase();

        *counts.entry(cleaned.to_string()).or_insert(0) += 1;
    } 

    let rows:Vec<(String, i32)> = counts
                                .into_iter()
                                .collect();

    TableIterator::new(rows.into_iter())
}











