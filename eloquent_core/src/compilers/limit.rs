use crate::Paginate;

pub(crate) fn format(limit: &Option<u64>, paginate: &Option<Paginate>, sql: &mut String) -> String {
    if let Some(paginate) = paginate {
        sql.push_str(&format!(" LIMIT {}", paginate.per_page));
    } else if let Some(limit) = &limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }

    sql.to_string()
}
