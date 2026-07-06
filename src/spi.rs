use pgrx::prelude::*;

#[pg_extern]
pub fn pg_lab_row_count(table_name : &str) -> Option<i64> {
    let query = format!("select count(*) from {}", table_name);

    Spi::get_one::<i64>(&query).unwrap()

}