use pgrx::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(PostgresType, Copy, Clone, Debug, Serialize, Deserialize, pgrx::AggregateName)]
#[repr(C)]
pub struct Complex{
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

#[pg_operator(immutable, parallel_safe)]
fn complex_lt(a: Complex, b: Complex) -> bool{
    complex_cmp(a, b) < 0
}

#[pg_operator(immutable, parallel_safe)]
fn complex_lte(a: Complex, b: Complex) -> bool{
    complex_cmp(a, b) <= 0
}

#[pg_operator(immutable, parallel_safe)]
fn complex_gt(a: Complex, b: Complex) -> bool{
    complex_cmp(a, b) > 0
}

#[pg_operator(immutable, parallel_safe)]
fn complex_gte(a: Complex, b: Complex) -> bool{
    complex_cmp(a, b) >= 0
}


// #[derive(pgrx::AggregateName)]
// pub struct ComplexSum{
//     re: f64,
//     im: f64
// }

pub struct ComplexSum;

#[pg_aggregate]
impl Aggregate<Complex> for ComplexSum {
    type State = Complex;
    type Args = Complex;

    const INITIAL_CONDITION: Option<&'static str> = Some(r#"{"re":0.0,"im":0.0}"#);

    #[pgrx(immutable, parallel_safe)]
    fn state(mut current: Self::State, next: Self::Args, _fcinfo: pg_sys::FunctionCallInfo) -> Self::State {
      current.re += next.re;
      current.im += next.im;
      current
    }
}


