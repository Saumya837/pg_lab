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

    match Spi::get_one_with_args::<i64>(query, &[table_name.into()]){
        Ok(Some(size)) => size,
        Ok(None) => pgrx::error!("Table '{}' has no I/O stats yet", table_name),
        Err(_) => pgrx::error!("Table '{}' does not exist in 'public' schema", table_name),
    }
}


#[pg_extern]
fn pg_lab_get_duplicate_count(table_name: &str, col_name: &str) -> i64 {
    // manual identifier escaping: double up any embedded quotes
    let (safe_table_name, safe_col_name)  = Spi::get_two_with_args::<String, String>(
        "SELECT quote_ident($1), quote_ident($2)",
        &[table_name.into(), col_name.into()]
    ).unwrap();

    let subquery: String = format!("SELECT 1 FROM {} GROUP BY {} HAVING count(*) > 1", safe_table_name.unwrap(), safe_col_name.unwrap());

    let query = format!("SELECT count(*) FROM ({}) a", subquery);

    Spi::get_one::<i64>(&query).unwrap().unwrap_or(0)
}

#[pg_extern]
fn pg_lab_get_null_count(table_name: &str, col_name: &str) -> i64 {
    let (safe_table_name, safe_col_name) = Spi::get_two_with_args::<String, String>("SELECT quote_ident($1), quote_ident($2)",
                                                                    &[table_name.into(), col_name.into()]
                                                                ).unwrap();

    let query = format!("SELECT count(*) FROM {} where {} is NULL",
                                                safe_table_name.unwrap(), safe_col_name.unwrap());

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
    let (safe_table_name, safe_col_name) = Spi::get_two_with_args::<String,String>(
                                            "SELECT quote_ident($1), quote_ident($2)",
                                            &[table_name.into(), col_name.into()])
                                            .unwrap();    
    
    let query = format!("Select max({})::text from {}",safe_col_name.unwrap(), safe_table_name.unwrap());

    Spi::get_one::<String>(&query).unwrap()                                 
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

    let query = format!("Insert INTO {} (name) VALUES ($1)", safe_table_name);
    for val in values.iter().flatten() {
        let result = PgTryBuilder::new(|| -> Result<bool, SpiError> {
            Spi::run_with_args(&query, &[val.clone().into()])?;
            Ok(true)
        })
        .catch_others(|_| Ok(false))
        .execute()
        .unwrap();

        if result {
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

        let query = format!(
            "SELECT count(*) FILTER(WHERE {} IS NULL) as nulls,
            count(DISTINCT {}) as distincts, 
            max({}::text) as max_val
            FROM {}",safe_col, safe_col, safe_col, safe_table
        );

        let mut row_result = (0i64, 0i64, None::<String>);

        Spi::connect(|client| {
            let tuples = client.select(&query, None, &[]).unwrap();

            for row in tuples {
                let nulls: i64 = row["nulls"].value().unwrap().unwrap_or(0);
                let distincts: i64 = row["distincts"].value().unwrap().unwrap_or(0);
                let max_val: Option<String> = row["max_val"].value().unwrap();
                row_result = (nulls, distincts, max_val); 
            }        
        });

        result.push((col, row_result.0, row_result.1, row_result.2));
    }

    TableIterator::new(result.into_iter())
}

#[pg_extern]
fn pg_lab_execute_partial_tolereant(query: Array<String>) -> i64 {
    //skip any queries that fail, and return the number of successful queries

    let query_set: Vec<String> = query.iter().flatten().collect();

    let mut success_count: i64 = 0;

    for sql in &query_set{
        let result = PgTryBuilder::new(move || -> Result<bool, SpiError> {
            Spi::run(&sql)?;
            Ok(true)
        })
        .catch_others(|_| Ok(false))
        .execute()
        .unwrap();

        if result {
            success_count += 1;
        }    
    }
   
    success_count
}

#[pg_extern]
fn pg_lab_execute_rollback_all_native(queries: Array<String>) -> bool {

    let owned_queries: Vec<String> = queries.iter().flatten().collect();

    unsafe { 
        //pg_sys is a library which bridges the gap to use the postgres construct
        let oldcontext = pg_sys::CurrentMemoryContext;
        let oldowner = pg_sys::CurrentResourceOwner;

        //a new memory context begins
        pg_sys::BeginInternalSubTransaction(std::ptr::null());

        //switch back to old context so that queries run in familiar memory, not the fresh subtransaction context
        pg_sys::MemoryContextSwitchTo(oldcontext);

        let success = PgTryBuilder::new(move || -> Result<bool, SpiError> {
            for sql in &owned_queries {
                Spi::run(&sql)?;
            }
            Ok(true)
        })
        .catch_others(|_| Ok(false))
        .execute()
        .unwrap();
        
        // Memory Context Changes: If the subtransaction was successful, we release it.
        // If it failed, we roll back and release it. 
        // This ensures that any memory allocated during the subtransaction is properly cleaned up.
        if success {
            pg_sys::ReleaseCurrentSubTransaction();
        } else {
            pg_sys::RollbackAndReleaseCurrentSubTransaction();
        }

        // Restore outer context/owner regardless of success or failure
        pg_sys::MemoryContextSwitchTo(oldcontext);
        pg_sys::CurrentResourceOwner = oldowner;

        success
    }
}

#[pg_extern]
fn pg_lab_safe_execute_with_message(sql: &str) -> String {
    PgTryBuilder::new(|| -> Result<String, SpiError> {
        Spi::run(sql)?;
        Ok("Success".to_string())
    })
    .catch_when(PgSqlErrorCode::ERRCODE_DIVISION_BY_ZERO, |_| {
        Ok("failed: division by zero".to_string())
    })
    .catch_when(PgSqlErrorCode::ERRCODE_INVALID_TEXT_REPRESENTATION, |_| {
        Ok("failed: Invalid text representation error".to_string())
    })
    .catch_others(|_| Ok("failed: other error".to_string()))
    .execute()
    .unwrap()
}

#[pg_extern]
fn pg_lab_describe_spi_error(sql: &str) -> String {
    let result: Result<Option<i64>, SpiError> = Spi::get_one(sql);

    match result {
        Ok(Some(val)) => format!("success: {}", val),
        Ok(None) => "success: no rows".to_string(),
        Err(SpiError::InvalidPosition) => "spi error: tuple table out of bounds".to_string(),
        Err(SpiError::CursorNotFound(name)) => format!("spi error: cursor '{}' not found", name),
        Err(other) => format!("spi error: {:?}", other),
    }
}

#[pg_extern]
fn pg_lab_entries_where_clause(table_name: &str, column_name: &str, operator: &str, value: i64) -> i64 {
    // Step 1: operator whitelist check
    let safe_operator = match operator {
        "=" | ">" | "<" | "!=" | ">=" | "<=" => operator,
        _ => pgrx::error!("Invalid operator: {}", operator),
    };

    // Step 2: quote_ident table_name aur column_name
    let (Some(safe_table), Some(safe_column)) = Spi::get_two_with_args::<String, String>
                                                                    ("Select quote_ident($1), quote_ident($2)", &[table_name.into(), column_name.into()])
                                                                    .unwrap() else{
                                                                         pgrx::error!("Failed to quote identifiers");
                                                                    };

    // Step 3: build query with safe pieces, value still parameterized via $1
    let query = format!("SELECT count(*) FROM {} WHERE {} {} $1", safe_table, safe_column, safe_operator);

    Spi::get_one_with_args::<i64>(&query, &[value.into()])
        .unwrap()
        .unwrap_or(0)
}

#[pg_extern]
fn pg_lab_safe_order_by(table_name: &str, column_name: &str, direction: &str) -> TableIterator<'static, (
                                                                                                name!(id, i64),
                                                                                                name!(name, String),
                                                                                            )>
{
    let safe_direction = match direction.to_uppercase().as_str() {
        "ASC" | "DESC" => direction.to_uppercase(),
        _ => pgrx::error!("Invalid sort direction: {}", direction)  
    };  


    // Step 2: quote_ident table_name aur column_name
    let (Some(safe_table_name), Some(safe_column_name)) = Spi::get_two_with_args::<String, String>
                                                                    ("Select quote_ident($1), quote_ident($2)", &[table_name.into(), column_name.into()])
                                                                    .unwrap() else{
                                                                         pgrx::error!("Failed to quote identifiers");
                                                                    };

    let query = "Select Exists(select column_name from information_schema.columns 
                                where table_name = $1 and column_name = $2) " ;

    let column_exist = Spi::get_one_with_args::<bool>(query, 
                                            &[table_name.into(), column_name.into()])
                                            .unwrap()
                                            .unwrap_or(false);

    if !column_exist {
        pgrx::error!("Column '{}' does not exist in table '{}'", column_name, table_name);
    }

    let query = format!("select id, name from {} order by {} {}", safe_table_name, safe_column_name, safe_direction);

    let mut res = Vec::new();

    Spi::connect(|client|  {
        let tuples = client.select(&query, None, &[]).unwrap();

        for row in tuples{
            let id: i64 = row["id"].value()
                            .unwrap()
                            .unwrap();
            let name: String = row["name"].value()
                                    .unwrap()
                                    .unwrap();
            res.push((id, name));
        }
    });

    TableIterator::new(res.into_iter())
}

#[pg_extern]
fn pg_lab_multi_column_filter(table_name: &str, columns: Array<String>, values: Array<String>) -> i64 {

    let safe_table  = Spi::get_one_with_args::<String>(
        "Select quote_ident($1)", &[table_name.into()]
    ).unwrap().unwrap();

    let cols: Vec<String> = columns.iter().flatten().collect();
    let vals: Vec<String> = values.iter().flatten().collect();

    if cols.len() != vals.len() {
        pgrx::error!("columns and values must have the same length");
    }

    let mut safe_columns: Vec<String> = Vec::new();

    for col in &cols{
        let query = "Select Exists(select column_name from information_schema.columns where table_name = $1 and column_name = $2)";

        let column_exist: bool = Spi::get_one_with_args::<bool>(query, &[table_name.into(), col.clone().into()])
                                        .unwrap()
                                        .unwrap();
        
        if !column_exist {
            pgrx::error!("Column '{}' does not exist in table '{}'", col, table_name)
        }

        let safe_col = Spi::get_one_with_args::<String>("Select quote_ident($1)", &[col.clone().into()])
                            .unwrap()
                            .unwrap();

        safe_columns.push(safe_col);
    }

    let where_parts: Vec<String> = safe_columns.iter().enumerate().map(|(i, col)| format!("{}::text = ${}", col, i+1)).collect();

    let where_clause = where_parts.join(" AND ");

    let query = format!("Select count(*) from {} where {}", safe_table, where_clause);

    let args: Vec<_> = vals.iter().map(|v| v.clone().into()).collect();

    Spi::get_one_with_args::<i64>(&query, &args).unwrap().unwrap_or(0)
}

#[pg_extern]
fn pg_lab_compare_quoting(input: &str) -> String {
    let quoted_ident: String = Spi::get_one_with_args::<String>(
        "SELECT quote_ident($1)", &[input.into()]
    ).unwrap().unwrap();

    let quoted_literal: String = Spi::get_one_with_args::<String>(
        "SELECT quote_literal($1)", &[input.into()]
    ).unwrap().unwrap();

    format!("quote_ident: {} | quote_literal: {}", quoted_ident, quoted_literal)
}

#[pg_extern]
fn pg_lab_paginated_query(table_name: &str, page: i32, page_size: i32) -> TableIterator<'static, (
                                                                                        name!(id, i64),
                                                                                        name!(name, String)     
                                                                                    )>
{

    let table_exist_check = format!("Select Exists(select 1 from information_schema.columns where table_name = $1)");

    let table_exist = Spi::get_one_with_args::<bool>(&table_exist_check, &[table_name.into()]).unwrap().unwrap_or(false);

     if !table_exist{
        pgrx::error!("Table not found in the Schema");
    }

    if page < 1 || page_size < 1 {
       pgrx::error!("page and page_size must be positive integers");
    }


    let safe_table_name = Spi::get_one_with_args::<String>("Select quote_ident($1)", &[table_name.into()])
                                                                .unwrap()
                                                                .unwrap();

    let mut result = Vec::new();

    let offset: i32 = (page - 1) * page_size;

    let query = format!("Select id, name from {} ORDER BY id limit $1 OFFSET $2", safe_table_name);

    Spi::connect(|client| {
        let tuples = client.select(&query, None, &[page_size.into(), offset.into()]).unwrap();

        for row in tuples{
            let id = row["id"].value().unwrap().unwrap();
            let name = row["name"].value().unwrap().unwrap();

            result.push((id, name));
        }
    });

    TableIterator::new(result.into_iter())

}

#[pg_extern]
fn pg_lab_paginated_top_category_products(page: i32, page_size: i32, include_null_category: bool) -> TableIterator<'static, (
                                                                            name!(category, Option<String>),
                                                                            name!(name, String),
                                                                            name!(price, f64),
                                                                        )>
{
    if page < 1 || page_size < 1 {
       pgrx::error!("page and page_size must be positive integers");
    }

    let offset: i32 = (page - 1) * page_size;

    let mut result = Vec::new();

    Spi::connect(|client| {
        let max_prod_category_query = "SELECT category FROM products WHERE ($1 OR category IS NOT NULL) GROUP BY category ORDER BY count(*) DESC LIMIT 1";

        let max_prod_category_table = client.select(max_prod_category_query, None, &[include_null_category.into()]).unwrap();

        let category: Option<String> = match max_prod_category_table.first().get::<String>(1){
                                                                        Ok(Some(c)) => Some(c),
                                                                        Ok(None) if include_null_category => None,
                                                                        _ => pgrx::error!("no products found"),
                                                                    };
       
        let products = match category {
                                                Some(c) => {
                                                    let query = "Select category, name, price::float8 from products where category = $1 Order by price DESC limit $2 Offset $3";
                                                    let result = client.select(query, None, &[c.clone().into(), page_size.into(), offset.into()]).unwrap();
                                                    result
                                                }
                                                None => {
                                                    let query = "SELECT category, name, price::float8 FROM products WHERE category IS NULL ORDER BY price DESC LIMIT $1 OFFSET $2";
                                                    let result = client.select(query, None, &[page_size.into(), offset.into()]).unwrap();
                                                    result
                                                }
                                            };

        for prod in products{
            let category:Option<String> = prod["category"].value().unwrap();
            let name = prod["name"].value().unwrap().unwrap();
            let price = prod["price"].value().unwrap().unwrap_or(0.0);
            result.push((category, name, price));
        }
    });

    TableIterator::new(result.into_iter())
}






