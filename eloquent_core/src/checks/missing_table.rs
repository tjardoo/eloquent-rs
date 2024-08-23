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
