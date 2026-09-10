use pgrx::prelude::*;
use pgrx::{pg_shmem_init};
pg_module_magic!();


mod shmemcounter;
mod text;
mod math;
mod iter;
mod spi;
mod triggers;
mod complex_type;
mod toast_type;
mod gist;
mod explain_analyze;

use shmemcounter::{RECON_COUNTER_A, RECON_COUNTER_B};  // <- ye line add karo, path ko simple naam mein la do

#[pg_guard]
pub extern "C-unwind" fn _PG_init() {
    pg_shmem_init!(RECON_COUNTER_A);
    pg_shmem_init!(RECON_COUNTER_B);
}  

