use crate::{error::EloquentError, PerformChecks, QueryBuilder};

pub struct MultipleCrudActions;

impl PerformChecks for MultipleCrudActions {
    fn check(builder: &QueryBuilder) -> Result<(), EloquentError> {
        let mut crud_actions = 0;

        if !builder.selects.is_empty() {
            crud_actions += 1;
        }

        if !builder.inserts.is_empty() {
            crud_actions += 1;
        }

        if !builder.updates.is_empty() {
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

#[cfg(test)]
mod tests {
    use crate::{error::EloquentError, QueryBuilder};

    #[test]
    fn test_multiple_crud_actions() {
        let result = QueryBuilder::new()
            .table("flights")
            .select("destination")
            .insert("origin", "JFK")
            .sql();

        match result {
            Err(EloquentError::MultipleCrudActions) => (),
            Err(_error) => panic!(),
            Ok(_value) => panic!(),
        }
    }
}
