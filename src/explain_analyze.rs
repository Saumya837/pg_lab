use pgrx::prelude::*;
use pgrx::Json;

#[pg_extern]
fn pg_lab_query_plan_type(table_name: &str, col_a: &str, col_b: &str, row_id: default!(Option<i64>, "NULL"),) -> String {

    let (Some(safe_table), Some(safe_col_a), Some(safe_col_b)) = Spi::get_three_with_args::<String, String, String>
                                                                            ("Select quote_ident($1), quote_ident($2), quote_ident($3)",
                                                                                    &[table_name.into(), col_a.into(), col_b.into()]).unwrap() else{
                                                                                        pgrx::error!("Failed to quote identifier");
                                                                                    };

    let sub_query: String = format!("Select {}::text, {}::text from {}", safe_col_a, safe_col_b, safe_table);

    let result: Json  =  match row_id{
        Some(row) => {
                         let sql = format!("{} WHERE id = $1", sub_query);
                        let query = format!("EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON) {}", sql);
                        Spi::get_one_with_args::<Json>(&query, &[row.into()]).unwrap().unwrap()
                    } ,
         
        None => {
                                let sql = format!("{}",sub_query);
                                let query = format!("EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON) {}", sql);
                                Spi::get_one_with_args::<Json>(&query, &[]).unwrap().unwrap()
                            }
    };
   

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
    let node_type: String = node["Node Type"].as_str().unwrap_or("Unknown").to_string();

    let relation_name: Option<String> = node["Relation Name"].as_str().map(|s| s.to_string());

    let actual_time_ms: f64 = node["Actual Total Time"].as_f64().unwrap_or(0.0);

    let actual_rows: i64 = node["Actual Rows"].as_i64().unwrap_or(0);

    results.push((depth, node_type, relation_name, actual_time_ms, actual_rows));

    if let Some(children) = node["Plans"].as_array() {
        for child in children {
            extract_scan_node(child, depth + 1, results);
        }
    }
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

fn find_children_time(node: &serde_json::Value) -> f64 {
    let child_time = match node["Plans"].as_array(){
       Some(children) => children.iter().map(|child| child["Actual Total Time"].as_f64().unwrap_or(0.0)).sum(),
       None => 0.0
    };
    child_time
}

fn find_max_time_node(node: &serde_json::Value, current_max: &mut (String, Option<String>, f64)) -> Option<String> {
    let node_type: String = node["Node Type"].as_str().unwrap_or("Unknown").to_string();
    let total_time: f64 = node["Actual Total Time"].as_f64().unwrap_or(0.0);
    let own_relation: Option<String> = node["Relation Name"].as_str().map(|s| s.to_string());

    let children_time: f64 = find_children_time(node);

    let self_time = total_time - children_time;

    let mut best_child_relation: Option<String> = None;
    if let Some(children) = node["Plans"].as_array() {
        for child in children {
            let child_relation = find_max_time_node(child, current_max);
            if best_child_relation.is_none() {
                best_child_relation = child_relation; 
            }
        }
    }

    let effective_relation = own_relation.or(best_child_relation);

    if self_time > current_max.2 {
        *current_max = (node_type, effective_relation.clone(), self_time);
    }

    effective_relation  // parent ko wapas de do
}

#[pg_extern]
fn pg_lab_find_bottleneck(sql: &str) -> String 
{
    let explain_analyze_query = format!("Explain (Analyze, Buffers, Format JSON) {}", sql);
    let result : Json = Spi::get_one::<Json>(&explain_analyze_query).unwrap().unwrap();
    let root_plan = &result.0[0]["Plan"];
    let mut curr_max= ("".to_string(), None, 0.0);

    find_max_time_node(root_plan, &mut curr_max);
    format!("The bottleneck is: {} (relation: {}) with self-time: {}", curr_max.0,  curr_max.1.unwrap_or("N/A".to_string()), curr_max.2)
}

#[pg_extern]
fn pg_lab_suggest_index(table_name: &str, column_name: &str) -> String {

   let table_exists= Spi::get_one_with_args::<bool>( "select Exists(Select 1 from information_schema.tables where table_name = $1)", &[table_name.into()]).unwrap().unwrap_or(false);

   if !table_exists {
        pgrx::error!("table {} doen't exist", table_name);
   }

   let col_exists = Spi::get_one_with_args::<bool>( "select Exists(Select 1 from information_schema.columns where table_name = $1 and column_name = $2)", &[table_name.into(), column_name.into()]).unwrap().unwrap_or(false);

    if !col_exists {
        pgrx::error!("column {} doen't exist", column_name);
    }

   let (Some(safe_table_name), Some(safe_column_name))= 
                                                Spi::get_two_with_args::<String, String>("Select quote_ident($1), quote_ident($2)", 
                                                    &[table_name.into(), column_name.into()]).unwrap() 
                                                    else{
                                                            pgrx::error!("Failed to quote identifiers");
                                                        };

    let query = format!("Select count(DISTINCT {})::float8 / count(*)::float8 from {}", safe_column_name, safe_table_name);

    let selectivity = Spi::get_one_with_args::<f64>(&query, &[]).unwrap().unwrap();

    if selectivity > 0.3 {
        format!("High selectivity ({}) -- index on {} likely helpful", selectivity, column_name)
    } else {
        format!("Low selectivity ({}) -- index on {} likely won't help much", selectivity, column_name)
    }
}

// #[pg_extern]
// fn pg_lab_cache_hit_ratio(sql: &str) -> String {

//     let query = format!("Explain (Analyze, BUFFERS, FORMAT JSON) ({}) ", sql);

//     let result: Json = Spi::get_one_with_args::<Json>(&query, &[]).unwrap().unwrap();


//     let node = result.0[0]["Plan"];


//     "".to_string()
// }




















