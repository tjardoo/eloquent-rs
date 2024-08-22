use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct OrderByWithoutSelectedOrAggregateFunction;

impl PerformChecks for OrderByWithoutSelectedOrAggregateFunction {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        if builder.selects.is_empty() {
            return Ok(());
        }

        for order_by in &builder.order_by {
            if !builder.selects.iter().any(|select| {
                select.format_column_name_without_alias() == order_by.column
                    || select
                        .alias
                        .as_ref()
                        .map(|alias| alias == &order_by.column)
                        .unwrap_or(false)
            }) {
                return Err(EloquentError::OrderByWithNonSelectedOrAggregateFunction(
                    order_by.column.clone(),
                ));
            }
        }

        Ok(())
    }
}
