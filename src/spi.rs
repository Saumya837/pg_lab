use pgrx::prelude::*;

#[pg_extern]
fn pg_lab_count_users() -> Option<i64> {
    Spi::get_one("SELECT count(*) FROM pg_class").unwrap()
}

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