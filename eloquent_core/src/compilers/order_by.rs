use crate::{OrderColumn, Paginate};

pub(crate) fn format(
    order_by: &[OrderColumn],
    paginate: &Option<Paginate>,
    sql: &mut String,
) -> String {
    let mut columns: Vec<String> = paginate
        .iter()
        .map(|paginate| format!("{} ASC", paginate.column))
        .collect();

    columns.extend(
        order_by
            .iter()
            .map(|order| format!("{} {}", order.column, order.order)),
    );

    if !columns.is_empty() {
        sql.push_str(" ORDER BY ");
        sql.push_str(&columns.join(", "));
    }

    sql.to_string()
}
