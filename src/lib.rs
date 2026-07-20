use pgrx::prelude::*;
pg_module_magic!();

mod text;
mod math;
mod iter;
mod spi;
mod triggers;
mod complex_type;
mod toast_type;
mod gist_complex;
