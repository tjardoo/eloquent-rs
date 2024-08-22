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
