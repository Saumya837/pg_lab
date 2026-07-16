use std::sync::OnceLock;

use pgrx::{prelude::*, spi::SpiError};


#[pg_extern]
fn pg_lab_row_count(table_name : &str) -> Option<i64> {
    let safe_name = Spi::get_one_with_args::<String>(
        "SELECT quote_ident($1)",
        &[table_name.into()]
    ).unwrap().unwrap();
    
    // Then use it in the query
    let query = format!("SELECT count(*) FROM {}", safe_name);
    Spi::get_one::<i64>(&query).unwrap()
}

#[pg_extern]
fn pg_lab_table_info(table_name : &str) -> TableIterator<'static,
                                                            (
                                                                name!(parameter, String),
                                                                name!(type, String)
                                                            )> {
        let mut results = Vec::new();
        
        Spi::connect(|client| {
            let query = "Select column_name::text, data_type::text from information_schema.columns where table_name = $1";
            let tup_table = client.select(query, None, &[table_name.into()]).unwrap();

            for row in tup_table{
                let col : String = row["column_name"].value().unwrap().unwrap();
                let dtype: String = row["data_type"].value().unwrap().unwrap();
                results.push((col, dtype))
            }
        });

        TableIterator::new(results.into_iter())
}

#[pg_extern]
fn pg_lab_try_execute(sql: &str) -> bool {
    PgTryBuilder::new(|| -> Result<bool, SpiError>{
        Spi::run(sql)?;
        Ok(true)
    })
    .catch_others(|_| Ok(false))
    .execute()
    .unwrap()
}

#[pg_extern]
fn pg_lab_row_count_cached(table_name: &str) -> Option<i64> {
    // Step 1: sanitize using cached query
    let safe_name = Spi::get_one_with_args::<String>(
         "SELECT quote_ident($1)",
        &[table_name.into()]
    ).unwrap().unwrap();

    // Step 2: cache the count query template too
    static COUNT_QUERY: OnceLock<String> = OnceLock::new();
    
    let count_query = COUNT_QUERY.get_or_init(|| {
        "SELECT count(*) FROM ".to_string()
    });

    let full_query = format!("{}{}", count_query, safe_name);
    
    Spi::get_one::<i64>(&full_query).unwrap()
}


