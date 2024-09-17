pub(crate) fn format(table: &str, sql: &mut String) -> String {
    sql.push_str("DELETE FROM ");
    sql.push_str(table);

    sql.to_string()
}
