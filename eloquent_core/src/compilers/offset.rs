pub(crate) fn format(offset: &Option<u64>, sql: &mut String) -> String {
    if let Some(offset) = &offset {
        sql.push_str(&format!(" OFFSET {}", offset));
    }

    sql.to_string()
}
