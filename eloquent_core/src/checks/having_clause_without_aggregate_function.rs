use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct HavingClauseWithoutAggregateFunction;

impl PerformChecks for HavingClauseWithoutAggregateFunction {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        for having in &builder.havings {
            for condition in &having.conditions {
                if !builder.selects.iter().any(|select| {
                    (select.format_column_name_without_alias() == condition.field
                        && select.function.is_some())
                        || select.alias == Some(condition.field.clone())
                }) {
                    return Err(EloquentError::HavingClauseWithoutAggregateFunction(
                        condition.field.clone(),
                    ));
                }
            }
        }

        Ok(())
    }
}
