use crate::{builder::QueryBuilder, Logic, Operator, ToSql};

pub fn build_statement(builder: QueryBuilder) -> String {
    let mut sql = "SELECT ".to_string();

    if builder.selects.is_empty() {
        sql.push_str("*");
    } else {
        sql.push_str(&builder.selects.join(", "));
    }

    sql.push_str(&format!(" FROM {}", builder.table));

    let mut params: Vec<&Box<dyn ToSql>> = Vec::new();

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
                        format!("{}IN ({})", condition.field, placeholders)
                    } else {
                        format!("{}NOT IN ({})", condition.field, placeholders)
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
            }
            conditions_str.push(')');
            first_condition = false;
        }

        sql.push_str(&conditions_str);
    }

    let formatted_sql = sql.replace('?', "{}");

    let formatted_sql = params
        .iter()
        .map(|p| p.as_ref().to_sql())
        .fold(formatted_sql, |acc, val| acc.replacen("{}", &val, 1));

    formatted_sql
}
