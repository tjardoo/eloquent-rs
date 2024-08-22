use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct MultipleCrudActions;

impl PerformChecks for MultipleCrudActions {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        let mut crud_actions = 0;

        if builder.selects.len() > 1 {
            crud_actions += 1;
        }

        if builder.inserts.len() > 1 {
            crud_actions += 1;
        }

        if builder.updates.len() > 1 {
            crud_actions += 1;
        }

        if builder.delete {
            crud_actions += 1;
        }

        if crud_actions > 1 {
            return Err(EloquentError::MultipleCrudActions);
        }

        Ok(())
    }
}
