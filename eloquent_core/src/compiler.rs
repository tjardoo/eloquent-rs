use crate::{error::EloquentError, JoinType, Logic, Operator, QueryBuilder, ToSql};

pub fn build_statement(builder: QueryBuilder) -> Result<String, EloquentError> {
    builder.perform_checks()?;

    let mut sql = "SELECT ".to_string();
    let mut params: Vec<&Box<dyn ToSql>> = Vec::new();

    if builder.selects.is_empty() {
        sql.push('*');
    } else {
        sql.push_str(
            &builder
                .selects
                .iter()
                .map(|s| s.format_column_name())
                .collect::<Vec<String>>()
                .join(", "),
        );
    }

    sql.push_str(&format!(" FROM {}", builder.table));

    for join in builder.joins {
        sql.push(' ');

        sql.push_str(match join.join_type {
            JoinType::Inner => "JOIN",
            JoinType::Left => "LEFT JOIN",
            JoinType::Right => "RIGHT JOIN",
            JoinType::Full => "FULL JOIN",
        });

        sql.push(' ');
        sql.push_str(&join.table);
        sql.push_str(" ON ");
        sql.push_str(&join.left_hand);
        sql.push_str(" = ");
        sql.push_str(&join.right_hand);
    }

    if !builder.conditions.is_empty() || !builder.closures.is_empty() {
        sql.push_str(" WHERE ");

        let mut conditions_str = String::new();
        let mut first_condition = true;

        for (i, condition) in builder.conditions.iter().enumerate() {
            if i > 0 {
                conditions_str.push_str(match condition.logic {
                    Logic::And => " AND ",
                    Logic::Or => " OR ",
                });
            }

            let condition_sql = match &condition.operator {
                Operator::Equal => format!("{} = ?", condition.field),
                Operator::NotEqual => format!("{} != ?", condition.field),
                Operator::GreaterThan => format!("{} > ?", condition.field),
                Operator::GreaterThanOrEqual => format!("{} >= ?", condition.field),
                Operator::LessThan => format!("{} < ?", condition.field),
                Operator::LessThanOrEqual => format!("{} <= ?", condition.field),
                Operator::Like => format!("{} LIKE ?", condition.field),
                Operator::In | Operator::NotIn => {
                    let placeholders = vec!["?"; condition.values.len()].join(", ");
                    if condition.operator == Operator::In {
                        format!("{} IN ({})", condition.field, placeholders)
                    } else {
                        format!("{} NOT IN ({})", condition.field, placeholders)
                    }
                }
                Operator::IsNull => format!("{} IS NULL", condition.field),
                Operator::IsNotNull => format!("{} IS NOT NULL", condition.field),
            };

            conditions_str.push_str(&condition_sql);
            if !matches!(condition.operator, Operator::IsNull | Operator::IsNotNull) {
                params.extend(condition.values.iter());
            }

            first_condition = false;
        }

        for (logic, closure) in builder.closures.iter() {
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

                let condition_sql = format!("{} {} ?", condition.field, condition.operator);

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

    if !builder.group_by.is_empty() {
        sql.push_str(" GROUP BY ");
        sql.push_str(&builder.group_by.join(", "));
    }

    if !builder.havings.is_empty() {
        sql.push_str(" HAVING ");

        sql.push_str(
            &builder
                .havings
                .into_iter()
                .map(|clause| format!("{} {} {}", clause.column, clause.operator, clause.value))
                .collect::<Vec<String>>()
                .join(", "),
        );
    }

    if !builder.order_by.is_empty() {
        sql.push_str(" ORDER BY ");
        sql.push_str(&builder.order_by.join(", "));
    }

    if let Some(limit) = &builder.limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }

    if let Some(offset) = &builder.offset {
        sql.push_str(&format!(" OFFSET {}", offset));
    }

    let formatted_sql = sql.replace('?', "{}");

    let formatted_sql = params
        .iter()
        .map(|p| p.as_ref().to_sql())
        .fold(formatted_sql, |acc, val| acc.replacen("{}", &val, 1));

    Ok(formatted_sql)
}
