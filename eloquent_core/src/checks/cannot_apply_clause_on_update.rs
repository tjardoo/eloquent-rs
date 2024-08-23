use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct CannotApplyClauseOnUpdate;

impl PerformChecks for CannotApplyClauseOnUpdate {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        if builder.updates.is_empty() {
            return Ok(());
        }

        if !builder.group_by.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnUpdate(
                "GROUP BY".to_string(),
            ));
        }

        if !builder.havings.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnUpdate(
                "HAVING".to_string(),
            ));
        }

        if !builder.joins.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnUpdate("JOIN".to_string()));
        }

        Ok(())
    }
}
