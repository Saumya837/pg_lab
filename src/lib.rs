use pgrx::prelude::*;

pg_module_magic!();

#[pg_extern]
fn pg_lab_hello() -> &'static str {
    "Hello from pg_lab!"
}

#[pg_extern]
fn pg_lab_version() -> &'static str {
    "0.1.0"
}

#[pg_extern]
fn pg_greet(name: Option<String>) -> Option<String>
{
    match name {
        Some(name) => Some(format!("Hello, {}!", name)),
        None => None,
    }
}

#[pg_extern]
fn pg_add(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    Some(a? + b?)
}

#[pg_extern]
fn pg_div(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    match b{
        b if b == Some(0.0) => error!("Division by zero is not allowed"),
        _ => Some(a? / b?)
    } 
}

#[pg_extern]
fn pg_lab_repeat(input_string: Option<String>, freq: Option<i32>) -> Option<String> {
    match (input_string, freq){
        (None, None) => error!("Please provide some input string and frequency"),
        (None, _) => error!("Please provide some input string"),
        (_, None) => error!("Please provide some frequency"),
        (_, Some(f)) if f <= 0 => error!("Frequency must be a positive integer"),
       (Some(input_string), Some(freq)) => Some(input_string.repeat(freq as usize))
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello() {
        assert_eq!("Hello from pg_lab!", crate::pg_lab_hello());
    }

    #[pg_test]
    fn test_version() {
        assert_eq!("0.1.0", crate::pg_lab_version());
    }
}