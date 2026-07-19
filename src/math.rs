use pgrx::prelude::*;

#[pg_extern]
fn pg_add(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    Some(a? + b?)
}

#[pg_extern]
fn pg_div(a: Option<f64>, b: Option<f64>) -> Option<f64> {
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
fn pg_lab_clamp(value: AnyNumeric, min: AnyNumeric, max: AnyNumeric) -> AnyNumeric{
    if value > max{
        max
    }else if value < min{
        min
    }else{
        value 
    }
}

#[pg_extern]
fn pg_lab_celcius_to_farenheight(c: f64) -> f64 {
    let f = c * (1.8) + 32.0;
    f
}

#[pg_extern]
fn pg_lab_factorial(num: i64) -> Option<i64> {
    if num < 0 {
        None
    } else if num == 0{
        Some(1)
    }else{
        pg_lab_factorial(num - 1).map(|f| num * f)
    }
}

#[pg_extern]
fn pg_lab_fibonacci(num: i32) -> Option<i64>{
    if num < 0 {
        None
    } else if num < 2{
        Some(num as i64)
    }else{
        let (mut a, mut b) = (0, 1);
        for _ in 2..num{
            let next = a + b;
            a = b;
            b = next;
        }
        Some(b)
    }
}




// SELECT * FROM pg_lab_stats(ARRAY[1,2,3,4,5]);

