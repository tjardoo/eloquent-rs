use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct CannotApplyClauseOnDelete;

impl PerformChecks for CannotApplyClauseOnDelete {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        if !builder.delete {
            return Ok(());
        }

        if !builder.group_by.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnDelete(
                "GROUP BY".to_string(),
            ));
        }

        if !builder.havings.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnDelete(
                "HAVING".to_string(),
            ));
        }

        if !builder.joins.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnDelete("JOIN".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::EloquentError, QueryBuilder};

    #[test]
    fn test_cannot_apply_clause_on_delete() {
        let result = QueryBuilder::new()
            .table("flights")
            .join("airports", "flights.origin_airport", "airports.code")
            .delete()
            .sql();

        match result {
            Err(EloquentError::CannotApplyClauseOnDelete(clause)) => {
                assert_eq!(clause, "JOIN")
            }
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }
}
