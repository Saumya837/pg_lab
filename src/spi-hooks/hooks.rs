use pgrx::prelude::*;
use pgrx::pg_sys;
use std::sync::atomic::{AtomicI64, Ordering};

#[pg_extern]
fn pg_lab_query_count() -> i64 {
    static QUERY_COUNT: AtomicI64 = AtomicI64::new(0);


}