use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct CannotApplyClauseOnInsert;

impl PerformChecks for CannotApplyClauseOnInsert {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        if builder.inserts.is_empty() {
            return Ok(());
        }

        if !builder.conditions.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnInsert(
                "WHERE".to_string(),
            ));
        }

        if !builder.group_by.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnInsert(
                "GROUP BY".to_string(),
            ));
        }

        if !builder.order_by.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnInsert(
                "ORDER BY".to_string(),
            ));
        }

        if !builder.havings.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnInsert(
                "HAVING".to_string(),
            ));
        }

        if !builder.joins.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnInsert("JOIN".to_string()));
        }

        if builder.limit.is_some() {
            return Err(EloquentError::CannotApplyClauseOnInsert(
                "LIMIT".to_string(),
            ));
        }

        if builder.offset.is_some() {
            return Err(EloquentError::CannotApplyClauseOnInsert(
                "OFFSET".to_string(),
            ));
        }

        Ok(())
    }
}
