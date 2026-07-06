use pgrx::prelude::*;

#[pg_extern]
pub fn pg_lab_row_count(table_name : &str) -> Option<i64> {
     let safe_name = Spi::get_one_with_args::<String>(
        "SELECT quote_ident($1)",
        &[table_name.into()]
    ).unwrap().unwrap();
    
    // Then use it in the query
    let query = format!("SELECT count(*) FROM {}", safe_name);
    Spi::get_one::<i64>(&query).unwrap()
}