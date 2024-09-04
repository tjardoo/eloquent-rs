use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct GroupByWithoutSelectedOrAggregateFunction;

impl PerformChecks for GroupByWithoutSelectedOrAggregateFunction {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        for group_by in &builder.group_by {
            if !builder.selects.iter().any(|select| {
                &select.format_column_name_without_alias() == group_by
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

#[cfg(test)]
mod tests {
    use crate::{error::EloquentError, QueryBuilder};

    #[test]
    fn test_group_by_without_selected_or_aggregate_function() {
        let result = QueryBuilder::new()
            .table("flights")
            .group_by("origin")
            .sql();

        match result {
            Err(EloquentError::GroupByWithNonSelectedOrAggregateFunction(column)) => {
                assert_eq!(column, "origin")
            }
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }
}
