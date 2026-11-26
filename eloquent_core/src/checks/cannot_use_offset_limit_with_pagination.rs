use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct CannotUseOffsetLimitWithPagination;

impl PerformChecks for CannotUseOffsetLimitWithPagination {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        if builder.paginate.is_none() {
            return Ok(());
        }

        if builder.offset.is_some() {
            return Err(EloquentError::CannotUseOffsetLimitWithPagination(
                "OFFSET".to_string(),
            ));
        }

        if builder.limit.is_some() {
            return Err(EloquentError::CannotUseOffsetLimitWithPagination(
                "LIMIT".to_string(),
            ));
        }

        if !builder.order_by.is_empty() {
            return Err(EloquentError::CannotUseOffsetLimitWithPagination(
                "ORDER BY".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::EloquentError, QueryBuilder};

    #[test]
    fn test_cannot_use_offset_with_pagination() {
        let result = QueryBuilder::new()
            .table("departures")
            .paginate::<u64>("id", None, 10)
            .offset(100)
            .sql();

        match result {
            Err(EloquentError::CannotUseOffsetLimitWithPagination(clause)) => {
                assert_eq!(clause, "OFFSET")
            }
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }

    #[test]
    fn test_cannot_use_limit_with_pagination() {
        let result = QueryBuilder::new()
            .table("departures")
            .paginate::<u64>("id", None, 10)
            .limit(5)
            .sql();

        match result {
            Err(EloquentError::CannotUseOffsetLimitWithPagination(clause)) => {
                assert_eq!(clause, "LIMIT")
            }
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }

    #[test]
    fn test_cannot_use_order_by_with_pagination() {
        let result = QueryBuilder::new()
            .table("departures")
            .paginate("id", Some(1000), 10)
            .order_by_desc("departure_time")
            .sql();

        assert_eq!(
            result,
            Err(EloquentError::CannotUseOffsetLimitWithPagination(
                "ORDER BY".to_string()
            ))
        );
    }
}
