use crate::{error::EloquentError, Update};

pub(crate) fn format(
    table: &str,
    updates: &[Update],
    sql: &mut String,
) -> Result<String, EloquentError> {
    sql.push_str("UPDATE ");
    sql.push_str(table);
    sql.push_str(" SET ");

    let assignments = updates
        .iter()
        .map(|update| {
            Ok(format!(
                "{} = {}",
                update.column,
                update.value.to_sql_value()?
            ))
        })
        .collect::<Result<Vec<String>, EloquentError>>()?;

    sql.push_str(&assignments.join(", "));

    Ok(sql.to_string())
}
