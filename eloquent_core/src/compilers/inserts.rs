use crate::{Inserts, ToSql};

#[allow(clippy::borrowed_box)]
pub(crate) fn format<'a>(
    table: &str,
    inserts: &'a Inserts,
    sql: &mut String,
    params: &mut Vec<&'a Box<(dyn ToSql + 'static)>>,
) -> String {
    sql.push_str("INSERT INTO ");
    sql.push_str(table);
    sql.push_str(" (");

    let columns: Vec<_> = inserts.keys().map(String::as_str).collect();
    sql.push_str(&columns.join(", "));

    sql.push_str(") VALUES ");

    let row_count = inserts.values().next().map_or(0, |values| values.len());

    let mut value_placeholders = vec![];
    for i in 0..row_count {
        let row_values: Vec<_> = columns
            .iter()
            .map(|column| {
                let values = inserts.get(*column).unwrap();
                params.push(&values[i]);
                "?".to_string()
            })
            .collect();

        value_placeholders.push(format!("({})", row_values.join(", ")));
    }

    sql.push_str(&value_placeholders.join(", "));

    sql.to_string()
}
