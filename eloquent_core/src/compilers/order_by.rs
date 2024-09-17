use crate::OrderColumn;

pub(crate) fn format(order_by: &[OrderColumn], sql: &mut String) -> String {
    if !order_by.is_empty() {
        sql.push_str(" ORDER BY ");
        order_by.iter().for_each(|order| {
            sql.push_str(&order.column);
            sql.push(' ');
            sql.push_str(&order.order.to_string());

            if order != order_by.last().unwrap() {
                sql.push_str(", ");
            }
        });
    }

    sql.to_string()
}
