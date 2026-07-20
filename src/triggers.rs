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

#[pg_trigger]
fn pg_lab_convert_name_upper_case<'a>(
    trigger: &'a PgTrigger<'a>,
) -> Result<Option<PgHeapTuple<'a, AllocatedByRust>>, PgTriggerError>{
    let mut row = trigger
                                                        .new()
                                                        .unwrap()
                                                        .into_owned();

    let name: String = row
                        .get_by_name("name")
                        .unwrap()
                        .unwrap();
    
    let mut words = Vec::new();


    for word in name.split_whitespace(){
        let cleaned: String = word
                        .chars()
                        .filter(|c| c.is_alphabetic())
                        .collect();

        if let Some(first) = cleaned.chars().next(){
            let capitalized: String = if first.is_lowercase() {
                first.to_uppercase().collect::<String>() + &cleaned[first.len_utf8()..]
            }
            else{
                cleaned
            };
            words.push(capitalized);
        };
    }

    let res = words.join(" ");

    row.set_by_name("name", res).unwrap();

    Ok(Some(row))
}

#[pg_trigger]
fn pg_lab_enter_log<'a>(
    trigger: &'a PgTrigger<'a>,
) -> Result<Option<PgHeapTuple<'a, AllocatedByRust>>, PgTriggerError> {
    let table_name = trigger.table_name()?.to_string();
    let operation = trigger.op().unwrap();

    let op_str = match operation {
        PgTriggerOperation::Insert => "INSERT",
        PgTriggerOperation::Update => "UPDATE",
        PgTriggerOperation::Delete => "DELETE",
        _ => "UNKNOWN",
    };

    let old_val: Option<String> =
        trigger.old().and_then(|row| row.get_by_name("name").ok().flatten());
    let new_val: Option<String> =
        trigger.new().and_then(|row| row.get_by_name("name").ok().flatten());

    Spi::connect_mut(|client| -> Result<(), pgrx::spi::SpiError> {
        client.update(
            "INSERT INTO audit_log (table_name, operation, old_value, new_value, changed_at)
            VALUES ($1, $2, $3, $4, now())",
            None,
            &[
                table_name.into(),
                op_str.into(),
                old_val.into(),
                new_val.into(),
            ],
        )?;

        Ok(())
    })
    .unwrap();

    match operation {
        PgTriggerOperation::Delete => Ok(trigger.old().map(|t| t.into_owned())),
        _ => Ok(trigger.new().map(|t| t.into_owned())),
    }
}

#[pg_trigger]
fn pg_lab_avoid_del_active_record<'a>(
    trigger: &'a PgTrigger<'a>
) -> Result<Option<PgHeapTuple<'a, AllocatedByRust>>, PgTriggerError> {
    
    let row = trigger.old().unwrap().into_owned();
    
    let status: Option<String> = row.get_by_name("status").unwrap();
    
    if status.as_deref() == Some("active") {
        error!("Cannot delete active record");
    }
    
    Ok(Some(row))  // allow delete if not active
}

