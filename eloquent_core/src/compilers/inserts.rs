use std::collections::HashSet;

use crate::{error::EloquentError, InsertRow};

pub(crate) fn format(
    table: &str,
    inserts: &[InsertRow],
    sql: &mut String,
) -> Result<String, EloquentError> {
    let columns = insert_columns(inserts)?;

    sql.push_str("INSERT INTO ");
    sql.push_str(table);
    sql.push_str(" (");
    sql.push_str(&columns.join(", "));
    sql.push_str(") VALUES ");

    let mut rows = Vec::with_capacity(inserts.len());
    for row in inserts {
        let values = columns
            .iter()
            .map(|column| {
                row.value(column)
                    .ok_or(EloquentError::InconsistentInsertColumns)?
                    .to_sql_value()
            })
            .collect::<Result<Vec<String>, EloquentError>>()?;

        rows.push(format!("({})", values.join(", ")));
    }

    sql.push_str(&rows.join(", "));

    Ok(sql.to_string())
}

pub(crate) fn insert_columns(rows: &[InsertRow]) -> Result<Vec<&str>, EloquentError> {
    let columns: Vec<&str> = rows
        .first()
        .map(|row| {
            row.values
                .iter()
                .map(|(column, _)| column.as_str())
                .collect()
        })
        .unwrap_or_default();

    let mut seen = HashSet::new();
    if let Some(duplicate) = columns.iter().find(|column| !seen.insert(**column)) {
        return Err(EloquentError::DuplicatedColumnNames(duplicate.to_string()));
    }

    let consistent = rows.iter().all(|row| {
        row.values.len() == columns.len() && columns.iter().all(|column| row.has_column(column))
    });

    if !consistent {
        return Err(EloquentError::InconsistentInsertColumns);
    }

    Ok(columns)
}
