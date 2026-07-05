use pgrx::prelude::*;

#[pg_extern]
pub fn pg_add(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    Some(a? + b?)
}

#[pg_extern]
pub fn pg_div(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    match b{
        b if b == Some(0.0) => {
            ereport!(ERROR, 
            PgSqlErrorCode::ERRCODE_DIVISION_BY_ZERO,
            "Division by zero is not allowed");
        }
        _ => Some(a? / b?)
    } 
}

#[pg_extern]
pub fn pg_lab_clamp(value: AnyNumeric, min: AnyNumeric, max: AnyNumeric) -> AnyNumeric{
    if value > max{
        max
    }else if value < min{
        min
    }else{
        value 
    }
}