use std::collections::HashSet;

use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct DuplicatedColumns;

impl PerformChecks for DuplicatedColumns {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        let mut seen = HashSet::new();

        for select in &builder.selects {
            if !seen.insert((&select.column, &select.function)) {
                return Err(EloquentError::DuplicatedColumnNames(select.column.clone()));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::EloquentError, QueryBuilder};

    #[test]
    fn test_duplicated_column_names() {
        let result = QueryBuilder::new()
            .table("flights")
            .select("origin")
            .select("origin")
            .sql();

        match result {
            Err(EloquentError::DuplicatedColumnNames(column)) => assert_eq!(column, "origin"),
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }
}
