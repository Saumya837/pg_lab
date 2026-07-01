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