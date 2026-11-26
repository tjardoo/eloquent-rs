use crate::{error::EloquentError, Logic, Paginate, WhereClause};

pub(crate) fn format(
    conditions: &[WhereClause],
    paginate: &Option<Paginate>,
    sql: &mut String,
) -> Result<String, EloquentError> {
    let (filter, has_or) = format_clauses(conditions)?;

    let cursor = match paginate {
        Some(Paginate {
            column,
            last_id: Some(last_id),
            ..
        }) => Some(format!("{} > {}", column, last_id.to_sql_value()?)),
        _ => None,
    };

    match (cursor, filter.is_empty()) {
        (None, true) => {}
        (None, false) => {
            sql.push_str(" WHERE ");
            sql.push_str(&filter);
        }
        (Some(cursor), true) => {
            sql.push_str(" WHERE ");
            sql.push_str(&cursor);
        }
        (Some(cursor), false) => {
            sql.push_str(" WHERE ");
            sql.push_str(&cursor);
            sql.push_str(" AND ");

            // `cursor AND a OR b` would let `b` bypass the cursor
            if has_or {
                sql.push('(');
                sql.push_str(&filter);
                sql.push(')');
            } else {
                sql.push_str(&filter);
            }
        }
    }

    Ok(sql.to_string())
}

fn format_clauses(clauses: &[WhereClause]) -> Result<(String, bool), EloquentError> {
    let mut sql = String::new();
    let mut has_or = false;

    let mut push = |logic: &Logic, clause: &str| {
        if !sql.is_empty() {
            match logic {
                Logic::And => sql.push_str(" AND "),
                Logic::Or => {
                    has_or = true;
                    sql.push_str(" OR ");
                }
            }
        }

        sql.push_str(clause);
    };

    for clause in clauses {
        match clause {
            WhereClause::Condition(condition) => {
                push(&condition.logic, &condition.format_sql()?);
            }
            WhereClause::Group(group) => {
                let (nested, _) = format_clauses(&group.clauses)?;

                if !nested.is_empty() {
                    push(&group.logic, &format!("({})", nested));
                }
            }
        }
    }

    Ok((sql, has_or))
}
