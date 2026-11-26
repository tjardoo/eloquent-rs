use crate::{error::EloquentError, Having};

pub(crate) fn format(havings: &[Having], sql: &mut String) -> Result<String, EloquentError> {
    if !havings.is_empty() {
        let clauses = havings
            .iter()
            .flat_map(|having| having.conditions.iter())
            .map(|condition| condition.format_sql())
            .collect::<Result<Vec<String>, EloquentError>>()?;

        sql.push_str(" HAVING ");
        sql.push_str(&clauses.join(" AND "));
    }

    Ok(sql.to_string())
}
