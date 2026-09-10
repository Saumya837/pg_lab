use pgrx::prelude::*;
use pgrx::PgLwLock; 

// Static global — poore extension ke lifetime mein ek hi instance,
// saare backend processes ke beech genuinely shared
pub static RECON_COUNTER_A: PgLwLock<i64> = unsafe { PgLwLock::new(c"recon_counter_a") };
pub static RECON_COUNTER_B: PgLwLock<i64> = unsafe { PgLwLock::new(c"recon_counter_b") };

#[pg_extern]
fn pg_lab_a_counter_increment() -> i64 {
    let mut guard = RECON_COUNTER_A.exclusive();
    *guard += 1;
    *guard
}

#[pg_extern]
fn pg_lab_a_counter_read() -> i64 {
    let guard = RECON_COUNTER_A.share();
    *guard
}

#[pg_extern]
fn pg_lab_b_counter_increment() -> i64 {
    let mut guard = RECON_COUNTER_B.exclusive();
    *guard += 1;
    *guard
}

#[pg_extern]
fn pg_lab_b_counter_read() -> i64 {
    let guard = RECON_COUNTER_B.share();
    *guard
}