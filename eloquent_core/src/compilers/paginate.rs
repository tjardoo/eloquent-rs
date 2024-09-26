use crate::Paginate;

pub(crate) fn format(paginate: &Option<Paginate>, sql: &mut String) -> String {
    if let Some(paginate) = paginate {
        sql.push_str(&format!(" ORDER BY {} ASC", paginate.column));

        sql.push_str(" LIMIT ");
        sql.push_str(&paginate.per_page.to_string());
    }

    sql.to_string()
}
