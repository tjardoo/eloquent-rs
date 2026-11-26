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

        if !builder.order_by.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnUpdate(
                "ORDER BY".to_string(),
            ));
        }

        if builder.limit.is_some() {
            return Err(EloquentError::CannotApplyClauseOnUpdate(
                "LIMIT".to_string(),
            ));
        }

        if builder.offset.is_some() {
            return Err(EloquentError::CannotApplyClauseOnUpdate(
                "OFFSET".to_string(),
            ));
        }

        if builder.paginate.is_some() {
            return Err(EloquentError::CannotApplyClauseOnUpdate(
                "PAGINATE".to_string(),
            ));
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

    #[test]
    fn test_cannot_apply_limit_on_update() {
        let result = QueryBuilder::new()
            .table("flights")
            .update("status", "cancelled")
            .r#where("origin", "AMS")
            .limit(1)
            .sql();

        assert_eq!(
            result,
            Err(EloquentError::CannotApplyClauseOnUpdate(
                "LIMIT".to_string()
            ))
        );
    }

    #[test]
    fn test_cannot_apply_order_by_on_update() {
        let result = QueryBuilder::new()
            .table("flights")
            .update("status", "cancelled")
            .order_by_asc("id")
            .sql();

        assert_eq!(
            result,
            Err(EloquentError::CannotApplyClauseOnUpdate(
                "ORDER BY".to_string()
            ))
        );
    }

    #[test]
    fn test_update_limit_is_not_dropped_when_validation_is_skipped() {
        let result = QueryBuilder::new()
            .table("flights")
            .update("status", "cancelled")
            .r#where("origin", "AMS")
            .order_by_asc("id")
            .limit(1)
            .skip_validation()
            .sql();

        assert_eq!(
            result.unwrap(),
            "UPDATE flights SET status = 'cancelled' WHERE origin = 'AMS' ORDER BY id ASC LIMIT 1"
        );
    }
}
