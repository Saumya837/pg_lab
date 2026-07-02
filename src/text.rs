use pgrx::prelude::*;

#[pg_extern]
pub fn pg_lab_version() -> &'static str {
    "0.1.0"
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
    match (input_string, freq){
        (None, None) => error!("Please provide some input string and frequency"),
        (None, _) => error!("Please provide some input string"),
        (_, None) => error!("Please provide some frequency"),
        (_, Some(f)) if f <= 0 => error!("Frequency must be a positive integer"),
       (Some(input_string), Some(freq)) => Some(input_string.repeat(freq as usize))
    }
}