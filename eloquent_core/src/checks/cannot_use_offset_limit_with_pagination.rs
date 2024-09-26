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
}
