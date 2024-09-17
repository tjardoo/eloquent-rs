use crate::{Insert, ToSql};

#[allow(clippy::borrowed_box)]
pub(crate) fn format<'a>(
    table: &str,
    inserts: &'a [Insert],
    sql: &mut String,
    params: &mut Vec<&'a Box<(dyn ToSql + 'static)>>,
) -> String {
    sql.push_str("INSERT INTO ");
    sql.push_str(table);
    sql.push_str(" (");

    sql.push_str(
        &inserts
            .iter()
            .map(|insert| insert.column.clone())
            .collect::<Vec<String>>()
            .join(", "),
    );

    sql.push_str(") VALUES (");

    sql.push_str(
        &inserts
            .iter()
            .map(|insert| {
                params.push(&insert.value);
                "?".to_string()
            })
            .collect::<Vec<String>>()
            .join(", "),
    );

    sql.push(')');

    sql.to_string()
}
