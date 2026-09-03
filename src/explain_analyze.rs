use pgrx::prelude::*;
use pgrx::Json;

#[pg_extern]
fn pg_lab_query_plan_type(table_name: &str, col_a: &str, col_b: &str, row_id: i64) -> String {

    let (Some(safe_table), Some(safe_col_a), Some(safe_col_b)) = Spi::get_three_with_args::<String, String, String>
                                                                            ("Select quote_ident($1), quote_ident($2), quote_ident($3)",
                                                                                    &[table_name.into(), col_a.into(), col_b.into()]).unwrap() else{
                                                                                        pgrx::error!("Failed to quote identifier");
                                                                                    };
    
    let query = format!("EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON) Select {}::text, {}::text from {} where id = $1", safe_col_a, safe_col_b, safe_table);

    let result: Json  =  Spi::get_one_with_args::<Json>(&query, &[row_id.into()]).unwrap().unwrap();

    result.0.to_string()
}

#[pg_extern]
fn pg_lab_scan_summary(table_name: &str, col_a: &str, col_b: &str) -> TableIterator<'static, (
                                                                                                            name!(node_type, String),
                                                                                                            name!(actual_time_ms, f64),
                                                                                                            name!(shared_hit, i64),
                                                                                                            name!(shared_read, i64),
                                                                                                            name!(estimated_rows, i64),
                                                                                                            name!(actual_rows, i64),
                                                                                                        )> {
                                                                                                            
    let (Some(safe_table), Some(safe_col_a), Some(safe_col_b)) = Spi::get_three_with_args::<String, String, String>
                                                                            ("Select quote_ident($1), quote_ident($2), quote_ident($3)",
                                                                                    &[table_name.into(), col_a.into(), col_b.into()]).unwrap() else{
                                                                                        pgrx::error!("Failed to quote identifier");
                                                                                    };
    
    let query = format!("EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON) Select {}::text, {}::text from {}", safe_col_a, safe_col_b, safe_table);

    let mut  container = Vec::new();

    let result: Json  =  Spi::get_one_with_args::<Json>(&query, &[]).unwrap().unwrap();



    let plan = result.0[0]["Plan"].as_object().unwrap_or_else(|| {
        pgrx::error!("Failed to extract plan from EXPLAIN output");
    });

    let node_type: String = plan["Node Type"].as_str().unwrap_or("Unknown").to_string();

    let actual_time_ms: f64 = plan["Actual Total Time"].as_f64().unwrap_or(0.0).to_owned();

    let shared_hit: i64 = plan["Shared Hit Blocks"].as_i64().unwrap_or(0).to_owned();

    let shared_read: i64 = plan["Shared Read Blocks"].as_i64().unwrap_or(0).to_owned();
     
    let estimated_rows: i64 = plan["Plan Rows"].as_i64().unwrap_or(0).to_owned();

    let actual_rows: i64 = plan["Actual Rows"].as_i64().unwrap_or(0).to_owned();

    container.push((node_type, actual_time_ms, shared_hit, shared_read, estimated_rows, actual_rows));

    TableIterator::new(container.into_iter())
}


fn extract_scan_node(node: &serde_json::Value,
                     depth: i32,
                     results: &mut Vec<(i32, String, Option<String>, f64, i64)>)
{
    if let Some(children) = node["Plans"].as_array() {
        for child in children {
            extract_scan_node(child, depth + 1, results);
        }
    }

    let node_type: String = node["Node Type"].as_str().unwrap_or("Unknown").to_string();

    let relation_name: Option<String> = node["Relation Name"].as_str().map(|s| s.to_string());

    let actual_time_ms: f64 = node["Actual Total Time"].as_f64().unwrap_or(0.0);

    let actual_rows: i64 = node["Actual Rows"].as_i64().unwrap_or(0);

    results.push((depth, node_type, relation_name, actual_time_ms, actual_rows));
}

#[pg_extern]
fn pg_lab_scan_tree(sql: &str) -> TableIterator<'static, (
                                                            name!(depth, i32),                                
                                                            name!(node_type, String),
                                                            name!(relation_name, Option<String>),
                                                            name!(actual_time_ms, f64),
                                                            name!(actual_rows, i64),
                                                )>
{
    let explain_query = format!("EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON) {}", sql);

    let result: Json =  Spi::get_one::<Json>(&explain_query).unwrap().unwrap();

    let root_plan = &result.0[0]["Plan"];

    let mut results : Vec<(i32, String, Option<String>, f64, i64)> = Vec::new();

    extract_scan_node(root_plan, 0, &mut results);

    TableIterator::new(results.into_iter())
}







