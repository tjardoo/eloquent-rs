use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct OrderByWithoutSelectedOrAggregateFunction;

impl PerformChecks for OrderByWithoutSelectedOrAggregateFunction {
    fn perform_checks(builder: &QueryBuilder) -> Result<(), EloquentError> {
        if builder.selects.is_empty() {
            return Ok(());
        }

        for order_by in &builder.order_by {
            if !builder.selects.iter().any(|select| {
                &select.column == order_by
                    || select
                        .alias
                        .as_ref()
                        .map(|alias| alias == order_by)
                        .unwrap_or(false)
            }) {
                return Err(EloquentError::OrderByWithNonSelectedOrAggregateFunction(
                    order_by.clone(),
                ));
            }
        }

        Ok(())
    }
}
