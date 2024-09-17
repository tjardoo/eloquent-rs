use crate::Join;

pub(crate) fn format(joins: &[Join], sql: &mut String) -> String {
    for join in joins {
        sql.push(' ');

        sql.push_str(&join.join_type.to_string());

        sql.push(' ');
        sql.push_str(&join.table);
        sql.push_str(" ON ");
        sql.push_str(&join.left_hand);
        sql.push_str(" = ");
        sql.push_str(&join.right_hand);
    }

    sql.to_string()
}
