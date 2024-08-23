use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct GroupByWithoutSelectedOrAggregateFunction;

impl PerformChecks for GroupByWithoutSelectedOrAggregateFunction {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        for group_by in &builder.group_by {
            if !builder.selects.iter().any(|select| {
                &select.column == group_by
                    || select
                        .alias
                        .as_ref()
                        .map(|alias| alias == group_by)
                        .unwrap_or(false)
            }) {
                return Err(EloquentError::GroupByWithNonSelectedOrAggregateFunction(
                    group_by.clone(),
                ));
            }
        }

        Ok(())
    }
}
