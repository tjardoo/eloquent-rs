use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct HavingClauseWithoutAggregateFunction;

impl PerformChecks for HavingClauseWithoutAggregateFunction {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        for having in &builder.havings {
            let column = &having.column;

            if !builder.selects.iter().any(|select| {
                (&select.format_column_name_without_alias() == column && select.function.is_some())
                    || select.alias == Some(column.to_string())
            }) {
                return Err(EloquentError::HavingClauseWithoutAggregateFunction(
                    column.clone(),
                ));
            }
        }

        Ok(())
    }
}
