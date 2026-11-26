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

        if !builder.order_by.is_empty() {
            return Err(EloquentError::CannotApplyClauseOnDelete(
                "ORDER BY".to_string(),
            ));
        }

        if builder.limit.is_some() {
            return Err(EloquentError::CannotApplyClauseOnDelete(
                "LIMIT".to_string(),
            ));
        }

        if builder.offset.is_some() {
            return Err(EloquentError::CannotApplyClauseOnDelete(
                "OFFSET".to_string(),
            ));
        }

        if builder.paginate.is_some() {
            return Err(EloquentError::CannotApplyClauseOnDelete(
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

    #[test]
    fn test_cannot_apply_limit_on_delete() {
        let result = QueryBuilder::new()
            .table("flights")
            .delete()
            .r#where("origin", "AMS")
            .limit(10)
            .sql();

        assert_eq!(
            result,
            Err(EloquentError::CannotApplyClauseOnDelete(
                "LIMIT".to_string()
            ))
        );
    }

    #[test]
    fn test_cannot_apply_pagination_on_delete() {
        let result = QueryBuilder::new()
            .table("flights")
            .delete()
            .paginate("id", Some(100), 10)
            .sql();

        assert_eq!(
            result,
            Err(EloquentError::CannotApplyClauseOnDelete(
                "PAGINATE".to_string()
            ))
        );
    }

    #[test]
    fn test_delete_limit_is_not_dropped_when_validation_is_skipped() {
        let result = QueryBuilder::new()
            .table("flights")
            .delete()
            .r#where("origin", "AMS")
            .order_by_asc("id")
            .limit(10)
            .skip_validation()
            .sql();

        assert_eq!(
            result.unwrap(),
            "DELETE FROM flights WHERE origin = 'AMS' ORDER BY id ASC LIMIT 10"
        );
    }
}
