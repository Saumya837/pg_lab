use pgrx::prelude::*;
use pgrx::PgLwLock; 

// Static global — poore extension ke lifetime mein ek hi instance,
// saare backend processes ke beech genuinely shared
pub static RECON_COUNTER: PgLwLock<i64> = unsafe { PgLwLock::new(c"recon_counter") };

#[pg_extern]
fn pg_lab_counter_increment() -> i64 {
    let mut guard = RECON_COUNTER.exclusive();
    *guard += 1;
    *guard
}

#[pg_extern]
fn pg_lab_counter_read() -> i64 {
    let guard = RECON_COUNTER.share();
    *guard
}