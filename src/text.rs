use pgrx::prelude::*;
use pgrx::StringInfo;

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





