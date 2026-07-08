use pgrx::prelude::*;

#[pg_trigger]
fn pg_lab_set_updated_at<'a>(
    trigger: &'a PgTrigger<'a>,
) -> Result<Option<PgHeapTuple<'a, AllocatedByRust>>, PgTriggerError> {
    let mut row = trigger
        .new()
        .unwrap()
        .into_owned();

    let now = Spi::get_one::<TimestampWithTimeZone>("SELECT now()")
        .unwrap()
        .unwrap();

    row.set_by_name("updated_at", now).unwrap();

    Ok(Some(row))
}

#[pg_trigger]
fn pg_lab_set_check_valid_salary<'a>(
    trigger: &'a PgTrigger<'a>,
) -> Result<Option<PgHeapTuple<'a, AllocatedByRust>>, PgTriggerError> {
    let row = trigger
        .new()
        .unwrap()
        .into_owned();

    let salary =  row.get_by_name::<AnyNumeric> ("salary").unwrap().unwrap();
    if salary < AnyNumeric::from(0){
        error!("Salary Cannot be negative")
    }
        
    Ok(Some(row))
}