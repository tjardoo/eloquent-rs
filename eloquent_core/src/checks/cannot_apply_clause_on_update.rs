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

#[cfg(test)]
mod tests {
    use crate::{error::EloquentError, QueryBuilder};

    #[test]
    fn test_cannot_apply_clause_on_update() {
        let result = QueryBuilder::new()
            .table("flights")
            .join("airports", "flights.origin_airport", "airports.code")
            .update("origin_airport", "AMS")
            .sql();

        match result {
            Err(EloquentError::CannotApplyClauseOnUpdate(clause)) => {
                assert_eq!(clause, "JOIN")
            }
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }
}
