use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct OrderByWithoutSelectedOrAggregateFunction;

impl PerformChecks for OrderByWithoutSelectedOrAggregateFunction {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        // ordering by a non-selected column is only invalid with SELECT DISTINCT
        if !builder.distinct || builder.selects.is_empty() {
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

#[cfg(test)]
mod tests {
    use crate::{error::EloquentError, QueryBuilder};

    #[test]
    fn test_order_by_non_selected_column() {
        let result = QueryBuilder::new()
            .table("users")
            .select("name")
            .order_by_desc("created_at")
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT name FROM users ORDER BY created_at DESC"
        );
    }

    #[test]
    fn test_order_by_without_selected_or_aggregate_function_on_distinct() {
        let result = QueryBuilder::new()
            .table("flights")
            .select_distinct("destination")
            .order_by_asc("origin")
            .sql();

        match result {
            Err(EloquentError::OrderByWithNonSelectedOrAggregateFunction(column)) => {
                assert_eq!(column, "origin")
            }
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }

    #[test]
    fn test_order_by_selected_column_on_distinct() {
        let result = QueryBuilder::new()
            .table("flights")
            .select_distinct("destination")
            .order_by_asc("destination")
            .sql();

        assert_eq!(
            result.unwrap(),
            "SELECT DISTINCT destination FROM flights ORDER BY destination ASC"
        );
    }
}
