pub(crate) fn format(group_by: &[String], sql: &mut String) -> String {
    if !group_by.is_empty() {
        sql.push_str(" GROUP BY ");
        sql.push_str(&group_by.join(", "));
    }

    sql.to_string()
}
