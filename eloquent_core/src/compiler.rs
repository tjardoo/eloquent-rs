use crate::{
    builders::{
        delete::DeleteBuilder, insert::InsertBuilder, select::SelectBuilder, update::UpdateBuilder,
    },
    error::EloquentError,
    Action, JoinType, Logic, Operator, QueryBuilder, SqlBuilder, ToSql,
};

pub fn build_statement(builder: &QueryBuilder) -> Result<String, EloquentError> {
    if builder.enable_checks {
        builder.perform_checks()?;
    }

    let mut sql = String::new();
    let mut params: Vec<&Box<dyn ToSql>> = Vec::new();

    sql = match builder.get_action() {
        Action::Select => SelectBuilder::build(builder, &mut sql, &mut params)?,
        Action::Insert => InsertBuilder::build(builder, &mut sql, &mut params)?,
        Action::Update => UpdateBuilder::build(builder, &mut sql, &mut params)?,
        Action::Delete => DeleteBuilder::build(builder, &mut sql, &mut params)?,
    };

    let formatted_sql = sql.replace('?', "{}");

    let formatted_sql = params
        .iter()
        .map(|p| p.as_ref().to_sql())
        .fold(formatted_sql, |acc, val| acc.replacen("{}", &val, 1));

    if formatted_sql.contains("{}") {
        return Err(EloquentError::MissingPlaceholders);
    }

    Ok(formatted_sql)
}

pub(crate) fn add_selects(builder: &QueryBuilder, sql: &mut String) -> String {
    sql.push_str("SELECT ");

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

    sql.push_str(" FROM ");
    sql.push_str(builder.table.as_ref().unwrap());

    sql.to_string()
}

#[allow(clippy::borrowed_box)]
pub(crate) fn add_inserts<'a>(
    builder: &'a QueryBuilder,
    sql: &mut String,
    params: &mut Vec<&'a Box<(dyn ToSql + 'static)>>,
) -> String {
    sql.push_str("INSERT INTO ");
    sql.push_str(builder.table.as_ref().unwrap());
    sql.push_str(" (");

    sql.push_str(
        &builder
            .inserts
            .iter()
            .map(|insert| insert.column.clone())
            .collect::<Vec<String>>()
            .join(", "),
    );

    sql.push_str(") VALUES (");

    sql.push_str(
        &builder
            .inserts
            .iter()
            .map(|insert| {
                params.push(&insert.value);
                "?".to_string()
            })
            .collect::<Vec<String>>()
            .join(", "),
    );

    sql.push(')');

    sql.to_string()
}

#[allow(clippy::borrowed_box)]
pub(crate) fn add_updates<'a>(
    builder: &'a QueryBuilder,
    sql: &mut String,
    params: &mut Vec<&'a Box<(dyn ToSql + 'static)>>,
) -> String {
    sql.push_str("UPDATE ");
    sql.push_str(builder.table.as_ref().unwrap());
    sql.push_str(" SET ");

    sql.push_str(
        &builder
            .updates
            .iter()
            .map(|update| {
                params.push(&update.value);
                format!("{} = ?", update.column)
            })
            .collect::<Vec<String>>()
            .join(", "),
    );

    sql.to_string()
}

pub(crate) fn add_joins(builder: &QueryBuilder, sql: &mut String) -> String {
    for join in &builder.joins {
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

    sql.to_string()
}

#[allow(clippy::borrowed_box)]
pub(crate) fn add_conditions<'a>(
    builder: &'a QueryBuilder,
    sql: &mut String,
    params: &mut Vec<&'a Box<(dyn ToSql + 'static)>>,
) -> Result<String, EloquentError> {
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

    Ok(sql.to_string())
}

pub(crate) fn add_group_by(builder: &QueryBuilder, sql: &mut String) -> String {
    if !builder.group_by.is_empty() {
        sql.push_str(" GROUP BY ");
        sql.push_str(&builder.group_by.join(", "));
    }

    sql.to_string()
}

pub(crate) fn add_havings(
    builder: &QueryBuilder,
    sql: &mut String,
) -> Result<String, EloquentError> {
    if !builder.havings.is_empty() {
        sql.push_str(" HAVING ");

        let havings = &builder.havings;

        sql.push_str(
            &havings
                .iter()
                .map(|clause| format!("{} {} {}", clause.column, clause.operator, clause.value))
                .collect::<Vec<String>>()
                .join(", "),
        );
    }

    Ok(sql.to_string())
}

pub(crate) fn add_order_by(builder: &QueryBuilder, sql: &mut String) -> String {
    if !builder.order_by.is_empty() {
        sql.push_str(" ORDER BY ");
        builder.order_by.iter().for_each(|order| {
            sql.push_str(&order.column);
            sql.push(' ');
            sql.push_str(match order.order {
                crate::Order::Asc => "ASC",
                crate::Order::Desc => "DESC",
            });

            if order != builder.order_by.last().unwrap() {
                sql.push_str(", ");
            }
        });
    }

    sql.to_string()
}

pub(crate) fn add_limit_offset(builder: &QueryBuilder, sql: &mut String) -> String {
    if let Some(limit) = &builder.limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }

    if let Some(offset) = &builder.offset {
        sql.push_str(&format!(" OFFSET {}", offset));
    }

    sql.to_string()
}
