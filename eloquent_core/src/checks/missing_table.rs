use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct MissingTable;

impl PerformChecks for MissingTable {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        if builder.table.is_none() {
            return Err(EloquentError::MissingTable);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::EloquentError, QueryBuilder};

    #[test]
    fn test_missing_table() {
        let result = QueryBuilder::new().sql();

        match result {
            Err(EloquentError::MissingTable) => (),
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }
}
