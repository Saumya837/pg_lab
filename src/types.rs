use pgrx::prelude::*;
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

