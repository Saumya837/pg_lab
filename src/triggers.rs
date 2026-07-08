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