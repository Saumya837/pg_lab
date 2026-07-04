use pgrx::prelude::*;

#[pg_extern]
fn pg_lab_array_sum(vals: Array<i32>) -> Option<i64>{
    let mut result : i64 = 0;
    let mut has_values = false;

    for val in vals.iter(){
        if let Some(v) = val{
            result += v as i64;
            has_values = true;
        };
    }
    if has_values { Some(result) } else { None }
}

#[pg_extern]
fn pg_lab_array_max(vals: Array<i32>) -> Option<i64>{
    let mut max_val : i64 = i64::MIN;
    let mut has_values = false;

    for val in vals.iter(){
        if let Some(v) = val{
            if v as i64 > max_val{
                max_val = v as i64;
            }
            has_values = true;
        };
    }
    if has_values { Some(max_val) } else { None }
}