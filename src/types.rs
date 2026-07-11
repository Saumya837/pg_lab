use pgrx::{PgSqlErrorCode::ERRCODE_SINGLETON_SQL_JSON_ITEM_REQUIRED, prelude::*};
use serde::{Serialize, Deserialize};

#[derive(PostgresType, Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C)]
struct Complex{
    re: f64,
    im: f64
}

#[pg_operator(immutable, parallel_safe)]
fn complex_add(a: Complex, b: Complex) -> Complex{
    Complex { 
        re: (a.re + b.re),
        im: (a.im + b.im) 
    }
}

#[pg_operator(immutable, parallel_safe)]
fn complex_equal(a: Complex, b: Complex) -> bool {
    a.re == b.re && a.im == b.im
}

#[pg_extern(immutable, parallel_safe)]
fn complex_mag(a: Complex) -> f64 {
    (a.re * a.re - a.im * a.im).sqrt()
}

#[pg_extern(immutable, parallel_safe)]
fn complex_cmp(a: Complex, b: Complex) -> i32 {
    if a.re < b.re { -1 }
    else if a.re > b.re { 1 }
    else if a.im < b.im { -1 }
    else if a.im > b.im { -1 }
    else { 0 }
}
