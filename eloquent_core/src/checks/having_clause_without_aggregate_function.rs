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

#[cfg(test)]
mod tests {
    use crate::{error::EloquentError, QueryBuilder};

    #[test]
    fn test_having_clause_without_aggregate_function() {
        let result: Result<_, EloquentError> = QueryBuilder::new()
            .table("flights")
            .having("origin", 300)
            .sql();

        match result {
            Err(EloquentError::HavingClauseWithoutAggregateFunction(column)) => {
                assert_eq!(column, "origin")
            }
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }
}
