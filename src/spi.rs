use std::{sync::OnceLock};
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

#[pg_extern]
fn pg_lab_table_exists(table_name: &str) ->  bool {
    let query_str = "SELECT Exists(SELECT 1 from information_schema.tables where table_schema = 'public' and table_name = $1)";
    Spi::get_one_with_args::<bool>(
       query_str, &[table_name.into()])
        .unwrap()
        .unwrap_or(false)
}

#[pg_extern]
fn pg_lab_count_tables() -> i64 {
    Spi::get_one::<i64>(
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public'"
    )
    .unwrap()
    .unwrap_or(0)
}

#[pg_extern]
fn pg_lab_column_exists(table_name: &str, column_name: &str) ->  bool {
    Spi::get_one_with_args::<bool>(
        "SELECT Exists(SELECT 1 from information_schema.columns where table_name = $1 and column_name = $2)", &[table_name.into(), column_name.into()])
        .unwrap()
        .unwrap_or(false)
}

#[pg_extern]
fn pg_lab_count_indexes(table_name: &str) -> i64 {
    let query_str = "SELECT COUNT(*) FROM pg_indexes WHERE schemaname = 'public' AND tablename = $1";
    Spi::get_one_with_args::<i64>(
        query_str, &[table_name.into()]).unwrap().unwrap_or(0)
}

#[pg_extern]
fn pg_lab_get_current_user() -> String {
    Spi::get_one::<String>("Select current_user::text")
    .unwrap()
    .unwrap()
}


#[pg_extern]
fn pg_lab_get_db_name() -> String {
    Spi::get_one::<String>("Select current_database()::text")
    .unwrap()
    .unwrap()
}

#[pg_extern]
fn pg_lab_get_server() -> String {
    Spi::get_one("Show server_version")
    .unwrap()
    .unwrap()
}

#[pg_extern]
fn pg_lab_count_columns(table_name : &str) -> i64 {
   let query = "SELECT count(*) FROM information_schema.columns WHERE table_name = $1 AND table_schema = 'public'"; 

    Spi::get_one_with_args::<i64>(
        query, &[table_name.into()]
    ).unwrap().unwrap_or(0)

}

#[pg_extern]
fn pg_lab_table_column_info(table_name : &str) -> TableIterator<'static, (
                                                                name!(column_name, String),
                                                                name!(dtype, String)
                                                        )> {

    let mut res = Vec::new();

    Spi::connect(|client| {
            let query = "SELECT column_name::text, data_type::text FROM information_schema.columns WHERE table_name = $1 AND table_schema = 'public'";
            let tuple_selected = client.select(query, None, &[table_name.into()]).unwrap();

            for row in tuple_selected {
                let column: String = row["column_name"].value().unwrap().unwrap();
                let dtype: String = row["data_type"].value().unwrap().unwrap();
                res.push((column, dtype));
            }
    });

    if res.is_empty(){
        pgrx::error!("Table_name '{}' not found in 'public' schema", table_name);
    }

    TableIterator::new(res.into_iter())
}


#[pg_extern]
fn pg_lab_table_column_type(table_name: &str, column_name: &str) -> Option<String> {
    let query = "SELECT data_type::text FROM information_schema.columns 
                 WHERE table_name = $1 AND column_name = $2 AND table_schema = 'public'";
    Spi::get_one_with_args::<String>(query, &[table_name.into(), column_name.into()]).unwrap()
}

#[pg_extern]
fn pg_lab_get_primary_key(table_name: &str) -> Option<String> {
    let query = "SELECT string_agg(kcu.column_name, ', ') 
                        from information_schema.table_constraints tc
                        join information_schema.key_column_usage kcu
                        on tc.constraint_name = kcu.constraint_name
                        where tc.table_name = $1
                        AND tc.table_schema = 'public'
                        AND tc.constraint_type = 'PRIMARY KEY'";
    
    Spi::get_one_with_args::<String>(query, &[table_name.into()]).unwrap_or(None)
}

#[pg_extern]
fn pg_lab_table_size(table_name: &str) -> i64 {
    let query = "SELECT pg_total_relation_size(relid) AS total_size
                            FROM pg_catalog.pg_statio_user_tables
                            where schemaname = 'public'
                            AND relname = $1";
    
    Spi::get_one_with_args::<i64>(query, &[table_name.into()]).unwrap().unwrap()
}


#[pg_extern]
fn pg_lab_get_duplicate_count(table_name: &str, col_name: &str) -> i64 {
    // manual identifier escaping: double up any embedded quotes
    let safe_table_name: String  = Spi::get_one_with_args::<String>(
        "SELECT quote_ident($1)",
        &[table_name.into()]
    ).unwrap().unwrap();

    let safe_col_name: String  = Spi::get_one_with_args::<String>(
        "SELECT quote_ident($1)",
        &[col_name.into()]
    ).unwrap().unwrap();

    let subquery: String = format!("SELECT 1 FROM {} GROUP BY {} HAVING count(*) > 1", safe_table_name, safe_col_name);

    let query = format!("SELECT count(*) FROM ({}) a", subquery);

    Spi::get_one::<i64>(&query).unwrap().unwrap_or(0)
}

#[pg_extern]
fn pg_lab_get_null_count(table_name: &str, col_name: &str) -> i64 {
    let safe_table_name: String = Spi::get_one_with_args::<String>(
        "SELECT quote_ident($1)",
        &[table_name.into()]
    ).unwrap().unwrap();

    let safe_col_name: String = Spi::get_one_with_args::<String>(
        "SELECT quote_ident($1)",
        &[col_name.into()]
    ).unwrap().unwrap();

    let query = format!("SELECT count(*) FROM {} where {} is NULL", safe_table_name, safe_col_name);

    Spi::get_one::<i64>(&query).unwrap().unwrap_or(0)
}

#[pg_extern]
fn pg_lab_safe_query(sql: &str) -> Option<String> {
    PgTryBuilder::new(|| -> Result<Option<String>, SpiError> {
        Spi::get_one::<String>(sql)
    }) 
    .catch_others(|_| Ok(None))
    .execute()
    .unwrap()
}

#[pg_extern]
fn pg_lab_get_max_value(table_name : &str, col_name: &str) -> Option<String> {
    let safe_table_name: String = Spi::get_one_with_args::<String>(
                                            "SELECT quote_ident($1)",
                                            &[table_name.into()])
                                            .unwrap().unwrap();
    
    let safe_col_name: String = Spi::get_one_with_args::<String>(
                                                    "Select quote_ident($1)",
                                                    &[col_name.into()])  
                                                    .unwrap().unwrap();      
    
    let query = format!("Select max({})::text from {}",safe_col_name, safe_table_name);

    Spi::get_one
    ::<String>(&query).unwrap()                                 
}

#[pg_extern]
fn pg_lab_get_list_tables() -> TableIterator<'static, 
                                                (
                                                    name!(table_name, String),
                                                )
                                            >
{
    let mut result: Vec<String>  = Vec::new();

    Spi::connect(|client| {
        let query = "Select table_name::text from information_schema.tables where table_schema = 'public'";

        let tuples = client.select(query, None, &[]).unwrap();

        for row in tuples {
            let curr_row = row.get::<String>(1).unwrap().unwrap();
            result.push(curr_row);
        }
    });
    TableIterator::new(result.into_iter().map(|s| (s, )))
}


#[pg_extern]
fn pg_lab_get_columns(table_name: &str) -> TableIterator<'static, 
                                                        (
                                                            name!(column_name, String),
                                                            name!(data_type, String)
                                                        )> 
{
    let mut result = Vec::new();

    Spi::connect(|client | {
        let query = "Select column_name::text, data_type from information_schema.columns where table_schema = 'public' and table_name = $1";

        let tuples = client.select(query, None, &[table_name.into()]).unwrap();

        for row in tuples {
            let col_name = row["column_name"].value().unwrap().unwrap();
            let data_type = row["data_type"].value().unwrap().unwrap();
            result.push((col_name, data_type));
        }

    });

    TableIterator::new(result.into_iter())
}

#[pg_extern]
fn pg_lab_find_large_tables(min_size_bytes: i64) -> TableIterator<'static, (
                                                    name!(table_name, String), 
                                                    name!(size_bytes, i64)
                                                )> 
{
    let mut result = Vec::new();

    Spi::connect(|client | {
        let query = "SELECT relname::text as tablename, pg_total_relation_size(relid) as table_size
                                FROM pg_catalog.pg_statio_user_tables
                                WHERE schemaname = 'public'
                                AND pg_total_relation_size(relid) >= $1";

        let tuples = client.select(query, None, &[min_size_bytes.into()]).unwrap();

        for row in tuples{
            let table_name = row["tablename"].value().unwrap().unwrap();
            let table_size = row["table_size"].value().unwrap().unwrap();

            result.push((table_name, table_size));
        }
    });

     TableIterator::new(result.into_iter())    
}

#[pg_extern]
fn pg_lab_bulk_insert(table_name: &str, values: Array<String>) -> i64 {
    //return the number of 
    let safe_table_name = Spi::get_one_with_args::<String>(
                                                                "select quote_ident($1)", &[table_name.into()]
                                                            ).unwrap().unwrap();

    let mut success_count: i64 = 0;

    for val in values.iter().flatten(){
        let sql = format!("Insert Into {} (name) Values ('{}')", safe_table_name, val);
        if pg_lab_try_execute(&sql){
            success_count += 1;
        }
    }
    success_count
}

#[pg_extern]
fn pg_lab_table_stats(table_name: &str) -> TableIterator<'static, (
                                                            name!(column, String),
                                                            name!(nulls, i64),
                                                            name!(distinct, i64),
                                                            name!(max_val, Option<String>)
                                                        )> 
{
    let mut result = Vec::new();

    // Step 1: safely quote the table name once
    let safe_table = Spi::get_one_with_args::<String>(
        "SELECT quote_ident($1)", &[table_name.into()]
    ).unwrap().unwrap();

    // Step 2: get all column names for this table
    let mut columns = Vec::new();
    Spi::connect(|client| {
        let query = "SELECT column_name::text FROM information_schema.columns 
                     WHERE table_schema = 'public' AND table_name = $1";
        let tuples = client.select(query, None, &[table_name.into()]).unwrap();
        for row in tuples {
            let col_name: String = row["column_name"].value().unwrap().unwrap();
            columns.push(col_name);
        }
    });

    // Step 3: for each column, compute nulls, distinct, max
    for col in columns {
        let safe_col = Spi::get_one_with_args::<String>(
            "SELECT quote_ident($1)", &[col.clone().into()]
        ).unwrap().unwrap();

        let null_query = format!(
            "SELECT count(*) FROM {} WHERE {} IS NULL", safe_table, safe_col
        );
        let distinct_query = format!(
            "SELECT count(DISTINCT {}) FROM {}", safe_col, safe_table
        );
        let max_query = format!(
            "SELECT max({}::text) FROM {}", safe_col, safe_table
        );

        let nulls: i64 = Spi::get_one(&null_query).unwrap().unwrap_or(0);
        let distinct: i64 = Spi::get_one(&distinct_query).unwrap().unwrap_or(0);
        let max_val: Option<String> = Spi::get_one(&max_query).unwrap_or(None);

        result.push((col, nulls, distinct, max_val));
    }

    TableIterator::new(result.into_iter())
}




