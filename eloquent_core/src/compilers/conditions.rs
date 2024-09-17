use crate::{error::EloquentError, Condition, Logic, Operator, ToSql};

#[allow(clippy::borrowed_box)]
pub(crate) fn format<'a>(
    conditions: &'a [Condition],
    closures: &'a [(Logic, Vec<Condition>)],
    sql: &mut String,
    params: &mut Vec<&'a Box<(dyn ToSql + 'static)>>,
) -> Result<String, EloquentError> {
    if !conditions.is_empty() || !closures.is_empty() {
        sql.push_str(" WHERE ");

        let mut conditions_str = String::new();
        let mut first_condition = true;

        for (i, condition) in conditions.iter().enumerate() {
            if i > 0 {
                conditions_str.push_str(match condition.logic {
                    Logic::And => " AND ",
                    Logic::Or => " OR ",
                });
            }

            let condition_sql = condition.format_sql();

            conditions_str.push_str(&condition_sql);
            if !matches!(condition.operator, Operator::IsNull | Operator::IsNotNull) {
                params.extend(condition.values.iter());
            }

            first_condition = false;
        }

        for (logic, closure) in closures.iter() {
            if !first_condition {
                match logic {
                    Logic::And => conditions_str.push_str(" AND "),
                    Logic::Or => conditions_str.push_str(" OR "),
                }
            }

            conditions_str.push('(');
            for (i, condition) in closure.iter().enumerate() {
                if i > 0 {
                    conditions_str.push_str(match condition.logic {
                        Logic::And => " AND ",
                        Logic::Or => " OR ",
                    });
                }

                let condition_sql = condition.format_sql();

                conditions_str.push_str(&condition_sql);
                if !matches!(condition.operator, Operator::IsNull | Operator::IsNotNull) {
                    params.extend(condition.values.iter());
                }
            }
            conditions_str.push(')');
            first_condition = false;
        }

        sql.push_str(&conditions_str);
    }

    Ok(sql.to_string())
}
