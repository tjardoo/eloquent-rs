use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct CannotApplyClauseOnDelete;

impl PerformChecks for CannotApplyClauseOnDelete {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        if !builder.delete {
            return Ok(());
        }

        if !builder.group_by.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnDelete(
                "GROUP BY".to_string(),
            ));
        }

        if !builder.havings.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnDelete(
                "HAVING".to_string(),
            ));
        }

        if !builder.joins.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnDelete("JOIN".to_string()));
        }

        Ok(())
    }
}
