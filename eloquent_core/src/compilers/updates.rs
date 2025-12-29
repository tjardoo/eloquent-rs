use crate::{ToSql, Update};

#[allow(clippy::borrowed_box)]
pub(crate) fn format<'a>(
    table: &str,
    updates: &'a [Update],
    sql: &mut String,
    params: &mut Vec<&'a Box<dyn ToSql + 'static>>,
) -> String {
    sql.push_str("UPDATE ");
    sql.push_str(table);
    sql.push_str(" SET ");

    sql.push_str(
        &updates
            .iter()
            .map(|update| {
                params.push(&update.value);
                format!("{} = ?", update.column)
            })
            .collect::<Vec<String>>()
            .join(", "),
    );

    sql.to_string()
}
