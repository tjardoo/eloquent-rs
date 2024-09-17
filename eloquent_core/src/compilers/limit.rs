pub(crate) fn format(limit: &Option<u64>, sql: &mut String) -> String {
    if let Some(limit) = &limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }

    sql.to_string()
}
