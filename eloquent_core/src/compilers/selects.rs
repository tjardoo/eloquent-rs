use crate::Select;

pub(crate) fn format(table: &str, selects: &[Select], sql: &mut String) -> String {
    sql.push_str("SELECT ");

    if selects.is_empty() {
        sql.push('*');
    } else {
        sql.push_str(
            &selects
                .iter()
                .map(|s| s.format_column_name())
                .collect::<Vec<String>>()
                .join(", "),
        );
    }

    sql.push_str(" FROM ");
    sql.push_str(table);

    sql.to_string()
}
