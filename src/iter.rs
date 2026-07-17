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

#[pg_extern]
fn pg_lab_stats(vals: Array<i32>) -> TableIterator<'static, (name!(min, Option<i64>), name!(max, Option<i64>), 
                                                            name!(sum, Option<i64>), name!(avg, Option<f64>))> {
    let mut max_val : i64 = 0;
    let mut min_val : i64 = 0;
    let mut sum : i64 = 0;
    let mut count : i64 = 0;
    let mut has_values = false;

    for val in vals.iter(){
        if let Some(v) = val{
            if !has_values {
                max_val = v as i64;
                min_val = v as i64;
            } else if (v as i64) > max_val {
                max_val = v as i64;
            } else if (v as i64) < min_val{
                min_val = v as i64;
            }

            sum += v as i64;
            count += 1;
            has_values = true;
        }
    }

    let (min, max, sum_opt, avg) = if has_values {
        (
            Some(min_val),
            Some(max_val),
            Some(sum),
            Some(sum as f64 / count as f64),
        )
    } else {
        (None, None, None, None)
    };

    TableIterator::once((min, max, sum_opt, avg))
}
