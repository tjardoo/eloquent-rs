use crate::{error::EloquentError, Having};

pub(crate) fn format(havings: &[Having], sql: &mut String) -> Result<String, EloquentError> {
    if !havings.is_empty() {
        sql.push_str(" HAVING ");

        let havings = &havings;

        sql.push_str(
            &havings
                .iter()
                .map(|clause| {
                    clause
                        .conditions
                        .iter()
                        .map(|condition| condition.format_sql())
                        .collect::<Vec<String>>()
                        .join(" AND ")
                })
                .collect::<Vec<String>>()
                .join(", "),
        );
    }

    Ok(sql.to_string())
}
